from decimal import Decimal
from enum import Enum
from typing import Literal
from pydantic import BaseModel
from ..database_instance import db

class User(BaseModel):
    id: int
    name: str
    className: Literal["маг", "воин", "танк"]
    balance: int = 0
    health: int
    mana: int
    damage: int
    in_game: bool = False

    async def set_in_game(self, value: bool) -> None:
        self.in_game = value
        await db._execute("UPDATE dl SET in_game = $1 WHERE id = $2", value, self.id)

    async def increase_balance(self, value: int) -> int:
        new_value = await db.dl.increase_stat(self.id, "balance", value)
        self.balance = new_value
        return self.balance

    async def decrease_balance(self, value: int) -> int:
        new_value = await db.dl.decrease_stat(self.id, "balance", value)
        self.balance = new_value
        return self.balance

    async def increase_health(self, value: int) -> int:
        return await db.dl.increase_stat(self.id, "health", value)
    
    async def decrease_health(self, value: int) -> int:
        return await db.dl.decrease_stat(self.id, "health", value)
    
    async def increase_mana(self, value: int) -> int:
        return await db.dl.increase_stat(self.id, "mana", value)
    
    async def decrease_mana(self, value: int) -> int:
        return await db.dl.decrease_stat(self.id, "mana", value)

    async def increase_damage(self, value: int) -> int:
        return await db.dl.increase_stat(self.id, "damage", value)
    
    async def decrease_damage(self, value: int) -> int:
        return await db.dl.decrease_stat(self.id, "damage", value)

CLASSES = {
    "маг":  {"health": 59, "mana": 200, "damage": 140},
    "воин": {"health": 100, "mana": 10, "damage": 100},
    "танк": {"health": 159, "mana": 0, "damage": 110}
}

class Monster(BaseModel):
    name: str
    health: int
    reward: int
    damage: int
    image: str

class DLManager:
    async def create_user(
        self,
        user_id: int,
        name: str,
        className: str,
    ) -> User:
        class_data = CLASSES.get(className.lower())
        if not class_data:
            raise ValueError(f"Неизвестный класс: {className}")

        await db._execute(
            "INSERT INTO dl (id, name, class, balance, health, mana, damage, in_game) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            user_id,
            name,
            className.lower(),
            0,
            class_data["health"],
            class_data["mana"],
            class_data["damage"],
            False,
        )
        
        return None

    async def get_user(self, user_id: int):
        row = await db._fetchrow("SELECT * FROM dl WHERE id = $1", user_id)

        if not row:
            return None

        row = dict(row)
        row["className"] = row["class"]

        return User(**row)
    
    async def get_monsters(self):
        rows = await db._fetch("SELECT * FROM dl_monsters")
        return [Monster(**row) for row in rows]
    
    async def increase_stat(self, user_id: int, stat: str, value: int):
        query = f"UPDATE dl SET {stat.lower()} = {stat.lower()} + $1 WHERE id = $2 RETURNING {stat.lower()}"
        returns = await db._fetchval(query, value, user_id)
        return returns

    async def decrease_stat(self, user_id: int, stat: str, value: int):
        query = f"UPDATE dl SET {stat.lower()} = {stat.lower()} - $1 WHERE id = $2 RETURNING {stat.lower()}"
        returns = await db._fetchval(query, value, user_id)
        return returns

    async def delete_user(self, user_id: int):
        await db._execute("DELETE FROM dl WHERE id = $1", user_id)