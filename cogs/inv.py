from datetime import datetime
import inspect
import os
from discord.ext import commands, tasks
import discord
import aiosqlite
import asyncio
import json
import random
from discord import app_commands
import pytz
import ext

filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))

class Inv(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        self.add_quest.start()
        self.timeout_quests_timer.start()

    @app_commands.command( description="Магазин", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def shop(self, ctx: discord.Interaction):
        await ctx.response.send_message("Добро пожаловать в круглосуточный магазин <<**У легенды**>>\n||Внимание! После завершения операции ваша душа будет\nавтоматически передана в вечное пользование Uzbia Inc.||", ephemeral=True, view=shop())

    @app_commands.command( description="Инвентарь", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def inventory(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT inv FROM users WHERE id = ?", (inter.user.id,))
            usr = await cursor.fetchone()
            
            if not usr[0]:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
            inv: dict = json.loads(usr[0])
            if len(inv) == 0:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
            inv_items = []

            for i in inv.keys():
                inv_items.append(f"**{i}** - {inv[i]}")
            
            await inter.response.send_message("Название предмета - количество у Вас в инвентаре:\n\n" + ",\n".join(inv_items) , ephemeral=True)

    def zov():
        return [app_commands.Choice(name="Талон на секс", value="талон на секс")]

    @app_commands.command( description="Использовать предмет из инвентаря", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(what="Что использовать?", user="На ком использовать?")
    @app_commands.choices(what=zov())
    @app_commands.check(ext.check)
    async def use(self, ctx: discord.Interaction, what: str, user: discord.User = None):
        if what == "талон на секс":
            if user == None: return await ctx.response.send_message("Укажите пользователя", ephemeral=True)
            await use_sex_talon(ctx, user)
        
    @app_commands.command(description="Квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def quests(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT quests FROM users WHERE id = ?", (inter.user.id,))
            quests = await cursor.fetchone()

        if not quests or not quests[0] or len(json.loads(quests[0])) == 0:
            embed = discord.Embed(title="Квесты", color=discord.Color.random(), 
                                description="У вас нет доступных квестов.")
        else:
            embed = discord.Embed(title="Квесты", color=discord.Color.random())
            quests: list = json.loads(quests[0])
            for quest in quests:
                ends = datetime.fromisoformat(quest["ends"]) if quest["ends"] != None else None
                embed.add_field(name=quest["name"], value=f"{quest["desc"]}\nВыполнено - {quest["progress"]}/{quest["progress_max"]}\nНаграда: {quest["reward"]} бебр\nИстекает: {"<t:" + str(int(ends.timestamp())) + ":R>" if ends != None else "Никогда"}", inline=False)

        await inter.response.send_message(embed=embed, ephemeral=True)

    @app_commands.command(description="Дать рандомный квест", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def add_random_quest(self, inter: discord.Interaction, user: discord.User = None):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
        await inter.response.send_message("Начинаю выдавать...", ephemeral=True)
        await ext.add_random_quest(user, self.bot)
        await inter.edit_original_response(content="Успешно!")

    @app_commands.command(description="Выполненые квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def completed_quests(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT completed_quests FROM users WHERE id = ?", (inter.user.id,))
            quests = await cursor.fetchone()

        if not quests or not quests[0] or len(json.loads(quests[0])) == 0:
            embed = discord.Embed(title="Выполненные квесты", color=discord.Color.random(), 
                                description="У вас нет выполненных квестов.")
            return await inter.response.send_message(embed=embed, ephemeral=True)
        
        quests: list = json.loads(quests[0])

        with open(path+"/completed_quests-"+str(inter.user.id) + ".txt", "w") as f:
            for quest in quests:
                f.write(f"{quest['name']} - {quest["desc"]}, награда - {quest['reward']}, истекло - {quest["ends"] if quest["ends"] != None else "Никогда"}\n")

        await inter.response.send_message(content="Выполненные квесты:", ephemeral=True, file=discord.File(path+"/completed_quests-"+str(inter.user.id) + ".txt"))
        os.remove(path+"/completed_quests-"+str(inter.user.id) + ".txt")

    @app_commands.command(description="Просроченные квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def expired_quests(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT ended_quests FROM users WHERE id = ?", (inter.user.id,))
            quests = await cursor.fetchone()

        if not quests or not quests[0] or len(json.loads(quests[0])) == 0:
            embed = discord.Embed(title="Просроченные квесты", color=discord.Color.random(), 
                                description="У вас нет просроченных квестов.")
            return await inter.response.send_message(embed=embed, ephemeral=True)
        
        quests: list = json.loads(quests[0])
        
        with open(path+"/expired_quests-"+str(inter.user.id) + ".txt", "w") as f:
            for quest in quests:
                f.write(f"{quest['name']} - {quest["desc"]}, награда - {quest['reward']}, истекло -  {quest["ends"] if quest["ends"] != None else "Никогда"}\n")

        await inter.response.send_message(content="Просроченные квесты:", ephemeral=True, file=discord.File(path+"/expired_quests-"+str(inter.user.id) + ".txt"))
        os.remove(path+"/expired_quests-"+str(inter.user.id) + ".txt")

    @tasks.loop(seconds=1)
    async def add_quest(self):
        now = datetime.now(pytz.timezone('Europe/Moscow'))
        
        target_hour = 12 if self.bot.mode == "PROD" else 12
        target_minute = 0 if self.bot.mode == "PROD" else 38
        
        if now.hour == target_hour and now.minute == target_minute:
            if 0 <= now.second < 1:
                await ext.add_random_quest(bot=self.bot)
                await asyncio.sleep(60 - now.second)

    @tasks.loop(seconds=10)
    async def timeout_quests_timer(self):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM users")
            users = await cursor.fetchall()

            for user in users:
                quests = json.loads(user[3])
                for quest in quests:
                    now = datetime.now(pytz.timezone('Europe/Moscow'))
                    quest_end_time = datetime.fromisoformat(quest["ends"]).astimezone(pytz.timezone('Europe/Moscow')) if quest["ends"] != None else None

                    if quest_end_time != None and quest_end_time <= now:
                        usr = await ext.get_or_fetch_user(bot=self.bot, id=user[0])
                        self.bot.loop.create_task(ext.handle_quest_timeout(usr, quest))
                        await asyncio.sleep(0.15)

async def use_sex_talon(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Нельзя использовать на боте", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Нельзя использовать на себе", ephemeral=True)
    async with aiosqlite.connect(dbn) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT inv FROM users WHERE id = ?", (inter.user.id,))
        usr = await cursor.fetchone()
        
        if usr:
            if not usr[0]:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
            inv: dict = json.loads(usr[0])
            if len(inv) == 0:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
        else:
            return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)

        for i in inv.keys():
            if i == "Талон на секс":
                giffs = ["https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif", "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif", "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif"
                            , "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif", "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif"]
                randgif = random.choice(giffs)
                soglaz=discord.Embed(title=f"**{inter.user.name} использует талон на секс и ебётся с {user.name}**", color=discord.Color.random())
                soglaz.set_image(url=randgif)
                await inter.response.send_message(embed=soglaz)
                inv[i] -= 1
                if inv[i] <= 0:
                    await cursor.execute("UPDATE users SET inv =? WHERE id =?", (None, inter.user.id))
                else:
                    await cursor.execute("UPDATE users SET inv =? WHERE id =?", (json.dumps(inv), inter.user.id))
                await db.commit()
                return
        
    await inter.response.send_message("У вас нет талонов на секс", ephemeral=True)

class shop(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Талон на секс [2200]", style=discord.ButtonStyle.success, custom_id="talon_na_sex")
    async def talon_na_sex(self, interaction: discord.Interaction, button: discord.ui.Button):
        await interaction.response.send_message("Талон на секс позволяет занятся сексом с любым пользователем дискорда, без его согласия.\nВы подтверждаете покупку?", ephemeral=True, view=confirm_buy(item="Талон на секс"))

class confirm_buy(discord.ui.View):
    def __init__(self, item: str):
        super().__init__(timeout=None)
        self.item = item
    
    @discord.ui.button(label="Приобрести", style=discord.ButtonStyle.success, custom_id="buy")
    async def buy(self, interaction: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT balance, inv FROM sbp WHERE id = ?", (interaction.user.id,))
            usr = await cursor.fetchone()

            if usr[0] < 2200:
                return await interaction.response.send_message("Недостаточно средств!", ephemeral=True)
            
            if usr[1]:
                inv = json.loads(usr[1]) 
                inv[self.item] = inv.get(self.item, 0) + 1
            else:
                inv = {self.item: 1}

            inv_json = json.dumps(inv)

            await cursor.execute("UPDATE sbp SET balance = balance - 2200 WHERE id = ?", (interaction.user.id, ))
            await cursor.execute("UPDATE users SET inv = ? WHERE id = ?", (inv_json, interaction.user.id))
            await db.commit()

        await interaction.response.edit_message(content="Вы успешно приобрели " + self.item, view=None)

async def setup(bot: commands.Bot):
    global dbn
    dbn = bot.dbn
    await bot.add_cog(Inv(bot))
    print("INV cog loaded")
