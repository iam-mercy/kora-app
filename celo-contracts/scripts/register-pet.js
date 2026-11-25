const hre = require("hardhat");
const { validateContractAddress, assertContractDeployed } = require("./lib/registration");

// Usage: CONTRACT_ADDRESS=0x... PET_NAME=Rex PET_SPECIES=Dog PET_BREED=Labrador PET_BIRTHDAY=2020-01-01 npx hardhat run scripts/register-pet.js --network <alfajores|celo|hardhat>
function findPetRegisteredEvent(receipt, registry) {
  const registryAddress = String(registry.target).toLowerCase();
  return receipt.logs
    .filter((log) => String(log.address).toLowerCase() === registryAddress)
    .map((log) => {
      try {
        return registry.interface.parseLog(log);
      } catch {
        return null;
      }
    })
    .find((parsed) => parsed && parsed.name === "PetRegistered");
}

async function main() {
  const contractAddress = validateContractAddress(process.env.CONTRACT_ADDRESS);

  const pet = {
    name: process.env.PET_NAME || "Rex",
    species: process.env.PET_SPECIES || "Dog",
    breed: process.env.PET_BREED || "Labrador",
    birthday: process.env.PET_BIRTHDAY || "2020-01-01",
  };

  const missing = Object.entries(pet)
    .filter(([, value]) => !value || !value.trim())
    .map(([key]) => key);
  if (missing.length > 0) {
    throw new Error(
      `Missing required pet registration parameters: ${missing.join(", ")}. ` +
        "Set PET_NAME, PET_SPECIES, PET_BREED, and PET_BIRTHDAY to non-empty values."
    );
  }

  console.log("Registering pet with parameters:", pet);

  await assertContractDeployed(hre.ethers.provider, contractAddress, hre.network.name);

  const registry = await hre.ethers.getContractAt("KoraRegistry", contractAddress);

  const tx = await registry.registerPet(pet.name, pet.species, pet.breed, pet.birthday);
  const receipt = await tx.wait();

  const event = findPetRegisteredEvent(receipt, registry);

  if (!event || !event.args) {
    throw new Error("PetRegistered event not found in transaction receipt");
  }

  console.log(`Pet registered with petId: ${event.args.petId}`);
}

if (require.main === module) {
  main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
  });
}

module.exports = { findPetRegisteredEvent };
