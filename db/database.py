import asyncpg
import logging
from typing import Optional, Any, List

class DatabaseManager:
    def __init__(self, database_url: str):
        self.database_url = database_url
        self.logger = logging.getLogger("database.manager")
        self.pool: Optional[asyncpg.Pool] = None

    async def connect(self):
        self.pool = await asyncpg.create_pool(self.database_url, max_size=50)
        self.logger.info("DB Connected")

        from .models.users import UserManager
        from .models.sbp import SBPManager
        from .models.dl import DLManager
        from .models.ga import GiveAwaysManager
        from .models.quests import QuestsManager
        from .models.marriages import MarriagesManager
        from .models.harems import HaremsManager

        self.users = UserManager()
        self.sbp = SBPManager()
        self.dl = DLManager()
        self.ga = GiveAwaysManager()
        self.quests = QuestsManager()
        self.marriages = MarriagesManager()
        self.harems = HaremsManager()

    async def disconnect(self):
        if self.pool:
            await self.pool.close()

    async def _execute(self, query: str, *params: Any) -> None:
        if not self.pool:
            raise RuntimeError("Пул соединений не установлен. Вызовите метод connect().")
        async with self.pool.acquire() as conn:
            try:
                await conn.execute(query, *params)
            except Exception as e:
                self.logger.exception("Ошибка выполнения запроса '%s': %s", query, e)
                raise

    async def _fetchrow(self, query: str, *params: Any) -> Optional[asyncpg.Record]:
        if not self.pool:
            raise RuntimeError("Пул соединений не установлен. Вызовите метод connect().")
        async with self.pool.acquire() as conn:
            try:
                return await conn.fetchrow(query, *params)
            except Exception as e:
                self.logger.exception("Ошибка получения строки по запросу '%s': %s", query, e)
                raise

    async def _fetch(self, query: str, *params: Any) -> List[asyncpg.Record]:
        if not self.pool:
            raise RuntimeError("Пул соединений не установлен. Вызовите метод connect().")
        async with self.pool.acquire() as conn:
            try:
                return await conn.fetch(query, *params)
            except Exception as e:
                self.logger.exception("Ошибка получения строк по запросу '%s': %s", query, e)
                raise

    async def _fetchval(self, query: str, *params: Any):
        if not self.pool:
            raise RuntimeError("Пул соединений не установлен. Вызовите метод connect().")
        async with self.pool.acquire() as conn:
            try:
                return await conn.fetchval(query, *params)
            except Exception as e:
                self.logger.exception("Ошибка получения значения по запросу '%s': %s", query, e)
                raise