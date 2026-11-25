const fs = require("fs");
const path = require("path");

// Write JSON to a temp file in the same directory, then rename into place.
// A failure leaves any previous complete file untouched.
function writeJsonAtomic(filePath, data, { flag } = {}) {
  const json = JSON.stringify(data, null, 2);
  const tmp = `${filePath}.${process.pid}.${Date.now()}.tmp`;
  try {
    fs.writeFileSync(tmp, json);
    if (flag === "wx" && fs.existsSync(filePath)) {
      throw new Error(`Refusing to overwrite existing file: ${filePath}`);
    }
    fs.renameSync(tmp, filePath);
  } catch (error) {
    try {
      fs.rmSync(tmp, { force: true });
    } catch {}
    throw error;
  }
}

// Writes an immutable per-deployment record under deployments/<network>/ and
// updates the deployments/<network>.json "latest" pointer atomically.
function recordDeployment(deploymentsDir, { address, network, timestamp }) {
  const artifact = { address, network, timestamp };
  const historyDir = path.join(deploymentsDir, network);
  fs.mkdirSync(historyDir, { recursive: true });

  const safeTs = timestamp.replace(/[:.]/g, "-");
  const recordPath = path.join(historyDir, `${safeTs}-${address}.json`);
  writeJsonAtomic(recordPath, artifact, { flag: "wx" });

  const latestPath = path.join(deploymentsDir, `${network}.json`);
  writeJsonAtomic(latestPath, artifact);
  return { recordPath, latestPath };
}

module.exports = { writeJsonAtomic, recordDeployment };
