// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/utils/Pausable.sol";

/// @title  KoraRegistry
/// @notice Central registry for vets, pets, and medical records on the Kora App platform.
/// @dev    Inherits OpenZeppelin Pausable for emergency-stop functionality.
contract KoraRegistry is Pausable {
    // -------------------------------------------------------------------------
    // Constants — string length limits (issue #919)
    // -------------------------------------------------------------------------

    /// @notice Maximum byte length for short string fields (name, species, breed, birthday).
    uint256 public constant MAX_SHORT_LEN = 64;

    /// @notice Maximum byte length for long string fields (diagnosis, treatment, notes).
    uint256 public constant MAX_LONG_LEN  = 1000;

    /// @notice Maximum number of records scanned/returned in date-range query when limit is omitted.
    uint256 public constant MAX_QUERY_LIMIT = 100;

    /// @notice Maximum page size for paginated record-type queries (issue #227).
    uint256 public constant MAX_PAGE_SIZE = 50;

    // -------------------------------------------------------------------------
    // State
    // -------------------------------------------------------------------------

    /// @notice Address of the contract administrator.
    address public admin;

    /// @notice Category of a medical record.
    enum RecordType { Checkup, Vaccination, Surgery, LabResult, Other }

    /// @notice Registered veterinarian.
    /// @param vetAddress     On-chain address of the vet.
    /// @param licenseNumber  Vet's professional licence number (original casing).
    /// @param specialization Vet's area of specialization.
    /// @param isVerified     True when the admin has verified the vet.
    /// @param isRevoked      True when the admin has revoked the vet's access.
    struct Vet {
        address vetAddress;
        string  licenseNumber;
        string  specialization;
        bool    isVerified;
        bool    isRevoked;
    }

    /// @notice Registered pet.
    /// @param petId   Unique identifier assigned at registration.
    /// @param owner   Current owner's address.
    /// @param name    Pet's name.
    /// @param species Species (e.g. "Dog", "Cat").
    /// @param breed   Breed descriptor.
    /// @param birthday Date of birth string.
    /// @param active  False when the pet has been deactivated.
    /// @param pendingOwner Proposed recipient awaiting acceptance.
    struct Pet {
        uint256 petId;
        address owner;
        string  name;
        string  species;
        string  breed;
        string  birthday;
        bool    active;
        address pendingOwner;
    }

    /// @notice A single medical record entry for a pet.
    /// @param recordId   Unique record identifier.
    /// @param petId      Pet this record belongs to.
    /// @param vet        Address of the vet who created the record.
    /// @param recordType Category of the record.
    /// @param diagnosis  Diagnosis text.
    /// @param treatment  Treatment text.
    /// @param notes      Additional notes (may be empty).
    /// @param timestamp  Block timestamp at time of creation.
    struct MedicalRecord {
        uint256 recordId;
        uint256 petId;
        address vet;
        RecordType recordType;
        string  diagnosis;
        string  treatment;
        string  notes;
        uint256 timestamp;
    }

    uint256 private _petCounter;
    uint256 private _recordCounter;
    uint256 private _vetCount;

    mapping(address => Vet)      public vets;
    mapping(uint256 => Pet)      public pets;
    mapping(uint256 => MedicalRecord[]) private _petRecords;
    mapping(address => uint256[]) private _ownerPets;
    mapping(uint256 => uint256) private _recordPet;   // recordId => petId
    mapping(uint256 => uint256) private _recordIndex; // recordId => index in _petRecords[petId]
    mapping(bytes32 => address)  private _licenseToVet;

    // Ordered list of all ever-registered vet addresses (issue #926)
    address[] private _vetAddresses;

    // -------------------------------------------------------------------------
    // Events
    // -------------------------------------------------------------------------

    /// @notice Emitted when a new vet registers.
    /// @param vet           Address of the registering vet.
    /// @param licenseNumber Licence number supplied at registration.
    event VetRegistered(address indexed vet, string licenseNumber);

    /// @notice Emitted when a vet updates their specialization.
    /// @param vet            Address of the vet.
    /// @param specialization New specialization string.
    event VetSpecializationUpdated(address indexed vet, string specialization);

    /// @notice Emitted when the admin verifies a vet.
    /// @param vet Address of the verified vet.
    event VetVerified(address indexed vet);

    /// @notice Emitted when the admin revokes a vet.
    /// @param vet Address of the revoked vet.
    event VetRevoked(address indexed vet);

    /// @notice Emitted when a new pet is registered.
    /// @param petId ID assigned to the new pet.
    /// @param owner Address of the registering owner.
    event PetRegistered(uint256 indexed petId, address indexed owner);

    /// @notice Emitted when a pet is deactivated.
    /// @param petId ID of the deactivated pet.
    event PetDeactivated(uint256 indexed petId);

    /// @notice Emitted when a previously deactivated pet is reactivated.
    /// @param petId ID of the reactivated pet.
    event PetReactivated(uint256 indexed petId);

    /// @notice Emitted when a medical record is added for a pet.
    /// @param petId    ID of the pet.
    /// @param recordId ID assigned to the new record.
    /// @param vet      Address of the vet who added the record.
    event MedicalRecordAdded(uint256 indexed petId, uint256 indexed recordId, address indexed vet);
    /// @notice Emitted when a medical record is corrected.
    /// @dev    The original field values are preserved in the event log for full auditability.
    /// @param recordId         ID of the corrected record.
    /// @param petId            Pet the record belongs to.
    /// @param correctedBy      Address that performed the correction.
    /// @param originalDiagnosis Previous diagnosis text.
    /// @param originalTreatment Previous treatment text.
    /// @param originalNotes     Previous notes text.
    /// @param newDiagnosis      Updated diagnosis text.
    /// @param newTreatment      Updated treatment text.
    /// @param newNotes          Updated notes text.
    event MedicalRecordCorrected(
        uint256 indexed recordId,
        uint256 indexed petId,
        address indexed correctedBy,
        string  originalDiagnosis,
        string  originalTreatment,
        string  originalNotes,
        string  newDiagnosis,
        string  newTreatment,
        string  newNotes
    );

    /// @notice Emitted when the admin role is transferred.
    /// @param previousAdmin The outgoing admin address.
    /// @param newAdmin      The incoming admin address.
    event AdminTransferred(address indexed previousAdmin, address indexed newAdmin);
    event AdminTransferProposed(address indexed currentAdmin, address indexed pendingAdmin);

    event PetTransferInitiated(uint256 indexed petId, address indexed from, address indexed to);
    event PetTransferAccepted(uint256 indexed petId, address indexed from, address indexed to);
    event PetTransferCancelled(uint256 indexed petId, address indexed owner, address indexed pendingOwner);

    address public pendingAdmin;

    // -------------------------------------------------------------------------
    // Modifiers
    // -------------------------------------------------------------------------
    modifier onlyAdmin() {
        require(msg.sender == admin, "KoraRegistry: not admin");
        _;
    }

    modifier onlyPetOwner(uint256 petId) {
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        require(pets[petId].owner == msg.sender, "KoraRegistry: not pet owner");
        _;
    }

    // -------------------------------------------------------------------------
    // Vet functions
    // -------------------------------------------------------------------------

    /// @notice Registers the caller as a veterinarian.
    /// @param licenseNumber  Professional licence number (must be unique).
    /// @param specialization Area of specialization.
    function registerVet(string calldata licenseNumber, string calldata specialization) external whenNotPaused {
        require(vets[msg.sender].vetAddress == address(0), "KoraRegistry: vet already registered");
        require(bytes(licenseNumber).length > 0, "KoraRegistry: license required");
        require(bytes(licenseNumber).length <= MAX_SHORT_LEN, "KoraRegistry: license too long");
        require(bytes(specialization).length <= MAX_SHORT_LEN, "KoraRegistry: specialization too long");

        bytes32 licenseKey = keccak256(bytes(licenseNumber));
        require(_licenseToVet[licenseKey] == address(0), "KoraRegistry: license already registered");

        vets[msg.sender] = Vet({
            vetAddress: msg.sender,
            licenseNumber: licenseNumber,
            specialization: specialization,
            isVerified: false,
            isRevoked: false
        });
        _licenseToVet[licenseKey] = msg.sender;
        _vetAddresses.push(msg.sender);
        _vetCount++;

        emit VetRegistered(msg.sender, licenseNumber);
    }

    /// @notice Update the calling vet's own specialization.
    /// @param specialization New specialization string (1–128 bytes).
    function updateSpecialization(string calldata specialization) external whenNotPaused {
        require(vets[msg.sender].vetAddress == msg.sender, "KoraRegistry: not a registered vet");
        require(
            bytes(specialization).length > 0 && bytes(specialization).length <= 128,
            "KoraRegistry: invalid specialization length"
        );
        vets[msg.sender].specialization = specialization;
        emit VetSpecializationUpdated(msg.sender, specialization);
    }

    /// @notice Marks a registered vet as verified.
    /// @param vet Address of the vet to verify.
    function verifyVet(address vet) external onlyAdmin whenNotPaused {
        require(vets[vet].vetAddress != address(0), "KoraRegistry: vet not registered");
        require(!vets[vet].isRevoked, "KoraRegistry: vet is revoked");
        vets[vet].isVerified = true;
        emit VetVerified(vet);
    }

    /// @notice Revokes a registered vet's access.
    /// @param vet Address of the vet to revoke.
    function revokeVet(address vet) external onlyAdmin whenNotPaused {
        require(vets[vet].vetAddress != address(0), "KoraRegistry: vet not registered");
        vets[vet].isRevoked = true;
        vets[vet].isVerified = false;
        emit VetRevoked(vet);
    }

    /// @notice Returns the total number of vets ever registered.
    function vetCount() external view returns (uint256) {
        return _vetCount;
    }

    /// @notice Returns the address of a vet at a given index in the registration list.
    /// @param index Index into the ordered vet address list.
    function vetAddressAt(uint256 index) external view returns (address) {
        return _vetAddresses[index];
    }

    // -------------------------------------------------------------------------
    // Pet functions
    // -------------------------------------------------------------------------

    /// @notice Registers a new pet owned by the caller.
    /// @param name     Pet's name.
    /// @param species  Species (e.g. "Dog").
    /// @param breed    Breed descriptor.
    /// @param birthday Date of birth string.
    /// @return petId   The newly assigned pet ID.
    function registerPet(
        string calldata name,
        string calldata species,
        string calldata breed,
        string calldata birthday
    ) external whenNotPaused returns (uint256 petId) {
        require(bytes(name).length > 0, "KoraRegistry: name required");
        require(bytes(name).length <= MAX_SHORT_LEN, "KoraRegistry: name too long");
        require(bytes(species).length <= MAX_SHORT_LEN, "KoraRegistry: species too long");
        require(bytes(breed).length <= MAX_SHORT_LEN, "KoraRegistry: breed too long");
        require(bytes(birthday).length <= MAX_SHORT_LEN, "KoraRegistry: birthday too long");

        _petCounter++;
        petId = _petCounter;

        pets[petId] = Pet({
            petId: petId,
            owner: msg.sender,
            name: name,
            species: species,
            breed: breed,
            birthday: birthday,
            active: true,
            pendingOwner: address(0)
        });
        _ownerPets[msg.sender].push(petId);

        emit PetRegistered(petId, msg.sender);
    }

    /// @notice Deactivates a pet owned by the caller.
    /// @param petId ID of the pet to deactivate.
    function deactivatePet(uint256 petId) external onlyPetOwner(petId) whenNotPaused {
        require(petId > 0 && petId <= _petCounter, "KoraRegistry: pet does not exist");
        require(pets[petId].active, "KoraRegistry: already inactive");
        pets[petId].active = false;
        emit PetDeactivated(petId);
    }

    /// @notice Reactivates a previously deactivated pet owned by the caller.
    /// @param petId ID of the pet to reactivate.
    function reactivatePet(uint256 petId) external onlyPetOwner(petId) whenNotPaused {
        require(petId > 0 && petId <= _petCounter, "KoraRegistry: pet does not exist");
        require(!pets[petId].active, "KoraRegistry: already active");
        pets[petId].active = true;
        emit PetReactivated(petId);
    }

    /// @notice Returns all pet IDs owned by an address.
    /// @param owner Address to query.
    function getPetsByOwner(address owner) external view returns (uint256[] memory) {
        return _ownerPets[owner];
    }

    // -------------------------------------------------------------------------
    // Medical record functions
    // -------------------------------------------------------------------------

    /// @notice Adds a medical record for a pet. Caller must be a verified, non-revoked vet.
    /// @param petId      Pet the record belongs to.
    /// @param recordType Category of the record.
    /// @param diagnosis  Diagnosis text.
    /// @param treatment  Treatment text.
    /// @param notes      Additional notes (may be empty).
    /// @return recordId  The newly assigned record ID.
    function addMedicalRecord(
        uint256 petId,
        RecordType recordType,
        string calldata diagnosis,
        string calldata treatment,
        string calldata notes
    ) external whenNotPaused returns (uint256 recordId) {
        require(vets[msg.sender].vetAddress != address(0), "KoraRegistry: vet not registered");
        require(vets[msg.sender].isVerified, "KoraRegistry: vet not verified");
        require(!vets[msg.sender].isRevoked, "KoraRegistry: vet is revoked");
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        require(bytes(diagnosis).length > 0, "KoraRegistry: diagnosis required");
        require(bytes(diagnosis).length <= MAX_LONG_LEN, "KoraRegistry: diagnosis too long");
        require(bytes(treatment).length <= MAX_LONG_LEN, "KoraRegistry: treatment too long");
        require(bytes(notes).length <= MAX_LONG_LEN, "KoraRegistry: notes too long");

        _recordCounter++;
        recordId = _recordCounter;

        MedicalRecord memory record = MedicalRecord({
            recordId: recordId,
            petId: petId,
            vet: msg.sender,
            recordType: recordType,
            diagnosis: diagnosis,
            treatment: treatment,
            notes: notes,
            timestamp: block.timestamp
        });

        _petRecords[petId].push(record);
        _recordPet[recordId] = petId;
        _recordIndex[recordId] = _petRecords[petId].length - 1;

        emit MedicalRecordAdded(petId, recordId, msg.sender);
    }

    /// @notice Returns all medical records for a pet.
    /// @param petId ID of the pet.
    function getMedicalRecords(uint256 petId) external view returns (MedicalRecord[] memory) {
        return _petRecords[petId];
    }

    /// @notice Returns the number of medical records for a pet.
    /// @param petId ID of the pet.
    function getMedicalRecordCount(uint256 petId) external view returns (uint256) {
        return _petRecords[petId].length;
    }

    /// @notice Return a paginated page of medical records for a pet filtered by record type.
    /// @dev    Validates pet existence. `limit` is clamped to MAX_PAGE_SIZE.
    /// @param petId      ID of the pet.
    /// @param recordType Category to filter by.
    /// @param offset     Number of matching records to skip (0-based).
    /// @param limit      Maximum number of matching records to return (clamped to MAX_PAGE_SIZE).
    /// @return page      Slice of matching MedicalRecord structs for the requested page.
    function getPetRecordsByTypePaged(
        uint256 petId,
        RecordType recordType,
        uint256 offset,
        uint256 limit
    ) external view returns (MedicalRecord[] memory page) {
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        if (limit == 0) {
            return new MedicalRecord[](0);
        }
        if (limit > MAX_PAGE_SIZE) {
            limit = MAX_PAGE_SIZE;
        }

        MedicalRecord[] storage all = _petRecords[petId];
        uint256 total = all.length;

        // Count total matching records to determine page boundaries
        uint256 matchCount;
        for (uint256 i = 0; i < total; i++) {
            if (all[i].recordType == recordType) matchCount++;
        }

        if (offset >= matchCount) {
            return new MedicalRecord[](0);
        }

        uint256 end = offset + limit;
        if (end > matchCount) end = matchCount;
        uint256 size = end - offset;

        page = new MedicalRecord[](size);
        uint256 matched;
        uint256 j;
        for (uint256 i = 0; i < total && j < size; i++) {
            if (all[i].recordType == recordType) {
                if (matched >= offset) {
                    page[j] = all[i];
                    j++;
                }
                matched++;
            }
        }
    }

    /// @notice Correct an existing medical record.
    /// @dev    Only the vet who originally created the record, or the admin, may call this.
    ///         The original field values are emitted in MedicalRecordCorrected for auditability.
    /// @param recordId  The ID of the record to correct.
    /// @param diagnosis Updated diagnosis text (non-empty, ≤ MAX_LONG_LEN).
    /// @param treatment Updated treatment text (non-empty, ≤ MAX_LONG_LEN).
    /// @param notes     Updated notes (may be empty, ≤ MAX_LONG_LEN).
    function correctMedicalRecord(
        uint256 recordId,
        string calldata diagnosis,
        string calldata treatment,
        string calldata notes
    ) external whenNotPaused {
        uint256 petId = _recordPet[recordId];
        require(petId != 0, "KoraRegistry: record does not exist");

        uint256 index = _recordIndex[recordId];
        MedicalRecord storage record = _petRecords[petId][index];

        require(
            msg.sender == record.vet || msg.sender == admin,
            "KoraRegistry: not authorized to correct"
        );
        require(bytes(diagnosis).length > 0, "KoraRegistry: diagnosis required");
        require(bytes(diagnosis).length <= MAX_LONG_LEN, "KoraRegistry: diagnosis too long");
        require(bytes(treatment).length <= MAX_LONG_LEN, "KoraRegistry: treatment too long");
        require(bytes(notes).length <= MAX_LONG_LEN, "KoraRegistry: notes too long");

        emit MedicalRecordCorrected(
            recordId,
            petId,
            msg.sender,
            record.diagnosis,
            record.treatment,
            record.notes,
            diagnosis,
            treatment,
            notes
        );

        record.diagnosis = diagnosis;
        record.treatment = treatment;
        record.notes = notes;
    }

    // -------------------------------------------------------------------------
    // Pet transfer functions
    // -------------------------------------------------------------------------

    /// @notice Initiates transfer of a pet to a new owner.
    /// @param petId   ID of the pet to transfer.
    /// @param newOwner Address of the proposed new owner.
    function initiatePetTransfer(uint256 petId, address newOwner) external onlyPetOwner(petId) whenNotPaused {
        require(newOwner != address(0), "KoraRegistry: invalid new owner");
        require(newOwner != msg.sender, "KoraRegistry: cannot transfer to self");
        pets[petId].pendingOwner = newOwner;
        emit PetTransferInitiated(petId, msg.sender, newOwner);
    }

    /// @notice Accepts a pending pet transfer. Caller must be the pending owner.
    /// @param petId ID of the pet to accept.
    function acceptPetTransfer(uint256 petId) external whenNotPaused {
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        require(pets[petId].pendingOwner == msg.sender, "KoraRegistry: not pending owner");

        address previousOwner = pets[petId].owner;
        pets[petId].owner = msg.sender;
        pets[petId].pendingOwner = address(0);

        emit PetTransferAccepted(petId, previousOwner, msg.sender);
    }

    /// @notice Whether `petId` has been registered in the system.
    /// @param petId The pet to query.
    /// @return      True if the pet exists (active or deactivated), false if not registered.
    function petExists(uint256 petId) public view returns (bool) {
        return pets[petId].owner != address(0);
    }

    /// @notice Return all medical records for a pet.
    /// @param petId ID of the pet to query.
    /// @return      Full array of MedicalRecord structs for the pet.
    function getPetRecords(uint256 petId) external view returns (MedicalRecord[] memory) {
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        return _petRecords[petId];
    }

    /// @notice Return record IDs for `petId` whose timestamp falls within [startDate, endDate], bounded by `limit`.
    /// @param petId     The pet to query.
    /// @param startDate Lower bound Unix timestamp (inclusive).
    /// @param endDate   Upper bound Unix timestamp (inclusive).
    /// @param limit     Maximum number of matching records to return.
    /// @return ids      Array of matching record IDs.
    function getPetRecordsByDateRange(
        uint256 petId,
        uint256 startDate,
        uint256 endDate,
        uint256 limit
    ) public view returns (uint256[] memory ids) {
        require(pets[petId].owner != address(0), "KoraRegistry: pet does not exist");
        require(startDate <= endDate, "KoraRegistry: invalid date range");
        if (limit == 0) {
            return new uint256[](0);
        }
        MedicalRecord[] storage all = _petRecords[petId];
        uint256 total = all.length;

        uint256 count;
        for (uint256 i = 0; i < total && count < limit; i++) {
            uint256 ts = all[i].timestamp;
            if (ts >= startDate && ts <= endDate) count++;
        }

        ids = new uint256[](count);
        uint256 j;
        for (uint256 i = 0; i < total && j < count; i++) {
            uint256 ts = all[i].timestamp;
            if (ts >= startDate && ts <= endDate) {
                ids[j] = all[i].recordId;
                j++;
            }
        }
    }

    // -------------------------------------------------------------------------
    // Admin functions
    // -------------------------------------------------------------------------

    /// @notice Return a page of pet IDs owned by `owner`.
    /// @param owner  The owner address to query.
    /// @param offset Starting index (0-based).
    /// @param limit  Maximum number of items to return.
    /// @return page  Slice of the owner's pet ID array.
    function getPetsByOwnerPaged(
        address owner,
        uint256 offset,
        uint256 limit
    ) external view returns (uint256[] memory page) {
        if (limit > MAX_PAGE_LIMIT) {
            limit = MAX_PAGE_LIMIT;
        }
        uint256[] storage all = _ownerPets[owner];
        uint256 total = all.length;
        if (offset >= total || limit == 0) {
            return new uint256[](0);
        }
        if (limit > MAX_PAGE_SIZE) {
            limit = MAX_PAGE_SIZE;
        }
        uint256 end = offset + limit;
        if (end > total) end = total;
        uint256 size = end - offset;
        page = new uint256[](size);
        for (uint256 i = 0; i < size; i++) {
            page[i] = all[offset + i];
        }
    }

    /// @notice Return a page of medical records for `petId`.
    /// @param petId  The pet to query.
    /// @param offset Starting index (0-based).
    /// @param limit  Maximum number of items to return.
    /// @return page  Slice of the pet's medical record array.
    function getPetRecordsPaged(
        uint256 petId,
        uint256 offset,
        uint256 limit
    ) external view returns (MedicalRecord[] memory page) {
        if (limit > MAX_PAGE_LIMIT) {
            limit = MAX_PAGE_LIMIT;
        }
        MedicalRecord[] storage all = _petRecords[petId];
        uint256 total = all.length;
        if (offset >= total || limit == 0) {
            return new MedicalRecord[](0);
        }
        if (limit > MAX_PAGE_SIZE) {
            limit = MAX_PAGE_SIZE;
        }
        uint256 end = offset + limit;
        if (end > total) end = total;
        uint256 size = end - offset;
        page = new MedicalRecord[](size);
        for (uint256 i = 0; i < size; i++) {
            page[i] = all[offset + i];
        }
    }

    // -------------------------------------------------------------------------
    // Constructor
    // -------------------------------------------------------------------------

    /// @notice Return a page of Vet structs from the registered vet directory.
    /// @param offset Starting index into the vet address list (0-based).
    /// @param limit  Maximum number of Vet structs to return.
    /// @return page  Slice of Vet structs for the requested page.
    function getVets(uint256 offset, uint256 limit) external view returns (Vet[] memory page) {
        if (limit > MAX_PAGE_LIMIT) {
            limit = MAX_PAGE_LIMIT;
        }
        uint256 total = _vetAddresses.length;
        if (offset >= total || limit == 0) {
            return new Vet[](0);
        }
        if (limit > MAX_PAGE_SIZE) {
            limit = MAX_PAGE_SIZE;
        }
        uint256 end = offset + limit;
        if (end > total) end = total;
        uint256 size = end - offset;
        page = new Vet[](size);
        for (uint256 i = 0; i < size; i++) {
            page[i] = vets[_vetAddresses[offset + i]];
        }
    }
}
