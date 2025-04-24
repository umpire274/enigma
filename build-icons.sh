#!/bin/bash
set -e

# Config
ASSETS_DIR="assets"
ICON_SOURCE="${ASSETS_DIR}/icon.png"

# Dependencies check
if ! command -v magick &>/dev/null; then
    echo "❌ ImageMagick required. Install with:"
    echo "Linux: sudo apt install imagemagick"
    echo "macOS: brew install imagemagick"
    exit 1
fi

[[ "$OSTYPE" == "darwin"* ]] || { echo "⚠️ macOS .icns can only be generated on macOS"; exit 1; }

mkdir -p "${ASSETS_DIR}"

# Generate base icon if missing
if [ ! -f "${ICON_SOURCE}" ]; then
    magick -size 256x256 xc:black -fill white -draw $'
    circle 128,128 128,64
    text 100,135 "E"' "${ICON_SOURCE}"
fi

# Windows .ico
echo "🪟 Generating Windows icon..."
magick "${ICON_SOURCE}" -define icon:auto-resize=256,128,64,48,32,16 "${ASSETS_DIR}/icon.ico"

# macOS .icns
echo "🍏 Generating macOS icon set..."
mkdir -p "${ASSETS_DIR}/enigma.iconset"

sizes=(16 32 64 128 256 512)
for size in "${sizes[@]}"; do
    magick "${ICON_SOURCE}" -resize "${size}x${size}" "${ASSETS_DIR}/enigma.iconset/icon_${size}x${size}.png"
    magick "${ICON_SOURCE}" -resize "$((size*2))x$((size*2))" "${ASSETS_DIR}/enigma.iconset/icon_${size}x${size}@2x.png"
done

iconutil -c icns "${ASSETS_DIR}/enigma.iconset" -o "${ASSETS_DIR}/icon.icns"
rm -rf "${ASSETS_DIR}/enigma.iconset"

# Linux icons
echo "🐧 Generating Linux icons..."
for size in 16 32 48 64 128 256; do
    magick "${ICON_SOURCE}" -resize "${size}x${size}" "${ASSETS_DIR}/icon_${size}x${size}.png"
done

echo "✅ All icons generated in ${ASSETS_DIR}/"
ls -lh "${ASSETS_DIR}"/*.{ico,icns,png}