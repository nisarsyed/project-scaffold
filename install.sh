#!/bin/sh
# Project Scaffold Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/nisarsyed/project-scaffold/main/install.sh | sh

set -e

REPO="nisarsyed/project-scaffold"
BINARY_NAME="scaffold"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect OS
OS="$(uname -s)"
case "$OS" in
    Linux*)  OS="unknown-linux-gnu" ;;
    Darwin*) OS="apple-darwin" ;;
    MINGW*|MSYS*|CYGWIN*) OS="pc-windows-msvc" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    arm64|aarch64) ARCH="aarch64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

TARGET="${ARCH}-${OS}"
echo "Detected platform: $TARGET"

# Get latest release
LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$LATEST" ]; then
    echo "Error: Could not fetch latest release"
    exit 1
fi
echo "Latest version: $LATEST"

# Download
if [ "$OS" = "pc-windows-msvc" ]; then
    ARCHIVE="scaffold-${TARGET}.zip"
else
    ARCHIVE="scaffold-${TARGET}.tar.gz"
fi

URL="https://github.com/$REPO/releases/download/$LATEST/$ARCHIVE"
echo "Downloading $URL..."

TMPDIR=$(mktemp -d)
cd "$TMPDIR"

if [ "$OS" = "pc-windows-msvc" ]; then
    curl -fsSL "$URL" -o "$ARCHIVE"
    unzip -q "$ARCHIVE"
else
    curl -fsSL "$URL" | tar xz
fi

# Install
echo "Installing to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_NAME" "$INSTALL_DIR/"
else
    sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
fi

# Cleanup
cd - > /dev/null
rm -rf "$TMPDIR"

echo ""
echo "✓ $BINARY_NAME installed successfully!"
echo ""
echo "Run 'scaffold --help' to get started."
