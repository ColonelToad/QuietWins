#!/usr/bin/env node

/**
 * Build script to create platform-specific NLP service binaries using PyInstaller
 * - Windows: nlp_service.exe
 * - macOS/Linux: nlp_service
 * Run this before building the Tauri app for production on the target platform.
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const nlpServicePy = path.join(__dirname, '../src-tauri/nlp_service.py');
const resourcesDir = path.join(__dirname, '../src-tauri/resources');
const platform = process.platform; // win32, darwin, linux
const binaryName = platform === 'win32' ? 'nlp_service.exe' : 'nlp_service';
const outputBinary = path.join(resourcesDir, binaryName);

console.log(`[build-nlp] Target platform detected: ${platform}`);
console.log('[build-nlp] Checking if PyInstaller is installed...');

try {
  execSync('pyinstaller --version', { stdio: 'inherit' });
} catch {
  console.error('[build-nlp] PyInstaller not found. Install with: pip install pyinstaller');
  process.exit(1);
}

console.log('[build-nlp] Creating resources directory...');
if (!fs.existsSync(resourcesDir)) {
  fs.mkdirSync(resourcesDir, { recursive: true });
}

console.log(`[build-nlp] Building ${binaryName} with PyInstaller...`);
try {
  // Build nlp_service.py directly (no spaCy model inclusion)
  const cmd = `pyinstaller --distpath "${resourcesDir}" --workpath build --name nlp_service --console "${nlpServicePy}"`;
  execSync(cmd, { stdio: 'inherit' });
  console.log(`[build-nlp] Successfully created ${outputBinary}`);
} catch (error) {
  console.error('[build-nlp] Failed to build NLP service exe:', error.message);
  process.exit(1);
}

console.log('[build-nlp] Done!');
