import random
import discord, aiosqlite, inspect, os
from discord.ext import commands
import json
import asyncio
import datetime
import pytz

filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
dbn = path + "/gipsobob.sql"

first_quest = {"name": "Начальный квест", "desc": "Добро пожаловать в систему квестов! Вы начинаете своё приключение с первого квеста. Чтобы его выполнить, вам необходимо поцеловать любого пользователя 3 раза. Если вы используете компьютер, вам нужно нажать ПКМ (правую кнопку мыши). Если же вы пользуетесь мобильным устройством, то зажмите по человеку и выберите пункты «Приложения» и затем «Поцеловать».",
                                                                                           "progress": 0, "progress_max": 3, "reward": 100, "id": "first_q", "do": "kiss", "ends": None}

class turnon1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, custom_id="turnon111")
    async def gkrkkkgrg(self, inter: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute(f"UPDATE users SET ended_quests_notif = 1 WHERE id = {inter.user.id}")
            await db.commit()
        await inter.response.edit_message(view=turnoff1())

class turnoff1(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, custom_id="turnoff111")
    async def gkrejgdfdlg(self, inter: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute(f"UPDATE users SET ended_quests_notif = 0 WHERE id = {inter.user.id}")
            await db.commit()
        await inter.response.edit_message(view=turnon1())

async def check_first_quest(inter: discord.Interaction, me):
    completed_quests: list = json.loads(me[4])
    quests: list = json.loads(me[3])
    for quest in completed_quests:
        if quest["id"] == "first_q":
            return True
            
    for quest in quests:
        if quest["id"] == "first_q":
            return True

    quests.append(first_quest)
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("UPDATE `users` SET quests =? WHERE id =?", (json.dumps(quests), inter.user.id))
        await db.commit()

    return False

async def update_quest(user: discord.User | discord.Member, do: str, add: int = 1):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM `users` WHERE id = ?", (user.id,))
        me = await cursor.fetchone()

        quests: list = json.loads(me[3])
        completed_quests: list = json.loads(me[4])
        quests_updated = False
        
        for quest in quests[:]:
            if quest["do"] == do:
                quest["progress"] += add
                quests_updated = True
                if quest["progress"] >= quest["progress_max"]:
                    quests.remove(quest)
                    completed_quests.append(quest)
                    await user.send(embed=discord.Embed(
                        title=f"Вы выполнили квест {quest['name']}",
                        description=f"{quest['desc']}\n\nВаша награда - {quest['reward']} бебр!",
                        color=0x00ff00
                    ))
                    await cursor.execute(
                        "UPDATE `sbp` SET balance = balance + ? WHERE id = ?",
                        (quest["reward"], user.id)
                    )

        if quests_updated:
            await cursor.execute(
                "UPDATE `users` SET quests = ?, completed_quests = ? WHERE id = ?",
                (json.dumps(quests), json.dumps(completed_quests), user.id)
            )
            await db.commit()

    return True

async def check(inter: discord.Interaction):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM `users` WHERE id = ?", (inter.user.id,))
        me = await cursor.fetchone()
        
    if not me:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            quests = [first_quest]
            await cursor.execute("INSERT INTO `users` (id, quests) VALUES (?, ?)", (inter.user.id, json.dumps(quests)))
            await db.commit()
            return True

    if me[1] == 1:
        return False
    
    await check_first_quest(inter, me)
    return True

async def add_random_quest(user: discord.User = None, bot: commands.Bot = None):
    with open(path+'/random_quests.json', 'r', encoding="utf-8") as json_file:
            random_quests = json.load(json_file)
    loop = asyncio.get_event_loop()
    if user != None:            
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT quests FROM `users` WHERE id = ?", (user.id,))
            me = await cursor.fetchone()
            quests: list = json.loads(me[0])
            if len(quests) >= 5: return
            rq = random.choice(random_quests)
            if rq["ends"] != None:
                a = int(rq["ends"][0])
                b = str(rq["ends"][1])
                day = a if b == "d" else None
                hour = a if b == "h" else None
                if day != None:
                    date = datetime.datetime.now(pytz.timezone('Europe/Moscow')) + datetime.timedelta(days=day)
                elif hour != None:
                    date = datetime.datetime.now(pytz.timezone('Europe/Moscow')) + datetime.timedelta(hours=hour)

                rq["ends"] = date.isoformat()
            quests.append(rq)
            await cursor.execute("UPDATE `users` SET (quests) = (?) WHERE id =?", (json.dumps(quests), user.id))
            await db.commit()

        loop.create_task(timeout_quests_timer(user=user, quest=rq))
    else:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `users`")
            users = await cursor.fetchall()

        for user in users:
            quests: list = json.loads(user[3])
            if len(quests) >= 5: continue
            rq = random.choice(random_quests)
            if rq["ends"] != None:
                a = int(rq["ends"][0])
                b = str(rq["ends"][1])
                day = a if b == "d" else None
                hour = a if b == "h" else None
                if day != None:
                    date = datetime.datetime.now(pytz.timezone('Europe/Moscow')) + datetime.timedelta(days=day)
                elif hour != None:
                    date = datetime.datetime.now(pytz.timezone('Europe/Moscow')) + datetime.timedelta(hours=hour)

                rq["ends"] = date.isoformat()
            usr = await get_or_fetch_user(id=user[0], bot=bot)
            loop.create_task(timeout_quests_timer(user=usr, quest=rq))
            quests.append(rq)
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("UPDATE `users` SET (quests) = (?) WHERE id =?", (json.dumps(quests), user[0]))
                await db.commit()
                
            await asyncio.sleep(5)

async def timeout_quests_timer(user: discord.User | discord.Member, quest: dict):
    while True:
        ends = datetime.datetime.fromisoformat(quest["ends"]) - datetime.datetime.now(pytz.timezone('Europe/Moscow'))
        if ends.total_seconds() <= 0:
            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM users WHERE id = ?", (user.id,))
                user1 = await cursor.fetchone()

            quests: list = json.loads(user1[3])
            completed_quests: list = json.loads(user1[4])
            ended_quests: list = json.loads(user1[5])

            quests_to_remove = [q for q in quests if q == quest]

            for q in quests_to_remove:
                if q not in completed_quests:
                    ended_quests.append(q)

            quests = [q for q in quests if q not in quests_to_remove]

            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute(
                    "UPDATE users SET quests = ?, completed_quests = ?, ended_quests = ? WHERE id = ?",
                    (json.dumps(quests), json.dumps(completed_quests), json.dumps(ended_quests), user.id)
                )
                await db.commit()

            if user1[6] == 1:
                await user.send(embed=discord.Embed(title=f"Квест {quest['name']} истёк", description="Увы...", color=discord.Color.random()),
                                view=turnoff1())
            break

        await asyncio.sleep(5)

async def get_or_fetch_user(bot: commands.Bot, id: str | int):
    user = bot.get_user(id)
    if user is None:
        user = await bot.fetch_user(id)

    return user