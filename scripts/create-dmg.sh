#!/usr/bin/env bash
# create-dmg.sh - Create a macOS .dmg installer from .app bundle
# Usage: ./scripts/create-dmg.sh [wgpu|glow]
#
# Requires: dist/Universal.Android.Debloater.app/ (from package-macos.sh)
# Creates:  dist/Universal.Android.Debloater-<version>-<graphics>.dmg

set -euo pipefail

GRAPHICS="${1:-wgpu}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_DIR/dist"
APP_NAME="Universal.Android.Debloater"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Validate ──────────────────────────────────────────────────────────
if [[ ! -d "$APP_BUNDLE" ]]; then
    log_error ".app bundle not found. Run './scripts/package-macos.sh' first."
    exit 1
fi

# ── Get version ───────────────────────────────────────────────────────
VERSION=$(grep '^version = ' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
VERSION="${VERSION:-0.7.0}"

GRAPHICS_LABEL=""
if [[ "$GRAPHICS" == "glow" ]]; then
    GRAPHICS_LABEL="-OpenGL"
fi

DMG_NAME="Universal.Android.Debloater-${VERSION}${GRAPHICS_LABEL}-macos.dmg"
DMG_PATH="$DIST_DIR/$DMG_NAME"

log_info "Creating DMG: $DMG_NAME"

# ── Create temporary staging directory ────────────────────────────────
STAGING_DIR="$DIST_DIR/.dmg-staging"
rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

# ── Copy .app to staging ─────────────────────────────────────────────
cp -R "$APP_BUNDLE" "$STAGING_DIR/$APP_NAME.app"

# ── Create Applications symlink ───────────────────────────────────────
ln -s /Applications "$STAGING_DIR/Applications"

# ── Create background image (optional, creates a simple gradient) ─────
BACKGROUND_DIR="$STAGING_DIR/.background"
mkdir -p "$BACKGROUND_DIR"

# Generate a simple PNG background using Python (available on macOS)
python3 -c "
from PIL import Image, ImageDraw
# Try with PIL, fall back to simple solid color
img = Image.new('RGB', (600, 400), color=(240, 240, 245))
draw = ImageDraw.Draw(img)
draw.rectangle([0, 0, 600, 400], fill=(240, 240, 245))
img.save('$BACKGROUND_DIR/background.png')
" 2>/dev/null || echo "PIL not available, using solid background" || \
cp /dev/null "$BACKGROUND_DIR/background.png" 2>/dev/null || true

# ── Build DMG using hdiutil ──────────────────────────────────────────
log_info "Building DMG with hdiutil..."

# Create a temporary sparse image
TEMP_DMG="$DIST_DIR/.temp-dmg.sparseimage"
hdiutil create -srcfolder "$STAGING_DIR" -volname "$APP_NAME" -fs HFS+ \
    -fsargs "-c c=64,a=16,e=16" -format UDRW -size 200m "$TEMP_DMG" 2>/dev/null

# Mount it
MOUNT_POINT="/Volumes/$APP_NAME"
hdiutil attach "$TEMP_DMG" -noverify -quiet 2>/dev/null || true

# Copy files if mount succeeded
if [[ -d "$MOUNT_POINT" ]]; then
    cp -R "$STAGING_DIR/$APP_NAME.app" "$MOUNT_POINT/"
    ln -s /Applications "$MOUNT_POINT/Applications" 2>/dev/null || true

    # Set window properties via AppleScript
    osascript <<EOF
tell application "Finder"
    tell disk "$APP_NAME"
        open
        set current view of container window to icon view
        set toolbar visible of container window to false
        set statusbar visible of container window to false
        set the bounds of container window to {400, 100, 1000, 500}
        set theViewOptions to the icon view options of container window
        set arrangement of theViewOptions to not arranged
        set icon size of theViewOptions to 128
        set position of item "$APP_NAME.app" of container window to {150, 180}
        set position of item "Applications" of container window to {450, 180}
        close
        open
        update without registering applications
        delay 2
        close
    end tell
end tell
EOF

    # Unmount
    hdiutil detach "$MOUNT_POINT" -quiet 2>/dev/null || true
fi

# Convert to compressed DMG
log_info "Converting to compressed DMG..."
hdiutil convert "$TEMP_DMG" -format UDZO -imagekey zlib-level=9 -o "$DMG_PATH" 2>/dev/null

# Clean up temp files
rm -f "$TEMP_DMG" "${TEMP_DMG}.shadow" 2>/dev/null || true
rm -rf "$STAGING_DIR"

# ── Code sign the DMG ────────────────────────────────────────────────
if command -v codesign &>/dev/null; then
    codesign --force --sign - "$DMG_PATH" 2>/dev/null || true
fi

# ── Verify ────────────────────────────────────────────────────────────
if [[ -f "$DMG_PATH" ]]; then
    DMG_SIZE=$(du -sh "$DMG_PATH" | cut -f1)
    log_info "✅ DMG created successfully ($DMG_SIZE)"
    log_info "   Location: $DMG_PATH"
    log_info ""
    log_info "To test: open $DMG_PATH"
else
    log_error "Failed to create DMG"
    exit 1
fi
