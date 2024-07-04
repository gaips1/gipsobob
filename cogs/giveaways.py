from disnake.ext import commands
import disnake
import aiosqlite
import os
from datetime import datetime, timedelta
import random
import asyncio
import pytz
import schedule
import ast

class dm(disnake.ui.View):
    def __init__(self, id):
        super().__init__(timeout=None)
        self.id = id

    @disnake.ui.button(label="Не хочу участвовать", style=disnake.ButtonStyle.danger)
    async def delete_ga(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `ga` WHERE id = ?", (self.id,))
            giveaway = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `users` WHERE id = ?", (inter.author.id,))
            me = await cursor.fetchone()
        if not me:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("INSERT INTO `users` (id) VALUES (?)", (inter.author.id,))
                await db.commit()
        
            me = [0, 0]

        if me[1] == 1:
            return await inter.response.send_message("Вы забанены в боте.", ephemeral=True)
        
        users: list = ast.literal_eval(giveaway[2])
        users.remove(inter.author.id)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE `ga` SET users = ? WHERE id = ?", (str(users), self.id,))
            await db.commit()
        
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)

        await inter.followup.send("Успешно!", ephemeral=True)

class gab(disnake.ui.View):
    def __init__(self, giveaway):
        super().__init__(timeout=None)
        self.giveaway: list = giveaway

    @disnake.ui.button(label="Принять участие", style=disnake.ButtonStyle.success, custom_id="fdef")
    async def join_ga(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `ga` WHERE id = ?", (inter.message.id,))
            giveaway = await cursor.fetchone()
        users = ast.literal_eval(giveaway[2])
        if inter.author.id in users: return await inter.response.send_message("Вы уже участвуете в розыгрыше!", ephemeral=True, view=dm(inter.message.id))
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            users.append(inter.author.id)
            await cursor.execute("UPDATE `ga` SET users = ? WHERE id = ?", (str(users), inter.message.id,))
            await db.commit()
        
        await inter.response.send_message("Успешно!", ephemeral=True)

class GA(commands.Cog):
    def __init__(self, bot):
        self.bot: commands.InteractionBot = bot
        bot.loop.create_task(self.start_giveaway())
        bot.loop.create_task(self.runs())

    async def runs(self):
        await self.bot.wait_until_ready()
        while True:
            target_time = datetime.now(pytz.timezone('Europe/Moscow')).replace(hour=12, minute=00, second=0, microsecond=0)
            if datetime.now(pytz.timezone('Europe/Moscow')) > target_time:
                target_time += timedelta(days=1)
            time_difference = target_time - datetime.now(pytz.timezone('Europe/Moscow'))
            for i in range(int(time_difference.total_seconds()), 0, -1):
                await asyncio.sleep(1)

            await self.evday()

    async def evday(self):
        await self.bot.wait_until_ready()
        now = datetime.now(pytz.timezone('Europe/Moscow')).replace(minute=0, second=0, microsecond=0)
        ends = now + timedelta(hours=12)

        ends = int(ends.timestamp())
        embed = disnake.Embed(title=f"Ежедневный розыгрыш 100 бебр", description=
                              f"**Чтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{ends}:R>**",
                              color=disnake.Color.random())
        channel = await self.bot.fetch_channel(843475272107163648)
        serv: disnake.Guild = self.bot.get_guild(621378615174758421)
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

            channel: disnake.TextChannel = await self.bot.fetch_channel(giveaway[4])
            msg: disnake.Message = await channel.fetch_message(giveaway[0])
            await msg.edit(components=[disnake.ui.Button(label="Розыгрыш обкончен", style=disnake.ButtonStyle.success, disabled=True)])
            users = ast.literal_eval(giveaway[2])
            if len(users) == 0: return await msg.reply(embed=disnake.Embed(title="К сожалению, никто не поучаствовал в розыгрыше :(", colour=0xf50000))
            winner = random.choice(users)
            winner: disnake.User = await self.bot.getch_user(winner)

            await msg.reply(embed=disnake.Embed(title="Ура! У нас есть победитель",
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

            await winner.send("Поздравляю с победой в розыгрыше!", components=[disnake.ui.Button(label="Розыгрыш", style=disnake.ButtonStyle.url, url=msg.jump_url)])

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

    @commands.slash_command(description="Создать розыгрыш", integration_types=[0,1], contexts=[0,1,2], options=[
        disnake.Option(name="amount", description="На сколько бебр розыгрыш?", required=True, type=disnake.OptionType.integer),
        disnake.Option(name="opis", description="Описание розыгрыша", required=True, type=disnake.OptionType.string),
        disnake.Option(name="ends", description="Когда заканчивается розыгрыш", required=True, type=disnake.OptionType.string),
    ])
    async def create_giveaway(self, inter: disnake.ApplicationCommandInteraction):
        if inter.author.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)

        embed = disnake.Embed(title=f"Розыгрыш {inter.options.get("amount")} бебр", description=
                              f"**{inter.options.get("opis")}\nЧтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{inter.options.get("ends")}:R>**",
                              color=disnake.Color.random())
        msg = await inter.channel.send(embed=embed, view=gab([0, int(inter.options.get("ends")), [], int(inter.options.get("amount")), 0]))

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("INSERT INTO `ga` (id, timestamp, users, amount, channel) VALUES (?, ?, ?, ?, ?)", (msg.id, inter.options.get("ends"), "[]", int(inter.options.get("amount")), msg.channel.id))
            await db.commit()

        await inter.response.send_message("Успешно!", ephemeral=True)

        await self.start_giveaway([msg.id, int(inter.options.get("ends")), [], int(inter.options.get("amount")), msg.channel.id])

def setup(bot: commands.InteractionBot):
    bot.add_cog(GA(bot))
    global dbn
    dbn = bot.dbn
    print("GA cog loaded")