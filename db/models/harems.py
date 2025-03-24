import datetime
from pydantic import BaseModel
from ..database_instance import db

class Harem(BaseModel):
    user_id: int
    users: list[int] = []
    created_at: int

    @property
    def created_at_datetime(self) -> datetime.datetime:
        return datetime.datetime.fromtimestamp(self.created_at)
    
    async def delete(self):
        await db.harems.delete_harem(self.user_id)
    
    async def add_user(self, user_id: int):
        await db.harems.add_user_to_harem(self.user_id, user_id)

    async def remove_user(self, user_id: int):
        await db.harems.remove_user_from_harem(self.user_id, user_id)

class HaremsManager:
    async def create_harem(self, user_id: int):
        timestamp = int(datetime.datetime.now().timestamp())
        await db._execute("INSERT INTO harems (user_id, created_at) VALUES ($1, $2)", user_id, timestamp)
        return Harem(user_id=user_id, created_at=timestamp)
    
    async def get_harem(self, user_id: int):
        row = await db._fetchrow("SELECT * FROM harems WHERE user_id = $1", user_id)

        if row is None:
            return None
        
        return Harem(user_id=row["user_id"], users=row["users"], created_at=row["created_at"])
    
    async def get_user_harem(self, user_id: int):
        row = await db._fetchrow("SELECT * FROM harems WHERE $1 = ANY(users)", user_id)
        
        if row is None:
            return None
            
        return Harem(user_id=row["user_id"], users=row["users"], created_at=row["created_at"])
    
    async def get_harems(self):
        rows = await db._fetch("SELECT * FROM harems")
        return [Harem(user_id=row["user_id"], users=row["users"], created_at=row["created_at"]) for row in rows]
    
    async def add_user_to_harem(self, user_id: int, user_id_to_add: int):
        await db._execute(
            "UPDATE harems SET users = array_append(users, $1) WHERE user_id = $2",
            user_id_to_add, user_id
        )
    
    async def remove_user_from_harem(self, user_id: int, user_id_to_remove: int):
        await db._execute(
            "UPDATE harems SET users = array_remove(users, $1) WHERE user_id = $2",
            user_id_to_remove, user_id)
    
    async def delete_harem(self, user_id: int):
        await db._execute("DELETE FROM harems WHERE user_id = $1", user_id)