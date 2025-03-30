from datetime import datetime, timedelta
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
    GTX1050_TI = {"name": "GTX 1050 TI", "ins": 0.01, "price": 50}
    GTX1060_TI = {"name": "GTX 1060 TI", "ins": 0.035, "price": 200}
    RTX3060_TI = {"name": "RTX 3060 TI", "ins": 0.15, "price": 2000}
    RTX4080 = {"name": "RTX 4080", "ins": 0.7, "price": 8500}
    RTX4090_TI = {"name": "RTX 4090 TI", "ins": 1.45, "price": 30000}
    TESLA_T4 = {"name": "Tesla T4", "ins": 3.6, "price": 60000}
    TESLA_T4_X2 = {"name": "Tesla T4 X2", "ins": 7.5, "price": 120000}

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
                description=f"Баланс УзбиКойнов: {usr[1]:.3f}\nУзбиКойнов в минуту: {usr[2]:.3f}",
                color=discord.Color.random()
            )

            await inter.response.edit_message(embed=embed, view=FarmMenu.FarmMMView())

        @discord.ui.button(label="Мои видеокарты", style=discord.ButtonStyle.blurple, emoji="🕹️",row=2)
        async def mycards(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM farms WHERE id =?", (inter.user.id,))
                user = await cursor.fetchone()

            await inter.response.edit_message(embed=await FarmMenu.get_cards_info(inter), view=FarmMenu.FarmCardsView(overheat_notif=user[5]))

        @discord.ui.button(label="Вывод UCS", style=discord.ButtonStyle.danger, emoji="📤",row=3)
        async def vivoducs(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.send_modal(FarmMenu.exportucs())

        @discord.ui.button(label="Бустеры", style=discord.ButtonStyle.danger, emoji="💥",row=3)
        async def boostersbtn(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT booster FROM farms WHERE id =?", (inter.user.id,))
                booster = await cursor.fetchone()

            booster: dict = json.loads(booster[0])
            bstr = f"{booster["name"]} - истекает <t:{booster["ends_timestamp"]}:R>" if len(booster) > 0 else "❌❌❌"

            await inter.response.send_message(embed=discord.Embed(
                title="Покупка бустеров",
                description=f"Бустеры - это временное увеличение вашей статистики, заработка, и т.д.\nБустер, работающий в данный момент:\n\n" + bstr,
                color=discord.Color.random()
            ), ephemeral=True, view=FarmMenu.FarmBoostersBuyView())

    class FarmBoostersBuyView(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        async def buybooster(self, inter: discord.Interaction, booster_to_buy: str):
            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT balance FROM sbp WHERE id =?", (inter.user.id,))
                bal = await cursor.fetchone()
                await cursor.execute("SELECT booster FROM farms WHERE id =?", (inter.user.id,))
                booster = await cursor.fetchone()

                booster: dict = json.loads(booster[0])
                bal: int = bal[0]

                if len(booster) > 0:
                    return await inter.response.send_message("У тебя уже есть активный бустер!", ephemeral=True)

                if booster_to_buy == "x1_5_10min":
                    if bal < 1499: return await inter.response.send_message("Недостаточно средств!", ephemeral=True)
                    ends = datetime.now() + timedelta(minutes=10)
                    booster = {"name": "x1.5 заработок на 10 минут", "ends": ends.isoformat(), "ends_timestamp": int(ends.timestamp())}
                    await cursor.execute("UPDATE sbp SET balance = balance - 1499 WHERE id =?", (inter.user.id,))
                    await cursor.execute("UPDATE farms SET booster = ? WHERE id =?", (json.dumps(booster), inter.user.id,))
                    await db.commit()

                elif booster_to_buy == "x1_5_30min":
                    if bal < 4399: return await inter.response.send_message("Недостаточно средств!", ephemeral=True)
                    ends = datetime.now() + timedelta(minutes=30)
                    booster = {"name": "x1.5 заработок на 30 минут", "ends": ends.isoformat(), "ends_timestamp": int(ends.timestamp())}
                    await cursor.execute("UPDATE sbp SET balance = balance - 4399 WHERE id =?", (inter.user.id,))
                    await cursor.execute("UPDATE farms SET booster = ? WHERE id =?", (json.dumps(booster), inter.user.id,))
                    await db.commit()

                elif booster_to_buy == "x2_10min":
                    if bal < 5999: return await inter.response.send_message("Недостаточно средств!", ephemeral=True)
                    ends = datetime.now() + timedelta(minutes=10)
                    booster ={"name": "x2 заработок на 10 минут", "ends": ends.isoformat(), "ends_timestamp": int(ends.timestamp())}
                    await cursor.execute("UPDATE sbp SET balance = balance - 5999 WHERE id =?", (inter.user.id,))
                    await cursor.execute("UPDATE farms SET booster = ? WHERE id =?", (json.dumps(booster), inter.user.id,))
                    await db.commit()

                elif booster_to_buy == "x2_30min":
                    if bal < 17599: return await inter.response.send_message("Недостаточно средств!", ephemeral=True)
                    ends = datetime.now() + timedelta(minutes=30)
                    booster = {"name": "x2 заработок на 30 минут", "ends": ends.isoformat(), "ends_timestamp": int(ends.timestamp())}
                    await cursor.execute("UPDATE sbp SET balance = balance - 17599 WHERE id =?", (inter.user.id,))
                    await cursor.execute("UPDATE farms SET booster = ? WHERE id =?", (json.dumps(booster), inter.user.id,))
                    await db.commit()

            bstr = f"{booster["name"]} - истекает <t:{booster["ends_timestamp"]}:R>" if len(booster) > 0 else "❌❌❌"

            await inter.response.edit_message(content="Успешно!", embed=discord.Embed(
                title="Покупка бустеров",
                description=f"Бустеры - это временное увеличение вашей статистики, заработка, и т.д.\nБустер, работающий в данный момент:\n\n" + bstr,
                color=discord.Color.random()
            ))

        @discord.ui.button(label="x1.5 заработок на 10 минут [1,499 Б]", style=discord.ButtonStyle.success, emoji="🛒",row=1)
        async def x1_5_10minbuybtn(self, inter: discord.Interaction, button: discord.ui.Button):
            await self.buybooster(inter, "x1_5_10min")

        @discord.ui.button(label="x1.5 заработок на 30 минут [4,399 Б]", style=discord.ButtonStyle.success, emoji="🛒",row=2)
        async def x1_5_30minbuybtn(self, inter: discord.Interaction, button: discord.ui.Button):
            await self.buybooster(inter, "x1_5_30min")

        @discord.ui.button(label="x2 заработок на 10 минут [5,999 Б]", style=discord.ButtonStyle.success, emoji="🛒",row=3)
        async def x2_10minbuybtn(self, inter: discord.Interaction, button: discord.ui.Button):
            await self.buybooster(inter, "x2_10min")

        @discord.ui.button(label="x2 заработок на 30 минут [17,599 Б]", style=discord.ButtonStyle.success, emoji="🛒",row=4)
        async def x2_30minbuybtn(self, inter: discord.Interaction, button: discord.ui.Button):
            await self.buybooster(inter, "x2_30min")

    class FarmCardsView(discord.ui.View):
        def __init__(self, overheat_notif: int):
            super().__init__(timeout=None)
            button = discord.ui.Button(
                label="Включить уведомления о перегрузке сервера",
                style=discord.ButtonStyle.success,
                row=2,
                emoji="🔔"
            ) if overheat_notif == 1 or overheat_notif == 0 else discord.ui.Button(
                label="Выключить уведомления о перегрузке сервера",
                style=discord.ButtonStyle.danger,
                row=2,
                emoji="🔔"
            )
            button.callback = self.on_off_notif_cards_callback()
            self.add_item(button)

        def on_off_notif_cards_callback(self):
            async def on_off_notif_cards(inter: discord.Interaction):
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("SELECT * FROM farms WHERE id =?", (inter.user.id,))
                    user = await cursor.fetchone()

                    if user[5] == 0:
                        return await inter.response.send_message(embed=discord.Embed(
                            title="Покупка уведомлений о перегрузках сервера",
                            description="Данная функция стоит 5.000 бебр!\nПодтверждаете покупку?",
                            color=discord.Color.random()
                        ), ephemeral=True, view=FarmMenu.FarmCardsBuyNotifView())
                    
                    elif user[5] == 1:
                        await cursor.execute("UPDATE farms SET overheat_notif = 2 WHERE id =?", (inter.user.id,))
                        await db.commit()
                        return await inter.response.edit_message(embed=await FarmMenu.get_cards_info(inter), view=FarmMenu.FarmCardsView(overheat_notif=2))
                    
                    elif user[5] == 2:
                        await cursor.execute("UPDATE farms SET overheat_notif = 1 WHERE id =?", (inter.user.id,))
                        await db.commit()
                        return await inter.response.edit_message(embed=await FarmMenu.get_cards_info(inter), view=FarmMenu.FarmCardsView(overheat_notif=1))

            return on_off_notif_cards

        @discord.ui.button(label="Назад", style=discord.ButtonStyle.blurple, emoji="🔙")
        async def backtomeny111111(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.edit_message(embed=await FarmMenu.get_cards_info(inter), view=FarmMenu.FarmMMView())

        @discord.ui.button(label="Перезагрузить сервер", style=discord.ButtonStyle.success, emoji="♻️")
        async def restartcards(self, inter: discord.Interaction, button: discord.ui.Button):
            new_notif = datetime.now(pytz.timezone('Europe/Moscow')) + timedelta(hours=3)
            async with aiosqlite.connect(dbn) as db:
                await db.execute("UPDATE farms SET overheat = ? WHERE id =?", (new_notif.isoformat(), inter.user.id,))
                await db.commit()

            await inter.response.send_message(f"Успешно!\nРекомендую перезапустить сервер <t:{int(new_notif.timestamp())}:f>", ephemeral=True)

    class FarmCardsBuyNotifView(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Подтверждаю [5000 Б]", style=discord.ButtonStyle.success, emoji="🛒")
        async def buycardsnotif(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM sbp WHERE id =?", (inter.user.id,))
                user = await cursor.fetchone()

                if user[1] < 5000:
                    return await inter.response.send_message("Недостаточно средств!", ephemeral=True)
                
                await cursor.execute("UPDATE sbp SET balance = balance -? WHERE id =?", (5000, inter.user.id,))
                await cursor.execute("UPDATE farms SET overheat_notif = 1 WHERE id =?", (inter.user.id,))
                await db.commit()

            await inter.response.edit_message(content="Успешно!", view=None, embed=None)

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

    async def get_cards_info(inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms WHERE id =?", (inter.user.id,))
            user = await cursor.fetchone()

        cards = json.loads(user[3])
        if user[4] != "1":
            time = datetime.fromisoformat(user[4])
            string = f"Ваш сервер в данных момент не перегружен, рекомендую перезапустить его <t:{int(time.timestamp())}:f>"
        else:
            string = "Ваш сервер в данный момент **ПЕРЕГРУЖЕН**, перезапустите его нажатием соответствующей кнопки"

        if user[5] == 2:
            string += f"\nУведомления о перегрузке серверов: включены"
        else:
            string += f"\nУведомления о перегрузке серверов: выключены"

        embed = discord.Embed(
            title="Мои видеокарты",
            description=f'{"\n".join([f"**{card['name']}**: +{card['ins']} UCS - {card["count"]} шт." for card in cards])}\n\n{string}',
            color=discord.Color.random()
        )
        return embed

    class exportucs(discord.ui.Modal, title = "Вывод UCS | 0.002 за 1 бебру"):
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
            ucs = value / 0.002

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
        self.overheat_timer.start()
        self.boosters_timer.start()
        self.sperma_timer.start()

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

    @tasks.loop(minutes=1)
    async def timer(self):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms")
            users = await cursor.fetchall()
            for user in users:
                booster = json.loads(user[6])
                if len(booster) == 0:
                    await cursor.execute(f"UPDATE farms SET balance = balance + ? WHERE overheat != 1 AND id =?", (user[2], user[0],))
                else:
                    if booster["name"] == "x1.5 заработок на 10 минут" or booster["name"] == "x1.5 заработок на 30 минут":
                        await cursor.execute(f"UPDATE farms SET balance = balance + ? WHERE overheat != 1 AND id =?", (user[2]*1.5, user[0],))
                    elif booster["name"] == "x2 заработок на 10 минут" or booster["name"] == "x2 заработок на 30 минут":
                        await cursor.execute(f"UPDATE farms SET balance = balance + ? WHERE overheat != 1 AND id =?", (user[2]*2, user[0],))

            await db.commit()

    @tasks.loop(seconds=10)
    async def overheat_timer(self):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms")
            users = await cursor.fetchall()

            for user in users:
                if user[4] == "1":
                    continue

                elif user[4] == "2":
                    new_notif = (datetime.now(pytz.timezone('Europe/Moscow')) + timedelta(hours=3)).isoformat()
                    await cursor.execute("UPDATE farms SET overheat = ? WHERE id =?", (new_notif, user[0],))
                    await db.commit()
                else:
                    if datetime.fromisoformat(user[4]) <= datetime.now(pytz.timezone('Europe/Moscow')):
                        await cursor.execute("UPDATE farms SET overheat = 1 WHERE id =?", (user[0],))
                        if user[5] == 2:
                            usr = await ext.get_or_fetch_user(id=user[0])
                            await usr.send("Видеокарты на твоей ферме перегрелись!\nСкорее перезапусти сервера!")

                        await db.commit()

                await asyncio.sleep(0.35)

    @tasks.loop(seconds=1)
    async def boosters_timer(self):
         async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms")
            users = await cursor.fetchall()

            for user in users:
                booster: dict = json.loads(user[6])
                if len(booster) > 0:
                    if datetime.fromisoformat(booster["ends"]) <= datetime.now():
                        await cursor.execute("UPDATE farms SET booster = '{}' WHERE id =?", (user[0],))
                        await db.commit()
                        usr = await ext.get_or_fetch_user(id=user[0])
                        await usr.send(f"Твой бустер '**{booster["name"]}**' закончился!")

    @tasks.loop(hours=48)
    async def sperma_timer(self):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM farms")
            users = await cursor.fetchall()

            for user in users:
                cards: list = json.loads(user[3])
                if random.random() < 0.1:
                    try:
                        rc = random.choice(cards)
                    except:
                        continue
                    if rc["count"] == 1:
                        cards.remove(rc)
                    else:
                        rc["count"] -= 1

                    await cursor.execute("UPDATE farms SET cards = ? WHERE id =?", (json.dumps(cards), user[0],))
                    await db.commit()
                    try:
                        usr = await ext.get_or_fetch_user(id=user[0])
                        await usr.send(f"Твоя видеокарта '**{rc["name"]}**' сгорела!")
                    except:
                        continue
                
async def setup(bot: commands.Bot):
    global dbn
    dbn = bot.dbn
    await bot.add_cog(Farm(bot))
    print("Farm cog loaded")