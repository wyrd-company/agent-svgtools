#!/usr/bin/env node
// Postinstall: download the platform-correct agent-svgtools binary from the
// matching GitHub Release and unpack it next to the launcher shim.
//
// The release assets are produced by .github/workflows/cd.yml as
//   agent-svgtools-<version>-<target>.tar.gz   (unix)
//   agent-svgtools-<version>-<target>.zip      (windows)
// each with a sibling .sha256 we verify after download.

"use strict";

const fs = require("fs");
const os = require("os");
const path = require("path");
const https = require("https");
const crypto = require("crypto");
const { spawnSync } = require("child_process");

const REPO = "wyrd-company/agent-trace";
const version = require("./package.json").version;

// Map Node's platform/arch to a Rust target triple + archive extension.
const TARGETS = {
  "darwin/x64": { target: "x86_64-apple-darwin", ext: "tar.gz" },
  "darwin/arm64": { target: "aarch64-apple-darwin", ext: "tar.gz" },
  "linux/x64": { target: "x86_64-unknown-linux-musl", ext: "tar.gz" },
  "linux/arm64": { target: "aarch64-unknown-linux-musl", ext: "tar.gz" },
  "win32/x64": { target: "x86_64-pc-windows-msvc", ext: "zip" },
};

function selectTarget() {
  const key = `${process.platform}/${process.arch}`;
  const t = TARGETS[key];
  if (!t) {
    throw new Error(
      `unsupported platform ${key}. Build from source via \`cargo install agent-svgtools\`.`
    );
  }
  return t;
}

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    const get = (u, redirects) => {
      https
        .get(u, { headers: { "User-Agent": "agent-svgtools-installer" } }, (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
            if (redirects <= 0) return reject(new Error("too many redirects"));
            res.resume();
            return get(res.headers.location, redirects - 1);
          }
          if (res.statusCode !== 200) {
            res.resume();
            return reject(new Error(`download failed (${res.statusCode}) for ${u}`));
          }
          res.pipe(file);
          file.on("finish", () => file.close(() => resolve()));
        })
        .on("error", (err) => {
          fs.rm(dest, { force: true }, () => reject(err));
        });
    };
    get(url, 5);
  });
}

async function fetchText(url) {
  const tmp = path.join(os.tmpdir(), `agent-svgtools-${Date.now()}.txt`);
  await download(url, tmp);
  const text = fs.readFileSync(tmp, "utf8");
  fs.rmSync(tmp, { force: true });
  return text;
}

function sha256(file) {
  const hash = crypto.createHash("sha256");
  hash.update(fs.readFileSync(file));
  return hash.digest("hex");
}

function extract(archive, destDir, ext) {
  // `tar` (bsdtar on macOS/Windows, GNU tar on Linux) handles both .tar.gz
  // and .zip, and ships with every supported platform.
  const args =
    ext === "zip" ? ["-xf", archive, "-C", destDir] : ["-xzf", archive, "-C", destDir];
  const r = spawnSync("tar", args, { stdio: "inherit" });
  if (r.status !== 0) {
    throw new Error(`failed to extract ${archive}`);
  }
}

async function main() {
  const { target, ext } = selectTarget();
  const base = `https://github.com/${REPO}/releases/download/${version}`;
  const asset = `agent-svgtools-${version}-${target}.${ext}`;
  const url = `${base}/${asset}`;

  const binDir = path.join(__dirname, "bin");
  fs.mkdirSync(binDir, { recursive: true });
  const archive = path.join(binDir, asset);

  console.log(`agent-svgtools: downloading ${asset}`);
  await download(url, archive);

  // Verify the checksum if the .sha256 sidecar is present.
  try {
    const expected = (await fetchText(`${url}.sha256`)).trim().split(/\s+/)[0];
    const actual = sha256(archive);
    if (expected && expected !== actual) {
      throw new Error(`checksum mismatch: expected ${expected}, got ${actual}`);
    }
  } catch (e) {
    console.warn(`agent-svgtools: checksum not verified (${e.message})`);
  }

  extract(archive, binDir, ext);
  fs.rmSync(archive, { force: true });

  const binName = process.platform === "win32" ? "agent-svgtools.exe" : "agent-svgtools";
  const binPath = path.join(binDir, binName);
  if (!fs.existsSync(binPath)) {
    throw new Error("archive did not contain the agent-svgtools binary");
  }
  if (process.platform !== "win32") {
    fs.chmodSync(binPath, 0o755);
  }
  console.log("agent-svgtools: installed");
}

main().catch((err) => {
  console.error(`agent-svgtools: install failed: ${err.message}`);
  process.exit(1);
});
