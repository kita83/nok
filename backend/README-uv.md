# NOK Backend - UV Development Environment

このプロジェクトはPythonの高速パッケージマネージャー「uv」を使用して依存関係を管理しています。

## セットアップ

### 1. uvのインストール
```bash
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# または Homebrew（macOS）
brew install uv
```

### 2. 依存関係のインストール
```bash
# プロジェクトディレクトリで実行
uv sync
```

## 使用方法

### アプリケーションの起動
```bash
# サーバーを起動
uv run python main.py
```

### 開発用コマンド

```bash
# コードフォーマット
uv run ruff format .

# リンティング（自動修正付き）
uv run ruff check --fix .

# リンティングのみ（修正なし）
uv run ruff check .

# フォーマット + リンティング（推奨）
uv run ruff format . && uv run ruff check --fix .

# テスト実行
uv run pytest
```

### 新しい依存関係の追加

```bash
# 本番依存関係の追加
uv add package-name

# 開発依存関係の追加
uv add --dev package-name

# 特定のバージョンを指定
uv add "package-name>=1.0.0"
```

### 依存関係の削除

```bash
uv remove package-name
```

### 仮想環境の管理

```bash
# 仮想環境の状態確認
uv pip list

# 仮想環境の場所確認
uv venv --show-path

# 依存関係の更新
uv sync --upgrade
```

## プロジェクト構成

- `pyproject.toml`: プロジェクトの設定と依存関係
- `uv.lock`: ロックファイル（コミット必須）
- `.venv/`: 仮想環境（.gitignoreに含める）

## 使用ツールの利点

### uv
- **高速**: pipの10-100倍高速なインストール
- **確実性**: 依存関係の解決が確実
- **統一性**: パッケージ管理とプロジェクト管理を統一
- **互換性**: pipとの完全互換性

### Ruff
- **超高速**: RustベースでBlack/isort/flake8より10-100倍高速
- **オールインワン**: フォーマッター + リンター + import整理が統合
- **互換性**: Black、isort、flake8のルールと互換
- **設定が簡単**: 単一の設定ファイルで全て管理 