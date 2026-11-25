const { expect } = require("chai");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { ethers, network } = require("hardhat");
const { recordDeployment, writeJsonAtomic } = require("../scripts/lib/deployment-record");
const { validateContractAddress, assertContractDeployed } = require("../scripts/lib/registration");

describe("deployment record", () => {
  let dir;
  beforeEach(() => { dir = fs.mkdtempSync(path.join(os.tmpdir(), "dep-")); });
  afterEach(() => fs.rmSync(dir, { recursive: true, force: true }));

  it("retains both receipts across two deployments and updates latest", () => {
    const a = { address: "0x" + "1".repeat(40), network: "hardhat", timestamp: "2024-01-01T00:00:00.000Z" };
    const b = { address: "0x" + "2".repeat(40), network: "hardhat", timestamp: "2024-01-02T00:00:00.000Z" };
    const r1 = recordDeployment(dir, a);
    const r2 = recordDeployment(dir, b);
    expect(JSON.parse(fs.readFileSync(r1.recordPath)).address).to.equal(a.address);
    expect(JSON.parse(fs.readFileSync(r2.recordPath)).address).to.equal(b.address);
    expect(fs.readdirSync(path.join(dir, "hardhat"))).to.have.length(2);
    expect(JSON.parse(fs.readFileSync(r2.latestPath)).address).to.equal(b.address);
  });

  it("preserves previous artifact when a write fails", () => {
    const file = path.join(dir, "x.json");
    writeJsonAtomic(file, { ok: 1 });
    const circular = {}; circular.self = circular;
    expect(() => writeJsonAtomic(file, circular)).to.throw();
    expect(JSON.parse(fs.readFileSync(file))).to.deep.equal({ ok: 1 });
    expect(fs.readdirSync(dir)).to.deep.equal(["x.json"]);
  });
});

describe("registration target validation", () => {
  it("rejects malformed, bad-checksum, and zero addresses", () => {
    expect(() => validateContractAddress("nope")).to.throw(/not a valid address/);
    const good = ethers.getAddress("0x" + "ab".repeat(20));
    const bad = good.slice(0, 3) + good.slice(3).replace(/[a-f]/i, (c) => c === c.toUpperCase() ? c.toLowerCase() : c.toUpperCase());
    expect(() => validateContractAddress(bad)).to.throw(/not a valid address/);
    expect(() => validateContractAddress(ethers.ZeroAddress)).to.throw(/zero address/);
    expect(validateContractAddress(good)).to.equal(good);
  });

  it("rejects an externally owned address before any transaction", async () => {
    const [signer] = await ethers.getSigners();
    const before = await ethers.provider.getTransactionCount(signer.address);
    let err;
    try { await assertContractDeployed(ethers.provider, signer.address, network.name); } catch (e) { err = e; }
    expect(err.message).to.include(signer.address).and.include(network.name);
    expect(await ethers.provider.getTransactionCount(signer.address)).to.equal(before);
  });

  it("accepts a deployed contract", async () => {
    const target = "0x" + "cd".repeat(20);
    await ethers.provider.send("hardhat_setCode", [target, "0x6001600055"]);
    await assertContractDeployed(ethers.provider, target, network.name);
  });
});
