import os
from collections.abc import AsyncGenerator

from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker, create_async_engine

from .models import Base

# データベースURL（開発環境ではSQLite、本番ではPostgreSQL）
DATABASE_URL = os.getenv("DATABASE_URL", "sqlite+aiosqlite:///./nok.db")

# エンジンとセッションの作成
engine = create_async_engine(
    DATABASE_URL,
    echo=True,  # SQL文をログ出力（開発時のみ）
    future=True,
)

AsyncSessionLocal = async_sessionmaker(
    engine, class_=AsyncSession, expire_on_commit=False
)


async def init_db():
    """データベースの初期化"""
    async with engine.begin() as conn:
        await conn.run_sync(Base.metadata.create_all)


async def get_db() -> AsyncGenerator[AsyncSession, None]:
    """データベースセッションの取得"""
    async with AsyncSessionLocal() as session:
        try:
            yield session
        finally:
            await session.close()


async def close_db():
    """データベース接続の終了"""
    await engine.dispose()
