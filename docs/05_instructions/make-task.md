# タスク作成ガイド

## 概要

このガイドは、AI（LLM）を活用して効率的に開発タスクを作成・分割するための指示とプロンプトを提供します。CMSプロジェクトのタスクを明確で実行可能な形式で作成するために使用してください。

## 使用シーン

- 機能実装タスクの作成
- バグ修正タスクの作成
- リファクタリングタスクの作成
- テストタスクの作成
- ドキュメント作成タスクの作成

## 基本プロンプト

```
# 指示

あなたは経験豊富なプロジェクトマネージャーです。以下の情報に基づいて、[タスク対象]の実装に必要なタスクリストを作成してください。

## タスク対象
[タスク対象の名前と簡単な説明]

## 関連する仕様・要件
[関連する仕様書や要件の概要]

## 技術スタック
[使用する技術スタックの情報]

## 出力形式
以下の構造に従ったタスクリストを作成してください：

1. タスク概要
2. 前提条件と依存関係
3. 詳細タスク（サブタスクのリスト）
   - 各サブタスクには以下を含める：
     - タスクID
     - タスク名
     - 説明
     - 予想工数（ストーリーポイントまたは時間）
     - 担当ロール（バックエンド/フロントエンド/インフラ等）
     - 受け入れ基準
4. 技術的な注意点
5. テスト要件
```

## 具体例：コンテンツ公開ワークフロー実装タスク

```
# 指示

あなたは経験豊富なプロジェクトマネージャーです。以下の情報に基づいて、CMSのコンテンツ公開ワークフロー機能の実装に必要なタスクリストを作成してください。

## タスク対象
CMSのコンテンツ公開ワークフロー機能 - コンテンツの作成から承認、公開までのプロセスを管理する機能

## 関連する仕様・要件
- コンテンツは「ドラフト」「レビュー中」「承認済み」「公開」「アーカイブ」の状態を持つ
- 権限に応じて、状態を変更できるユーザーが制限される
- 公開予定日時を設定できる
- 承認者へ通知が送信される
- 変更履歴が記録される
- コメントを付けてレビュー依頼ができる

## 技術スタック
- バックエンド: Python, FastAPI, PostgreSQL
- フロントエンド: TypeScript, React, Next.js
- インフラ: Docker, Google Cloud Platform

## 出力形式
以下の構造に従ったタスクリストを作成してください：

1. タスク概要
2. 前提条件と依存関係
3. 詳細タスク（サブタスクのリスト）
   - 各サブタスクには以下を含める：
     - タスクID
     - タスク名
     - 説明
     - 予想工数（ストーリーポイントまたは時間）
     - 担当ロール（バックエンド/フロントエンド/インフラ等）
     - 受け入れ基準
4. 技術的な注意点
5. テスト要件
```

## タスク作成のベストプラクティス

1. **適切な粒度でタスクを分割する**
   - 1タスクは1-2日で完了できる大きさが理想的
   - 大きすぎるタスクは細分化する
   - 小さすぎるタスクはグループ化する

2. **明確な受け入れ基準を設定する**
   - タスク完了の判断基準を明確にする
   - 検証可能な形で記述する

3. **依存関係を明確にする**
   - タスク間の依存関係を明示する
   - 並行して進められるタスクを識別する

4. **技術的な詳細を適切に含める**
   - 実装に必要な技術的情報を提供する
   - 過度に実装を制約しない

5. **リスクと注意点を記載する**
   - 潜在的な問題や課題を事前に識別する
   - 回避策や対応方法を提案する

6. **テスト要件を含める**
   - タスクの検証方法を明確にする
   - 必要なテストケースを示す

7. **担当ロールを明確にする**
   - タスクを担当するべき役割を指定する
   - 必要なスキルセットを示す

## AIへの効果的な指示のコツ

1. **具体的なコンテキストを提供する**
   - プロジェクトの状況や背景を説明する
   - 既存のコードベースや制約について情報を提供する

2. **優先順位と依存関係を明確にする**
   - タスクの重要度や順序を示す
   - ブロッカーとなる依存関係を強調する

3. **期待する詳細レベルを指定する**
   - 高レベルの概要か詳細な分解かを明示する
   - 例を示して期待する形式を伝える

4. **チームの構成や経験レベルを伝える**
   - チームの規模やスキルセットに合わせたタスク分割を依頼する
   - 新人メンバーがいる場合は、より詳細な説明を求める

5. **既存のタスク管理システムの形式に合わせる**
   - JIRAやTrelloなど使用しているツールの形式を指定する
   - 必要なフィールドや情報を明示する

## タスク分割の例

### 大きすぎるタスク（避けるべき）
「コンテンツ公開ワークフロー機能を実装する」

### 適切に分割されたタスク（推奨）
1. データベースにワークフロー状態とトランジションテーブルを追加する
2. コンテンツ状態変更APIエンドポイントを実装する
3. 権限に基づく状態変更の制御ロジックを実装する
4. 公開スケジューリング機能のバックエンドロジックを実装する
5. 通知システムとの連携を実装する
6. ワークフロー状態表示UIコンポーネントを実装する
7. 状態変更操作のUIを実装する
8. 公開スケジュール設定UIを実装する
9. コメント機能のUIとバックエンドを実装する
10. E2Eテストを作成する

## 注意事項

- タスクは実装者が理解できる明確さで記述してください。
- 技術的な詳細と業務的な目的のバランスを取ってください。
- タスクの見積もりは経験に基づいて行い、不確実性がある場合は範囲で示してください。
- AIの出力は常に人間のレビューを経てから採用してください。
- タスクは固定ではなく、実装の進行に伴い調整が必要な場合があります。
