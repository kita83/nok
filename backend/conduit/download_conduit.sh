#!/bin/bash

# Conduit Matrix homeserver download script
# This script downloads the appropriate Conduit binary for the current platform

set -e

CONDUIT_VERSION="v0.7.0"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# プラットフォーム検出
case "$(uname -s)" in
    Linux*)
        case "$(uname -m)" in
            x86_64) PLATFORM="x86_64-unknown-linux-musl" ;;
            aarch64) PLATFORM="aarch64-unknown-linux-musl" ;;
            *) echo "❌ Unsupported Linux architecture: $(uname -m)"; exit 1 ;;
        esac
        ;;
    Darwin*)
        case "$(uname -m)" in
            x86_64) PLATFORM="x86_64-apple-darwin" ;;
            arm64) PLATFORM="aarch64-apple-darwin" ;;
            *) echo "❌ Unsupported macOS architecture: $(uname -m)"; exit 1 ;;
        esac
        ;;
    *)
        echo "❌ Unsupported operating system: $(uname -s)"
        echo "Please download Conduit manually from: https://conduit.rs/deploying/generic.html"
        exit 1
        ;;
esac

CONDUIT_URL="https://gitlab.com/famedly/conduit/-/releases/${CONDUIT_VERSION}/downloads/conduit-${PLATFORM}"
CONDUIT_BINARY="./conduit"

# バイナリが既に存在する場合はスキップ
if [ -f "$CONDUIT_BINARY" ]; then
    echo "✅ Conduit binary already exists: $CONDUIT_BINARY"
    exit 0
fi

echo "🔍 Platform detected: $PLATFORM"
echo "📥 Downloading Conduit ${CONDUIT_VERSION} for ${PLATFORM}..."
echo "🌐 URL: $CONDUIT_URL"

# ダウンロード実行
if command -v curl >/dev/null 2>&1; then
    curl -L --fail "$CONDUIT_URL" -o "$CONDUIT_BINARY"
elif command -v wget >/dev/null 2>&1; then
    wget "$CONDUIT_URL" -O "$CONDUIT_BINARY"
else
    echo "❌ Neither curl nor wget found. Please install one of them."
    exit 1
fi

# 実行権限を付与
chmod +x "$CONDUIT_BINARY"

# 検証
if [ -x "$CONDUIT_BINARY" ]; then
    echo "✅ Conduit downloaded successfully!"
    echo "📁 Location: $(pwd)/$CONDUIT_BINARY"
    echo "📏 Size: $(du -h "$CONDUIT_BINARY" | cut -f1)"
else
    echo "❌ Failed to download or make executable: $CONDUIT_BINARY"
    exit 1
fi

echo ""
echo "🚀 You can now start Conduit with: ./start_conduit.sh"