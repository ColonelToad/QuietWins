#!/usr/bin/env node

/**
 * Post-build script to bundle NLP service with production builds
 * Copies platform-specific NLP binary from resources/ to the bundle output directory (when present)
 */

const fs = require('fs');
const path = require('path');

const platform = process.platform; // win32 | darwin | linux
const binaryName = platform === 'win32' ? 'nlp_service.exe' : 'nlp_service';
const nlpBinary = path.join(__dirname, `../src-tauri/resources/${binaryName}`);
const bundleDir = path.join(__dirname, '../src-tauri/target/release/bundle');

// Only run this in production builds
if (process.env.TAURI_ENV_PROFILE !== 'release') {
  console.log('[post-build] Skipping NLP service bundling (not a release build)');
  process.exit(0);
}

if (!fs.existsSync(nlpBinary)) {
  console.warn('[post-build] NLP service binary not found; skipping copy. Path:', nlpBinary);
  process.exit(0);
}

// Copy to platform-specific bundle dirs when they exist (best-effort)
if (platform === 'win32') {
  const windowsDirs = ['msi', 'nsis'].map((d) => path.join(bundleDir, d));
  for (const dir of windowsDirs) {
    if (fs.existsSync(dir)) {
      const dest = path.join(dir, binaryName);
      fs.copyFileSync(nlpBinary, dest);
      console.log(`[post-build] Copied ${binaryName} to:`, dest);
    }
  }
} else if (platform === 'darwin') {
  const appDir = path.join(bundleDir, 'macos');
  if (fs.existsSync(appDir)) {
    const dest = path.join(appDir, binaryName);
    fs.copyFileSync(nlpBinary, dest);
    console.log(`[post-build] Copied ${binaryName} to:`, dest);
  }
} else if (platform === 'linux') {
  const linuxDir = path.join(bundleDir, 'deb');
  if (fs.existsSync(linuxDir)) {
    const dest = path.join(linuxDir, binaryName);
    fs.copyFileSync(nlpBinary, dest);
    console.log(`[post-build] Copied ${binaryName} to:`, dest);
  }
}

console.log('[post-build] Done');
