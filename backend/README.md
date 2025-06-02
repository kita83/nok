# nok Backend

nokアプリケーションのバックエンドAPI。FastAPIとWebSocketを使用してリアルタイムコミュニケーション機能を提供します。

## 機能

- **REST API**: ユーザー、ルーム、メッセージ管理
- **WebSocket**: リアルタイムメッセージング、ノック機能、ステータス同期
- **データベース**: SQLite（開発）/ PostgreSQL（本番）
- **認証**: 後で実装予定（現在は認証なし）

## アーキテクチャ

```
backend/
├── app/
│   ├── __init__.py
│   ├── models.py          # SQLAlchemyモデル & Pydanticスキーマ
│   ├── database.py        # データベース接続設定
│   ├── websocket_manager.py  # WebSocket接続管理
│   ├── event_dispatcher.py  # イベント配信システム
│   └── routers/
│       ├── __init__.py
│       ├── users.py       # ユーザー管理API
│       ├── rooms.py       # ルーム管理API
│       └── messages.py    # メッセージ管理API
├── main.py               # FastAPIアプリケーション
├── setup_data.py         # 初期データセットアップ
├── requirements.txt      # Python依存関係
└── README.md
```

## セットアップ

### 1. 依存関係のインストール

```bash
cd backend
pip install -r requirements.txt
```

### 2. データベースと初期データのセットアップ

```bash
python setup_data.py
```

### 3. サーバーの起動

```bash
python main.py
```

または

```bash
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

## API エンドポイント

### ヘルスチェック
- `GET /` - API情報
- `GET /health` - ヘルスチェック

### ユーザー管理
- `GET /api/users/` - ユーザー一覧
- `POST /api/users/` - ユーザー作成
- `GET /api/users/{user_id}` - ユーザー詳細
- `PUT /api/users/{user_id}` - ユーザー更新
- `DELETE /api/users/{user_id}` - ユーザー削除
- `GET /api/users/online/list` - オンラインユーザー一覧

### ルーム管理
- `GET /api/rooms/` - ルーム一覧
- `POST /api/rooms/` - ルーム作成
- `GET /api/rooms/{room_id}` - ルーム詳細
- `PUT /api/rooms/{room_id}` - ルーム更新
- `DELETE /api/rooms/{room_id}` - ルーム削除
- `GET /api/rooms/{room_id}/members` - ルームメンバー一覧
- `POST /api/rooms/{room_id}/join` - ルーム参加
- `POST /api/rooms/{room_id}/leave` - ルーム退出

### メッセージ管理
- `GET /api/messages/` - メッセージ一覧（フィルタ可能）
- `POST /api/messages/` - メッセージ送信
- `GET /api/messages/{message_id}` - メッセージ詳細
- `DELETE /api/messages/{message_id}` - メッセージ削除
- `GET /api/messages/room/{room_id}/history` - ルームメッセージ履歴
- `GET /api/messages/dm/{user1_id}/{user2_id}` - DMメッセージ履歴

### WebSocket
- `WS /ws/{user_id}` - WebSocket接続

## WebSocket メッセージ形式

### クライアント → サーバー

```json
{
  "type": "knock",
  "target_user_id": "user-uuid"
}

{
  "type": "message",
  "room_id": "room-uuid",
  "content": "Hello World!"
}

{
  "type": "join_room",
  "room_id": "room-uuid"
}

{
  "type": "leave_room",
  "room_id": "room-uuid"
}
```

### サーバー → クライアント

```json
{
  "type": "knock",
  "sender_id": "user-uuid",
  "sender_name": "Alice",
  "content": "Alice がノックしました",
  "timestamp": "2023-12-01T10:00:00Z"
}

{
  "type": "message",
  "message_id": "msg-uuid",
  "sender_id": "user-uuid",
  "sender_name": "Alice",
  "room_id": "room-uuid",
  "room_name": "メインルーム",
  "content": "Hello World!",
  "timestamp": "2023-12-01T10:00:00Z"
}

{
  "type": "user_status",
  "user_id": "user-uuid",
  "user_name": "Alice",
  "status": "online",
  "timestamp": "2023-12-01T10:00:00Z"
}
```

## 開発

### API ドキュメント

サーバー起動後、以下のURLでSwagger UIにアクセスできます：
- `http://localhost:8000/docs` - Swagger UI
- `http://localhost:8000/redoc` - ReDoc

### データベースファイル

開発時は `nok.db` ファイルがSQLiteデータベースとして作成されます。

### ログ

アプリケーションログはコンソールに出力されます。ログレベルは `INFO` に設定されています。

## 今後の実装予定

- [ ] ユーザー認証（JWT）
- [ ] Redisセッションストア
- [ ] 本番環境用の設定（PostgreSQL、環境変数）
- [ ] テストケース
- [ ] Docker化
- [ ] ファイルアップロード機能
- [ ] 通知機能の拡張

## テスト

WebSocketの動作確認は、以下のようなJavaScriptコードでテストできます：

```javascript
const ws = new WebSocket('ws://localhost:8000/ws/test-user-id');

ws.onopen = function() {
    console.log('WebSocket接続開始');
    
    // ノックを送信
    ws.send(JSON.stringify({
        type: 'knock',
        target_user_id: 'target-user-id'
    }));
};

ws.onmessage = function(event) {
    const message = JSON.parse(event.data);
    console.log('受信:', message);
};
``` 