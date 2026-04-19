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
    linux)
        BINARY="sinbo-linux-x86_64"
        LSP_BINARY="sinbo-lsp-linux-x86_64"
        ;;
    darwin)
        if [ "$ARCH" = "aarch64" ]; then
            BINARY="sinbo-macos-aarch64"
            LSP_BINARY="sinbo-lsp-macos-aarch64"
        else
            BINARY="sinbo-macos-x86_64"
            LSP_BINARY="sinbo-lsp-macos-x86_64"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

LATEST=$(curl -s https://api.github.com/repos/opmr0/sinbo/releases/latest | grep '"tag_name"' | head -1 | cut -d '"' -f 4)

if [ -z "$LATEST" ]; then
    echo "Failed to fetch latest release"
    exit 1
fi

echo "Installing sinbo $LATEST for $OS ($ARCH)..."

curl -L -o /tmp/sinbo "https://github.com/opmr0/sinbo/releases/download/$LATEST/$BINARY.tar.gz" | tar xz -C /tmp
chmod +x /tmp/sinbo
sudo mv /tmp/sinbo /usr/local/bin/sinbo

echo "sinbo installed successfully!"

echo "Installing sinbo-lsp..."

curl -L -o /tmp/sinbo-lsp "https://github.com/opmr0/sinbo/releases/download/$LATEST/$LSP_BINARY.tar.gz" | tar xz -C /tmp
chmod +x /tmp/sinbo-lsp
sudo mv /tmp/sinbo-lsp /usr/local/bin/sinbo-lsp

echo "sinbo-lsp installed successfully!"
echo ""
echo "Run 'sinbo --help' to get started"
echo "See https://github.com/opmr0/sinbo/sinbo-lsp for editor setup"