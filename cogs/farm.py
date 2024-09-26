from datetime import datetime
from decimal import Decimal
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
from enum import Enum

class Cards(Enum):
    GTX760 = {"name": "GTX 760", "ins": 0.001, "price": 10}
    GTX1050_TI = {"name": "GTX 1050 TI", "ins": 0.015, "price": 50}
    GTX1060_TI = {"name": "GTX 1060 TI", "ins": 0.05, "price": 200}
    RTX3060_TI = {"name": "RTX 3060 TI", "ins": 0.2, "price": 1000}
    RTX4080 = {"name": "RTX 4080", "ins": 1.0, "price": 5000}
    RTX4090_TI = {"name": "RTX 4090 TI", "ins": 2.5, "price": 20000}
    TESLA_T4 = {"name": "Tesla T4", "ins": 5.0, "price": 50000}
    TESLA_T4_X2 = {"name": "Tesla T4 X2", "ins": 12.0, "price": 100000}

    @property
    def ins(self):
        return self.value["ins"]

    @property
    def price(self):
        return self.value["price"]
    
    @property
    def name(self):
        return self.value["name"]

class FarmMenu():
    class FarmBuyView(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Покупаю! [5000 Б]", style=discord.ButtonStyle.success, emoji="💵")
        async def yesbuyfarm(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT balance FROM sbp WHERE id =?", (inter.user.id,))
                balance = await cursor.fetchone()
                if not balance: return await inter.response.send_message(ephemeral=True,embed=discord.Embed(
                    title="Нет счёта!", description="Вижу, тебе нечем платить!\nДля начала зарегистрируйся в СБП используя /reg, а затем заработай бебр.\nВообще-то, существует много способов их заработать, например ввод капчи /captcha, казино /casino, выполнение ежедневных квестов /quests, а так-же участие в розыгрышах у меня на сервере! (ссылка у меня в профиле)", color=discord.Color.red()))
                if balance[0] < 5000: return await inter.response.send_message(ephemeral=True,embed=discord.Embed(
                    title="Вы бомж!", description="Увы, оплата не прошла — у тебя не хватает бебр на счету.\nВообще-то, существует много способов их заработать, например ввод капчи /captcha, казино /casino, выполнение ежедневных квестов /quests, а так-же участие в розыгрышах у меня на сервере! (ссылка у меня в профиле)", color=discord.Color.red()))

                await cursor.execute("UPDATE sbp SET balance = balance - 5000 WHERE id =?", (inter.user.id,))
                await cursor.execute("INSERT INTO `farms` (id) VALUES (?)", (inter.user.id,))
                await db.commit()

            await inter.response.edit_message(embed=discord.Embed(
                    title="Успешная покупка!", description="Спасибо за покупку моей фермы, теперь она твоя, развлекайся!", color=discord.Color.green()), view=FarmMenu.FarmMMView())
            
    class FarmMMView(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Купить новую видеокарту", style=discord.ButtonStyle.success, emoji="🛒")
        async def buycards(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.send_message(content="Каталог видеокарт:", view=FarmMenu.BuyCardsView(), embed=None, ephemeral=True)

        @discord.ui.button(label="Мой счёт", style=discord.ButtonStyle.blurple, emoji="💵",row=2)
        async def mybalance(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM farms WHERE id =?", (inter.user.id,))
                usr = await cursor.fetchone()

            embed = discord.Embed(
                title="Мой счёт",
                description=f"Баланс УзбиКойнов: {usr[1]:.3f}\nУзбиКойнов в секунду: {usr[2]:.3f}",
                color=discord.Color.random()
            )

            await inter.response.edit_message(embed=embed, view=FarmMenu.FarmMMView())

        @discord.ui.button(label="Мои видеокарты", style=discord.ButtonStyle.blurple, emoji="🕹️",row=2)
        async def mycards(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT cards FROM farms WHERE id =?", (inter.user.id,))
                cards = await cursor.fetchone()
            cards = json.loads(cards[0])

            embed = discord.Embed(
                title="Мои видеокарты",
                description="\n".join([f"**{card['name']}**: +{card['ins']} UCS - {card["count"]} шт." for card in cards]),
                color=discord.Color.random()
            )
            await inter.response.edit_message(embed=embed, view=FarmMenu.FarmMMView())

        @discord.ui.button(label="Вывод UCS", style=discord.ButtonStyle.danger, emoji="📤",row=3)
        async def vivoducs(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.send_modal(FarmMenu.exportucs())

    class BuyCardsView(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)
            for i, card in enumerate(Cards):
                label = f"{card.value['name']} [{card.value['price']:,} UCS]"
                button = discord.ui.Button(
                    label=label,
                    style=discord.ButtonStyle.success,
                    emoji="💵",
                    row=i // 2 
                )
                button.callback = self.create_button_callback(card)
                self.add_item(button)

        def create_button_callback(self, card):
            async def button_callback(interaction: discord.Interaction):
                if await FarmMenu.buy_card(card=card, user_id=interaction.user.id):
                    await interaction.response.send_message("Успешно!", ephemeral=True, embed=None)
                else:
                    await interaction.response.send_message("Не хватает УзбиКойнов!", ephemeral=True)
            
            return button_callback
        
    async def buy_card(card: Cards, user_id: int | str):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms WHERE id =?", (user_id,))
            usr = await cursor.fetchone()

        if usr[1] < card.price: return False

        cards = json.loads(usr[3])
        for card1 in cards:
            if card1["name"] == card.name:
                card1["count"] += 1
                break
        else:
            cards.append({"name": card.name, "ins": card.ins, "count": 1})
        
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE farms SET (cards, balance, ins) = (?, balance -?, ins +?) WHERE id =?", (json.dumps(cards), card.price, card.ins, user_id))
            await db.commit()

        return True

    class exportucs(discord.ui.Modal, title = "Вывод UCS | 0.005 за 1 бебру"):
        def __init__(self):
            super().__init__()

        value = discord.ui.TextInput(label="Введите желаемое количество бебр для вывода", required=True)

        async def on_submit(self, inter: discord.Interaction):
            try:
                value = float(self.value.value)
            except:
                return await inter.response.send_message("Введите целое, либо дробное число (0.1)!", ephemeral=True)
            
            if value < 500:
                return await inter.response.send_message("Минимальный вывод - 500 Бебр (100.000 UCS)", ephemeral=True)
            
            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT balance FROM farms WHERE id =?", (inter.user.id,))
                balance = await cursor.fetchone()
            
            balance = balance[0]
            ucs = value / 0.005

            if balance < ucs:
                return await inter.response.send_message(f"Не хватает {(ucs-balance):.3f} UCS!", ephemeral=True)
            
            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute("UPDATE farms SET balance = balance - ? WHERE id =?", (ucs, inter.user.id))
                await cursor.execute("UPDATE sbp SET balance = balance + ? WHERE id =?", (value, inter.user.id))
                await db.commit()

            await inter.response.send_message(f"Успешно перевёл {value} бебр ({ucs:.3f} UCS) на твой счёт!", ephemeral=True)

class Farm(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        self.timer.start()

    @app_commands.command( description="Майнинговая ферма << У Легенды >>", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def farm(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms WHERE id =?", (inter.user.id,))
            usr = await cursor.fetchone()

        if not usr:
            embed = discord.Embed(
                title="Майнинговая ферма << У Легенды >>",
                description=f"Приветствую тебя на **моей** ферме, {inter.user.global_name}!\nВижу, ты хочешь чтобы я подарил тебе её, но без платы я не могу этого сделать.\nПеред этим тебе нужно будет заплатить мне немного бебр!\n\nНу что, согласен?",
                color=discord.Color.random()
            )

            await inter.response.send_message(embed=embed, view=FarmMenu.FarmBuyView(), ephemeral=True)
        else:
            await inter.response.send_message("Добро пожаловать обратно, " + inter.user.global_name, view=FarmMenu.FarmMMView(), ephemeral=True)

    @tasks.loop(seconds=1)
    async def timer(self):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE farms SET balance = balance + ins")
            await db.commit()

async def setup(bot: commands.Bot):
    global dbn
    dbn = bot.dbn
    await bot.add_cog(Farm(bot))
    print("Farm cog loaded")