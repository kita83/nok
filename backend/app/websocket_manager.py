import json
import logging

from fastapi import WebSocket

logger = logging.getLogger(__name__)


class WebSocketManager:
    """WebSocket接続を管理するクラス"""

    def __init__(self):
        # アクティブな接続を管理 {user_id: websocket}
        self.active_connections: dict[str, WebSocket] = {}
        # ルームメンバーを管理 {room_id: {user_id, ...}}
        self.room_members: dict[str, set[str]] = {}

    async def connect(self, websocket: WebSocket, user_id: str):
        """WebSocket接続を受け入れる"""
        await websocket.accept()
        self.active_connections[user_id] = websocket
        logger.info(f"User {user_id} connected via WebSocket")

    async def disconnect(self, user_id: str):
        """WebSocket接続を切断する"""
        if user_id in self.active_connections:
            del self.active_connections[user_id]

        # ユーザーを全ルームから削除
        for room_id in list(self.room_members.keys()):
            if user_id in self.room_members[room_id]:
                self.room_members[room_id].discard(user_id)
                if not self.room_members[room_id]:  # ルームが空になったら削除
                    del self.room_members[room_id]

        logger.info(f"User {user_id} disconnected from WebSocket")

    async def send_personal_message(self, user_id: str, message: dict):
        """特定のユーザーにメッセージを送信"""
        if user_id in self.active_connections:
            try:
                websocket = self.active_connections[user_id]
                await websocket.send_text(json.dumps(message))
            except Exception as e:
                logger.error(f"Error sending message to {user_id}: {e}")
                # 接続が切れている場合は削除
                await self.disconnect(user_id)

    async def send_room_message(
        self, room_id: str, message: dict, exclude_user: str = None
    ):
        """ルーム内の全ユーザーにメッセージを送信"""
        if room_id not in self.room_members:
            return

        for user_id in self.room_members[room_id].copy():
            if exclude_user and user_id == exclude_user:
                continue
            await self.send_personal_message(user_id, message)

    async def broadcast_message(self, message: dict, exclude_user: str = None):
        """全接続ユーザーにメッセージを送信"""
        for user_id in list(self.active_connections.keys()):
            if exclude_user and user_id == exclude_user:
                continue
            await self.send_personal_message(user_id, message)

    def join_room(self, user_id: str, room_id: str):
        """ユーザーをルームに追加"""
        if room_id not in self.room_members:
            self.room_members[room_id] = set()
        self.room_members[room_id].add(user_id)
        logger.info(f"User {user_id} joined room {room_id}")

    def leave_room(self, user_id: str, room_id: str):
        """ユーザーをルームから削除"""
        if room_id in self.room_members:
            self.room_members[room_id].discard(user_id)
            if not self.room_members[room_id]:  # ルームが空になったら削除
                del self.room_members[room_id]
            logger.info(f"User {user_id} left room {room_id}")

    def get_online_users(self) -> list[str]:
        """オンラインユーザーのリストを取得"""
        return list(self.active_connections.keys())

    def get_room_members(self, room_id: str) -> list[str]:
        """ルームメンバーのリストを取得"""
        return list(self.room_members.get(room_id, set()))

    def is_user_online(self, user_id: str) -> bool:
        """ユーザーがオンラインかどうかを確認"""
        return user_id in self.active_connections
