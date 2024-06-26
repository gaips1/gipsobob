from disnake.ext import commands
import disnake
import aiosqlite
import os
from datetime import datetime
import random
import asyncio
import pytz
import inspect, os.path

filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
images = path + "/images/"
images = images.replace("cogs/", "")

class DL(commands.Cog):
    def __init__(self, bot):
        self.bot: commands.InteractionBot = bot

    class confdel(disnake.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @disnake.ui.button(label="Да", style=disnake.ButtonStyle.danger)
        async def yesdel_dl(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("DELETE FROM dl WHERE id = ?", (inter.author.id,))
                await db.commit()
            await inter.response.edit_message("Пока, путник!", view=None, embed=None)

        @disnake.ui.button(label="Нет", style=disnake.ButtonStyle.success)
        async def nodel_dl(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            await inter.response.edit_message("Молодец, что одумался!", view=DL.mn(), embed=None)

    class seller(disnake.ui.Modal):
        def __init__(self):
            components = [
                disnake.ui.TextInput(
                    label="Всё по 399 монет! (хп/урон/мана)",
                    custom_id="item",
                    style=disnake.TextInputStyle.short,
                    required=True,
                    max_length=5
                )
            ]
            super().__init__(title="Что желаешь купить, путник?", components=components)

        async def callback(self, inter: disnake.ModalInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
                user = await cursor.fetchone()

                if user[3] < 399: return await inter.response.send_message("Недостаточно средств", ephemeral=True)

                if inter.text_values["item"].lower() == "хп":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.author.id))
                    await cursor.execute("UPDATE `dl` SET health = ? WHERE id = ?", (user[4] + 10, inter.author.id))

                elif inter.text_values["item"].lower() == "урон":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.author.id))
                    await cursor.execute("UPDATE `dl` SET damage = ? WHERE id = ?", (user[6] + 10, inter.author.id))

                elif inter.text_values["item"].lower() == "мана":
                    await cursor.execute("UPDATE `dl` SET balance = ? WHERE id = ?", (user[3] - 399, inter.author.id))
                    await cursor.execute("UPDATE `dl` SET mana = ? WHERE id = ?", (user[5] + 10, inter.author.id))

                else: return await inter.response.send_message("Такого товара нет в наличии. Попробуйте ещё раз", ephemeral=True)
                await db.commit()
                await inter.response.send_message("Успешно!", ephemeral=True)

    class mn(disnake.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @disnake.ui.button(label="Отправиться в лабиринт", style=disnake.ButtonStyle.danger, row=1)
        async def labirint(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
                user = await cursor.fetchone()
                await cursor.execute('UPDATE dl SET ingm = ? WHERE id = ?', (1, inter.author.id))
                await db.commit()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.edit_message("Вы вошли в лабиринт.", embed=None, view=None)

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
            embed=disnake.Embed(title="Лабиринт", description=f"**Вы наткнулись на {r_name}!**", color=disnake.Color.random())
            embed.set_thumbnail(file=disnake.File(avatar))
            msg: disnake.InteractionMessage = await inter.original_message()

            await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
            await asyncio.sleep(1.75)
            player_hp = user[4]
            monster_hp = monster[1]
            while player_hp > 0 and monster_hp > 0:
                monster_hp -= user[6]
                await asyncio.sleep(2)
                embed=disnake.Embed(title=r_name, description=f"**Вы атакуете...**", color=disnake.Color.random())
                embed.set_thumbnail(file=disnake.File(avatar))
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                if monster_hp <= 0:
                    embed=disnake.Embed(title=r_name, description=f"**Ты победил {r_name}!\nЗа победу тебе выдали {monster[2]} монет!**", color=disnake.Color.random())
                    embed.set_thumbnail(file=disnake.File(avatar))
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())
                    async with aiosqlite.connect(dbn, timeout=20) as db:
                        cursor = await db.cursor()
                        await cursor.execute('UPDATE dl SET balance = ? WHERE id = ?', (user[3]+monster[2], inter.author.id))
                        await db.commit()
                    break
                else:
                    embed=disnake.Embed(title=r_name, description=f"**У {r_name} осталось {monster_hp} хп!**", color=disnake.Color.random())
                    embed.set_thumbnail(file=disnake.File(avatar))
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                embed=disnake.Embed(title=r_name, description=f"**{r_name} атакует...**", color=disnake.Color.random())
                embed.set_thumbnail(file=disnake.File(avatar))
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                await asyncio.sleep(2)
                player_hp -= monster[3]
                if player_hp <= 0:
                    slivm = random.randint(1,99)
                    embed=disnake.Embed(title=r_name, description=f"**Вы проиграли ): {r_name} спиздил у вас {slivm} монет**", color=disnake.Color.random())
                    embed.set_thumbnail(file=disnake.File(avatar))
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())
                    async with aiosqlite.connect(dbn, timeout=20) as db:
                        cursor = await db.cursor()
                        await cursor.execute('UPDATE dl SET balance = ? WHERE id = ?', (user[3]-slivm, inter.author.id))
                        await db.commit()
                    break
                else:
                    embed=disnake.Embed(title=r_name, description=f"**У вас осталось {player_hp} хп**", color=disnake.Color.random())
                    embed.set_thumbnail(file=disnake.File(avatar))
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                    continue

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute('UPDATE dl SET ingm = ? WHERE id = ?', (0, inter.author.id))
                await db.commit()

        @disnake.ui.button(label="Информация о игроке", style=disnake.ButtonStyle.success, row=1)
        async def playerinfo(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
                user = await cursor.fetchone()
                
            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            embed = disnake.Embed(title=f"Ваше имя: {user[1]}\nКласс: {user[2]}\nБаланс: {user[3]}\nХП: {user[4]}\nМана: {user[5]}\nУрон: {user[6]}", color=disnake.Color.random())
            now = datetime.now(pytz.timezone('Europe/Moscow'))
            embed.set_footer(text=f'Дромляндия: Онлайн • Сегодня в {now.strftime("%H:%M")}')

            await inter.response.edit_message(content="", embed=embed, view=self)

        @disnake.ui.button(label="Удалить персонажа", style=disnake.ButtonStyle.danger, row=2)
        async def deleteplayer(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
                user = await cursor.fetchone()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.edit_message(f"Вы действительно хотите удалить персонажа?\nВы потеряете {user[3]} монет!", embed=None, view=DL.confdel())

        @disnake.ui.button(label="Магазин", style=disnake.ButtonStyle.success, row=2)
        async def shop_dl(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
                user = await cursor.fetchone()

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.send_modal(DL.seller())

    class regmodal(disnake.ui.Modal):
        def __init__(self):
            components = [
                disnake.ui.TextInput(
                    label="Твоё имя",
                    custom_id="name",
                    style=disnake.TextInputStyle.short,
                    required=True,
                    max_length=50
                ),
                disnake.ui.TextInput(
                    label="Твой класс (маг/воин/танк)",
                    required=True,
                    custom_id="class",
                    style=disnake.TextInputStyle.short,
                    max_length=5
                )
            ]
            super().__init__(title="Анкета ввода данных", components=components)

        async def callback(self, inter: disnake.ModalInteraction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                if inter.text_values["class"].lower() == "маг":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (inter.text_values["name"], inter.author.id, 'маг', 0, 59, 200, 140))
                elif inter.text_values["class"].lower() == "воин" or inter.text_values["class"].lower() == "войн":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (inter.text_values["name"], inter.author.id, 'воин', 0, 100, 10, 100))
                elif inter.text_values["class"].lower() == "танк":
                    await cursor.execute("INSERT INTO `dl` (name, id, class, balance, health, mana, damage) VALUES (?, ?, ?, ?, ?, ?, ?)", (inter.text_values["name"], inter.author.id, 'танк', 0, 159, 0, 100))
                else:
                    return await inter.response.send_message("Вы ввели неправильный класс. Попробуйте ещё раз", ephemeral=True)
                await db.commit()

            await inter.response.edit_message(f"Добро пожаловать, {inter.text_values["class"]} {inter.text_values["name"]}", view=DL.mn())

    class regb(disnake.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @disnake.ui.button(label="Ввести данные", style=disnake.ButtonStyle.success)
        async def vvestidannie(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
            await inter.response.send_modal(DL.regmodal())

    @commands.slash_command(description="Войти в Дромляндия: Онлайн", integration_types=[0,1], contexts=[0,1,2])
    async def game(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `dl` WHERE id = ?", (inter.author.id,))
            user = await cursor.fetchone()
        if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=self.regb(), ephemeral=True)
        if user[7] == 1: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)
        await inter.response.send_message(f"Добро пожаловать обратно, {user[2]} {user[1]}", view=self.mn(), ephemeral=True)

def setup(bot: commands.InteractionBot):
    bot.add_cog(DL(bot))
    global dbn
    dbn = bot.dbn
    print("DL cog loaded")