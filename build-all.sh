#!/bin/bash
set -euo pipefail

# Configurazione principale
APP_NAME="enigma"
VERSION="0.5.7"
TARGETS=(
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"  # Apple Silicon
    "x86_64-unknown-linux-gnu"
)
ASSETS_DIR="assets"
OUTPUT_DIR="dist"
ICON_SOURCE="${ASSETS_DIR}/icon.png"

# Colori per il logging
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Funzione di logging
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Verifica iniziale
check_prerequisites() {
    log_info "Verifica prerequisiti..."

    # Verifica che build-icons.sh sia stato eseguito
    if [[ ! -f "${ASSETS_DIR}/icon.ico" || ! -f "${ASSETS_DIR}/icon.icns" ]]; then
        log_error "Esegui prima ./build-icons.sh per generare le icone!"
        exit 1
    fi

    # Verifica presenza cargo
    if ! command -v cargo &>/dev/null; then
        log_error "Rust toolchain non trovato. Installa con:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
}

# Installazione dipendenze specifiche per Linux
install_linux_deps() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        log_info "Installo dipendenze per Linux..."
        sudo apt-get update && sudo apt-get install -y \
            pkg-config \
            libssl-dev \
            gcc-mingw-w64-x86-64 \
            musl-tools \
            build-essential \
            curl \
            nsis  # Per gli installer Windows
    fi
}

# Configurazione ambiente per target specifico
setup_target_env() {
    local target=$1
    log_info "Configuro ambiente per ${target}..."

    case "$target" in
        *windows*)
            export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
            export TARGET_CC=x86_64-w64-mingw32-gcc
            export OPENSSL_DIR=/usr/x86_64-w64-mingw32
            ;;
        *apple*)
            # Gestione differenziata per Intel/Apple Silicon
            if [[ "$(uname -m)" == "arm64" ]]; then
                export OPENSSL_DIR=/opt/homebrew/opt/openssl@1.1
            else
                export OPENSSL_DIR=/usr/local/opt/openssl@1.1
            fi
            ;;
        *linux*)
            export OPENSSL_DIR=/usr
            ;;
    esac

    export PKG_CONFIG_ALLOW_CROSS=1
    export OPENSSL_STATIC=1
}

# Compilazione per un target specifico
build_target() {
    local target=$1
    local platform_dir="${OUTPUT_DIR}/${target}"
    local ext=""
    local icon_ext=""

    log_info "🛠️  Inizio compilazione per ${target}"

    # Configurazione specifica per piattaforma
    case "$target" in
        *windows*)
            ext=".exe"
            icon_ext=".ico"
            ;;
        *apple*)
            icon_ext=".icns"
            ;;
        *linux*)
            icon_ext=".png"
            ;;
    esac

    mkdir -p "$platform_dir"

    # Compilazione con logging dettagliato
    log_info "🔨 Compilazione in corso..."
    if ! cargo build --release --target "$target" --features openssl/vendored 2>&1 | tee "${platform_dir}/build.log"; then
        log_error "Compilazione fallita per ${target}"
        log_error "Consulta ${platform_dir}/build.log per i dettagli"
        return 1
    fi

    # Copia eseguibile e icone
    cp "target/${target}/release/${APP_NAME}${ext}" "${platform_dir}/"
    cp "${ASSETS_DIR}/icon${icon_ext}" "${platform_dir}/" 2>/dev/null || :

    # Pacchettizzazione specifica
    package_for_target "$target" "$platform_dir"
}

# Pacchettizzazione per piattaforma
package_for_target() {
    local target=$1
    local platform_dir=$2

    case "$target" in
        *windows*)
            package_windows "$platform_dir"
            ;;
        *apple*)
            package_macos "$platform_dir"
            ;;
        *linux*)
            package_linux "$platform_dir"
            ;;
    esac
}

package_windows() {
    local dir=$1
    log_info "📦 Creo installer Windows..."

    # Verifica NSIS
    if ! command -v makensis &>/dev/null; then
        log_warn "NSIS non installato, salto la creazione dell'installer"
        return
    fi

    makensis -V4 \
        -DAPP_NAME="${APP_NAME}" \
        -DVERSION="${VERSION}" \
        -DINPUT_DIR="${dir}" \
        -DOUTPUT_FILE="${OUTPUT_DIR}/${APP_NAME}_${VERSION}_windows.exe" \
        packaging/windows.nsi
}

package_macos() {
    local dir=$1
    log_info "🍏 Creo bundle macOS..."

    # Struttura bundle
    local app_dir="${dir}/${APP_NAME}.app"
    mkdir -p "${app_dir}/Contents/"{MacOS,Resources}

    # Copia eseguibile e risorse
    cp "${dir}/${APP_NAME}" "${app_dir}/Contents/MacOS/"
    cp "${ASSETS_DIR}/icon.icns" "${app_dir}/Contents/Resources/"

    # Info.plist
    cat > "${app_dir}/Contents/Info.plist" <<EOL
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.${APP_NAME}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOL

    # Creazione DMG
    hdiutil create -volname "${APP_NAME}" \
                  -srcfolder "${app_dir}" \
                  -ov -format UDZO \
                  "${OUTPUT_DIR}/${APP_NAME}_${VERSION}_macos.dmg"
}

package_linux() {
    local dir=$1
    log_info "🐧 Creo pacchetto Linux..."

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Pacchetto Debian
        mkdir -p "${dir}/DEBIAN"
        cat > "${dir}/DEBIAN/control" <<EOL
Package: ${APP_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: ${APP_NAME} Developers <team@example.com>
Description: Modern Enigma Machine Implementation
 A secure implementation of the classic Enigma machine with modern cryptography.
Homepage: https://github.com/umpire274/enigma
EOL

        mkdir -p "${dir}/usr/local/bin"
        cp "${dir}/${APP_NAME}" "${dir}/usr/local/bin/"
        dpkg-deb --build "${dir}" "${OUTPUT_DIR}/${APP_NAME}_${VERSION}_linux.deb"
    else
        # Archivio generico per cross-compilazione
        tar -czf "${OUTPUT_DIR}/${APP_NAME}_${VERSION}_linux.tar.gz" -C "${dir}" .
    fi
}

# Main execution
main() {
    check_prerequisites
    install_linux_deps

    # Creazione directory output
    mkdir -p "${OUTPUT_DIR}"

    # Compilazione per tutti i target
    for target in "${TARGETS[@]}"; do
        log_info "\n🚀 Inizio elaborazione per ${target}"
        setup_target_env "$target"

        if ! build_target "$target"; then
            log_warn "Saltato target ${target} a causa di errori"
            continue
        fi

        log_info "✅ Completato con successo ${target}"
    done

    # Riepilogo finale
    log_info "\n🎉 Build completato! Pacchetti disponibili in ${OUTPUT_DIR}/"
    tree -h "${OUTPUT_DIR}"
}

main "$@"