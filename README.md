# Universal Android Debloater

> **Download → Install → Debloat.** Remove bloatware from any Android device in minutes.

[![Latest Release](https://img.shields.io/github/v/release/kapilthakare/universal-android-debloater?label=version&color=blue)](https://github.com/kapilthakare/universal-android-debloater/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/kapilthakare/universal-android-debloater/total?color=green)](https://github.com/kapilthakare/universal-android-debloater/releases)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-orange.svg)](LICENSE)
[![CI](https://github.com/kapilthakare/universal-android-debloater/actions/workflows/ci.yml/badge.svg)](https://github.com/kapilthakare/universal-android-debloater/actions/workflows/ci.yml)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)](https://github.com/kapilthakare/universal-android-debloater/releases/latest)

---

## ⚡ Quick Install

### macOS
```bash
# Option 1: Download the .dmg from Releases
# Open → Drag to Applications → Launch

# Option 2: Homebrew (coming soon)
# brew install --cask universal-android-debloater
```

### Windows
```
# Option 1: Download the .exe installer from Releases
# Run → Follow prompts → Launch from Start Menu

# Option 2: Download the portable .zip
# Extract → Double-click uad_gui.exe
```

### Linux
```bash
# Debian/Ubuntu: Download .deb and install
sudo dpkg -i universal-android-debloater_*.deb
sudo apt install -f  # install dependencies

# Any dist: Download .AppImage, make executable, run
chmod +x Universal.Android.Debloater-*.AppImage
./Universal.Android.Debloater-*.AppImage
```

### Build from Source
```bash
git clone https://github.com/kapilthakare/universal-android-debloater.git
cd universal-android-debloater
cargo build --release
./target/release/uad_gui
```

---

## 📸 Screenshots

<img src="resources/screenshots/v0.5.0.png" width="850" alt="Universal Android Debloater GUI showing package list with enable/disable toggles">

---

## 🚀 Quick Debloat Guide (Beginners)

**Total time: ~5 minutes**

### Step 1: Prepare Your Phone
1. Open **Settings** → **About Phone**
2. Tap **Build Number** 7 times to enable Developer Options
3. Go to **Settings** → **Developer Options**
4. Enable **USB Debugging**

### Step 2: Connect & Launch
1. Connect your phone to computer via USB
2. When prompted on phone, tap **Allow** for USB debugging
3. Launch Universal Android Debloater

### Step 3: Debloat
1. The app will detect your device automatically
2. Browse packages by category (Google, Facebook, OEM, etc.)
3. **Safe for beginners:** Only remove packages marked ✅ **Recommended**
4. Click **Uninstall** to remove selected packages

### Step 4: Done!
- Changes are reversible — you can restore any package later
- Use the **Backup** feature before removing anything important
- Reboot your phone to apply changes

> ⚠️ **Always backup your data first!** While you cannot brick your device, removing essential packages may cause a bootloop (recoverable via factory reset).

---

## 📋 Summary

Universal Android Debloater (UAD) is a Free and Open-Source GUI application written in Rust that simplifies the removal of unnecessary and obscure system apps on Android devices. This can help improve **privacy**, **battery life**, and reduce the [attack surface](https://en.wikipedia.org/wiki/Attack_surface).

**You CANNOT brick your device with this software.** The worst case is a bootloop, which is recoverable via factory reset after ~5 failed boots.

---

## ✨ Features

- [x] **Uninstall/Disable** and **Restore/Enable** system packages
- [x] **Multi-user support** (work profiles, secondary users)
- [x] **Backup/Restore** device state before changes
- [x] **Multi-device support** (connect multiple phones simultaneously)
- [x] **Action logging** for audit trails
- [x] **Self-update** capability
- [x] **Remote debloat list** updates
- [x] **Dark, Light, and Lupin** themes
- [x] **Device-specific** persistent configuration

---

## 📦 Debloat Lists

### Universal Lists
- [x] GFAM (Google/Facebook/Amazon/Microsoft)
- [x] AOSP
- [x] Manufacturers (OEM)
- [x] Mobile carriers
- [x] Qualcomm / Mediatek / Miscellaneous

### Manufacturer Support

| Status | Manufacturers |
|--------|--------------|
| ✅ Complete | Asus, Google, LG, Huawei, Motorola, Nokia, OnePlus, Oppo, Realme, Samsung, Sony, Tecno, Unihertz, Vivo/iQOO, Xiaomi, ZTE |
| 🔄 Partial | Fairphone |
| ⏳ Planned | Archos, Blackberry, Gionee, HTC, TCL, Wiko |

### Mobile Carrier Support

| Country | Carriers |
|---------|----------|
| France | Orange, SFR, Free, Bouygues |
| USA | T-Mobile, Verizon, Sprint, AT&T |
| Germany | Telekom |
| UK | EE |

---

## 🔧 Prerequisites

### ADB (Android Debug Bridge)

UAD requires ADB to communicate with your device.

<details>
<summary><strong>macOS</strong></summary>

```bash
brew install android-platform-tools
```
</details>

<details>
<summary><strong>Windows</strong></summary>

1. Download [Android Platform Tools](https://dl.google.com/android/repository/platform-tools-latest-windows.zip)
2. Extract to `C:\platform-tools`
3. Add to PATH or launch UAD from the same directory
4. [Install USB drivers](https://developer.android.com/studio/run/oem-usb#Drivers) for your device
</details>

<details>
<summary><strong>Linux</strong></summary>

```bash
# Debian/Ubuntu
sudo apt install android-sdk-platform-tools

# Arch Linux
sudo pacman -S android-tools

# Fedora
sudo dnf install android-tools
```
</details>

Verify installation:
```bash
adb devices
```

---

## ⚙️ Configuration

UAD stores configuration in OS-specific directories:

| OS | Config Location | Cache Location |
|----|----------------|----------------|
| Linux | `~/.config/uad/` | `~/.cache/uad/` |
| macOS | `~/Library/Application Support/uad/` | `~/Library/Caches/uad/` |
| Windows | `%APPDATA%\uad\` | `%LOCALAPPDATA%\uad\` |

### Config File (`config.toml`)

```toml
[general]
theme = "Lupin"
expert_mode = false

[[devices]]
device_id = "ABC123"
disable_mode = false
multi_user_mode = true
```

---

## 🆘 Troubleshooting

<details>
<summary><strong>UAD won't detect my device</strong></summary>

- Ensure **USB Debugging** is enabled
- Try a different USB cable or port
- Run `adb devices` to verify detection
- Restart ADB: `adb kill-server && adb start-server`
- On Windows, ensure OEM USB drivers are installed
</details>

<details>
<summary><strong>UAD crashes on startup</strong></summary>

- Try the **OpenGL** version (filename contains `-OpenGL`)
- Check logs in the cache directory
- Ensure your GPU supports Vulkan (for default version)
</details>

<details>
<summary><strong>Package removal causes issues</strong></summary>

- Start with **Recommended** removals only
- Research packages before removing
- Use **Backup/Restore** to save device state
- Bootloop recovery: hold power + volume down for factory reset
</details>

<details>
<summary><strong>macOS: "App is damaged" or Gatekeeper warning</strong></summary>

- Right-click the app → **Open** (bypasses Gatekeeper for first launch)
- Or: `xattr -d com.apple.quarantine /Applications/Universal.Android.Debloater.app`
</details>

---

## 🛠️ Development

### Quick Start
```bash
git clone https://github.com/kapilthakare/universal-android-debloater.git
cd universal-android-debloater

# Run all checks
just ci

# Run the app
just run
```

### Available `just` Recipes
```bash
just check        # Check compilation
just fmt          # Format code
just lint         # Run clippy
just test         # Run tests
just build        # Build release
just run          # Run the app
just release      # Build + package for all platforms
just dmg          # Create macOS .dmg installer
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for full guidelines.

---

## 📄 License

This project is licensed under the [GPL-3.0 License](LICENSE).

---

## 🙏 Special Thanks

- [@mawilms](https://github.com/mawilms) for [Lembas](https://github.com/mawilms/lembas) — helped understand [Iced](https://github.com/hecrj/iced) GUI library
- [@casperstorm](https://github.com/casperstorm) for UI/UX inspiration
- All [contributors](https://github.com/kapilthakare/universal-android-debloater/graphs/contributors) who have improved the debloat lists

---

<p align="center">
  <sub>Built with ❤️ using Rust · <a href="https://github.com/kapilthakare/universal-android-debloater">GitHub</a></sub>
</p>
