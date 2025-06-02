from typing import Optional

from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy import desc, select
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy.orm import selectinload

from ..database import get_db
from ..models import Message, MessageCreate, MessageResponse, Room, User

router = APIRouter()


@router.get("/", response_model=list[MessageResponse])
async def list_messages(
    room_id: Optional[str] = Query(None, description="ルームIDでフィルタ"),
    user_id: Optional[str] = Query(None, description="ユーザーIDでフィルタ"),
    limit: int = Query(50, description="取得件数", le=100),
    offset: int = Query(0, description="オフセット"),
    db: AsyncSession = Depends(get_db),
):
    """メッセージ一覧を取得"""
    query = (
        select(Message)
        .options(selectinload(Message.sender))
        .order_by(desc(Message.created_at))
    )

    # フィルタを適用
    if room_id:
        query = query.where(Message.room_id == room_id)
    if user_id:
        query = query.where(Message.sender_id == user_id)

    # ページネーション
    query = query.limit(limit).offset(offset)

    result = await db.execute(query)
    messages = result.scalars().all()

    # レスポンス形式に変換
    message_responses = []
    for message in messages:
        message_response = MessageResponse.from_orm(message)
        message_response.sender_name = message.sender.name if message.sender else None
        message_responses.append(message_response)

    return message_responses


@router.get("/{message_id}", response_model=MessageResponse)
async def get_message(message_id: str, db: AsyncSession = Depends(get_db)):
    """特定のメッセージを取得"""
    result = await db.execute(
        select(Message)
        .options(selectinload(Message.sender))
        .where(Message.id == message_id)
    )
    message = result.scalars().first()

    if not message:
        raise HTTPException(status_code=404, detail="Message not found")

    message_response = MessageResponse.from_orm(message)
    message_response.sender_name = message.sender.name if message.sender else None
    return message_response


@router.post("/", response_model=MessageResponse)
async def create_message(
    message_data: MessageCreate,
    sender_id: str = Query(..., description="送信者のユーザーID"),
    db: AsyncSession = Depends(get_db)
):
    """新しいメッセージを作成"""
    # 送信者の存在確認
    sender = await db.get(User, sender_id)
    if not sender:
        raise HTTPException(status_code=404, detail="Sender not found")

    # ルームメッセージの場合、ルームの存在確認
    if message_data.room_id:
        room = await db.get(Room, message_data.room_id)
        if not room:
            raise HTTPException(status_code=404, detail="Room not found")

    # DMの場合、ターゲットユーザーの存在確認
    if message_data.target_user_id:
        target_user = await db.get(User, message_data.target_user_id)
        if not target_user:
            raise HTTPException(status_code=404, detail="Target user not found")

    # メッセージを作成
    message = Message(
        content=message_data.content,
        message_type=message_data.message_type,
        sender_id=sender_id,
        room_id=message_data.room_id,
        target_user_id=message_data.target_user_id,
    )

    db.add(message)
    await db.commit()
    await db.refresh(message)

    # 送信者情報を含めてレスポンス
    message_response = MessageResponse.from_orm(message)
    message_response.sender_name = sender.name
    return message_response


@router.delete("/{message_id}")
async def delete_message(message_id: str, db: AsyncSession = Depends(get_db)):
    """メッセージを削除"""
    message = await db.get(Message, message_id)
    if not message:
        raise HTTPException(status_code=404, detail="Message not found")

    await db.delete(message)
    await db.commit()
    return {"message": "Message deleted successfully"}


@router.get("/room/{room_id}/history", response_model=list[MessageResponse])
async def get_room_message_history(
    room_id: str,
    limit: int = Query(50, description="取得件数", le=100),
    before: Optional[str] = Query(None, description="この日時より前のメッセージを取得"),
    db: AsyncSession = Depends(get_db),
):
    """ルームのメッセージ履歴を取得"""
    # ルームの存在確認
    room = await db.get(Room, room_id)
    if not room:
        raise HTTPException(status_code=404, detail="Room not found")

    query = (
        select(Message)
        .options(selectinload(Message.sender))
        .where(Message.room_id == room_id)
        .order_by(desc(Message.created_at))
    )

    # before パラメータがある場合、その日時より前のメッセージを取得
    if before:
        try:
            from datetime import datetime

            before_dt = datetime.fromisoformat(before.replace("Z", "+00:00"))
            query = query.where(Message.created_at < before_dt)
        except ValueError:
            raise HTTPException(status_code=400, detail="Invalid date format")

    query = query.limit(limit)

    result = await db.execute(query)
    messages = result.scalars().all()

    # レスポンス形式に変換
    message_responses = []
    for message in messages:
        message_response = MessageResponse.from_orm(message)
        message_response.sender_name = message.sender.name if message.sender else None
        message_responses.append(message_response)

    return message_responses


@router.get("/dm/{user1_id}/{user2_id}", response_model=list[MessageResponse])
async def get_dm_history(
    user1_id: str,
    user2_id: str,
    limit: int = Query(50, description="取得件数", le=100),
    before: Optional[str] = Query(None, description="この日時より前のメッセージを取得"),
    db: AsyncSession = Depends(get_db),
):
    """2人のユーザー間のDM履歴を取得"""
    # ユーザーの存在確認
    user1 = await db.get(User, user1_id)
    user2 = await db.get(User, user2_id)
    if not user1 or not user2:
        raise HTTPException(status_code=404, detail="User not found")

    # 双方向のDMを取得
    query = (
        select(Message)
        .options(selectinload(Message.sender))
        .where(
            ((Message.sender_id == user1_id) & (Message.target_user_id == user2_id))
            | ((Message.sender_id == user2_id) & (Message.target_user_id == user1_id))
        )
        .where(
            Message.room_id.is_(None)  # DMはroom_idがNone
        )
        .order_by(desc(Message.created_at))
    )

    # before パラメータがある場合、その日時より前のメッセージを取得
    if before:
        try:
            from datetime import datetime

            before_dt = datetime.fromisoformat(before.replace("Z", "+00:00"))
            query = query.where(Message.created_at < before_dt)
        except ValueError:
            raise HTTPException(status_code=400, detail="Invalid date format")

    query = query.limit(limit)

    result = await db.execute(query)
    messages = result.scalars().all()

    # レスポンス形式に変換
    message_responses = []
    for message in messages:
        message_response = MessageResponse.from_orm(message)
        message_response.sender_name = message.sender.name if message.sender else None
        message_responses.append(message_response)

    return message_responses
