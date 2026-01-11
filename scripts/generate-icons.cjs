// Usage: node scripts/generate-icons.js
// Requires: npm install sharp png2icons

import sharp from 'sharp';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import png2icons from 'png2icons';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const variants = [
  { name: 'icon-light', svg: '../src-tauri/icons/icon-light.svg' },
  { name: 'icon-dark', svg: '../src-tauri/icons/icon-dark.svg' },
  { name: 'icon-warm', svg: '../src-tauri/icons/icon-warm.svg' }
];

const sizes = [16, 32, 48, 64, 128, 256, 512, 1024];

(async () => {
  for (const variant of variants) {
    // Generate PNGs
    for (const size of sizes) {
      const outPng = path.join(__dirname, `../src-tauri/icons/${variant.name}-${size}x${size}.png`);
      await sharp(path.join(__dirname, variant.svg))
        .resize(size, size)
        .png()
        .toFile(outPng);
      console.log(`Created ${outPng}`);
    }
    
    // Read 1024px PNG for ICNS/ICO generation
    const png1024Path = path.join(__dirname, `../src-tauri/icons/${variant.name}-1024x1024.png`);
    const pngBuffer = fs.readFileSync(png1024Path);
    
    // Create ICNS (macOS)
    const icnsBuffer = png2icons.createICNS(pngBuffer, png2icons.BILINEAR, 0);
    if (icnsBuffer) {
      fs.writeFileSync(
        path.join(__dirname, `../src-tauri/icons/${variant.name}.icns`),
        icnsBuffer
      );
      console.log(`Created ${variant.name}.icns`);
    }
    
    // Create ICO (Windows)
    const icoBuffer = png2icons.createICO(pngBuffer, png2icons.BILINEAR, 0, false);
    if (icoBuffer) {
      fs.writeFileSync(
        path.join(__dirname, `../src-tauri/icons/${variant.name}.ico`),
        icoBuffer
      );
      console.log(`Created ${variant.name}.ico`);
    }
  }
})();
