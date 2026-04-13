# package-windows.ps1 - Create Windows installer and portable package
# Usage: .\scripts\package-windows.ps1 [wgpu|glow]
#
# Requires: Built binary at target/release/uad_gui.exe
# Creates:  dist/Universal.Android.Debloater-<version>-windows.exe
#           dist/Universal.Android.Debloater-<version>-windows-portable.zip

param(
    [string]$Graphics = "wgpu"
)

$ErrorActionPreference = "Stop"

# ── Paths ─────────────────────────────────────────────────────────────
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectDir = Split-Path -Parent $ScriptDir
$DistDir = Join-Path $ProjectDir "dist"
$BinaryPath = Join-Path $ProjectDir "target\release\uad_gui.exe"

# ── Colors ────────────────────────────────────────────────────────────
function Write-Info    { Write-Host "[INFO] $args" -ForegroundColor Green }
function Write-Warn    { Write-Host "[WARN] $args" -ForegroundColor Yellow }
function Write-Error_  { Write-Host "[ERROR] $args" -ForegroundColor Red }

# ── Validate ──────────────────────────────────────────────────────────
if (-not (Test-Path $BinaryPath)) {
    Write-Error_ "Release binary not found. Run 'cargo build --release' first."
    exit 1
}

# ── Get version ───────────────────────────────────────────────────────
$CargoToml = Get-Content (Join-Path $ProjectDir "Cargo.toml") -Raw
$Version = ($CargoToml | Select-String '^version = "(.*?)"' -AllMatches).Matches.Groups[1].Value
if (-not $Version) { $Version = "0.7.0" }

$GraphicsLabel = if ($Graphics -eq "glow") { "-OpenGL" } else { "" }
$AppName = "Universal.Android.Debloater"
$InstallerName = "$AppName-$Version$GraphicsLabel-windows.exe"
$PortableName = "$AppName-$Version$GraphicsLabel-windows-portable.zip"

# ── Create dist directory ────────────────────────────────────────────
if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir | Out-Null
}

# ── Create portable ZIP ──────────────────────────────────────────────
Write-Info "Creating portable ZIP package..."

$PortableDir = Join-Path $DistDir "portable"
if (Test-Path $PortableDir) { Remove-Item -Recurse -Force $PortableDir }
New-Item -ItemType Directory -Path $PortableDir | Out-Null

# Copy binary
Copy-Item $BinaryPath (Join-Path $PortableDir "uad_gui.exe")

# Create README for portable
$PortableReadme = @"
Universal Android Debloater - Portable Version
================================================

QUICK START:
1. Double-click uad_gui.exe to launch
2. Connect your Android device with USB Debugging enabled
3. Start debloating!

REQUIREMENTS:
- Windows 10 or later
- Android device with USB Debugging enabled
- ADB drivers installed (most devices work out of the box)

TROUBLESHOOTING:
- If the app doesn't open, try the OpenGL version
- If device not detected, run: adb devices
- Full documentation: https://github.com/kapilthakare/universal-android-debloater

"@
$PortableReadme | Out-File -FilePath (Join-Path $PortableDir "README.txt") -Encoding UTF8

# Create ZIP
$PortableZipPath = Join-Path $DistDir $PortableName
Compress-Archive -Path "$PortableDir\*" -DestinationPath $PortableZipPath -Force
Remove-Item -Recurse -Force $PortableDir

Write-Info "Created portable: $PortableName"

# ── Create self-extracting installer (using 7-Zip SFX or simple EXE) ─
Write-Info "Creating installer..."

# Check if Inno Setup is available
$InnoSetup = Get-Command "iscc" -ErrorAction SilentlyContinue
if ($InnoSetup) {
    Write-Info "Inno Setup detected, building proper installer..."

    $IssPath = Join-Path $ProjectDir "resources\windows\installer.iss"
    if (Test-Path $IssPath) {
        & iscc $IssPath "/DAppVersion=$Version" "/DGraphics=$Graphics"
        Write-Info "Installer built with Inno Setup"
    } else {
        Write-Warn "Inno Setup script not found at $IssPath"
        Write-Warn "Falling back to simple self-extracting archive"
    }
} else {
    Write-Warn "Inno Setup not found. Creating simple installer..."
    Write-Warn "For a proper installer, install Inno Setup: https://jrsoftware.org/isdl.php"

    # Create a simple batch launcher
    $LauncherPath = Join-Path $DistDir "Install-Universal-Android-Debloat.bat"
    $LauncherContent = @"
@echo off
echo ============================================
echo  Universal Android Debloater Installer
echo ============================================
echo.
echo This will install Universal Android Debloater to your Programs folder.
echo.
pause

set INSTALL_DIR=%LOCALAPPDATA%\Programs\UniversalAndroidDebloat
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

copy /Y "%~dp0uad_gui.exe" "%INSTALL_DIR%\uad_gui.exe"

echo.
echo Creating Start Menu shortcut...
set SHORTCUT_PATH=%APPDATA%\Microsoft\Windows\Start Menu\Programs
if not exist "%SHORTCUT_PATH%" mkdir "%SHORTCUT_PATH%"

powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('$SHORTCUT_PATH\Android Debloater.lnk'); $Shortcut.TargetPath = '$INSTALL_DIR\uad_gui.exe'; $Shortcut.Save()"

echo.
echo Installation complete!
echo You can now launch "Android Debloater" from your Start Menu.
echo.
pause
"@
    $LauncherContent | Out-File -FilePath $LauncherPath -Encoding ASCII

    # Copy binary next to installer
    Copy-Item $BinaryPath (Join-Path $DistDir "uad_gui.exe")

    Write-Info "Created installer batch file"
}

# ── Summary ───────────────────────────────────────────────────────────
Write-Info ""
Write-Info "Windows packages created:"
Get-ChildItem $DistDir -File | ForEach-Object {
    Write-Info "  $($_.Name) ($([math]::Round($_.Length / 1MB, 1)) MB)"
}
