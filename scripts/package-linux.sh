#!/usr/bin/env bash
# package-linux.sh - Create Linux .deb and .AppImage packages
# Usage: ./scripts/package-linux.sh [wgpu|glow]
#
# Requires: Built binary at target/release/uad_gui
# Creates:  dist/universal-android-debloater_<version>_amd64.deb
#           dist/Universal.Android.Debloater-<version>-x86_64.AppImage

set -euo pipefail

GRAPHICS="${1:-wgpu}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_DIR/dist"
BINARY="$PROJECT_DIR/target/release/uad_gui"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Validate ──────────────────────────────────────────────────────────
if [[ ! -f "$BINARY" ]]; then
    log_error "Release binary not found. Run 'cargo build --release' first."
    exit 1
fi

# ── Get version ───────────────────────────────────────────────────────
VERSION=$(grep '^version = ' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
VERSION="${VERSION:-0.7.0}"

GRAPHICS_LABEL=""
if [[ "$GRAPHICS" == "glow" ]]; then
    GRAPHICS_LABEL="-opengl"
fi

mkdir -p "$DIST_DIR"

# ══════════════════════════════════════════════════════════════════════
# 1. Create .deb package
# ══════════════════════════════════════════════════════════════════════
log_info "Creating .deb package..."

DEB_DIR="$DIST_DIR/.deb-build"
rm -rf "$DEB_DIR"

# Debian package structure
PKG_NAME="universal-android-debloater"
PKG_DIR="$DEB_DIR/$PKG_NAME-$VERSION"
DEBIAN_DIR="$PKG_DIR/DEBIAN"
BIN_DIR="$PKG_DIR/usr/bin"
SHARE_DIR="$PKG_DIR/usr/share"
APP_DIR="$SHARE_DIR/applications"
ICON_DIR="$SHARE_DIR/icons/hicolor/256x256/apps"

mkdir -p "$DEBIAN_DIR" "$BIN_DIR" "$APP_DIR" "$ICON_DIR"

# Copy binary
cp "$BINARY" "$BIN_DIR/uad_gui"
chmod 755 "$BIN_DIR/uad_gui"

# Create control file
cat > "$DEBIAN_DIR/control" << EOF
Package: $PKG_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Depends: android-sdk-platform-tools, libgtk-3-0, libssl3
Maintainer: Kapil Thakare <kapil@primesandzooms.com>
Description: Universal Android Debloater - GUI for removing bloatware
 A cross-platform GUI application that simplifies the removal of
 unnecessary and obscure system apps on Android devices via ADB.
 Helps improve privacy, battery life, and reduce attack surface.
Homepage: https://github.com/kapilthakare/universal-android-debloater
EOF

# Create desktop entry
cat > "$APP_DIR/universal-android-debloater.desktop" << EOF
[Desktop Entry]
Name=Universal Android Debloater
Comment=Remove bloatware from Android devices
Exec=uad_gui
Icon=universal-android-debloater
Terminal=false
Type=Application
Categories=Utility;System;
Keywords=android;debloat;adb;bloatware;privacy;
EOF

# Copy icon (if available)
ICON_SRC="$PROJECT_DIR/resources/assets/icon.png"
if [[ -f "$ICON_SRC" ]]; then
    cp "$ICON_SRC" "$ICON_DIR/universal-android-debloater.png"
else
    # Create a placeholder icon using ImageMagick if available
    if command -v convert &>/dev/null; then
        convert -size 256x256 xc:'#4A90D9' -fill white -gravity center \
            -pointsize 48 -annotate 0 "UAD" "$ICON_DIR/universal-android-debloater.png" 2>/dev/null || true
    fi
fi

# Build .deb
cd "$DEB_DIR"
dpkg-deb --build --root-owner-group "$PKG_NAME-$VERSION"
DEB_OUTPUT="$DIST_DIR/${PKG_NAME}_${VERSION}_amd64${GRAPHICS_LABEL}.deb"
mv "$PKG_NAME-$VERSION.deb" "$DEB_OUTPUT"
cd "$PROJECT_DIR"

log_info "Created .deb: $(basename "$DEB_OUTPUT")"

# ══════════════════════════════════════════════════════════════════════
# 2. Create .AppImage
# ══════════════════════════════════════════════════════════════════════
log_info "Creating .AppImage package..."

APPIMAGE_DIR="$DIST_DIR/.appimage-build"
rm -rf "$APPIMAGE_DIR"
mkdir -p "$APPIMAGE_DIR/AppDir/usr/bin"
mkdir -p "$APPIMAGE_DIR/AppDir/usr/share/applications"
mkdir -p "$APPIMAGE_DIR/AppDir/usr/share/icons/hicolor/256x256/apps"

# Copy binary
cp "$BINARY" "$APPIMAGE_DIR/AppDir/usr/bin/uad_gui"
chmod 755 "$APPIMAGE_DIR/AppDir/usr/bin/uad_gui"

# Create AppRun
cat > "$APPIMAGE_DIR/AppDir/AppRun" << 'EOF'
#!/bin/bash
SELF="$(readlink -f "$0")"
HERE="${SELF%/*}"
exec "$HERE/usr/bin/uad_gui" "$@"
EOF
chmod +x "$APPIMAGE_DIR/AppDir/AppRun"

# Copy desktop file
cp "$APP_DIR/universal-android-debloater.desktop" "$APPIMAGE_DIR/AppDir/universal-android-debloater.desktop"

# Copy icon
if [[ -f "$ICON_DIR/universal-android-debloater.png" ]]; then
    cp "$ICON_DIR/universal-android-debloater.png" "$APPIMAGE_DIR/AppDir/universal-android-debloater.png"
    cp "$ICON_DIR/universal-android-debloater.png" "$APPIMAGE_DIR/AppDir/usr/share/icons/hicolor/256x256/apps/universal-android-debloater.png"
fi

# Download linuxdeploy if not present
LINUXDEPLOY="$DIST_DIR/linuxdeploy-x86_64.AppImage"
if [[ ! -f "$LINUXDEPLOY" ]]; then
    log_info "Downloading linuxdeploy..."
    curl -L -o "$LINUXDEPLOY" \
        "https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage"
    chmod +x "$LINUXDEPLOY"
fi

# Build AppImage
APPIMAGE_NAME="Universal.Android.Debloater-${VERSION}${GRAPHICS_LABEL}-x86_64.AppImage"
APPIMAGE_OUTPUT="$DIST_DIR/$APPIMAGE_NAME"

cd "$APPIMAGE_DIR"
"$LINUXDEPLOY" --appdir AppDir --output appimage \
    --desktop-file AppDir/universal-android-debloater.desktop \
    --icon-file AppDir/universal-android-debloater.png 2>/dev/null || {
    log_warn "linuxdeploy failed, creating manual AppImage..."

    # Fallback: manual AppImage creation
    ARCHIVE="$DIST_DIR/.appimage-archive"
    rm -rf "$ARCHIVE"
    mkdir -p "$ARCHIVE"
    cp -r AppDir/* "$ARCHIVE/"

    # Create self-extracting AppImage
    cat > "$ARCHIVE/run.sh" << 'RUNEOF'
#!/bin/bash
HERE="$(cd "$(dirname "$0")" && pwd)"
exec "$HERE/usr/bin/uad_gui" "$@"
RUNEOF
    chmod +x "$ARCHIVE/run.sh"

    # Simple tar.gz as fallback
    tar -czf "$APPIMAGE_OUTPUT.tar.gz" -C "$ARCHIVE" .
    log_info "Created tar.gz fallback (AppImage creation failed)"
}

# Move output if AppImage was created
if [[ -f "Universal.Android.Debloater-x86_64.AppImage" ]]; then
    mv "Universal.Android.Debloater-x86_64.AppImage" "$APPIMAGE_OUTPUT"
fi

cd "$PROJECT_DIR"

# ── Cleanup ───────────────────────────────────────────────────────────
rm -rf "$DEB_DIR" "$APPIMAGE_DIR"

# ── Summary ───────────────────────────────────────────────────────────
log_info ""
log_info "Linux packages created:"
find "$DIST_DIR" -maxdepth 1 \( -name "*.deb" -o -name "*.AppImage" -o -name "*.tar.gz" \) -exec ls -lh {} \; | while read -r line; do
    log_info "  $line"
done
