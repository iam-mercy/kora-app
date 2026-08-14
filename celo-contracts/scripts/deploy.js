const hre = require("hardhat");

async function main() {
  const Factory = await hre.ethers.getContractFactory("KoraRegistry");
  const registry = await Factory.deploy();
  await registry.waitForDeployment();

  const address = await registry.getAddress();
  console.log(`KoraRegistry deployed to: ${address}`);
  console.log(`Network: ${hre.network.name}`);

  return address;
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
