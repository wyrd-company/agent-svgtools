#!/usr/bin/env node
// Launcher shim: exec the platform binary placed by install.js and forward
// argv, stdio, and the exit code (preserving the agent-svgtools 0/1/2 contract).

"use strict";

const path = require("path");
const fs = require("fs");
const { spawnSync } = require("child_process");

const binName = process.platform === "win32" ? "agent-svgtools.exe" : "agent-svgtools";
const binPath = path.join(__dirname, binName);

if (!fs.existsSync(binPath)) {
  console.error(
    "agent-svgtools: native binary not found. Reinstall the package so the " +
      "postinstall step can download it, or `cargo install agent-svgtools`."
  );
  process.exit(1);
}

const result = spawnSync(binPath, process.argv.slice(2), { stdio: "inherit" });
if (result.error) {
  console.error(`agent-svgtools: ${result.error.message}`);
  process.exit(1);
}
process.exit(result.status === null ? 1 : result.status);
