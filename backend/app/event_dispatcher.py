import logging
from datetime import datetime

from .database import AsyncSessionLocal
from .models import Message, Room, User
from .websocket_manager import WebSocketManager

logger = logging.getLogger(__name__)


class EventDispatcher:
    """イベントの配信を管理するクラス"""

    def __init__(self, websocket_manager: WebSocketManager):
        self.websocket_manager = websocket_manager

    async def dispatch_knock(self, sender_id: str, target_user_id: str):
        """ノックイベントを配信"""
        async with AsyncSessionLocal() as session:
            # 送信者の情報を取得
            sender = await session.get(User, sender_id)
            if not sender:
                logger.error(f"Sender {sender_id} not found")
                return

            # ノックメッセージをデータベースに保存
            knock_message = Message(
                content=f"{sender.name} がノックしました",
                message_type="knock",
                sender_id=sender_id,
                target_user_id=target_user_id,
            )
            session.add(knock_message)
            await session.commit()
            await session.refresh(knock_message)  # created_at を取得

            # WebSocketでターゲットユーザーに送信
            message = {
                "type": "knock",
                "sender_id": sender_id,
                "sender_name": sender.name,
                "target_user_id": target_user_id,
                "content": knock_message.content,
                "timestamp": datetime.utcnow().isoformat(),
            }

            await self.websocket_manager.send_personal_message(target_user_id, message)
            logger.info(f"Knock sent from {sender_id} to {target_user_id}")

    async def dispatch_message(self, sender_id: str, room_id: str, content: str):
        """メッセージイベントを配信"""
        async with AsyncSessionLocal() as session:
            # 送信者の情報を取得
            sender = await session.get(User, sender_id)
            if not sender:
                logger.error(f"Sender {sender_id} not found")
                return

            # ルームの存在確認
            room = await session.get(Room, room_id)
            if not room:
                logger.error(f"Room {room_id} not found")
                return

            # メッセージをデータベースに保存
            message_record = Message(
                content=content,
                message_type="text",
                sender_id=sender_id,
                room_id=room_id,
            )
            session.add(message_record)
            await session.commit()
            await session.refresh(message_record)

            # WebSocketでルーム内の全ユーザーに送信
            message = {
                "type": "message",
                "message_id": message_record.id,
                "sender_id": sender_id,
                "sender_name": sender.name,
                "room_id": room_id,
                "room_name": room.name,
                "content": content,
                "timestamp": message_record.created_at.isoformat(),
            }

            await self.websocket_manager.send_room_message(
                room_id, message, exclude_user=sender_id
            )
            logger.info(f"Message sent from {sender_id} to room {room_id}")

    async def dispatch_user_status_change(self, user_id: str, status: str):
        """ユーザーステータス変更イベントを配信"""
        try:
            # まずWebSocketManagerでリアルタイムステータスを更新
            self.websocket_manager.update_user_status(user_id, status)

            async with AsyncSessionLocal() as session:
                # ユーザーのステータスを更新（DBは同期用）
                user = await session.get(User, user_id)
                if not user:
                    logger.error(f"User {user_id} not found")
                    return

                user.status = status
                user.updated_at = datetime.utcnow()
                await session.commit()

                # 全ユーザーにステータス変更を通知
                message = {
                    "type": "user_status",
                    "user_id": user_id,
                    "user_name": user.name,
                    "status": status,
                    "timestamp": datetime.utcnow().isoformat(),
                }

                await self.websocket_manager.broadcast_message(
                    message, exclude_user=user_id
                )
                logger.info(f"User {user_id} status changed to {status}")
        except Exception as e:
            logger.error(f"Failed to dispatch user status change for {user_id}: {e}")

    async def dispatch_room_join(self, user_id: str, room_id: str):
        """ルーム参加イベントを配信"""
        async with AsyncSessionLocal() as session:
            user = await session.get(User, user_id)
            room = await session.get(Room, room_id)

            if not user or not room:
                logger.error(f"User {user_id} or Room {room_id} not found")
                return

            # ユーザーをルームに追加（データベース）
            if user not in room.members:
                room.members.append(user)
                await session.commit()

            # WebSocketルームに追加
            self.websocket_manager.join_room(user_id, room_id)

            # ルーム内の他のユーザーに通知
            message = {
                "type": "room_join",
                "user_id": user_id,
                "user_name": user.name,
                "room_id": room_id,
                "room_name": room.name,
                "timestamp": datetime.utcnow().isoformat(),
            }

            await self.websocket_manager.send_room_message(
                room_id, message, exclude_user=user_id
            )
            logger.info(f"User {user_id} joined room {room_id}")

    async def dispatch_room_leave(self, user_id: str, room_id: str):
        """ルーム退出イベントを配信"""
        async with AsyncSessionLocal() as session:
            user = await session.get(User, user_id)
            room = await session.get(Room, room_id)

            if not user or not room:
                logger.error(f"User {user_id} or Room {room_id} not found")
                return

            # ユーザーをルームから削除（データベース）
            if user in room.members:
                room.members.remove(user)
                await session.commit()

            # WebSocketルームから削除
            self.websocket_manager.leave_room(user_id, room_id)

            # ルーム内の他のユーザーに通知
            message = {
                "type": "room_leave",
                "user_id": user_id,
                "user_name": user.name,
                "room_id": room_id,
                "room_name": room.name,
                "timestamp": datetime.utcnow().isoformat(),
            }

            await self.websocket_manager.send_room_message(room_id, message)
            logger.info(f"User {user_id} left room {room_id}")
