# Building Quiet Wins with Bundled NLP Service

This guide explains how to build and distribute Quiet Wins with the NLP service bundled.

## Prerequisites

Ensure you have Python and the required dependencies installed:
```bash
pip install pyinstaller
pip install -r src-tauri/requirements.txt
python -m spacy download en_core_web_sm
```

## Development

For development, the NLP service is started separately:
```bash
npm run tauri:dev
```

This runs `scripts/start-nlp.js` which starts the Python NLP service before launching the app.

## Production Build (Local)

To create a production executable with the bundled NLP service:

```bash
npm run tauri:build
```

This will:
1. Build the NLP service as a standalone exe using PyInstaller (`scripts/build-nlp.js`)
2. Place it in `src-tauri/resources/nlp_service.exe` (Windows) or `src-tauri/resources/nlp_service` (macOS/Linux)
3. Build the frontend
4. Create the Tauri app with the NLP service bundled
5. Run post-build script (optional, for additional processing)

The resulting executable will be in `src-tauri/target/release/bundle/`.

## CI/CD Automated Builds

The GitHub Actions workflow (`.github/workflows/release.yml`) automatically builds the app with the NLP service bundled when pushing to the `main` branch. The workflow:

1. Sets up Python 3.11
2. Installs PyInstaller and dependencies from `src-tauri/requirements.txt`
3. Downloads the spaCy language model (`en_core_web_sm`)
4. Builds the NLP service binary using `npm run build-nlp`
5. Verifies the binary was created
6. Runs the Tauri build which automatically bundles the resources directory
7. Uploads artifacts (MSI, NSIS for Windows; DMG for macOS; AppImage for Linux)

## How It Works

### Development (`npm run tauri:dev`)
- `scripts/start-nlp.js` starts the Python NLP service separately
- Tauri launches the dev server
- Both processes communicate via HTTP on `localhost:8000`

### Production (`npm run tauri:build` or CI/CD)
- `scripts/build-nlp.js` uses PyInstaller to create `nlp_service.exe` (Windows) or `nlp_service` (macOS/Linux)
- The binary is placed in `src-tauri/resources/`
- The `resources` directory is bundled by Tauri (configured in `tauri.conf.json`)
- When users run the built app, the Rust backend launches the bundled NLP service as a subprocess
- The NLP service runs independently but is now part of the distribution

## What Gets Bundled

- The main Tauri app executable
- The bundled `nlp_service.exe` or `nlp_service` (Python runtime + dependencies included)
- Icons and other assets
- All files in the `resources` directory

When users download and run the app, everything works out of the box—no separate Python installation needed.

## Troubleshooting

**"PyInstaller not found"**
```bash
pip install pyinstaller
```

**"spaCy model not found"**
```bash
python -m spacy download en_core_web_sm
```

**NLP service not starting**
Check the debug logs in the app. The backend will log if it successfully starts the bundled NLP service. The Rust code looks for the binary in the resource directory and will print an error if not found.

**Exe is too large**
The Python runtime and dependencies make `nlp_service.exe` ~100MB. This is normal for bundled Python apps. You can optimize by:
- Using `pyinstaller --onefile --optimize 2` for more aggressive optimization
- Removing unused dependencies from `nlp_service.py`

## Building for Different Platforms

The build script (`scripts/build-nlp.js`) automatically detects the platform:
- **Windows**: Creates `nlp_service.exe`
- **macOS/Linux**: Creates `nlp_service` (no extension)

The Rust code also auto-detects and looks for the appropriate binary name.

