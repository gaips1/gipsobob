import random
import discord, aiosqlite, inspect, os
import json
filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
dbn = path + "/gipsobob.sql"

async def check_first_quest(inter: discord.Interaction, me):
    completed_quests: list = json.loads(me[4])
    quests: list = json.loads(me[3])
    for quest in completed_quests:
        if quest == "first_q":
            return True
            
    for quest in quests:
        if quest["id"] == "first_q":
            return True

    quests.append({"name": "Начальный квест", "desc": "Добро пожаловать в систему квестов! Вы начинаете своё приключение с первого квеста. Чтобы его выполнить, вам необходимо поцеловать любого пользователя 3 раза. Если вы используете компьютер, вам нужно нажать ПКМ (правую кнопку мыши). Если же вы пользуетесь мобильным устройством, то зажмите по человеку и выберите пункты «Приложения» и затем «Поцеловать».",
                                                                                           "progress": 0, "progress_max": 3, "reward": 100, "id": "first_q", "do": "kiss"})
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
        comleted_quests: list = json.loads(me[4])
        for quest in quests:
            if quest["do"] == do:
                quest["progress"] += add
                if quest["progress"] >= quest["progress_max"]:
                    quests.remove(quest)
                    comleted_quests.append(quest['id'])
                    await user.send(embed=discord.Embed(title=f"Вы выполнили квест {quest['name']}", description=f"{quest['desc']}\n\nВаша награда - {quest['reward']} бебр!", color=0x00ff00))
                    await cursor.execute("UPDATE `sbp` SET balance = balance+? WHERE id =?", (quest["reward"], user.id))
                break
        else:
            return False
        await cursor.execute("UPDATE `users` SET (quests, completed_quests) = (?,?) WHERE id =?", (json.dumps(quests), json.dumps(comleted_quests), user.id))
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
            quests: list = json.loads(me[3])
            quests.append({"name": "Начальный квест", "desc": "Добро пожаловать в систему квестов! Вы начинаете своё приключение с первого квеста. Чтобы его выполнить, вам необходимо поцеловать любого пользователя 3 раза. Если вы используете компьютер, вам нужно нажать ПКМ (правую кнопку мыши). Если же вы пользуетесь мобильным устройством, то зажмите по человеку и выберите пункты «Приложения» и затем «Поцеловать».",
                                                                                           "progress": 0, "progress_max": 3, "reward": 100, "id": "first_q", "do": "kiss"})
            await cursor.execute("INSERT INTO `users` (id, quests) VALUES (?, ?)", (inter.user.id, json.dumps(quests)))
            await db.commit()
            return True

    if me[1] == 1:
        await inter.response.send_message("Вы забанены в боте.", ephemeral=True)
        return False
    
    await check_first_quest(inter, me)

    return True

async def add_random_quest(user: discord.User = None):
    with open(path+'/random_quests.json', 'r', encoding="utf-8") as json_file:
        random_quests = json.load(json_file)

    if user == None:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `users`")
            users = await cursor.fetchall()
        
            for user1 in users:
                quests: list = json.loads(user1[3])
                if len(quests) >= 5: break
                quests.append(random.choice(random_quests))
                await cursor.execute("UPDATE `users` SET (quests) = (?) WHERE id =?", (json.dumps(quests), user1[0]))

            await db.commit()

    else:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `users` WHERE id = ?", (user.id,))
            me = await cursor.fetchone()
            quests: list = json.loads(me[3])
            if len(quests) >= 5: return
            quests.append(random.choice(random_quests))
            await cursor.execute("UPDATE `users` SET (quests) = (?) WHERE id =?", (json.dumps(quests), user.id))
            await db.commit()