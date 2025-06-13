#!/bin/bash

# Conduit起動スクリプト
# Usage: ./start_conduit.sh

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

# 環境変数ファイルの読み込み
if [ -f ".env" ]; then
    echo "📄 Loading environment variables from .env"
    export $(grep -v '^#' .env | xargs)
elif [ -f ".env.example" ]; then
    echo "⚠️  Warning: .env file not found. Using default values."
    echo "   Please copy .env.example to .env and set secure values:"
    echo "   cp .env.example .env"
    echo ""
fi

# 必須環境変数のデフォルト値設定
export NOK_REGISTRATION_TOKEN="${NOK_REGISTRATION_TOKEN:-nokdev_registration_token}"
export NOK_EMERGENCY_PASSWORD="${NOK_EMERGENCY_PASSWORD:-nokdev123}"
export NOK_SERVER_NAME="${NOK_SERVER_NAME:-nok.local}"
export NOK_PORT="${NOK_PORT:-6167}"
export NOK_BIND="${NOK_BIND:-127.0.0.1}"
export NOK_DATABASE_PATH="${NOK_DATABASE_PATH:-./database/}"

# セキュリティ警告
if [ "$NOK_REGISTRATION_TOKEN" = "nokdev_registration_token" ] || [ "$NOK_EMERGENCY_PASSWORD" = "nokdev123" ]; then
    echo "🔒 SECURITY WARNING: Using default credentials!"
    echo "   For production use, please set secure values in .env file"
    echo ""
fi

# Conduitバイナリが存在しない場合はダウンロード
if [ ! -f "./conduit" ]; then
    echo "🔍 Conduit binary not found. Downloading..."
    ./download_conduit.sh
fi

echo "🚀 Starting Conduit homeserver..."
echo "📍 Server: ${NOK_SERVER_NAME}:${NOK_PORT}"
echo "📁 Database: ${NOK_DATABASE_PATH}"
echo "⚙️  Config: ./conduit.toml"
echo ""

# データベースディレクトリを作成
mkdir -p "${NOK_DATABASE_PATH}"

# 設定ファイルを環境変数で生成
echo "⚙️  Generating configuration from template..."
envsubst < conduit.toml.template > conduit.toml

# Conduitを実行
export CONDUIT_CONFIG=./conduit.toml
exec ./conduit