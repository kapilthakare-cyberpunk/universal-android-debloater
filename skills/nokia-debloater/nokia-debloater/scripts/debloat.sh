#!/bin/bash

# Nokia/HMD & Google Bloatware Removal Script
# Run this script to debloat a Nokia device connected via adb.

echo "Checking for connected device..."
adb devices

echo "Uninstalling Nokia/HMD Specific Bloatware..."
adb shell pm uninstall --user 0 com.hmdglobal.support || true
adb shell pm uninstall --user 0 com.hmdglobal.app.myphonehelper || true
adb shell pm uninstall --user 0 com.hmdglobal.app.legalinformation || true
adb shell pm uninstall --user 0 com.hmdglobal.app.activation || true
adb shell pm uninstall --user 0 com.hmdglobal.app.omacp || true
adb shell pm uninstall --user 0 com.hmdglobal.app.customizationclient || true
adb shell pm uninstall --user 0 com.hmdglobal.memorycleaner || true
adb shell pm uninstall --user 0 com.hmdglobal.app.bokeheditor || true
adb shell pm uninstall --user 0 com.hmdglobal.app.fmradio || true
adb shell pm uninstall --user 0 com.hmdglobal.app.setupwizardext || true
adb shell pm uninstall --user 0 com.hmdglobal.app.settings.provider || true
adb shell pm uninstall --user 0 com.hmdglobal.app.camera || true
adb shell pm uninstall --user 0 com.hmd.face.service || true

echo "Uninstalling Amazon App Manager..."
adb shell pm uninstall --user 0 com.amazon.appmanager || true

echo "Uninstalling Universal Google Bloatware..."
adb shell pm uninstall --user 0 com.google.android.apps.youtube.music || true
adb shell pm uninstall --user 0 com.google.android.videos || true
adb shell pm uninstall --user 0 com.google.android.apps.docs || true
adb shell pm uninstall --user 0 com.google.android.apps.wellbeing || true
adb shell pm uninstall --user 0 com.google.android.apps.safetyhub || true
adb shell pm uninstall --user 0 com.google.android.apps.tachyon || true
adb shell pm uninstall --user 0 com.google.android.apps.subscriptions.red || true
adb shell pm uninstall --user 0 com.google.android.apps.googleassistant || true
adb shell pm uninstall --user 0 com.google.android.projection.gearhead || true

echo "Uninstalling Miscellaneous Print Services..."
adb shell pm uninstall --user 0 com.android.bips || true
adb shell pm uninstall --user 0 com.android.printspooler || true
adb shell pm uninstall --user 0 com.google.android.printservice.recommendation || true

echo "Debloat Complete! Your Nokia device should now perform better."