# Installation Guide — Universal Android Debloater

> **For non-technical users.** Step-by-step instructions to get up and running in under 5 minutes.

---

## Table of Contents

1. [Download the App](#1-download-the-app)
2. [Install on Your Platform](#2-install-on-your-platform)
3. [Prepare Your Android Phone](#3-prepare-your-android-phone)
4. [Launch & Start Debloating](#4-launch--start-debloating)
5. [Troubleshooting](#5-troubleshooting)

---

## 1. Download the App

Go to the **[Releases page](https://github.com/kapilthakare/universal-android-debloater/releases/latest)** and download the file for your computer:

| Your Computer | Download This File |
|---------------|-------------------|
| **Mac** (Apple or Intel) | `Universal.Android.Debloater-*.dmg` |
| **Windows** PC | `Universal.Android.Debloater-*.exe` or `*-portable.zip` |
| **Linux** (Ubuntu/Debian) | `universal-android-debloater_*.deb` |
| **Linux** (any other) | `Universal.Android.Debloater-*.AppImage` |

> **Not sure which version?** Download the file **without** "OpenGL" in the name first. If it doesn't work, try the OpenGL version.

---

## 2. Install on Your Platform

### 🍎 macOS

1. **Open the `.dmg` file** you downloaded (double-click it)
2. **Drag the app** to the Applications folder
3. **Open Applications** folder and find "Universal Android Debloater"
4. **First launch:** Right-click the app → click **Open** → click **Open** again
   - This is a macOS security step. You only need to do this once.

### 🪟 Windows

**Option A: Installer (Recommended)**
1. **Run the `.exe` file** you downloaded
2. Follow the installation wizard
3. Launch from **Start Menu** → "Android Debloater"

**Option B: Portable (No Install)**
1. **Right-click the `.zip`** → Extract All
2. Open the extracted folder
3. **Double-click `uad_gui.exe`**

### 🐧 Linux

**Ubuntu/Debian (.deb)**
```bash
sudo dpkg -i universal-android-debloater_*.deb
sudo apt install -f
```
Then launch from your Applications menu or run `uad_gui` in terminal.

**Any Linux (.AppImage)**
```bash
# Make it executable
chmod +x Universal.Android.Debloater-*.AppImage

# Run it
./Universal.Android.Debloater-*.AppImage
```

---

## 3. Prepare Your Android Phone

Before connecting your phone, you need to enable **Developer Options**:

### Step 1: Enable Developer Options
1. Open **Settings** on your phone
2. Scroll to **About Phone** (or **About Device**)
3. Find **Build Number** (or **Software Information** → Build Number on Samsung)
4. **Tap "Build Number" 7 times**
5. You'll see: *"You are now a developer!"*

### Step 2: Enable USB Debugging
1. Go back to **Settings**
2. Find **Developer Options** (near the bottom)
3. Scroll to **USB Debugging**
4. **Turn it ON**
5. Confirm the warning prompt

### Step 3: Connect to Computer
1. Connect your phone to your computer with a **USB cable**
2. On your phone, a prompt will appear: **"Allow USB debugging?"**
3. Tap **Allow** (check "Always allow from this computer" if available)

---

## 4. Launch & Start Debloating

1. **Open Universal Android Debloater**
2. The app will automatically detect your connected device
3. You'll see a list of packages (apps) installed on your phone

### What Can I Safely Remove?

| Category | Safe to Remove? | Examples |
|----------|----------------|----------|
| ✅ **Recommended** | Yes — safe for most users | Facebook, Google apps you don't use |
| ⚠️ **Advanced** | Only if you know what it does | System UI components |
| ❌ **Unsafe** | No — may break your phone | Core system apps, launcher |

### To Remove a Package:
1. Click the checkbox next to the package name
2. Click **Uninstall Selected**
3. Confirm the action

### To Restore a Package:
1. Switch to the **Restore** tab
2. Select the package you want back
3. Click **Restore Selected**

> 💡 **Tip:** Use the **Backup** button before removing anything. This saves your current state so you can always revert.

---

## 5. Troubleshooting

### "Device not detected"
- Make sure **USB Debugging** is enabled
- Try a **different USB cable** (some cables are charge-only)
- Try a **different USB port** on your computer
- Run `adb devices` in terminal/command prompt — does it show your device?

### "App won't open on macOS"
- Right-click the app → **Open** (instead of double-clicking)
- Go to **System Preferences** → **Security & Privacy** → click **Open Anyway**

### "Windows SmartScreen blocked the app"
- Click **More info** → **Run anyway**
- This happens because the app isn't digitally signed (it's free and open-source)

### "I removed something important and my phone won't boot"
- Don't panic — your phone will enter **recovery mode** after ~5 failed boots
- From recovery mode, select **Factory Reset**
- This erases your data but restores the phone to working condition
- **This is why we recommend backing up first!**

### "Which packages should I remove?"
- Start with the **Recommended** list only
- If you're unsure about a package, **search for it online** before removing
- When in doubt, **leave it** — you can always remove it later

---

## Need More Help?

- 📖 [FAQ Wiki](https://github.com/kapilthakare/universal-android-debloater/wiki/FAQ)
- 🐛 [Report a Bug](https://github.com/kapilthakare/universal-android-debloater/issues/new)
- 💬 [Discussions](https://github.com/kapilthakare/universal-android-debloater/discussions)

---

<p align="center">
  <sub>Still stuck? Open an issue on GitHub — the community is happy to help!</sub>
</p>
