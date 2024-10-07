import random
import discord
import aiosqlite
import inspect
import os
import json
import asyncio
import datetime
import pytz
from discord.ext import commands, tasks

filename = inspect.getframeinfo(inspect.currentframe()).filename
path = os.path.dirname(os.path.abspath(filename))
dbn = os.path.join(path, "gipsobob.sql")

first_quest = {
    "name": "Начальный квест",
    "desc": "Добро пожаловать в систему квестов! Чтобы выполнить квест, вам нужно поцеловать любого пользователя 3 раза. Используйте ПКМ или долго нажмите на пользователя и выберите 'Поцеловать'.",
    "progress": 0,
    "progress_max": 3,
    "reward": 100,
    "id": "first_q",
    "do": "kiss",
    "ends": None
}

class turnon1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, custom_id="turgergergnon111")
    async def gkrkkkgrg(self, inter: discord.Interaction, button: discord.ui.Button):
        await self.update_notification(inter, 1)
        await inter.response.edit_message(view=turnoff1())

    async def update_notification(self, inter, status):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE users SET ended_quests_notif = ? WHERE id = ?", (status, inter.user.id))
            await db.commit()

class turnoff1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, custom_id="turgrwegewgnoff111")
    async def gkrejgdfdlg(self, inter: discord.Interaction, button: discord.ui.Button):
        await self.update_notification(inter, 0)
        await inter.response.edit_message(view=turnon1())

    async def update_notification(self, inter, status):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE users SET ended_quests_notif = ? WHERE id = ?", (status, inter.user.id))
            await db.commit()

class turnon2(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, custom_id="turgergergno222")
    async def gkrkkkgrg(self, inter: discord.Interaction, button: discord.ui.Button):
        await self.update_notification(inter, 1)
        await inter.response.edit_message(view=turnoff2())

    async def update_notification(self, inter, status):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE users SET new_quest_notif = ? WHERE id = ?", (status, inter.user.id))
            await db.commit()

class turnoff2(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, custom_id="turgrwegewgnoff1222")
    async def gkrejgdfdlg(self, inter: discord.Interaction, button: discord.ui.Button):
        await self.update_notification(inter, 0)
        await inter.response.edit_message(view=turnon2())

    async def update_notification(self, inter, status):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE users SET new_quest_notif = ? WHERE id = ?", (status, inter.user.id))
            await db.commit()

async def check_first_quest(inter: discord.Interaction, me):
    completed_quests = json.loads(me[4])
    quests = json.loads(me[3])
    
    if any(quest["id"] == "first_q" for quest in completed_quests + quests):
        return True

    quests.append(first_quest)
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("UPDATE users SET quests = ? WHERE id = ?", (json.dumps(quests), inter.user.id))
        await db.commit()

    return False

async def update_quest(user: discord.User | discord.Member, do: str, add: int = 1):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM users WHERE id = ?", (user.id,))
        me = await cursor.fetchone()

        quests = json.loads(me[3])
        completed_quests = json.loads(me[4])
        
        for quest in quests[:]:
            if quest["do"] == do:
                quest["progress"] += add
                if quest["progress"] >= quest["progress_max"]:
                    quests.remove(quest)
                    completed_quests.append(quest)
                    await user.send(embed=discord.Embed(
                        title=f"Вы выполнили квест {quest['name']}",
                        description=f"{quest['desc']}\n\nВаша награда - {quest['reward']} бебр!",
                        color=0x00ff00
                    ))
                    await cursor.execute("UPDATE sbp SET balance = balance + ? WHERE id = ?", (quest["reward"], user.id))

        await cursor.execute("UPDATE users SET quests = ?, completed_quests = ? WHERE id = ?", (json.dumps(quests), json.dumps(completed_quests), user.id))
        await db.commit()

async def check(inter: discord.Interaction):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM users WHERE id = ?", (inter.user.id,))
        me = await cursor.fetchone()

    if not me:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            quests = [first_quest]
            await cursor.execute("INSERT INTO users (id, quests) VALUES (?, ?)", (inter.user.id, json.dumps(quests)))
            await db.commit()
            return True

    if me[1] == 1:
        return False

    await check_first_quest(inter, me)
    return True

async def add_random_quest(user: discord.User = None, bot: commands.Bot = None):
    with open(os.path.join(path, 'random_quests.json'), 'r', encoding="utf-8") as json_file:
        random_quests = json.load(json_file)

    if user is not None:            
        await add_quest_to_user(user, random_quests)
    else:
        await add_quest_to_all_users(bot, random_quests)

async def add_quest_to_user(user: discord.User, random_quests):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM users WHERE id = ?", (user.id,))
        me = await cursor.fetchone()

        quests = json.loads(me[3])
        if len(quests) >= 5:
            return

        rq = random.choice(random_quests)
        rq["ends"] = await calculate_end_time(rq["ends"])

        asyncio.create_task(timeout_quests_timer(user=user, quest=rq))

        quests.append(rq)
        await cursor.execute("UPDATE users SET quests = ? WHERE id = ?", (json.dumps(quests), user.id))
        await db.commit()

        if me[7] == 1:
            await send_new_quest_notification(user, rq)

async def add_quest_to_all_users(bot: commands.Bot, random_quests):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM users")
        users = await cursor.fetchall()

    for user in users:
        user_obj = await get_or_fetch_user(bot, user[0])
        await add_quest_to_user(user_obj, random_quests)
        await asyncio.sleep(0.5)

async def send_new_quest_notification(user: discord.User, quest: dict):
    try:
        await user.send(embed=discord.Embed(
            title="Новый квест!",
            description=f"Вам был добавлен новый квест - {quest['name']}\nПодробнее в **/quests**",
            color=discord.Color.random()
        ), view=turnoff2())
    except Exception as e:
        print(f"Ошибка отправки сообщения пользователю {user.id}: {e}")

async def calculate_end_time(end_data):
    if end_data is None:
        return None
    
    amount, unit = int(end_data[0]), end_data[1]
    time_delta = datetime.timedelta(days=amount) if unit == "d" else datetime.timedelta(hours=amount)
    return (datetime.datetime.now(pytz.timezone('Europe/Moscow')) + time_delta).isoformat()

async def timeout_quests_timer(user: discord.User | discord.Member, quest: dict):
    while True:
        now = datetime.datetime.now(pytz.timezone('Europe/Moscow'))
        quest_end_time = datetime.datetime.fromisoformat(quest["ends"]).astimezone(pytz.timezone('Europe/Moscow'))

        if quest_end_time <= now:
            return await handle_quest_timeout(user, quest)
        
        await asyncio.sleep(5)

async def handle_quest_timeout(user: discord.User, quest: dict):
    async with aiosqlite.connect(dbn) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM users WHERE id = ?", (user.id,))
        me = await cursor.fetchone()

    if me is None:
        raise Exception(f"Пользователь {user.id} не найден в базе данных.")

    quests = json.loads(me[3])
    completed_quests = json.loads(me[4])
    ended_quests = json.loads(me[5])

    if quest in quests:
        if me[6] == 1:
            try:
                await user.send(embed=discord.Embed(
                    title=f"Квест '{quest['name']}' истёк",
                    description="Увы, время выполнения квеста истекло.",
                    color=discord.Color.random()
                ), view=turnoff1())
            except:
                print(f"Ошибка отправки сообщения пользователю {user.id}")

        quests.remove(quest)

    if quest not in completed_quests and quest not in ended_quests:
        ended_quests.append(quest)

    async with aiosqlite.connect(dbn) as db:
        cursor = await db.cursor()
        await cursor.execute(
            "UPDATE users SET quests = ?, completed_quests = ?, ended_quests = ? WHERE id = ?",
            (json.dumps(quests), json.dumps(completed_quests), json.dumps(ended_quests), user.id)
        )
        await db.commit()

async def get_or_fetch_user(bot: commands.Bot, id: str | int):
    user = bot.get_user(id)
    if user is None:
        user = await bot.fetch_user(id)

    return user