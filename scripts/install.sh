#!/usr/bin/env bash
# ==============================================================================
# mycheatsheet — Linux Desktop & CLI Installation Script
# Installs binary, desktop launcher, icons, and configuration.
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(dirname "$SCRIPT_DIR")"
NOTES_DIR="${REPO_DIR}/notes"
BIN_SRC="${REPO_DIR}/target/release/mycheatsheet"

INSTALL_BIN_DIR="${HOME}/.local/bin"
INSTALL_BIN="${INSTALL_BIN_DIR}/mycheatsheet"
DESKTOP_DIR="${HOME}/.local/share/applications"
DESKTOP_FILE="${DESKTOP_DIR}/mycheatsheet.desktop"
ICON_BASE_DIR="${HOME}/.local/share/icons/hicolor"
INSTALL_DATA_DIR="${HOME}/.local/share/mycheatsheet"
INSTALL_NOTES_DIR="${HOME}/.mycheatsheet/notes"
CONFIG_DIR="${HOME}/.config/mycheatsheet"
CONFIG_FILE="${CONFIG_DIR}/config.toml"

usage() {
    echo "mycheatsheet installer"
    echo ""
    echo "Usage:"
    echo "  ./scripts/install.sh [options]"
    echo ""
    echo "Options:"
    echo "  --uninstall    Remove mycheatsheet binary, icons, desktop entries, notes, and config"
    echo "  --help         Display this help message"
    echo ""
}

do_uninstall() {
    echo "Uninstalling mycheatsheet..."
    
    # Remove binary
    if [ -f "$INSTALL_BIN" ]; then
        rm -f "$INSTALL_BIN"
        echo "  [Removed] $INSTALL_BIN"
    fi

    # Remove desktop entry
    if [ -f "$DESKTOP_FILE" ]; then
        rm -f "$DESKTOP_FILE"
        echo "  [Removed] $DESKTOP_FILE"
    fi

    # Remove icons
    for size in 16 32 48 64 128 256 512; do
        icon_path="${ICON_BASE_DIR}/${size}x${size}/apps/mycheatsheet.png"
        if [ -f "$icon_path" ]; then
            rm -f "$icon_path"
            echo "  [Removed] $icon_path"
        fi
    done
    svg_path="${ICON_BASE_DIR}/scalable/apps/mycheatsheet.svg"
    if [ -f "$svg_path" ]; then
        rm -f "$svg_path"
        echo "  [Removed] $svg_path"
    fi

    # Remove installed notes data
    if [ -d "$INSTALL_NOTES_DIR" ]; then
        rm -rf "$INSTALL_NOTES_DIR"
        echo "  [Removed] $INSTALL_NOTES_DIR"
    fi
    if [ -d "${HOME}/.mycheatsheet" ]; then
        rm -rf "${HOME}/.mycheatsheet"
        echo "  [Removed] ${HOME}/.mycheatsheet"
    fi
    if [ -d "$INSTALL_DATA_DIR" ]; then
        rm -rf "$INSTALL_DATA_DIR"
        echo "  [Removed] $INSTALL_DATA_DIR"
    fi

    # Remove config
    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "  [Removed] $CONFIG_DIR"
    fi

    # Refresh desktop and icon databases
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -f -t "$ICON_BASE_DIR" >/dev/null 2>&1 || true
    fi
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
    fi

    echo "mycheatsheet uninstalled successfully."
    exit 0
}

# Parse flags
if [ "${1:-}" = "--uninstall" ]; then
    do_uninstall
elif [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    usage
    exit 0
fi

echo "Installing mycheatsheet..."

CLEANUP_TEMP=false
TEMP_DIR=""
if [ ! -d "$NOTES_DIR" ]; then
    echo "  [Fetch] Downloading repository notes and assets..."
    TEMP_DIR="$(mktemp -d)"
    git clone --depth 1 https://github.com/mnasies/mycheatsheet.git "$TEMP_DIR"
    REPO_DIR="$TEMP_DIR"
    NOTES_DIR="${REPO_DIR}/notes"
    BIN_SRC="${REPO_DIR}/target/release/mycheatsheet"
    CLEANUP_TEMP=true
fi

# 1. Ensure release binary is built
if [ ! -f "$BIN_SRC" ]; then
    echo "Building release binary with Cargo..."
    (cd "$REPO_DIR" && cargo build --release)
fi

# 2. Install binary to ~/.local/bin
mkdir -p "$INSTALL_BIN_DIR"
cp "$BIN_SRC" "$INSTALL_BIN"
chmod +x "$INSTALL_BIN"
echo "  [Installed] Binary -> $INSTALL_BIN"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$INSTALL_BIN_DIR:"* ]]; then
    echo "  [Notice] $INSTALL_BIN_DIR is not currently in your PATH."
    echo "           Add it with: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# 3. Install desktop launcher
mkdir -p "$DESKTOP_DIR"
cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=mycheatsheet
GenericName=Syntax Cheatsheet
Comment=Fast, offline cheatsheet reference for programming languages
Exec=${INSTALL_BIN}
Icon=mycheatsheet
Terminal=false
Type=Application
Categories=Development;Documentation;IDE;
Keywords=cheatsheet;reference;syntax;go;rust;cpp;javascript;
StartupNotify=true
Actions=Terminal;

[Desktop Action Terminal]
Name=Open in Terminal (TUI)
Exec=${INSTALL_BIN} --terminal
EOF
chmod 644 "$DESKTOP_FILE"
echo "  [Installed] Desktop entry -> $DESKTOP_FILE"

# 4. Install multi-resolution and scalable icons
for size in 16 32 48 64 128 256 512; do
    src_icon="${REPO_DIR}/assets/icons/icon_${size}x${size}.png"
    dest_dir="${ICON_BASE_DIR}/${size}x${size}/apps"
    if [ -f "$src_icon" ]; then
        mkdir -p "$dest_dir"
        cp "$src_icon" "${dest_dir}/mycheatsheet.png"
    fi
done

src_svg="${REPO_DIR}/assets/icon.svg"
if [ -f "$src_svg" ]; then
    dest_svg_dir="${ICON_BASE_DIR}/scalable/apps"
    mkdir -p "$dest_svg_dir"
    cp "$src_svg" "${dest_svg_dir}/mycheatsheet.svg"
fi
echo "  [Installed] Application icons -> $ICON_BASE_DIR"

# 5. Install default notes curriculum to ~/.local/share/mycheatsheet/notes
mkdir -p "$INSTALL_NOTES_DIR"
cp -r "${NOTES_DIR}/"* "$INSTALL_NOTES_DIR/"
echo "  [Installed] Default notes -> $INSTALL_NOTES_DIR"

# 6. Write default user configuration pointing to installed notes
mkdir -p "$CONFIG_DIR"
cat > "$CONFIG_FILE" << EOF
# mycheatsheet configuration
notes_dir = "${INSTALL_NOTES_DIR}"
EOF
echo "  [Configured] $CONFIG_FILE (pointing to ${INSTALL_NOTES_DIR})"

# 6. Update desktop & icon caches if tools exist
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$ICON_BASE_DIR" >/dev/null 2>&1 || true
fi
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

if [ "$CLEANUP_TEMP" = true ] && [ -n "$TEMP_DIR" ] && [ -d "$TEMP_DIR" ]; then
    rm -rf "$TEMP_DIR"
fi

echo ""
echo "Installation complete!"
echo "Run 'mycheatsheet' to launch in your browser, or 'mycheatsheet -t' for the terminal UI."

