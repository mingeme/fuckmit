#!/usr/bin/env node

"use strict";

import binLinks from "bin-links";
import fs from "fs";
import fetch from "node-fetch";
import path from "path";
import { extract } from "tar";
import zlib from "zlib";

const ARCH_MAPPING = {
  x64: "x86_64",
  arm64: "aarch64",
};

const PLATFORM_MAPPING = {
  darwin: "darwin",
  linux: "linux",
  win32: "windows",
};

const arch = ARCH_MAPPING[process.arch];
const platform = PLATFORM_MAPPING[process.platform];

// Read package.json
const readPackageJson = async () => {
  const contents = await fs.promises.readFile(path.join(process.cwd(), "package.json"));
  return JSON.parse(contents);
};

// Build the download url from package.json
const getDownloadUrl = (packageJson) => {
  const pkgName = packageJson.name;
  const version = packageJson.version;
  const repo = packageJson.repository;
  const url = `https://github.com/${repo}/releases/download/v${version}/${pkgName}-${platform}-${arch}-${version}.tar.gz`;
  return url;
};

const errGlobal = `Installing fuckmit as a global module is not supported.
Please use one of the supported package managers: https://github.com/mingeme/fuckmit#readme
`;
const errUnsupported = `Installation is not supported for ${process.platform} ${process.arch}`;

/**
 * Reads the configuration from application's package.json,
 * downloads the binary from package url and stores at
 * ./bin in the package's root.
 *
 *  See: https://docs.npmjs.com/files/package.json#bin
 */
async function main() {
  const yarnGlobal = JSON.parse(
    process.env.npm_config_argv || "{}"
  ).original?.includes("global");
  if (process.env.npm_config_global || yarnGlobal) {
    throw errGlobal;
  }
  if (!arch || !platform) {
    throw errUnsupported;
  }

  // Read from package.json and prepare for the installation.
  const pkg = await readPackageJson();
  if (platform === "windows") {
    // Update bin path in package.json
    pkg.bin[pkg.name] += ".exe";
  }

  // Set up paths for binary installation
  const binPath = pkg.bin[pkg.name];
  const binDir = path.dirname(path.join(process.cwd(), binPath));
  await fs.promises.mkdir(binDir, { recursive: true });

  // Download the binary.
  const url = getDownloadUrl(pkg);
  console.info("Downloading", url);
  const resp = await fetch(url);
  if (!resp.ok) {
    throw new Error(`Failed to download binary: ${resp.status} ${resp.statusText}`);
  }

  // Then, decompress the binary -- we will first Un-GZip, then we will untar.
  const ungz = zlib.createGunzip();
  const binName = path.basename(binPath);

  // Extract all files from the tarball to the bin directory
  const untar = extract({ cwd: binDir });

  // Pipe the data to the ungz stream.
  resp.body.pipe(ungz);

  // Pipe the data to the untar stream.
  ungz.pipe(untar);

  // Wait for the untar stream to finish.
  await new Promise((resolve, reject) => {
    untar.on("error", reject);
    untar.on("end", resolve);
  });

  // Use bin-links for npm/yarn
  await binLinks({
    path: path.resolve("."),
    pkg: { ...pkg, bin: { [pkg.name]: binPath } },
  });

  console.info("Install fuckmit successfully");
}

await main();
