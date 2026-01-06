# Building Quiet Wins with Bundled NLP Service

This guide explains how to build and distribute Quiet Wins with the NLP service bundled.

## Prerequisites

Ensure PyInstaller is installed:
```bash
pip install pyinstaller
```

## Development

For development, the NLP service is started separately:
```bash
npm run tauri:dev
```

This runs `scripts/start-nlp.js` which starts the Python NLP service before launching the app.

## Production Build

To create a production executable with the bundled NLP service:

```bash
npm run tauri:build
```

This will:
1. Build the NLP service as a standalone exe using PyInstaller (`scripts/build-nlp.js`)
2. Place it in `src-tauri/resources/nlp_service.exe`
3. Build the frontend
4. Create the Tauri app with the NLP service bundled

The resulting executable will be in `src-tauri/target/release/bundle/`.

## How It Works

### Development (`npm run tauri:dev`)
- `scripts/start-nlp.js` starts the Python NLP service separately
- Tauri launches the dev server
- Both processes communicate via HTTP on `localhost:8000`

### Production (`npm run tauri:build`)
- `scripts/build-nlp.js` uses PyInstaller to create `nlp_service.exe`
- The exe is bundled as a resource in `tauri.conf.json`
- When users run the built app, the Rust backend launches `nlp_service.exe` as a subprocess
- The NLP service runs independently but is now part of the distribution

## What Gets Bundled

- The main Tauri app executable
- The bundled `nlp_service.exe` (Python runtime + dependencies included)
- Icons and other assets

When users download and run the app, everything works out of the box—no separate Python installation needed.

## Troubleshooting

**"PyInstaller not found"**
```bash
pip install pyinstaller
```

**NLP service not starting**
Check the debug logs in the app. The backend will log if it successfully starts the bundled NLP service.

**Exe is too large**
The Python runtime and dependencies make `nlp_service.exe` ~100MB. This is normal for bundled Python apps. You can optimize by:
- Using `pyinstaller --onefile --optimize 2` for more aggressive optimization
- Removing unused dependencies from `nlp_service.py`

## Building for Different Platforms

Currently this is set up for Windows. For macOS/Linux:
- Update the resource name in `tauri.conf.json` (e.g., `nlp_service` instead of `nlp_service.exe`)
- The build script should auto-detect the platform
- Or create separate scripts for each platform
