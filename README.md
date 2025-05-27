# BDD E2Eテスト環境

Gherkin、Cucumber、PlaywrightによるBDD（Behavior Driven Development）E2Eテスト環境のサンプルプロジェクトです。

## 🚀 特徴

- **Gherkin**: 自然言語でビジネス要件を記述
- **Cucumber.js**: Gherkinシナリオの実行
- **Playwright**: モダンなE2Eテストライブラリ
- **Page Object Pattern**: 保守性の高いテストコード
- **日本語対応**: 日本語でのフィーチャー記述
- **自動レポート生成**: HTML/JSONレポート出力

## 📁 プロジェクト構造

```
.
├── features/                    # Gherkinフィーチャーファイル
│   ├── step_definitions/        # ステップ定義
│   ├── support/                 # テストサポートファイル
│   ├── user_management.feature  # ユーザー管理機能テスト
│   └── todo_management.feature  # TODO管理機能テスト
├── src/                        # アプリケーションソース
│   ├── app.js                  # サンプルWebアプリ
│   └── pages/                  # Page Objectパターン
├── scripts/                    # ユーティリティスクリプト
├── reports/                    # テストレポート出力先
└── docs/                       # ドキュメント
```

## 🛠️ セットアップ

### 1. 依存関係のインストール

```bash
npm install
```

### 2. Playwrightブラウザのインストール

```bash
npm run install-browsers
```

## 🧪 テスト実行

### 基本的なテスト実行

```bash
# サーバーとテストを自動実行
npm run test:with-server

# または手動でサーバー起動後にテスト実行
npm start  # 別ターミナルで実行
npm test   # テスト実行
```

### テスト実行オプション

```bash
# デバッグ用（詳細出力）
npm run test:debug

# HTMLレポート生成
npm run test:html

# JSONレポート生成
npm run test:json

# ヘッドレスモード無効化
HEADLESS=false npm test

# スローモーション実行
SLOW_MO=1000 npm test
```

## 📝 サンプル機能

### ユーザー管理機能
- ユーザーの追加
- ユーザーの削除
- ユーザー一覧表示

### TODO管理機能
- TODOの追加
- TODOの完了状態切り替え
- TODOの削除

## 📖 ドキュメント

詳細な使い方については以下のドキュメントを参照してください：

- [BDD E2Eテスト環境 使い方ガイド](./BDD_E2E_TESTING_GUIDE.md)

## 🎯 サンプルシナリオ

```gherkin
# language: ja
フィーチャ: ユーザー管理機能
  アプリケーションのユーザーとして
  ユーザーの追加、表示、削除ができるようにしたい
  ユーザー情報を効率的に管理するため

  シナリオ: 新しいユーザーを追加する
    前提 ユーザーがホームページにアクセスしている
    もし ユーザーが名前 "山田太郎"、メールアドレス "yamada@example.com"、年齢 "28" でユーザーを追加する
    ならば ユーザー一覧に "山田太郎" が表示される
```

## 🔧 環境変数

```bash
# アプリケーション設定
PORT=3000
BASE_URL=http://localhost:3000

# Playwright設定
HEADLESS=true
SLOW_MO=0

# テスト設定
TIMEOUT=30000
```

## 📊 レポート

テスト実行後、以下の場所にレポートが生成されます：

- HTMLレポート: `reports/cucumber-report.html`
- JSONレポート: `reports/cucumber-report.json`
- スクリーンショット: `reports/screenshots/`

## 🤝 貢献

1. このリポジトリをフォーク
2. フィーチャーブランチを作成 (`git checkout -b feature/amazing-feature`)
3. 変更をコミット (`git commit -m 'Add some amazing feature'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを作成

## 📄 ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 🙏 謝辞

- [Cucumber.js](https://cucumber.io/docs/cucumber/)
- [Playwright](https://playwright.dev/)
- [Gherkin](https://cucumber.io/docs/gherkin/)

---

**注意**: このプロジェクトは学習・デモンストレーション目的で作成されています。本番環境での使用前には適切なセキュリティ対策を実装してください。
