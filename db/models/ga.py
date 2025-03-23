import datetime
import json
from typing import Optional
from pydantic import BaseModel
import pytz
from ..database_instance import db

class Giveaway(BaseModel):
    id: int
    timestamp: int
    users: list[Optional[int]]
    prize: int
    channel_id: int

    @property
    def datetime(self):
        return datetime.datetime.fromtimestamp(self.timestamp, pytz.timezone('Europe/Moscow'))
    
    async def update_users_list(self) -> list[Optional[int]]:
        new_users = await db._fetchrow("SELECT users FROM giveaways WHERE id = $1", self.id)

        if not new_users:
            return []

        self.users = new_users["users"]
        return self.users
    
    async def add_user(self, user_id: int) -> list[Optional[int]]:
        users = await db._fetchval(
            "UPDATE giveaways SET users = array_append(users, $1) WHERE id = $2 RETURNING users",
            user_id, self.id
        )

        self.users = users
        return users
    
    async def remove_user(self, user_id: int) -> list[Optional[int]]:
        users = await db._fetchval(
            "UPDATE giveaways SET users = array_remove(users, $1) WHERE id = $2 RETURNING users",
            user_id, self.id
        )

        self.users = users
        return users
    
    async def delete(self):
        await db.ga.delete_giveaway(self.id)

class GiveAwaysManager:
    async def create_giveaway(self, id: int, channel_id: int, prize: int, timestamp: int):
        await db._execute(
            "INSERT INTO giveaways (id, channel_id, prize, timestamp, users) VALUES ($1, $2, $3, $4, $5)",
            id, channel_id, prize, timestamp, []
        )

        return Giveaway(id=id, channel_id=channel_id, prize=prize, timestamp=timestamp, users=[])
    
    async def get_giveaway(self, id: int) -> Optional[Giveaway]:
        giveaway = await db._fetchrow(
            "SELECT * FROM giveaways WHERE id = $1",
            id
        )

        if not giveaway:
            return None
        
        return Giveaway(**giveaway)
    
    async def get_giveaways(self) -> list[Giveaway]:
        giveaways = await db._fetch("SELECT * FROM giveaways")
        return [Giveaway(**giveaway) for giveaway in giveaways]
    
    async def delete_giveaway(self, id: int):
        await db._execute(
            "DELETE FROM giveaways WHERE id = $1",
            id
        )