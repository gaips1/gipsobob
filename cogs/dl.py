from discord.ext import commands
import discord
import aiosqlite
import os
from datetime import datetime
import random
import asyncio
import pytz
from discord import app_commands
import inspect, os.path

filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
images = path + "/images/"
images = images.replace("cogs/", "")

class DL(commands.Cog):
    def __init__(self, bot):
        self.bot: commands.Bot = bot

    class confdel(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Да", style=discord.ButtonStyle.danger)
        async def yesdel_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("DELETE FROM dl WHERE id = ?", (inter.user.id,))
                await db.commit()
            await inter.response.edit_message(content="Пока, путник!", view=None, embed=None)

        @discord.ui.button(label="Нет", style=discord.ButtonStyle.success)
        async def nodel_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            await inter.response.edit_message(content="Молодец, что одумался!", view=DL.mn(), embed=None)

    class seller(discord.ui.Modal):
        def __init__(self, *args, **kwargs):
            super().__init__(title="Что желаешь купить, путник?", *args, **kwargs)

        item = discord.ui.TextInput(label="Всё по 399 монет! (хп/урон/мана)", required=True)

        async def on_submit(self, inter: discord.Interaction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()

                if user[3] < 399: return await inter.response.send_message("Недостаточно средств", ephemeral=True)

                if self.item.value.lower() == "хп":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.user.id))
                    await cursor.execute("UPDATE `dl` SET health = ? WHERE id = ?", (user[4] + 10, inter.user.id))

                elif self.item.value.lower() == "урон":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.user.id))
                    await cursor.execute("UPDATE `dl` SET damage = ? WHERE id = ?", (user[6] + 10, inter.user.id))

                elif self.item.value.lower() == "мана":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.user.id))
                    await cursor.execute("UPDATE `dl` SET mana = ? WHERE id = ?", (user[5] + 10, inter.user.id))

                else: return await inter.response.send_message("Такого товара нет в наличии. Попробуйте ещё раз", ephemeral=True)
                await db.commit()
                await inter.response.send_message("Успешно!", ephemeral=True)

    class mn(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Отправиться в лабиринт", style=discord.ButtonStyle.danger, row=1)
        async def labirint(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()
                await cursor.execute('UPDATE dl SET ingm = ? WHERE id = ?', (1, inter.user.id))
                await db.commit()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.edit_message(content="Вы вошли в лабиринт.", embed=None, view=None)

            await asyncio.sleep(1.5)

            m_names = []
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT name FROM `ms`")
                monsters_name = await cursor.fetchall()
                for m_name in monsters_name:
                    m_names.append(m_name[0])
                r_name = random.choice(m_names)
                await cursor.execute("SELECT * FROM `ms` WHERE name = ?", (r_name,))
                monster = await cursor.fetchone()
            avatar = images + monster[4]
            embed=discord.Embed(title="Лабиринт", description=f"**Вы наткнулись на {r_name}!**", color=discord.Color.random())
            msg: discord.Interaction = await inter.original_response()

            await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
            await asyncio.sleep(1.75)
            player_hp = user[4]
            monster_hp = monster[1]
            while player_hp > 0 and monster_hp > 0:
                monster_hp -= user[6]
                await asyncio.sleep(2)
                embed=discord.Embed(title=r_name, description=f"**Вы атакуете...**", color=discord.Color.random())
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                if monster_hp <= 0:
                    embed=discord.Embed(title=r_name, description=f"**Ты победил {r_name}!\nЗа победу тебе выдали {monster[2]} монет!**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())
                    async with aiosqlite.connect(dbn, timeout=20) as db:
                        cursor = await db.cursor()
                        await cursor.execute('UPDATE dl SET balance = ? WHERE id = ?', (user[3]+monster[2], inter.user.id))
                        await db.commit()
                    break
                else:
                    embed=discord.Embed(title=r_name, description=f"**У {r_name} осталось {monster_hp} хп!**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                embed=discord.Embed(title=r_name, description=f"**{r_name} атакует...**", color=discord.Color.random())
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                player_hp -= monster[3]
                if player_hp <= 0:
                    slivm = random.randint(1,99)
                    embed=discord.Embed(title=r_name, description=f"**Вы проиграли ): {r_name} спиздил у вас {slivm} монет**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())
                    async with aiosqlite.connect(dbn, timeout=20) as db:
                        cursor = await db.cursor()
                        await cursor.execute('UPDATE dl SET balance = ? WHERE id = ?', (user[3]-slivm, inter.user.id))
                        await db.commit()
                    break
                else:
                    embed=discord.Embed(title=r_name, description=f"**У вас осталось {player_hp} хп**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                    continue

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute('UPDATE dl SET ingm = ? WHERE id = ?', (0, inter.user.id))
                await db.commit()

        @discord.ui.button(label="Информация о игроке", style=discord.ButtonStyle.success, row=1)
        async def playerinfo(self, inter: discord.Interaction, button: discord.ui.Button,):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()
                
            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            embed = discord.Embed(title=f"Ваше имя: {user[1]}\nКласс: {user[2]}\nБаланс: {user[3]}\nХП: {user[4]}\nМана: {user[5]}\nУрон: {user[6]}", color=discord.Color.random())
            now = datetime.now(pytz.timezone('Europe/Moscow'))
            embed.set_footer(text=f'Дромляндия: Онлайн • Сегодня в {now.strftime("%H:%M")}')

            await inter.response.edit_message(content="", embed=embed, view=self)

        @discord.ui.button(label="Удалить персонажа", style=discord.ButtonStyle.danger, row=2)
        async def deleteplayer(self, inter: discord.Interaction, button: discord.ui.Button,):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.edit_message(content=f"Вы действительно хотите удалить персонажа?\nВы потеряете {user[3]} монет!", embed=None, view=DL.confdel())

        @discord.ui.button(label="Магазин", style=discord.ButtonStyle.success, row=2)
        async def shop_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.send_modal(DL.seller())

    class regmodal(discord.ui.Modal):
        def __init__(self, *args, **kwargs):
            super().__init__(title="Анкета ввода данных", *args, **kwargs)

        name = discord.ui.TextInput(label="Твоё имя", required=True, max_length=50)
        clas = discord.ui.TextInput(label="Твой класс (маг/воин/танк)",required=True)

        async def on_submit(self, inter: discord.Interaction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                if self.clas.value == "маг":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (self.name.value, inter.user.id, 'маг', 0, 59, 200, 140))
                elif self.clas.value == "воин" or self.clas.value == "войн":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (self.name.value, inter.user.id, 'воин', 0, 100, 10, 100))
                elif self.clas.value == "танк":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (self.name.value, inter.user.id, 'танк', 0, 159, 0, 100))
                else:
                    return await inter.response.send_message("Вы ввели неправильный класс. Попробуйте ещё раз", ephemeral=True)
                await db.commit()

            await inter.response.edit_message(content=f"Добро пожаловать, {self.clas.value} {self.name.value}", view=DL.mn())

    class regb(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Ввести данные", style=discord.ButtonStyle.success)
        async def vvestidannie(self, inter: discord.Interaction, button: discord.ui.Button,):
            await inter.response.send_modal(DL.regmodal())

    @app_commands.command( description="Войти в Дромляндия: Онлайн", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def game(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.user.id,))
            user = await cursor.fetchone()
        if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=self.regb(), ephemeral=True)
        if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)
        await inter.response.send_message(f"Добро пожаловать обратно, {user[2]} {user[1]}", view=self.mn(), ephemeral=True)

async def setup(bot: commands.Bot):
    await bot.add_cog(DL(bot))
    global dbn
    dbn = bot.dbn
    print("DL cog loaded")