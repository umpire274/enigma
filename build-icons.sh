#!/bin/bash

# Creo la directory 'assets' se non esiste
mkdir -p assets

if [ ! -f assets/icon.png ]; then
  # Crea l'icona base con ImageMagick
  magick -size 256x256 xc:black -fill white -draw $'
    circle 128,128 128,64
    text 100,135 "E"' assets/icon.png
fi

# Converti per Windows
magick assets/icon.png -define icon:auto-resize=256,128,64,48,32,16 assets/icon.ico

# Generazione ICNS alternativa per macOS
mkdir -p macos.iconset

# Crea tutte le varianti richieste usando sips (tool nativo macOS)
sizes=(16 32 128 256 512 1024)
for size in "${sizes[@]}"; do
    # Genera le dimensioni base
    sips -z $size $size assets/icon.png --out "macos.iconset/icon_${size}x${size}.png"

    # Genera le varianti @2x (eccetto per 1024)
    if [ $size -ne 1024 ]; then
        dsize=$((size*2))
        sips -z $dsize $dsize assets/icon.png --out "macos.iconset/icon_${size}x${size}@2x.png"
    fi
done

# Crea il pacchetto ICNS usando iconutil (tool nativo)
iconutil -c icns macos.iconset -o assets/icon.icns

# Verifica con tool nativo
if ! file assets/icon.icns | grep -q "Mac OS X icon"; then
    echo "❌ Errore nella generazione dell'icona macOS"
    exit 1
fi

# Pulizia
rm -rf macos.iconset

# Crea le varianti Linux
for size in 16 32 48 64 128 256; do
  magick assets/icon.png -resize "${size}x${size}" "assets/icon_${size}x${size}.png"
done

echo "Icone generate in assets/"
