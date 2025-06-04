#!/bin/bash

# Conduit起動スクリプト
# Usage: ./start_conduit.sh

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

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