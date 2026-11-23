const hre = require("hardhat");
const fs = require("fs");
const { recordDeployment } = require("./lib/deployment-record");
const path = require("path");

async function main() {
  const KoraRegistry = await hre.ethers.getContractFactory("KoraRegistry");
  const registry = await KoraRegistry.deploy();
  await registry.waitForDeployment();

  const address = await registry.getAddress();
  console.log(`KoraRegistry deployed to: ${address}`);
  console.log(`Network: ${hre.network.name}`);

  const deploymentsDir = path.join(__dirname, "..", "deployments");
  if (!fs.existsSync(deploymentsDir)) {
    fs.mkdirSync(deploymentsDir, { recursive: true });
  }

  const { recordPath } = recordDeployment(deploymentsDir, {
    address,
    network: hre.network.name,
    timestamp: new Date().toISOString(),
  });
  console.log(`Deployment record written to ${path.relative(path.join(__dirname, ".."), recordPath)}`);
  console.log(`Latest pointer updated: deployments/${hre.network.name}.json`);

  return address;
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
