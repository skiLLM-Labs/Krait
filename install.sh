#!/bin/bash
set -e

echo "Installing Krait Programming Language v1.0.0..."

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="krait-linux-x64"
    elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
        ASSET_NAME="krait-linux-arm64"
    else
        echo "Unsupported Linux architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        ASSET_NAME="krait-macos-x64"
    elif [ "$ARCH" = "arm64" ]; then
        ASSET_NAME="krait-macos-apple-silicon"
    else
        echo "Unsupported macOS architecture: $ARCH"
        exit 1
    fi
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
else
    # Fallback profile targets if $SHELL env var isn't fully informative
    if [ -f "$HOME/.zshrc" ]; then
        PROFILE_SCRIPT="$HOME/.zshrc"
    elif [ -f "$HOME/.bashrc" ]; then
        PROFILE_SCRIPT="$HOME/.bashrc"
    fi
fi

if [ -n "$PROFILE_SCRIPT" ]; then
    if ! grep -q "$BIN_DIR" "$PROFILE_SCRIPT"; then
        echo -e "\nexport PATH=\"$BIN_DIR:\$PATH\"" >> "$PROFILE_SCRIPT"
        echo "Added Krait to PATH in $PROFILE_SCRIPT"
    fi
fi

echo "✅ Krait installed successfully!"
if [ -n "$PROFILE_SCRIPT" ]; then
    echo "Please restart your terminal or run: source $PROFILE_SCRIPT"
else
    echo "Please manually add $BIN_DIR to your system PATH environment variable."
fi
echo "Then type 'krait' to start."
