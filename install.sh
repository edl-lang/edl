#!/usr/bin/sh
set -e

# EDL Install Script

WORKSPACE_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$WORKSPACE_DIR"

# Build the CLI with cargo
echo "Building EDL Language..."
cargo build --release -p cli

# Determine binary name and install location
BINARY_SRC="target/release/cli"
INSTALL_DIR="$HOME/.local/bin"
BINARY_DEST="$INSTALL_DIR/edl"

# Ensure install directory exists
mkdir -p "$INSTALL_DIR"

# Copy and set permissions
cp "$BINARY_SRC" "$BINARY_DEST"
chmod +x "$BINARY_DEST"

# Add to PATH if not already present
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    SHELL_RC=""
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.bashrc"
    fi
    echo "\n# Add EDL to PATH" >> "$SHELL_RC"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
    echo "Added $INSTALL_DIR to PATH in $SHELL_RC. Restart your shell or run: export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo "✅ EDL installed to $BINARY_DEST"
echo "➡️  Run 'edl run file.edl' or 'edl repl' to get started!"