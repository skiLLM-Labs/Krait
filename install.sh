#!/bin/bash
set -e

echo "🐍 Installing Krait Programming Language v0.1.0..."

# Detect OS
OS="$(uname -s)"
if [ "$OS" = "Linux" ]; then
    ASSET_NAME="krait-linux-amd64"
elif [ "$OS" = "Darwin" ]; then
    ASSET_NAME="krait-macos-amd64"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Setup Directories
KRAIT_DIR="$HOME/.krait"
BIN_DIR="$KRAIT_DIR/bin"
mkdir -p "$BIN_DIR"

# Fetch latest release URL from GitHub API
echo "Fetching latest release from skiLLM-Labs/Krait..."
LATEST_URL=$(curl -s https://api.github.com/repos/skiLLM-Labs/Krait/releases/latest \
    | grep "browser_download_url.*$ASSET_NAME" \
    | cut -d : -f 2,3 \
    | tr -d \")

if [ -z "$LATEST_URL" ]; then
    echo "Error: Could not find binary for $ASSET_NAME."
    exit 1
fi

# Download binary
echo "Downloading $ASSET_NAME..."
curl -q --fail --location --progress-bar "$LATEST_URL" --output "$BIN_DIR/krait"

# Make executable
chmod +x "$BIN_DIR/krait"

# Add to PATH (bash/zsh)
PROFILE_SCRIPT=""
if [[ "$SHELL" == *"zsh"* ]]; then
    PROFILE_SCRIPT="$HOME/.zshrc"
elif [[ "$SHELL" == *"bash"* ]]; then
    PROFILE_SCRIPT="$HOME/.bashrc"
fi

if [ -n "$PROFILE_SCRIPT" ]; then
    if ! grep -q "$BIN_DIR" "$PROFILE_SCRIPT"; then
        echo -e "\nexport PATH=\"$BIN_DIR:\$PATH\"" >> "$PROFILE_SCRIPT"
        echo "Added Krait to PATH in $PROFILE_SCRIPT"
    fi
fi

echo "✅ Krait installed successfully!"
echo "Please restart your terminal or run: source $PROFILE_SCRIPT"
echo "Then type 'krait' to start."