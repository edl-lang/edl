#!/usr/bin/env bash
set -e

# Install script for EDL CLI

WORKSPACE_DIR=$(dirname "$0")
cd "$WORKSPACE_DIR"

# Build edl-cli
cargo build --release -p cli

# Copy binary to ~/.local/bin
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp target/release/cli "$INSTALL_DIR/edl"

chmod +x "$INSTALL_DIR/edl"

# Add to PATH if not present
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "\n# Add EDL to PATH" >> "$HOME/.bashrc"
    echo "export PATH=\"$INSTALL_DIR:$PATH\"" >> "$HOME/.bashrc"
    echo "Added $INSTALL_DIR to PATH in .bashrc. Restart your shell or run: export PATH=\"$INSTALL_DIR:$PATH\""
fi

echo "edl installed to $INSTALL_DIR/edl"
echo "Run 'edl run file.edl' or 'edl repl'!"