# nok-ink: React Ink Frontend

React Inkを使用したターミナルベースのnok仮想オフィスクライアントです。

## 概要

nok-inkは、Matrix protocolを使用した分散型仮想オフィスアプリケーション「nok」のモダンなターミナルUIフロントエンドです。React Inkを使用してリッチなターミナル体験を提供します。

## 特徴

- **マルチペインレイアウト**: lazygitにインスパイアされた情報豊富なUI
- **Matrix統合**: 完全なMatrix protocol準拠
- **リアルタイム通信**: 即座のメッセージ配信とノック機能
- **キーボードショートカット**: 効率的なナビゲーション
- **TypeScript**: 型安全性とコード品質

## 必要環境

- Node.js 18.0.0以上
- NPM または Yarn
- Matrix homeserver（Conduit推奨）

## インストール

### 1. 依存関係のインストール

```bash
npm install
```

### 2. Conduit homeserverの起動

```bash
# プロジェクトルートディレクトリで
cd ../backend/conduit
./start_conduit.sh
```

### 3. アプリケーションの実行

```bash
# 開発モード
npm run dev

# ビルド後の実行
npm run build
npm start
```

## 使用方法

### ログイン

1. アプリケーションを起動するとログイン画面が表示されます
2. ユーザー名とパスワードを入力（例: test1 / demo1234）
3. Enterキーでログイン

### 基本操作

- **Tab**: ペイン間のフォーカス移動
- **↑↓ / k/j**: リスト内のナビゲーション
- **Enter**: アイテムの選択/ルームへの参加
- **i**: メッセージ入力モード
- **k**: ノック送信（ユーザーペインで）
- **s**: 設定画面
- **q**: アプリケーション終了
- **Esc**: 入力モードのキャンセル

### ペイン構成

#### 左ペイン（40%幅）

1. **ルーム一覧**: 参加可能なルーム表示
2. **ユーザー一覧**: オンライン状態と共にユーザー表示
3. **メッセージ履歴**: 現在のルームのメッセージ

#### 右ペイン（60%幅）

4. **ステータス/Room Visualizer**: システム情報とASCIIアート

## コマンドライン引数

```bash
# ヘルプの表示
npm run dev -- --help

# バージョン情報
npm run dev -- --version

# カスタムホームサーバー
npm run dev -- --homeserver http://localhost:6167
```

## 環境変数

- `NOK_HOMESERVER`: Matrix homeserver URL（デフォルト: http://nok.local:6167）
- `NOK_USERNAME`: デフォルトログインユーザー名
- `NOK_DEVICE_ID`: Matrix device ID

## 開発

### ディレクトリ構造

```
frontend-ink/
├── src/
│   ├── components/      # React Inkコンポーネント
│   │   ├── panes/      # 個別ペインコンポーネント
│   │   └── ...
│   ├── hooks/          # カスタムフック
│   ├── store/          # 状態管理（Zustand）
│   ├── types/          # TypeScript型定義
│   ├── utils/          # ユーティリティ関数
│   └── index.tsx       # エントリーポイント
├── package.json
├── tsconfig.json
└── README.md
```

### 開発コマンド

```bash
# 開発サーバー起動
npm run dev

# TypeScriptコンパイル
npm run build

# 型チェック
npm run type-check

# リンティング
npm run lint
```

### デバッグ

ログやエラーメッセージは画面下部のデバッグエリアに表示されます。

## トラブルシューティング

### Raw mode is not supported

このエラーが発生した場合は、適切なターミナル環境で実行してください：

```bash
# 正しいターミナルで実行
terminal -e "npm run dev"
```

### Matrix接続エラー

1. Conduit homeserverが起動していることを確認
2. ネットワーク接続を確認
3. ユーザー名/パスワードが正しいことを確認

### ターミナルサイズ

最小ターミナルサイズ: 80x24文字

## 既存実装との関係

このInkフロントエンドは既存のRust/ratatui実装と並存しています：

- **Rust版**: `src/` ディレクトリ（ratatui使用）
- **Ink版**: `frontend-ink/` ディレクトリ（React Ink使用）

両方とも同じMatrix homeserverと互換性があります。

## 関連リンク

- [nok プロジェクト概要](../docs/00_overview/project_overview_ja.md)
- [Matrix protocol](https://matrix.org/)
- [React Ink](https://github.com/vadimdemedes/ink)
- [Conduit Matrix homeserver](https://gitlab.com/famedly/conduit)

## ライセンス

MIT License