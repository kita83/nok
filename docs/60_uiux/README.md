# 60_uiux

画面設計・UI/UX・デザインガイド・ワイヤーフレーム・コンポーネント設計など、ユーザーインターフェースに関する情報をまとめるディレクトリです。
- UIガイドライン
- ワイヤーフレーム
- UIコンポーネント設計
などを格納します。

## 画面仕様書の運用ルール

- 画面仕様書はこの`60_uiux`ディレクトリ配下に、画面ごとにファイルを作成してください。
  - 例: `login.md`, `user_list.md`, `order_detail.md` など
- 1画面＝1ファイルを基本とし、複雑な画面はサブディレクトリやファイル分割も検討してください。
- 画面仕様書には以下の内容を記載してください：
  - 画面レイアウト（ワイヤーフレームや図）
  - 画面項目定義（項目名、型、必須/任意、初期値など）
  - バリデーションルール
  - アクションイベント（ボタン押下時の挙動など）
  - 計算式や表示ロジック
  - 利用するビジネスロジックへの参照（`20_domain`配下の該当ファイルへのリンク）

## ビジネスロジック仕様書との関係

- 画面仕様書で利用するビジネスロジックは、`docs/20_domain/`配下にユースケースやドメイン単位でファイルを作成し、画面仕様書から参照してください。
- ビジネスロジック仕様書には「このロジックが利用される画面」への逆参照を記載すると、ドキュメント間のトレーサビリティが高まります。 