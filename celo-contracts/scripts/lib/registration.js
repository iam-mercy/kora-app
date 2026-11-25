const { ethers } = require("ethers");

// Validates syntax + checksum (mixed-case must be a valid EIP-55 checksum)
// and rejects the zero address. Throws before any RPC use.
function validateContractAddress(value) {
  if (!value || !String(value).trim()) {
    throw new Error("Set CONTRACT_ADDRESS to the deployed KoraRegistry address");
  }
  const input = String(value).trim();
  let address;
  try {
    address = ethers.getAddress(input);
  } catch {
    throw new Error(`CONTRACT_ADDRESS is not a valid address or has a bad checksum: ${input}`);
  }
  if (address === ethers.ZeroAddress) {
    throw new Error("CONTRACT_ADDRESS must not be the zero address");
  }
  return address;
}

// Ensures the target has deployed contract code on the connected network.
async function assertContractDeployed(provider, address, networkName) {
  const code = await provider.getCode(address);
  if (!code || code === "0x") {
    throw new Error(`No contract code found at ${address} on network "${networkName}"`);
  }
}

module.exports = { validateContractAddress, assertContractDeployed };
