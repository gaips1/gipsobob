from decimal import Decimal
from typing import Optional
from pydantic import BaseModel
from ..database_instance import db

class User(BaseModel):
    id: int
    balance: Decimal = Decimal('0')
    notifications: bool = True

    @property
    def balance_normalized(self) -> Decimal:
        return Decimal(str(float(self.balance)))
    
    @property
    def balance_text(self) -> str:
        return f"{self.balance_normalized:,}"

    async def set_notifications(self, value: bool):
        self.notifications = value
        await db._execute("UPDATE sbp SET notifications = $1 WHERE id = $2", value, self.id)

    async def set_balance(self, value: int | Decimal | float):
        self.balance = Decimal(str(value))
        await db.sbp.set_balance(self.id, self.balance)

    async def increase_balance(self, value: int | Decimal | float) -> float:
        return await db.sbp.increase_balance(self.id, value)

    async def decrease_balance(self, value: int | Decimal | float) -> float:
        return await db.sbp.decrease_balance(self.id, value)

class SBPManager:
    async def create_user(self, user_id: int) -> User:
        await db._execute("INSERT INTO sbp (id) VALUES ($1)", user_id)
        return User(id=user_id)

    async def get_user(self, user_id: int, create_if_not_exists: bool = False):
        row = await db._fetchrow("SELECT * FROM sbp WHERE id = $1", user_id)
        
        if row is None:
            if create_if_not_exists:
                return await self.create_user(user_id)
            else:
                return None
        else:
            return User(**row)
        
    async def create_invite(self, user_id: int, invited_user_id: int):
        await db._execute("INSERT INTO sbp_invites (user_id, invited_user_id) VALUES ($1, $2)", user_id, invited_user_id)

    async def set_balance(self, user_id: int, balance: int | Decimal | float):
        await db._execute("UPDATE sbp SET balance = $1 WHERE id = $2", balance, user_id)

    async def get_balance(self, user_id: int) -> Decimal:
        balance = await db._fetchrow("SELECT balance FROM sbp WHERE id = $1", user_id)
        return balance["balance"]

    async def increase_balance(self, user_id: int, value: int | Decimal | float) -> float:
        value = Decimal(str(value))
        new_balance = await db._fetchval(
            "UPDATE sbp SET balance = balance + $1 WHERE id = $2 RETURNING balance", 
            value, 
            user_id
        )

        return new_balance

    async def decrease_balance(self, user_id: int, value: int | Decimal | float) -> float:
        value = Decimal(str(value))
        new_balance = await db._fetchval(
            "UPDATE sbp SET balance = balance - $1 WHERE id = $2 RETURNING balance", 
            value, 
            user_id
        )

        return new_balance