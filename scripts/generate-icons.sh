#!/usr/bin/env bash
# generate-icons.sh - Generate platform-specific icons from a source PNG
# Usage: ./scripts/generate-icons.sh [source.png]
#
# Requires: A 1024x1024 PNG source image
# Generates: icon.icns (macOS), icon.ico (Windows), icon.png (Linux)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_DIR/resources/assets"

SOURCE="${1:-$ASSETS_DIR/icon-source.png}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# ── Validate source ──────────────────────────────────────────────────
if [[ ! -f "$SOURCE" ]]; then
    log_error "Source image not found: $SOURCE"
    log_info "Place a 1024x1024 PNG at resources/assets/icon-source.png and run this script."
    exit 1
fi

if ! command -v sips &>/dev/null && ! command -v convert &>/dev/null; then
    log_error "Requires sips (macOS) or convert (ImageMagick)"
    log_info "Install ImageMagick: brew install imagemagick"
    exit 1
fi

log_info "Generating icons from: $SOURCE"

# ── macOS: .icns ─────────────────────────────────────────────────────
log_info "Creating macOS .icns..."

ICONSET_DIR="$ASSETS_DIR/icon.iconset"
rm -rf "$ICONSET_DIR"
mkdir -p "$ICONSET_DIR"

SIZES=("16" "32" "64" "128" "256" "512")

for size in "${SIZES[@]}"; do
    if command -v sips &>/dev/null; then
        sips -z "$size" "$size" "$SOURCE" --out "$ICONSET_DIR/icon_${size}x${size}.png" 2>/dev/null
        sips -z $((size * 2)) $((size * 2)) "$SOURCE" --out "$ICONSET_DIR/icon_${size}x${size}@2x.png" 2>/dev/null
    elif command -v convert &>/dev/null; then
        convert "$SOURCE" -resize "${size}x${size}" "$ICONSET_DIR/icon_${size}x${size}.png"
        convert "$SOURCE" -resize "$((size * 2))x$((size * 2))" "$ICONSET_DIR/icon_${size}x${size}@2x.png"
    fi
done

if command -v iconutil &>/dev/null; then
    iconutil -c icns "$ICONSET_DIR" -o "$ASSETS_DIR/icon.icns"
    log_info "Created: $ASSETS_DIR/icon.icns"
else
    log_warn "iconutil not found — .icns not created (macOS only)"
fi

rm -rf "$ICONSET_DIR"

# ── Windows: .ico ────────────────────────────────────────────────────
log_info "Creating Windows .ico..."

if command -v convert &>/dev/null; then
    ICO_SIZES=("16" "32" "48" "64" "128" "256")
    CONVERT_ARGS=""
    for size in "${ICO_SIZES[@]}"; do
        CONVERT_ARGS+=" -resize ${size}x${size}"
    done
    convert "$SOURCE" $CONVERT_ARGS "$ASSETS_DIR/icon.ico"
    log_info "Created: $ASSETS_DIR/icon.ico"
else
    log_warn "ImageMagick not found — .ico not created"
    log_warn "Install: brew install imagemagick"
fi

# ── Linux: .png (256x256) ────────────────────────────────────────────
log_info "Creating Linux icon (256x256 PNG)..."

if command -v sips &>/dev/null; then
    sips -z 256 256 "$SOURCE" --out "$ASSETS_DIR/icon.png" 2>/dev/null
elif command -v convert &>/dev/null; then
    convert "$SOURCE" -resize 256x256 "$ASSETS_DIR/icon.png"
fi

if [[ -f "$ASSETS_DIR/icon.png" ]]; then
    log_info "Created: $ASSETS_DIR/icon.png"
fi

# ── Summary ──────────────────────────────────────────────────────────
log_info ""
log_info "Generated icons:"
ls -la "$ASSETS_DIR"/icon.* 2>/dev/null | while read -r line; do
    log_info "  $line"
done
