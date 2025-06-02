"""
開発用の初期データをセットアップするスクリプト
"""

import asyncio

from sqlalchemy import insert

from app.database import AsyncSessionLocal, init_db
from app.models import Room, User, room_members


async def setup_initial_data():
    """初期データの作成"""
    # データベースの初期化
    await init_db()

    async with AsyncSessionLocal() as session:
        # デフォルトルームの作成
        rooms = [
            Room(name="メインルーム", description="全体の雑談用ルーム", is_public=True),
            Room(name="開発チーム", description="開発関連の議論", is_public=True),
            Room(name="休憩室", description="カジュアルな会話", is_public=True),
        ]

        for room in rooms:
            session.add(room)

        # テストユーザーの作成
        users = [
            User(name="Alice", status="online"),
            User(name="Bob", status="away"),
            User(name="Charlie", status="offline"),
            User(name="Diana", status="busy"),
        ]

        for user in users:
            session.add(user)

        await session.commit()

        # リフレッシュして ID を取得
        for room in rooms:
            await session.refresh(room)
        for user in users:
            await session.refresh(user)

        # ルームメンバーシップを直接 SQL で追加
        main_room_id = rooms[0].id
        dev_room_id = rooms[1].id
        alice_id = users[0].id
        bob_id = users[1].id
        charlie_id = users[2].id

        # AliceとBobをメインルームに追加
        await session.execute(
            insert(room_members).values(
                [
                    {"user_id": alice_id, "room_id": main_room_id},
                    {"user_id": bob_id, "room_id": main_room_id},
                ]
            )
        )

        # Alice、Bob、Charlieを開発チームルームに追加
        await session.execute(
            insert(room_members).values(
                [
                    {"user_id": alice_id, "room_id": dev_room_id},
                    {"user_id": bob_id, "room_id": dev_room_id},
                    {"user_id": charlie_id, "room_id": dev_room_id},
                ]
            )
        )

        await session.commit()

        print("初期データのセットアップが完了しました！")

        # 作成されたデータの確認
        print("\n=== 作成されたルーム ===")
        for room in rooms:
            print(f"- {room.name} (ID: {room.id})")

        print("\n=== 作成されたユーザー ===")
        for user in users:
            print(f"- {user.name} (ID: {user.id}, Status: {user.status})")


if __name__ == "__main__":
    asyncio.run(setup_initial_data())
