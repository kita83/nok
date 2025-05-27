# BDD E2Eテスト環境 使い方ガイド

## 概要

このプロジェクトは、**Gherkin**、**Cucumber**、**Playwright**を使用したBDD（Behavior Driven Development）によるE2Eテスト環境です。

### 使用技術

- **Gherkin**: ビジネス要件を自然言語で記述するためのDSL
- **Cucumber.js**: Gherkinで書かれたシナリオを実行するテストフレームワーク
- **Playwright**: モダンなWebアプリケーションのE2Eテストライブラリ
- **Node.js + Express**: テスト対象のサンプルWebアプリケーション

## プロジェクト構造

```
.
├── features/                    # Gherkinフィーチャーファイル
│   ├── step_definitions/        # ステップ定義
│   │   ├── user_steps.js
│   │   └── todo_steps.js
│   ├── support/                 # テストサポートファイル
│   │   └── world.js            # Cucumberワールド設定
│   ├── user_management.feature  # ユーザー管理機能のテスト
│   └── todo_management.feature  # TODO管理機能のテスト
├── src/                        # アプリケーションソース
│   ├── app.js                  # メインアプリケーション
│   └── pages/                  # Page Objectパターン
│       └── HomePage.js
├── scripts/                    # ユーティリティスクリプト
│   └── run-tests.js           # テスト実行スクリプト
├── reports/                    # テストレポート出力先
│   └── screenshots/           # 失敗時スクリーンショット
├── cucumber.js                # Cucumber設定
├── package.json
└── .env.example               # 環境変数例
```

## セットアップ

### 1. 依存関係のインストール

```bash
npm install
```

### 2. Playwrightブラウザのインストール

```bash
npm run install-browsers
```

### 3. 環境変数の設定（オプション）

```bash
cp .env.example .env
# 必要に応じて .env ファイルを編集
```

## テストの実行

### 基本的なテスト実行

```bash
# サーバーとテストを自動で実行
npm run test:with-server

# または手動でサーバーを起動してからテスト実行
npm start  # 別ターミナルで実行
npm test   # テスト実行
```

### テスト実行オプション

```bash
# デバッグ用（詳細な出力）
npm run test:debug

# HTMLレポート生成
npm run test:html

# JSONレポート生成
npm run test:json

# ヘッドレスモードを無効にして実行
HEADLESS=false npm test

# スローモーション実行（デバッグ用）
SLOW_MO=1000 npm test
```

## フィーチャーファイルの書き方

### 基本構造

```gherkin
# language: ja
フィーチャ: 機能名
  機能の説明
  ビジネス価値の説明

  背景:
    前提 共通の前提条件

  シナリオ: シナリオ名
    前提 前提条件
    もし アクション
    ならば 期待結果

  シナリオアウトライン: パラメータ化されたシナリオ
    もし "<パラメータ>" を使用してアクションを実行する
    ならば "<期待値>" が表示される

    例:
      | パラメータ | 期待値 |
      | 値1       | 結果1  |
      | 値2       | 結果2  |
```

### 利用可能なステップ

#### 共通ステップ

- `前提 ユーザーがホームページにアクセスしている`

#### ユーザー管理ステップ

- `前提 ユーザー一覧に "ユーザー名" が存在する`
- `もし ユーザーが名前 "名前"、メールアドレス "メール"、年齢 "年齢" でユーザーを追加する`
- `もし ユーザーが "ユーザー名" を削除する`
- `ならば ユーザー一覧に "ユーザー名" が表示される`
- `ならば ユーザー一覧に "ユーザー名" が表示されない`
- `ならば ユーザー数が 数値 である`

#### TODO管理ステップ

- `前提 TODO一覧に "TODOタイトル" が存在する`
- `前提 TODO一覧に "TODOタイトル" が完了状態で存在する`
- `もし ユーザーが "TODOタイトル" というTODOを "ユーザー名" に追加する`
- `もし ユーザーが "TODOタイトル" を完了状態にする`
- `もし ユーザーが "TODOタイトル" を未完了状態にする`
- `もし ユーザーが "TODOタイトル" を削除する`
- `ならば TODO一覧に "TODOタイトル" が表示される`
- `ならば TODO一覧に "TODOタイトル" が表示されない`
- `ならば "TODOタイトル" が完了状態として表示される`
- `ならば "TODOタイトル" が未完了状態として表示される`
- `ならば TODO数が 数値 である`

## Page Objectパターン

### HomePage クラス

```javascript
const HomePage = require('../src/pages/HomePage');

// 使用例
const homePage = new HomePage(page);
await homePage.addUser('名前', 'email@example.com', 25);
await homePage.addTodo('タスク名', 'ユーザー名');
```

### 主要メソッド

- `addUser(name, email, age)`: ユーザー追加
- `addTodo(title, userName)`: TODO追加
- `deleteUser(name)`: ユーザー削除
- `deleteTodo(title)`: TODO削除
- `toggleTodo(title)`: TODO完了状態切り替え
- `isUserVisible(name)`: ユーザー表示確認
- `isTodoVisible(title)`: TODO表示確認
- `isTodoCompleted(title)`: TODO完了状態確認

## 新しいテストの追加

### 1. フィーチャーファイルの作成

`features/` ディレクトリに新しい `.feature` ファイルを作成します。

```gherkin
# language: ja
フィーチャ: 新機能
  新機能の説明

  シナリオ: 新しいシナリオ
    前提 前提条件
    もし アクション
    ならば 期待結果
```

### 2. ステップ定義の作成

`features/step_definitions/` ディレクトリに対応するステップ定義ファイルを作成します。

```javascript
const { Given, When, Then } = require('@cucumber/cucumber');
const { expect } = require('chai');

Given('前提条件', async function () {
  // 実装
});

When('アクション', async function () {
  // 実装
});

Then('期待結果', async function () {
  // 実装
});
```

### 3. Page Objectの拡張

必要に応じて `src/pages/` ディレクトリに新しいPage Objectクラスを作成します。

## デバッグとトラブルシューティング

### テスト失敗時の対応

1. **スクリーンショット確認**: `reports/screenshots/` ディレクトリの失敗時スクリーンショットを確認
2. **ヘッドレスモード無効化**: `HEADLESS=false npm test` でブラウザの動作を目視確認
3. **スローモーション実行**: `SLOW_MO=1000 npm test` で動作をゆっくり確認

### よくある問題

#### ブラウザが起動しない

```bash
# Playwrightブラウザを再インストール
npm run install-browsers
```

#### タイムアウトエラー

```bash
# タイムアウト時間を延長
TIMEOUT=60000 npm test
```

#### ポートが使用中

```bash
# 別のポートを使用
PORT=3001 npm start
BASE_URL=http://localhost:3001 npm test
```

## レポート

### HTMLレポート

```bash
npm run test:html
# reports/cucumber-report.html を開く
```

### JSONレポート

```bash
npm run test:json
# reports/cucumber-report.json を確認
```

## CI/CD統合

### GitHub Actions例

```yaml
name: E2E Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
      - run: npm install
      - run: npm run install-browsers
      - run: npm run test:with-server
      - uses: actions/upload-artifact@v3
        if: failure()
        with:
          name: test-results
          path: reports/
```

## ベストプラクティス

### フィーチャーファイル

- ビジネス価値を明確に記述
- 技術的な詳細は避ける
- 再利用可能なステップを作成
- シナリオアウトラインでデータ駆動テストを活用

### ステップ定義

- 1つのステップは1つの責任を持つ
- Page Objectパターンを活用
- 適切な待機処理を実装
- エラーメッセージを分かりやすく

### Page Object

- 要素の特定方法を一箇所に集約
- ビジネスロジックに基づいたメソッド名
- 適切な抽象化レベルを維持

## 参考資料

- [Cucumber.js公式ドキュメント](https://cucumber.io/docs/cucumber/)
- [Playwright公式ドキュメント](https://playwright.dev/)
- [Gherkin構文リファレンス](https://cucumber.io/docs/gherkin/)
- [BDDベストプラクティス](https://cucumber.io/docs/bdd/)