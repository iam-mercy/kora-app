const { expect } = require("chai");
const { ethers } = require("hardhat");
const { findPetRegisteredEvent } = require("../scripts/register-pet");

// Uses a minimal ABI so the check does not depend on the full contract compiling.
describe("register-pet script event filtering", function () {
  const iface = new ethers.Interface([
    "event PetRegistered(uint256 indexed petId, address indexed owner)",
  ]);
  const registryAddress = "0x1111111111111111111111111111111111111111";
  const registry = { target: registryAddress, interface: iface };
  const makeLog = (address, petId) => {
    const { topics, data } = iface.encodeEventLog("PetRegistered", [petId, ethers.ZeroAddress]);
    return { address, topics, data };
  };

  it("ignores a foreign PetRegistered event emitted ahead of the real one", function () {
    const foreign = makeLog("0x000000000000000000000000000000000000dEaD", 99n);
    const real = makeLog(registryAddress.toUpperCase().replace("0X", "0x"), 1n);

    expect(findPetRegisteredEvent({ logs: [foreign] }, registry)).to.equal(undefined);
    const found = findPetRegisteredEvent({ logs: [foreign, real] }, registry);
    expect(found.args.petId).to.equal(1n);
  });
});
