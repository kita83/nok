import uuid
from datetime import datetime
from typing import Optional

from pydantic import BaseModel
from sqlalchemy import Boolean, Column, DateTime, ForeignKey, String, Table, Text
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship

Base = declarative_base()

# ユーザーとルームの中間テーブル（多対多の関係）
room_members = Table(
    "room_members",
    Base.metadata,
    Column("user_id", String, ForeignKey("users.id"), primary_key=True),
    Column("room_id", String, ForeignKey("rooms.id"), primary_key=True),
    Column("joined_at", DateTime, default=datetime.utcnow),
)


class User(Base):
    __tablename__ = "users"

    id = Column(String, primary_key=True, default=lambda: str(uuid.uuid4()))
    name = Column(String(100), nullable=False)
    status = Column(String(20), default="offline")  # online, away, busy, offline
    created_at = Column(DateTime, default=datetime.utcnow)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)

    # リレーション（foreign_keysを明示的に指定）
    messages = relationship(
        "Message", back_populates="sender", foreign_keys="Message.sender_id"
    )
    rooms = relationship("Room", secondary=room_members, back_populates="members")


class Room(Base):
    __tablename__ = "rooms"

    id = Column(String, primary_key=True, default=lambda: str(uuid.uuid4()))
    name = Column(String(100), nullable=False)
    description = Column(Text, nullable=True)
    is_public = Column(Boolean, default=True)
    created_at = Column(DateTime, default=datetime.utcnow)
    updated_at = Column(DateTime, default=datetime.utcnow, onupdate=datetime.utcnow)

    # リレーション
    messages = relationship("Message", back_populates="room")
    members = relationship("User", secondary=room_members, back_populates="rooms")


class Message(Base):
    __tablename__ = "messages"

    id = Column(String, primary_key=True, default=lambda: str(uuid.uuid4()))
    content = Column(Text, nullable=False)
    message_type = Column(String(20), default="text")  # text, knock, system
    sender_id = Column(String, ForeignKey("users.id"), nullable=False)
    room_id = Column(String, ForeignKey("rooms.id"), nullable=True)  # DM の場合は None
    target_user_id = Column(
        String, ForeignKey("users.id"), nullable=True
    )  # DM や knock の場合のターゲット
    created_at = Column(DateTime, default=datetime.utcnow)

    # リレーション（foreign_keysを明示的に指定）
    sender = relationship("User", back_populates="messages", foreign_keys=[sender_id])
    room = relationship("Room", back_populates="messages")


# Pydantic モデル（API レスポンス用）
class UserResponse(BaseModel):
    id: str
    name: str
    status: str
    created_at: datetime

    class Config:
        from_attributes = True


class UserCreate(BaseModel):
    name: str


class UserUpdate(BaseModel):
    name: Optional[str] = None
    status: Optional[str] = None


class RoomResponse(BaseModel):
    id: str
    name: str
    description: Optional[str]
    is_public: bool
    created_at: datetime
    member_count: Optional[int] = None

    class Config:
        from_attributes = True


class RoomCreate(BaseModel):
    name: str
    description: Optional[str] = None
    is_public: bool = True


class RoomUpdate(BaseModel):
    name: Optional[str] = None
    description: Optional[str] = None
    is_public: Optional[bool] = None


class MessageResponse(BaseModel):
    id: str
    content: str
    message_type: str
    sender_id: str
    sender_name: Optional[str] = None
    room_id: Optional[str]
    target_user_id: Optional[str]
    created_at: datetime

    class Config:
        from_attributes = True


class MessageCreate(BaseModel):
    content: str
    message_type: str = "text"
    room_id: Optional[str] = None
    target_user_id: Optional[str] = None


# WebSocket メッセージ用のモデル
class WebSocketMessage(BaseModel):
    type: str  # knock, message, join_room, leave_room, user_status
    user_id: Optional[str] = None
    target_user_id: Optional[str] = None
    room_id: Optional[str] = None
    content: Optional[str] = None
    status: Optional[str] = None
    data: Optional[dict] = None
