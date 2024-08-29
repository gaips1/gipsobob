from discord.ext import commands, tasks
from discord import app_commands
import discord
import aiosqlite
import os
from datetime import datetime, timedelta
from check import check, update_quest
from discord.ui import Button, View
import random
import asyncio
import pytz
import schedule
import ast

class dm(discord.ui.View):
    def __init__(self, id):
        super().__init__(timeout=None)
        self.id = id

    @discord.ui.button(label="Не хочу участвовать", style=discord.ButtonStyle.danger)
    async def delete_ga(self, inter: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `ga` WHERE id = ?", (self.id,))
            giveaway = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `users` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()
        if not me:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("INSERT INTO `users` (id) VALUES (?)", (inter.user.id,))
                await db.commit()
        
            me = [0, 0]

        if me[1] == 1:
            return await inter.response.send_message("Вы забанены в боте.", ephemeral=True)
        
        users: list = ast.literal_eval(giveaway[2])
        users.remove(inter.user.id)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE `ga` SET users = ? WHERE id = ?", (str(users), self.id,))
            await db.commit()
        
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)

        await inter.followup.send("Успешно!", ephemeral=True)

        update_quest(inter.user, "join-giveaway")

class gab(discord.ui.View):
    def __init__(self, giveaway):
        super().__init__(timeout=None)
        self.giveaway: list = giveaway
        
    @discord.ui.button(label="Принять участие", style=discord.ButtonStyle.success, custom_id="fdef")
    async def join_ga(self, inter: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `ga` WHERE id = ?", (inter.message.id,))
            giveaway = await cursor.fetchone()
        users = ast.literal_eval(giveaway[2])
        if inter.user.id in users: return await inter.response.send_message("Вы уже участвуете в розыгрыше!", ephemeral=True, view=dm(inter.message.id))
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            users.append(inter.user.id)
            await cursor.execute("UPDATE `ga` SET users = ? WHERE id = ?", (str(users), inter.message.id,))
            await db.commit()
        
        await inter.response.send_message("Успешно!", ephemeral=True)

class view_giv(discord.ui.View):
    def __init__(self, url):
        super().__init__(timeout=None)
        self.url = url

        self.add_item(discord.ui.Button(label="Розыгрыш", style=discord.ButtonStyle.url, url=self.url))

class GA(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        bot.loop.create_task(self.start_giveaway())
        self.runs.start()

    @tasks.loop(seconds=1)
    async def runs(self):
        now = datetime.now(pytz.timezone('Europe/Moscow'))
        target_hour = 12
        target_minute = 0
        if now.hour == target_hour and now.minute == target_minute:
            if 0 <= now.second < 1:
                await self.evday()
                await asyncio.sleep(60 - now.second)

    async def evday(self):
        await self.bot.wait_until_ready()
        now = datetime.now(pytz.timezone('Europe/Moscow')).replace(minute=0, second=0, microsecond=0)
        ends = now + timedelta(hours=12)
        #ends = now + timedelta(minutes=38)

        ends = int(ends.timestamp())
        embed = discord.Embed(title=f"Ежедневный розыгрыш 100 бебр", description=
                              f"**Чтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{ends}:R>**",
                              color=discord.Color.random())
        
        channel = await self.bot.fetch_channel(843475272107163648)
        #channel = await self.bot.fetch_channel(1057539983927410709)

        serv: discord.Guild = self.bot.get_guild(621378615174758421)
        role = serv.get_role(968467508724138014)
        msg = await channel.send(content=role.mention, embed=embed, view=gab([0, ends, [], 100, 0]))

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("INSERT INTO `ga` (id, timestamp, users, amount, channel) VALUES (?, ?, ?, ?, ?)", (msg.id, ends, "[]", 100, msg.channel.id))
            await db.commit()

        await self.start_giveaway([msg.id, ends, [], 100, msg.channel.id])

    async def start_giveaway(self, giveaway: list = False):
        async def timer(time, giveaway):

            if datetime.now(pytz.timezone('Europe/Moscow')) > time:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("DELETE FROM `ga` WHERE id = ?", (giveaway[0],))
                    await db.commit()

            td = time - datetime.now(pytz.timezone('Europe/Moscow'))
            for i in range(int(td.total_seconds()), 0, -1):
                await asyncio.sleep(1)

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `ga` WHERE id = ?", (giveaway[0],))
                giveaway = await cursor.fetchone()
                await cursor.execute("DELETE FROM `ga` WHERE id = ?", (giveaway[0],))
                await db.commit()

            channel: discord.TextChannel = await self.bot.fetch_channel(giveaway[4])
            msg: discord.Message = await channel.fetch_message(giveaway[0])
            new_gab = View()
            new_gab.add_item(Button(label="Конкурс обкончен", disabled=True, style=discord.ButtonStyle.success))
            await msg.edit(view=new_gab)
            users = ast.literal_eval(giveaway[2])
            if len(users) == 0: return await msg.reply(embed=discord.Embed(title="К сожалению, никто не поучаствовал в розыгрыше :(", colour=0xf50000))
            winner = random.choice(users)
            winner: discord.User = await self.bot.fetch_user(winner)

            await msg.reply(embed=discord.Embed(title="Ура! У нас есть победитель",
                    description=f"Поздравим {winner.mention} с победой, приз уже на его счету!",
                        colour=0x00f51d))
            
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (winner.id,))
                usr = await cursor.fetchone()
                if not usr:
                    await cursor.execute("INSERT INTO `sbp` (id) VALUES (?)", (winner.id,))
                    await db.commit()

                await cursor.execute(f'UPDATE sbp SET balance = {int(usr[1])+int(giveaway[3])} WHERE id = {winner.id}')
                await db.commit()

            await winner.send("Поздравляю с победой в розыгрыше!", view=view_giv(msg.jump_url))

        if not giveaway:
            await self.bot.wait_until_ready()
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `ga`")
                all = await cursor.fetchall()

            if len(all)  ==  0: return
            for x in all:
                self.bot.add_view(gab(x))
                ends = datetime.fromtimestamp(timestamp=int(x[1]), tz=pytz.timezone('Europe/Moscow'))
                self.bot.loop.create_task(timer(ends, x))
        else:
            ends = datetime.fromtimestamp(timestamp=int(giveaway[1]), tz=pytz.timezone('Europe/Moscow'))
            self.bot.loop.create_task(timer(ends, giveaway))

    @app_commands.command( description="Создать розыгрыш", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(amount="На сколько бебр розыгрыш?", opis="Описание розыгрыша", ends="Когда заканчивается розыгрыш?")
    @app_commands.check(check)
    async def create_giveaway(self, inter: discord.Interaction, amount: int, opis: str, ends: str):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)

        embed = discord.Embed(title=f"Розыгрыш {amount} бебр", description=
                              f"**{opis}\nЧтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{ends}:R>**",
                              color=discord.Color.random())
        msg = await inter.channel.send(embed=embed, view=gab([0, int(ends), [], int(amount), 0]))

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("INSERT INTO `ga` (id, timestamp, users, amount, channel) VALUES (?, ?, ?, ?, ?)", (msg.id, ends, "[]", int(amount), msg.channel.id))
            await db.commit()

        await inter.response.send_message("Успешно!", ephemeral=True)

        await self.start_giveaway([msg.id, int(ends), [], int(amount), msg.channel.id])

async def setup(bot: commands.Bot):
    await bot.add_cog(GA(bot))
    global dbn
    dbn = bot.dbn
    print("GA cog loaded")