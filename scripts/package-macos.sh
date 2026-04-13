#!/usr/bin/env bash
# package-macos.sh - Create macOS .app bundle from release binary
# Usage: ./scripts/package-macos.sh [wgpu|glow]
#
# Creates: dist/Universal.Android.Debloater.app/
# Requires: Built binary at target/release/uad_gui

set -euo pipefail

GRAPHICS="${1:-wgpu}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DIST_DIR="$PROJECT_DIR/dist"
APP_NAME="Universal.Android.Debloater"
APP_BUNDLE="$DIST_DIR/$APP_NAME.app"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Validate ──────────────────────────────────────────────────────────
if [[ ! -f "$PROJECT_DIR/target/release/uad_gui" ]]; then
    log_error "Release binary not found. Run 'cargo build --release' first."
    exit 1
fi

# ── Setup .app bundle structure ───────────────────────────────────────
log_info "Creating .app bundle: $APP_BUNDLE"

rm -rf "$APP_BUNDLE"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"

# ── Copy binary ───────────────────────────────────────────────────────
log_info "Copying binary..."
cp "$PROJECT_DIR/target/release/uad_gui" "$APP_BUNDLE/Contents/MacOS/uad_gui"
chmod +x "$APP_BUNDLE/Contents/MacOS/uad_gui"

# ── Copy Info.plist ───────────────────────────────────────────────────
log_info "Copying Info.plist..."
cp "$PROJECT_DIR/resources/macos/Info.plist" "$APP_BUNDLE/Contents/Info.plist"

# ── Embed version from Cargo.toml ─────────────────────────────────────
VERSION=$(grep '^version = ' "$PROJECT_DIR/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
if [[ -n "$VERSION" ]]; then
    log_info "Setting version: $VERSION"
    if command -v /usr/libexec/PlistBuddy &>/dev/null; then
        /usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || true
        /usr/libexec/PlistBuddy -c "Set :CFBundleVersion ${VERSION//./}" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || true
    fi
fi

# ── Copy icon (if available) ─────────────────────────────────────────
ICON_SRC="$PROJECT_DIR/resources/assets/icon.icns"
if [[ -f "$ICON_SRC" ]]; then
    log_info "Copying app icon..."
    cp "$ICON_SRC" "$APP_BUNDLE/Contents/Resources/AppIcon.icns"
else
    log_warn "No icon found at $ICON_SRC — app will use default macOS icon"
    log_warn "To add an icon, place a 1024x1024 PNG at resources/assets/icon.png and run:"
    log_warn "  mkdir -p /tmp/iconset && sips -z 16 16 16 32 32 64 64 128 128 256 256 512 512 resources/assets/icon.png --out /tmp/iconset/icon_*.png && iconutil -c icns /tmp/iconset -o $ICON_SRC"
fi

# ── Create PkgInfo ────────────────────────────────────────────────────
echo -n "APPL????" > "$APP_BUNDLE/Contents/PkgInfo"

# ── Code sign (ad-hoc, for local distribution) ────────────────────────
if command -v codesign &>/dev/null; then
    log_info "Code signing (ad-hoc)..."
    ENTITLEMENTS="$PROJECT_DIR/resources/macos/entitlements.plist"
    if [[ -f "$ENTITLEMENTS" ]]; then
        codesign --force --deep --sign - --entitlements "$ENTITLEMENTS" "$APP_BUNDLE" 2>/dev/null || \
        codesign --force --deep --sign - "$APP_BUNDLE" 2>/dev/null || \
        log_warn "Code signing failed — app will still work locally but may show Gatekeeper warning"
    else
        codesign --force --deep --sign - "$APP_BUNDLE" 2>/dev/null || \
        log_warn "Code signing failed"
    fi
fi

# ── Verify bundle ─────────────────────────────────────────────────────
log_info "Verifying bundle structure..."
if [[ -d "$APP_BUNDLE" ]]; then
    APP_SIZE=$(du -sh "$APP_BUNDLE" | cut -f1)
    log_info "✅ .app bundle created successfully ($APP_SIZE)"
    log_info "   Location: $APP_BUNDLE"
    log_info ""
    log_info "To test: open $APP_BUNDLE"
    log_info "To create DMG: ./scripts/create-dmg.sh $GRAPHICS"
else
    log_error "Failed to create .app bundle"
    exit 1
fi
