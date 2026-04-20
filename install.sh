#!/bin/bash
set -e

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="aarch64"
else
    ARCH="x86_64"
fi

case "$OS" in
    linux)  BINARY="sinbo-linux-x86_64" ;;
    darwin)
        if [ "$ARCH" = "aarch64" ]; then
            BINARY="sinbo-macos-aarch64"
        else
            BINARY="sinbo-macos-x86_64"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Installing sinbo for $OS ($ARCH)..."

LATEST=$(curl -s https://api.github.com/repos/sinbo-cli/sinbo/releases/latest | grep '"tag_name"' | head -1 | cut -d '"' -f 4)

if [ -z "$LATEST" ]; then
    echo "Failed to fetch latest release"
    exit 1
fi

echo "Downloading $LATEST..."
curl -L -o /tmp/sinbo "https://github.com/sinbo-cli/sinbo/releases/download/$LATEST/$BINARY"
chmod +x /tmp/sinbo

echo "Installing to /usr/local/bin (may require sudo)..."
sudo mv /tmp/sinbo /usr/local/bin/sinbo

echo ""
echo "sinbo installed successfully!"
echo "Run 'sinbo --help' to get started"
