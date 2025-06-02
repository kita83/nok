from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession

from ..database import get_db
from ..models import User, UserCreate, UserResponse, UserUpdate

router = APIRouter()


@router.get("/", response_model=list[UserResponse])
async def list_users(db: AsyncSession = Depends(get_db)):
    """ユーザー一覧を取得"""
    result = await db.execute(select(User))
    users = result.scalars().all()
    return users


@router.get("/{user_id}", response_model=UserResponse)
async def get_user(user_id: str, db: AsyncSession = Depends(get_db)):
    """特定のユーザーを取得"""
    user = await db.get(User, user_id)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    return user


@router.post("/", response_model=UserResponse)
async def create_user(user_data: UserCreate, db: AsyncSession = Depends(get_db)):
    """新しいユーザーを作成"""
    # 同じ名前のユーザーが存在しないかチェック
    result = await db.execute(select(User).where(User.name == user_data.name))
    existing_user = result.scalars().first()
    if existing_user:
        raise HTTPException(
            status_code=400, detail="User with this name already exists"
        )

    user = User(name=user_data.name)
    db.add(user)
    await db.commit()
    await db.refresh(user)
    return user


@router.put("/{user_id}", response_model=UserResponse)
async def update_user(
    user_id: str, user_data: UserUpdate, db: AsyncSession = Depends(get_db)
):
    """ユーザー情報を更新"""
    user = await db.get(User, user_id)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    # 更新するフィールドのみ変更
    if user_data.name is not None:
        user.name = user_data.name
    if user_data.status is not None:
        user.status = user_data.status

    await db.commit()
    await db.refresh(user)
    return user


@router.delete("/{user_id}")
async def delete_user(user_id: str, db: AsyncSession = Depends(get_db)):
    """ユーザーを削除"""
    user = await db.get(User, user_id)
    if not user:
        raise HTTPException(status_code=404, detail="User not found")

    await db.delete(user)
    await db.commit()
    return {"message": "User deleted successfully"}


@router.get("/online/list", response_model=list[UserResponse])
async def list_online_users(db: AsyncSession = Depends(get_db)):
    """オンラインユーザー一覧を取得"""
    result = await db.execute(
        select(User).where(User.status.in_(["online", "away", "busy"]))
    )
    users = result.scalars().all()
    return users
