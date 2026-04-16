---
name: nokia-debloater
description: Automates the debloating of Nokia/HMD Android devices by removing telemetry, unnecessary system apps, and performance killers like Evenwell packages. Use this when the user asks to debloat, optimize, or speed up a Nokia phone.
---

# Nokia Debloater

This skill automates the removal of known bloatware and performance-killing apps from Nokia/HMD Android devices using ADB. It specifically targets telemetry apps, redundant manufacturer apps, and Google bloatware that drain the battery and consume system resources.

## When to Use This Skill

Use this skill when a user connects a Nokia Android device and wants to:
- Debloat the phone
- Remove unnecessary apps
- Speed up the device
- Optimize battery life and performance

## Prerequisites

1. The device must be connected via USB.
2. **Developer Options** must be enabled on the device.
3. **USB Debugging** must be turned on.
4. The device must be authorized (`adb devices` should list the device as `device`, not `unauthorized`).

## Usage Instructions

1. Verify that the device is connected by running:
   ```bash
   adb devices
   ```
2. Execute the bundled debloat script:
   ```bash
   bash scripts/debloat.sh
   ```
3. The script will automatically uninstall a predefined list of Nokia-specific apps, Amazon bloatware, and non-essential Google apps for user 0 (the current user).

## Safety Notes

- This process uses `pm uninstall --user 0`, meaning the packages are only uninstalled for the current user and not completely removed from the system partition.
- This prevents bricking the device. If the user factory resets the phone, these apps will be restored.
- We do not uninstall critical system components that would cause a bootloop.