#!/bin/bash

# Conduit起動スクリプト
# Usage: ./start_conduit.sh

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# Conduitバイナリが存在しない場合はダウンロード
if [ ! -f "./conduit" ]; then
    echo "🔍 Conduit binary not found. Downloading..."
    ./download_conduit.sh
fi

echo "🚀 Starting Conduit homeserver..."
echo "📍 Server: nok.local:6167"
echo "📁 Database: ./database/"
echo "⚙️  Config: ./conduit.toml"
echo ""

# データベースディレクトリを作成
mkdir -p database

# Conduitを実行
export CONDUIT_CONFIG=./conduit.toml
exec ./conduit