from typing import Optional
from pydantic import BaseModel
from db.models.quests import QuestsManager
from ..database_instance import db

class User(BaseModel):
    id: int
    is_banned: bool = False
    ended_quests_notifications: bool = True
    new_quests_notifications: bool = True

    async def set_banned(self, value: bool):
        self.is_banned = value
        await db._execute("UPDATE users SET is_banned = $1 WHERE id = $2", value, self.id)

    async def set_ended_quests_notifications(self, value: bool):
        self.ended_quests_notifications = value
        await db._execute(
            "UPDATE users SET ended_quests_notifications = $1 WHERE id = $2",
            value, self.id
        )

    async def set_new_quests_notifications(self, value: bool):
        self.new_quests_notifications = value
        await db._execute(
            "UPDATE users SET new_quests_notifications = $1 WHERE id = $2",
            value, self.id
        )

class UserManager:
    async def create_user(self, user_id: int) -> User:
        await db._execute("INSERT INTO users (id) VALUES ($1)", user_id)
        return User(id=user_id)

    async def get_user(self, user_id: int, create_if_not_exists: bool = False):
        row = await db._fetchrow("SELECT * FROM users WHERE id = $1", user_id)
        
        if row is None:
            if create_if_not_exists:
                return await self.create_user(user_id)
            else:
                return None
        else:
            return User(**row)
        
    async def get_users(self) -> list[User]:
        rows = await db._fetch("SELECT * FROM users")
        return [User(**row) for row in rows]