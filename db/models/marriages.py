import datetime
from typing import Optional
from pydantic import BaseModel
from ..database_instance import db

class Marriage(BaseModel):
    user_id: int
    partner_id: int
    created_at: int

    @property
    def created_at_datetime(self) -> datetime.datetime:
        return datetime.datetime.fromtimestamp(self.created_at)
    
    async def delete(self):
        await db.marriages.delete_marriage(self.user_id)

class MarriagesManager:
    async def create_marriage(self, user_id: int, partner_id: int):
        timestamp = int(datetime.datetime.now().timestamp())
        await db._execute("INSERT INTO marriages (user_id, partner_id, created_at) VALUES ($1, $2, $3)", user_id, partner_id, timestamp)
        return Marriage(user_id=user_id, partner_id=partner_id, created_at=timestamp)
    
    async def get_marriage(self, user_id: int):
        row = await db._fetchrow("SELECT * FROM marriages WHERE user_id = $1 OR partner_id = $2", user_id, user_id)
        
        if row is None:
            return None
        
        return Marriage(user_id=row["user_id"], partner_id=row["partner_id"], created_at=row["created_at"])
    
    async def get_marriages(self):
        rows = await db._fetch("SELECT * FROM marriages")
        return [Marriage(user_id=row["user_id"], partner_id=row["partner_id"], created_at=row["created_at"]) for row in rows]
    
    async def delete_marriage(self, user_id: int):
        await db._execute("DELETE FROM marriages WHERE user_id = $1 OR partner_id = $2", user_id, user_id)
