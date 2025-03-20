import random
import discord
import inspect
import os
import json
import asyncio
import datetime
import pytz
from discord.ext import commands, tasks
from db.database_instance import db
from db.models.quests import Quest
lock = asyncio.Lock()

filename = inspect.getframeinfo(inspect.currentframe()).filename
path = os.path.dirname(os.path.abspath(filename))

first_quest = {
    "name": "Начальный квест",
    "description": "Добро пожаловать в систему квестов! Чтобы выполнить квест, вам нужно поцеловать 3 разных пользователей. Используйте ПКМ или долго нажмите на пользователя и выберите 'Поцеловать'.",
    "progress": 0,
    "progress_max": 3,
    "reward": 100,
    "id": "first_q",
    "action_type": "kiss",
    "ends": None,
    "is_users_required": 2,
    "type": "active"
}

class turnon1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, custom_id="turgergergnon111")
    async def gkrkkkgrg(self, inter: discord.Interaction, button: discord.ui.Button):
        user = await db.users.get_user(inter.user.id)
        await user.set_ended_quests_notifications(True)
        await inter.response.edit_message(view=turnoff1())

class turnoff1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, custom_id="turgrwegewgnoff111")
    async def gkrejgdfdlg(self, inter: discord.Interaction, button: discord.ui.Button):
        user = await db.users.get_user(inter.user.id)
        await user.set_ended_quests_notifications(False)
        await inter.response.edit_message(view=turnon1())

class turnon2(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, custom_id="turgergergno222")
    async def gkrkkkgrg(self, inter: discord.Interaction, button: discord.ui.Button):
        user = await db.users.get_user(inter.user.id)
        await user.set_new_quests_notifications(True)
        await inter.response.edit_message(view=turnoff2())

class turnoff2(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, custom_id="turgrwegewgnoff1222")
    async def gkrejgdfdlg(self, inter: discord.Interaction, button: discord.ui.Button):
        user = await db.users.get_user(inter.user.id)
        await user.set_new_quests_notifications(False)
        await inter.response.edit_message(view=turnon2())

async def check_first_quest(inter: discord.Interaction, user):
    quests = await db.quests.get_user_quests(inter.user.id)
    
    if any(quest.id == "first_q" for quest in quests):
        return True

    first_quest["user_id"] = inter.user.id
    await db.quests.create_quest(first_quest)

    return False

async def update_quest(user: discord.User | discord.Member, do: str, add: int = 1, used_user: discord.User | discord.Member = None):
    quests = await db.quests.get_user_quests(user.id)
    
    for quest in quests:
        if quest.action_type == do:
            if await quest.update_progress(add, used_user.id if used_user else None):
                if quest.type == "completed":
                    try:
                        await user.send(embed=discord.Embed(
                            title=f"Вы выполнили квест {quest.name}",
                            description=f"{quest.description}\n\nВаша награда - {quest.reward} бебр!",
                            color=0x00ff00
                        ))
                    except:
                        pass
                        
                    await db.sbp.increase_balance(user.id, quest.reward)

async def check(inter: discord.Interaction):
    user = await db.users.get_user(inter.user.id, True)

    if user.is_banned:
        return False
    
    return True

async def add_random_quest(user: discord.User = None, bot: commands.Bot = None):
    with open(os.path.join(path, 'random_quests.json'), 'r', encoding="utf-8") as json_file:
        random_quests = json.load(json_file)

    if user is not None:            
        await add_quest_to_user(user, random_quests)
    else:
        await add_quest_to_all_users(bot, random_quests)

async def add_quest_to_user(user: discord.User, random_quests):
    quests_count = await db.quests.get_active_quests_count(user.id)
    if quests_count >= 5:
        return

    rq = random.choice(random_quests)
    rq["ends"] = await calculate_end_time(rq["ends"])
    rq["users"] = []
    rq["is_users_required"] = rq.get("users_required", 0)
    rq["user_id"] = user.id
    rq["type"] = "active"

    await db.quests.create_quest(rq)

    user_data = await db.users.get_user(user.id)
    if user_data.new_quests_notifications:
        await send_new_quest_notification(user, rq)

async def add_quest_to_all_users(bot: commands.Bot, random_quests):
    users = await db._fetch("SELECT id FROM users")
    for user in users:
        user_obj = await get_or_fetch_user(bot, user["id"])
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
    
    if isinstance(end_data, int):
        return end_data
        
    amount, unit = int(end_data[0]), end_data[1]
    time_delta = datetime.timedelta(days=amount) if unit == "d" else datetime.timedelta(hours=amount)
    return int((datetime.datetime.now(pytz.timezone('Europe/Moscow')) + time_delta).timestamp())

async def handle_quest_timeout(user: discord.User, quest: Quest):
    async with lock:
        if quest.type == "active":
            user_data = await db.users.get_user(user.id)
            if user_data.ended_quests_notifications:
                try:
                    await user.send(embed=discord.Embed(
                        title=f"Квест '{quest.name}' истёк",
                        description="Увы, время выполнения квеста истекло.",
                        color=discord.Color.random()
                    ), view=turnoff1())
                except:
                    print(f"Ошибка отправки сообщения пользователю {user.id}")

            await quest.mark_as_expired()

async def get_or_fetch_user(bot: commands.Bot, id: str | int):
    user = bot.get_user(id)
    if user is None:
        user = await bot.fetch_user(id)
    return user