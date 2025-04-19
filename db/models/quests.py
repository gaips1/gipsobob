import datetime
import pytz
from typing import Literal, Optional, List
from pydantic import BaseModel
from ..database_instance import db
from pydantic import Field

class Quest(BaseModel):
    id: str
    user_id: int
    name: str
    description: str
    action_type: str
    reward: int
    progress: int
    progress_max: int
    ends: int | None
    is_users_required: int
    type: Literal["active", "expired", "completed"]
    users: List[int] = Field(default_factory=list)

    @property
    def datetime(self):
        if self.ends is None:
            return None
        return datetime.datetime.fromtimestamp(self.ends, tz=pytz.timezone('Europe/Moscow'))

    async def update_progress(self, add: int = 1, used_user_id: int | None = None):
        if used_user_id is not None:
            if self.is_users_required == 2 and used_user_id in self.users:
                return False

        self.progress += add

        if self.is_users_required in [1, 2] and used_user_id is not None:
            self.users.append(used_user_id)

        if self.progress >= self.progress_max:
            self.type = "completed"
            self.ends = int(datetime.datetime.now(tz=pytz.timezone('Europe/Moscow')).timestamp())

        await db._execute(
            """UPDATE quests SET progress = $1, users = $2, type = $3, ends = $4
            WHERE id = $5 AND user_id = $6""",
            self.progress, self.users, self.type, self.ends, self.id, self.user_id
        )
        return True

    async def mark_as_expired(self):
        self.type = "expired"
        await db._execute(
            "UPDATE quests SET type = 'expired' WHERE id = $1 AND user_id = $2",
            self.id, self.user_id
        )

class QuestsManager:
    async def get_user_quests(self, user_id: int, type: Literal["active", "expired", "completed"] | None = "active") -> list[Quest]:
        if type:
            rows = await db._fetch(
                """SELECT * FROM quests WHERE user_id = $1 AND type = $2""",
                user_id, type
            )
        else:
            rows = await db._fetch(
                """SELECT * FROM quests WHERE user_id = $1""",
                user_id
            )

        return [Quest(**{**row, "users": row.get("users", [])}) for row in rows]
    
    async def get_quests(self, type: Literal["active", "expired", "completed"] | None = None) -> list[Quest]:
        if type:
            rows = await db._fetch(
                """SELECT * FROM quests WHERE type = $1""",
                type
            )
        else:
            rows = await db._fetch("""SELECT * FROM quests""")

        return [Quest(**{**row, "users": row.get("users", [])}) for row in rows]

    async def create_quest(self, quest_data: dict) -> Quest:
        await db._execute(
            """INSERT INTO quests (id, user_id, name, description, action_type, reward, progress, progress_max, ends, is_users_required, type, users)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)""",
            quest_data["id"], quest_data["user_id"], quest_data["name"], quest_data["description"],
            quest_data["action_type"], quest_data["reward"], quest_data["progress"], quest_data["progress_max"],
            quest_data["ends"], quest_data["is_users_required"], quest_data["type"], quest_data.get("users", [])
        )
        return Quest(**quest_data)

    async def get_active_quests_count(self, user_id: int) -> int:
        return await db._fetchval(
            "SELECT COUNT(*) FROM quests WHERE user_id = $1 AND type = 'active'",
            user_id
        )

    async def get_quest_by_id(self, quest_id: str, user_id: int) -> Quest | None:
        row = await db._fetchrow(
            "SELECT * FROM quests WHERE id = $1 AND user_id = $2",
            quest_id, user_id
        )
        return Quest(**row) if row else None