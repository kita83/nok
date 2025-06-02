from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy import func, select
from sqlalchemy.ext.asyncio import AsyncSession

from ..database import get_db
from ..models import Room, RoomCreate, RoomResponse, RoomUpdate, User, room_members

router = APIRouter()


@router.get("/", response_model=list[RoomResponse])
async def list_rooms(db: AsyncSession = Depends(get_db)):
    """ルーム一覧を取得"""
    # メンバー数も一緒に取得
    result = await db.execute(
        select(Room, func.count(room_members.c.user_id).label("member_count"))
        .outerjoin(room_members)
        .group_by(Room.id)
    )

    rooms_with_count = []
    for room, member_count in result.all():
        room_response = RoomResponse.from_orm(room)
        room_response.member_count = member_count
        rooms_with_count.append(room_response)

    return rooms_with_count


@router.get("/{room_id}", response_model=RoomResponse)
async def get_room(room_id: str, db: AsyncSession = Depends(get_db)):
    """特定のルームを取得"""
    room = await db.get(Room, room_id)
    if not room:
        raise HTTPException(status_code=404, detail="Room not found")

    # メンバー数を取得
    result = await db.execute(
        select(func.count(room_members.c.user_id)).where(
            room_members.c.room_id == room_id
        )
    )
    member_count = result.scalar() or 0

    room_response = RoomResponse.from_orm(room)
    room_response.member_count = member_count
    return room_response


@router.post("/", response_model=RoomResponse)
async def create_room(room_data: RoomCreate, db: AsyncSession = Depends(get_db)):
    """新しいルームを作成"""
    # 同じ名前のルームが存在しないかチェック
    result = await db.execute(select(Room).where(Room.name == room_data.name))
    existing_room = result.scalars().first()
    if existing_room:
        raise HTTPException(
            status_code=400, detail="Room with this name already exists"
        )

    room = Room(
        name=room_data.name,
        description=room_data.description,
        is_public=room_data.is_public,
    )
    db.add(room)
    await db.commit()
    await db.refresh(room)

    room_response = RoomResponse.from_orm(room)
    room_response.member_count = 0
    return room_response


@router.put("/{room_id}", response_model=RoomResponse)
async def update_room(
    room_id: str, room_data: RoomUpdate, db: AsyncSession = Depends(get_db)
):
    """ルーム情報を更新"""
    room = await db.get(Room, room_id)
    if not room:
        raise HTTPException(status_code=404, detail="Room not found")

    # 更新するフィールドのみ変更
    if room_data.name is not None:
        room.name = room_data.name
    if room_data.description is not None:
        room.description = room_data.description
    if room_data.is_public is not None:
        room.is_public = room_data.is_public

    await db.commit()
    await db.refresh(room)

    # メンバー数を取得
    result = await db.execute(
        select(func.count(room_members.c.user_id)).where(
            room_members.c.room_id == room_id
        )
    )
    member_count = result.scalar() or 0

    room_response = RoomResponse.from_orm(room)
    room_response.member_count = member_count
    return room_response


@router.delete("/{room_id}")
async def delete_room(room_id: str, db: AsyncSession = Depends(get_db)):
    """ルームを削除"""
    room = await db.get(Room, room_id)
    if not room:
        raise HTTPException(status_code=404, detail="Room not found")

    await db.delete(room)
    await db.commit()
    return {"message": "Room deleted successfully"}


@router.get("/{room_id}/members", response_model=list[dict])
async def get_room_members(room_id: str, db: AsyncSession = Depends(get_db)):
    """ルームのメンバー一覧を取得"""
    room = await db.get(Room, room_id)
    if not room:
        raise HTTPException(status_code=404, detail="Room not found")

    # ルームのメンバーを取得
    result = await db.execute(
        select(User, room_members.c.joined_at)
        .join(room_members)
        .where(room_members.c.room_id == room_id)
    )

    members = []
    for user, joined_at in result.all():
        members.append(
            {
                "id": user.id,
                "name": user.name,
                "status": user.status,
                "joined_at": joined_at.isoformat() if joined_at else None,
            }
        )

    return members


@router.post("/{room_id}/join")
async def join_room(room_id: str, user_id: str, db: AsyncSession = Depends(get_db)):
    """ルームに参加"""
    room = await db.get(Room, room_id)
    user = await db.get(User, user_id)

    if not room:
        raise HTTPException(status_code=404, detail="Room not found")
    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # 既に参加しているかチェック
    result = await db.execute(
        select(room_members)
        .where(room_members.c.room_id == room_id)
        .where(room_members.c.user_id == user_id)
    )
    if result.first():
        raise HTTPException(status_code=400, detail="User already in room")

    # ルームに参加
    if user not in room.members:
        room.members.append(user)
        await db.commit()

    return {"message": "Successfully joined room"}


@router.post("/{room_id}/leave")
async def leave_room(room_id: str, user_id: str, db: AsyncSession = Depends(get_db)):
    """ルームから退出"""
    room = await db.get(Room, room_id)
    user = await db.get(User, user_id)

    if not room:
        raise HTTPException(status_code=404, detail="Room not found")
    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # ルームから退出
    if user in room.members:
        room.members.remove(user)
        await db.commit()

    return {"message": "Successfully left room"}
