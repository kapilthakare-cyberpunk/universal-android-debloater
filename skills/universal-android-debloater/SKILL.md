---
name: universal-android-debloater
description: Use when the user wants to debloat, optimize, or clean up any Android device connected via ADB. This skill provides a safe, data-driven approach to removing manufacturer bloatware, non-essential Google apps, and performance-killing telemetry.
---

# Universal Android Debloater

This skill provides a systematic approach to debloating any Android device using ADB. It prioritizes device stability and reversibility while maximizing performance and privacy gains.

## When to Use This Skill

Use this skill when:
- A user connects an Android device and asks to "debloat", "clean up", "optimize", or "speed up".
- The device feels sluggish, has poor battery life, or is cluttered with pre-installed apps.
- You need to identify which system apps are safe to remove on a specific brand (Nokia, Samsung, Xiaomi, etc.).

## Prerequisites

1. **ADB Access:** `adb devices` must show the device as `device`.
2. **Developer Mode:** USB Debugging must be enabled on the phone.
3. **UAD Database:** Reference `universal-android-debloater/resources/assets/uad_lists.json` for package descriptions and safety ratings.

## Core Workflow

1.  **Identify Device:** Get the model and manufacturer.
    ```bash
    adb shell getprop ro.product.manufacturer
    adb shell getprop ro.product.model
    ```
2.  **List Packages:** Pull all installed packages to a local file for analysis.
    ```bash
    adb shell pm list packages -u | cut -d':' -f2 | sort > packages.txt
    ```
3.  **Search UAD Database:** Grep the `uad_lists.json` for the identified manufacturer and specific package IDs to find "Recommended" for removal.
4.  **Debloat Strategically:**
    - **Safe (Recommended):** Only remove apps marked as "Recommended" in UAD list or known non-essential apps (e.g., YouTube Music, Duo).
    - **Advanced:** Only if the user explicitly asks for "Deep Debloat" or "Maximum Clean".
5.  **Execution:** Use `pm uninstall --user 0 <package_id>` for non-destructive removal.

## ⚠️ The Safety Matrix (DO NOT UNINSTALL)

NEVER uninstall these core packages unless the user is an expert and has a backup plan:

| Category | Package Pattern | Reason |
| :--- | :--- | :--- |
| **System** | `com.android.packageinstaller` | Cannot install/update apps if removed. |
| **Auth/Core** | `com.google.android.gms`, `com.google.android.gsf` | Bricks Google Play Services and many apps. |
| **Input** | `com.google.android.inputmethod.latin` (Gboard) | No keyboard = cannot type password after reboot. |
| **Phone** | `com.google.android.dialer`, `com.android.phone` | Cannot make/receive calls. |
| **Files** | `com.android.documentsui` | Core file picker for all apps. |

## Quick Reference: Common Bloatware

- **Manufacturer:** `com.hmdglobal.*` (Nokia), `com.sec.android.*` (Samsung), `com.miui.*` (Xiaomi).
- **Social/Pre-builts:** `com.facebook.system`, `com.amazon.appmanager`, `com.netflix.partner.activation`, `com.whatsapp`, `com.whatsapp.w4b`.
- **Google (Safe to remove):** `com.google.android.apps.tachyon` (Duo), `com.google.android.videos` (Google TV), `com.google.android.apps.wellbeing`.

## Reversing Changes

If an app is needed again:
```bash
adb shell pm install-existing <package_id>
```

## Common Mistakes
- **Greedy Grepping:** Removing all "google" apps. Many are core system services.
- **No Keyboard:** Removing Gboard without another keyboard installed.
- **Removing Store:** Removing `com.android.vending` (Play Store) without a replacement like F-Droid.
