import json
import logging
from contextlib import asynccontextmanager

import uvicorn
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.middleware.cors import CORSMiddleware

from app.database import init_db
from app.event_dispatcher import EventDispatcher
from app.routers import messages, rooms, users
from app.websocket_manager import WebSocketManager

# ロギング設定
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """アプリケーションのライフサイクル管理"""
    # 起動時の処理
    await init_db()
    logger.info("Database initialized")
    yield
    # 終了時の処理（必要に応じて追加）


app = FastAPI(title="nok Backend API", version="1.0.0", lifespan=lifespan)

# CORS設定
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # 開発時は全許可、本番では制限する
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# WebSocketマネージャーとイベントディスパッチャー
websocket_manager = WebSocketManager()
event_dispatcher = EventDispatcher(websocket_manager)

# WebSocketManagerを依存性注入で利用可能にする
def get_websocket_manager():
    return websocket_manager

# ルーター登録
app.include_router(rooms.router, prefix="/api/rooms", tags=["rooms"])
app.include_router(users.router, prefix="/api/users", tags=["users"])
app.include_router(messages.router, prefix="/api/messages", tags=["messages"])


@app.get("/")
async def root():
    return {"message": "nok Backend API", "version": "1.0.0"}


@app.get("/health")
async def health_check():
    return {"status": "healthy"}


@app.websocket("/ws/{user_id}")
async def websocket_endpoint(websocket: WebSocket, user_id: str):
    """WebSocket接続エンドポイント"""
    try:
        await websocket_manager.connect(websocket, user_id)

        # ユーザーオンライン状態を通知
        await event_dispatcher.dispatch_user_status_change(user_id, "online")

        while True:
            # クライアントからのメッセージを受信
            data = await websocket.receive_text()
            message_data = json.loads(data)

            # メッセージタイプに応じて処理
            await handle_websocket_message(user_id, message_data)

    except WebSocketDisconnect:
        await websocket_manager.disconnect(user_id)
        # オフライン状態への変更を安全に実行
        try:
            await event_dispatcher.dispatch_user_status_change(user_id, "offline")
        except Exception as status_error:
            logger.error(f"Failed to update user status for {user_id}: {status_error}")
        logger.info(f"User {user_id} disconnected")
    except Exception as e:
        logger.error(f"WebSocket error for user {user_id}: {e}")
        await websocket_manager.disconnect(user_id)
        # エラー時もオフライン状態への変更を試行
        try:
            await event_dispatcher.dispatch_user_status_change(user_id, "offline")
        except Exception as status_error:
            logger.error(f"Failed to update user status for {user_id} after error: {status_error}")


async def handle_websocket_message(user_id: str, message_data: dict):
    """WebSocketメッセージの処理"""
    message_type = message_data.get("type")

    if message_type == "knock":
        target_user_id = message_data.get("target_user_id")
        await event_dispatcher.dispatch_knock(user_id, target_user_id)

    elif message_type == "message":
        room_id = message_data.get("room_id")
        content = message_data.get("content")
        await event_dispatcher.dispatch_message(user_id, room_id, content)

    elif message_type == "join_room":
        room_id = message_data.get("room_id")
        await event_dispatcher.dispatch_room_join(user_id, room_id)

    elif message_type == "leave_room":
        room_id = message_data.get("room_id")
        await event_dispatcher.dispatch_room_leave(user_id, room_id)

    elif message_type == "user_status":
        status = message_data.get("status")
        if status in ["online", "away", "busy", "offline"]:
            await event_dispatcher.dispatch_user_status_change(user_id, status)


if __name__ == "__main__":
    uvicorn.run("main:app", host="0.0.0.0", port=8001, reload=True, log_level="info")
