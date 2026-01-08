# Quiet Wins

A personal journaling app for macOS to track your daily wins and achievements. Built with Tauri, SvelteKit, and Rust.

## Features

- 🎯 **Quick Win Logging**: Log your daily achievements with a global shortcut (Cmd+Alt+Shift+W)
- 🗂️ **Smart Tagging**: Automatic tag suggestions using NLP and rule-based systems
- 📊 **Win Visualization**: Interactive D3 tag graph showing relationships between your wins
- 📈 **Stats & Recaps**: View 7-day and 30-day summaries with top tags and averages
- 🗑️ **Soft Delete**: 48-hour trash with easy recovery for accidentally deleted wins
- ✏️ **Undo/Redo**: Full editing history with undo/redo support
- 🔒 **Privacy Lock**: Optional password protection for your journal
- 🎨 **Themes**: Warm and Cool color schemes
- 🔔 **Notifications**: Daily and weekly recap reminders
- ♿ **Accessible**: ARIA labels, keyboard navigation, skip links

---

## Downloads

You can download pre-built executables for each platform:

- **Windows Installer (EXE):**  
  [tauri-app_0.1.0_x64-setup.exe](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_x64-setup.exe)
- **Windows Installer (MSI):**  
  [tauri-app_0.1.0_x64_en-US.msi](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_x64_en-US.msi)
- **macOS (Intel DMG):**  
  [tauri-app_0.1.0_x64.dmg](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_x64.dmg)
- **macOS (Apple Silicon DMG):**  
  [tauri-app_0.1.0_aarch64.dmg](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_aarch64.dmg)
- **macOS (Apple Silicon app tar.gz):**  
  [tauri-app_aarch64.app.tar.gz](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_aarch64.app.tar.gz)
- **macOS (Intel app tar.gz):**  
  [tauri-app_x64.app.tar.gz](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_x64.app.tar.gz)
- **Linux (AppImage):**  
  [tauri-app_0.1.0_amd64.AppImage](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_amd64.AppImage)
- **Linux (DEB):**  
  [tauri-app_0.1.0_amd64.deb](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app_0.1.0_amd64.deb)
- **Linux (RPM):**  
  [tauri-app-0.1.0-1.x86_64.rpm](https://github.com/ColonelToad/QuietWins/releases/latest/download/tauri-app-0.1.0-1.x86_64.rpm)

> **Note:** If you don't see a download for your OS, you can build from source using the instructions below.

---

## Prerequisites

Before you begin, ensure you have the following installed:

### macOS Requirements
- **macOS**: 10.15 (Catalina) or later
- **Xcode Command Line Tools**: Install with `xcode-select --install`
- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Version 18 or later from [nodejs.org](https://nodejs.org/)
- **pnpm or npm**: Package manager (npm comes with Node.js, or install pnpm with `npm install -g pnpm`)

### Verify Installation
```bash
# Check versions
node --version    # Should be 18+
rustc --version   # Should be 1.70+
cargo --version   # Comes with Rust
npm --version     # Comes with Node.js
```

---

## Building for Production

### Build the App
```bash
# Build the production app bundle
npm run tauri build
```

This creates:
- **DMG Installer**: `src-tauri/target/release/bundle/dmg/Quiet Wins_<version>_aarch64.dmg` (Apple Silicon)
- **DMG Installer**: `src-tauri/target/release/bundle/dmg/Quiet Wins_<version>_x64.dmg` (Intel)
- **App Bundle**: `src-tauri/target/release/bundle/macos/Quiet Wins.app`

### Install the App
1. Double-click the `.dmg` file
2. Drag "Quiet Wins" to your Applications folder
3. Open from Applications or Spotlight

### First Launch
On first launch, macOS may show a security warning. To allow the app:
1. Go to **System Preferences > Security & Privacy**
2. Click **"Open Anyway"** for Quiet Wins
3. Or right-click the app and select **"Open"**

---

## Building for Each OS

**Windows:**
Run on Windows 10/11 with Rust and Node.js installed:
```bash
npm run tauri build
```
Installer will be in `src-tauri/target/release/bundle/msi/` and `src-tauri/target/release/bundle/windows/`.

**macOS:**
You must build on a Mac (Apple Silicon or Intel). Clone the repo, install Rust and Node.js, then:
```bash
npm run tauri build
```
Installer will be in `src-tauri/target/release/bundle/dmg/` and `src-tauri/target/release/bundle/macos/`.

**Linux:**
Build on a Linux machine or VM:
```bash
npm run tauri build
```
Installer will be in `src-tauri/target/release/bundle/appimage/` and `src-tauri/target/release/bundle/linux/`.

---

### Quick Start
1. **Log a Win**: Press `Cmd+Alt+Shift+W` anywhere to open the quick input window
2. **Type your win**: Describe what you accomplished today
3. **Add tags** (optional): Tags are auto-suggested, or add your own
4. **Save**: Press Enter or click "Add Win"

### Features

#### Tray Menu
- Click the tray icon (🏆) for quick access:
  - **Log Win**: Open quick input window
  - **View Log**: See all your wins
  - **Quit**: Exit the app

#### Main Views
- **Log View**: See all wins with editing, undo/redo, and trash
- **Graph View**: Interactive D3 visualization of tag relationships
- **Recap View**: 7-day and 30-day summaries with stats
- **Settings**: Customize theme, notifications, privacy, and more

#### Keyboard Shortcuts
- `Cmd+Alt+Shift+W`: Quick log win (global)
- `Escape`: Close modal windows
- `Cmd+Z`: Undo (in edit mode)
- `Cmd+Shift+Z`: Redo (in edit mode)

#### Soft Delete & Trash
- Deleted wins go to **Trash** for 48 hours
- Click **🗑️ Trash** button to view deleted wins
- Click **↻ Restore** to recover a win
- After 48 hours, wins are permanently deleted

---

## Troubleshooting

### App Won't Open
- **Check Security Settings**: System Preferences > Security & Privacy
- **Right-click**: Try right-click > Open instead of double-click

### Build Fails
- **Update Rust**: `rustup update`
- **Clean Build**: `cargo clean` in `src-tauri/`, then rebuild
- **Check Node Version**: Must be 18+

### Database Issues
- **Reset Database**: Delete `~/Library/Application Support/com.quietwins.app/quietwins.sqlite`
- **Check Permissions**: Ensure app has disk access in System Preferences

### Notifications Not Working
- **Check Permissions**: System Preferences > Notifications > Quiet Wins
- **Enable in Settings**: Open app settings and toggle notifications on

---

## Configuration

### Customization
- **Theme**: Settings > Theme (Warm/Cool)
- **Notifications**: Settings > Notification Time and Frequency
- **Privacy Lock**: Settings > Enable Privacy Lock with password
- **Shortcuts**: Settings > Global Shortcut (default: Cmd+Alt+Shift+W)

### Advanced
Edit `src-tauri/tauri.conf.json` for:
- App name and bundle identifier
- Window size and position
- System tray icon
- Build configuration

---

## Contributing

This is a personal project, but suggestions and bug reports are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

---

## License

[Add your license here]

---

## Acknowledgments

- **Tauri**: Cross-platform app framework
- **SvelteKit**: Frontend framework
- **D3.js**: Graph visualization
- **Rust**: Backend language
- **SQLite**: Database

---

## Support

For issues or questions:
- Check existing documentation in `/docs`
- Review troubleshooting section above
- Check console logs: `Cmd+Option+I` in dev mode

---

**Version**: 1.0.0  
**Platform**: macOS 10.15+  
**Last Updated**: January 2026
