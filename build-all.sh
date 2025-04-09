#!/bin/bash
set -e

# Configurazione
APP_NAME="enigma"
VERSION="0.5.0"
TARGETS=("x86_64-pc-windows-gnu" "x86_64-apple-darwin" "x86_64-unknown-linux-gnu")
ASSETS_DIR="assets"
OUTPUT_DIR="dist"
ICON_SOURCE="assets/icon.png"

# Creazione directory output
mkdir -p ${OUTPUT_DIR}

# 1. Generazione icone (richiede ImageMagick)
echo "🖼️  Generazione icone..."

mkdir -p assets

if [ ! -f ${ICON_SOURCE} ]; then
  magick -size 256x256 xc:black -fill white -draw $'
    circle 128,128 128,64
    text 100,135 "E"' ${ICON_SOURCE}
fi

# Windows .ico
magick ${ICON_SOURCE} -define icon:auto-resize=256,128,64,48,32,16 ${ASSETS_DIR}/icon.ico

# macOS .icns
mkdir -p ${ASSETS_DIR}/enigma.iconset
sizes=(16 32 64 128 256 512)
for size in ${sizes[@]}; do
  magick ${ICON_SOURCE} -resize ${size}x${size} ${ASSETS_DIR}/enigma.iconset/icon_${size}x${size}.png
  magick ${ICON_SOURCE} -resize $((size*2))x$((size*2)) ${ASSETS_DIR}/enigma.iconset/icon_${size}x${size}@2x.png
done
iconutil -c icns ${ASSETS_DIR}/enigma.iconset -o ${ASSETS_DIR}/icon.icns
rm -rf ${ASSETS_DIR}/enigma.iconset

# Crea le varianti Linux
for size in 16 32 48 64 128 256; do
  magick assets/icon.png -resize "${size}x${size}" "assets/icon_${size}x${size}.png"
done

# 2. Compilazione per tutte le piattaforme
for target in ${TARGETS[@]}; do
  echo "🔧 Compilazione per ${target}..."
  
  case $target in
    *windows*)
      EXT=".exe"
      ICON_EXT=".ico"
      ;;
    *apple*)
      EXT=""
      ICON_EXT=".icns"
      ;;
    *linux*)
      EXT=""
      ICON_EXT=".png"
      ;;
  esac

  # Compilazione
  cargo build --release --target ${target}

  # Creazione directory output
  PLATFORM_DIR="${OUTPUT_DIR}/${target}"
  mkdir -p ${PLATFORM_DIR}

  # Copia binario
  cp target/${target}/release/${APP_NAME}${EXT} ${PLATFORM_DIR}/

  # Copia assets
  cp ${ASSETS_DIR}/icon${ICON_EXT} ${PLATFORM_DIR}/ 2>/dev/null || :

  # Pacchettizzazione specifica
  case $target in
    *windows*)
      echo "📦 Creazione installer Windows..."
      makensis -V4 -DAPP_NAME=${APP_NAME} \
               -DVERSION=${VERSION} \
               -DINPUT_DIR=${PLATFORM_DIR} \
               -DOUTPUT_FILE=${OUTPUT_DIR}/${APP_NAME}_${VERSION}_windows.exe \
               packaging/windows.nsi
      ;;

    *apple*)
      echo "🍏 Creazione bundle macOS..."
      mkdir -p ${PLATFORM_DIR}/${APP_NAME}.app/Contents/{MacOS,Resources}
      cp ${PLATFORM_DIR}/${APP_NAME} ${PLATFORM_DIR}/${APP_NAME}.app/Contents/MacOS/
      cp ${ASSETS_DIR}/icon.icns ${PLATFORM_DIR}/${APP_NAME}.app/Contents/Resources/
      cat > ${PLATFORM_DIR}/${APP_NAME}.app/Contents/Info.plist <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>
EOL
      hdiutil create -volname "${APP_NAME}" \
                     -srcfolder ${PLATFORM_DIR}/${APP_NAME}.app \
                     -ov -format UDZO \
                     ${OUTPUT_DIR}/${APP_NAME}_${VERSION}_macos.dmg
      ;;

    *linux*)
      echo "🐧 Creazione pacchetto Linux..."
      mkdir -p ${PLATFORM_DIR}/DEBIAN
      cat > ${PLATFORM_DIR}/DEBIAN/control <<EOL
Package: ${APP_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Your Name <your.email@example.com>
Description: Modern Enigma Machine Implementation
EOL
      mkdir -p ${PLATFORM_DIR}/usr/local/bin
      cp ${PLATFORM_DIR}/${APP_NAME} ${PLATFORM_DIR}/usr/local/bin/
      dpkg-deb --build ${PLATFORM_DIR} ${OUTPUT_DIR}/${APP_NAME}_${VERSION}_linux.deb
      ;;
  esac
done

echo "✅ Build completato! Trovi i pacchetti in ${OUTPUT_DIR}/"
