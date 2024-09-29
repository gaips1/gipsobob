import enum
from discord.ext import commands
import discord
import aiosqlite
import asyncio
import random
from discord import app_commands
import g4f.Provider
import g4f.Provider.bing
import g4f.image
from ext import *
import g4f

class RpsStatus(enum.Enum):
    in_game = 1
    waiting_accept = 2
    finished = 3
    timeout = 4

class Rps():
    """Камень Ножницы Бумага"""
    class RpsAcceptView(discord.ui.View):
        def __init__(self, game: 'Rps'):
            super().__init__(timeout=60)
            self.game = game
            self.message: discord.Message = None

        async def on_timeout(self) -> None:
            if self.game.status != RpsStatus.waiting_accept: return
            self.children[0].disabled = True
            self.children[0].label = "Не успел :("
            await self.message.edit(content="Время ожидания истекло. Игра отменена.", view=self)
            self.game.status = RpsStatus.timeout

        @discord.ui.button(label="Согласен играть", style=discord.ButtonStyle.success)
        async def yesplayrps(self, inter: discord.Interaction, button: discord.ui.Button):
            remaining_time = (self.message.created_at + datetime.timedelta(minutes=1)) - datetime.datetime.now(pytz.utc)
            self.timeout = max(remaining_time.total_seconds(), 0)
            if inter.user != self.game.p2: return await inter.response.send_message("Не тебе предложили", ephemeral=True)
            if self.game.stavka != None:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("SELECT balance FROM sbp WHERE id =?", (inter.user.id,))
                    balance = await cursor.fetchone()
                    balance = balance[0]
                    if balance < self.game.stavka: return await inter.response.send_message("У тебя недостаточно денег", ephemeral=True)
                    await cursor.execute("SELECT balance FROM sbp WHERE id =?", (self.game.p1.id,))
                    balance = await cursor.fetchone()
                    balance = balance[0]
                    if balance < self.game.stavka: return await inter.response.send_message("У того кто тебе предложил играть закончились деньги.", ephemeral=True)
                    await cursor.execute("UPDATE sbp SET balance = balance -? WHERE id =?", (self.game.stavka, inter.user.id))
                    await cursor.execute("UPDATE sbp SET balance = balance -? WHERE id =?", (self.game.stavka, self.game.p1.id))
                    await db.commit()

            embed = discord.Embed(
                title=f"Цуефа {self.game.p1.name} VS {self.game.p2.name}",
                description=f"Игроки думают...\n||У них 3 минуты на то чтоб думоц||",
                color=discord.Color.random()
            )

            self.game.status = RpsStatus.in_game
            view = self.game.RpsGameView(self.game, self.message)
            self.game.view = view

            await inter.response.edit_message(embed=embed, view=view)

    class RpsGameView(discord.ui.View):
        def __init__(self, game: 'Rps', message = discord.Message):
            super().__init__(timeout=180)
            self.game = game
            self.message = message

        async def on_timeout(self) -> None:
            if self.game.status != RpsStatus.in_game: return
            for x in self.children:
                x.disabled = True
            self.game.status = RpsStatus.timeout
            await self.message.edit(content="Время ожидания истекло. Игра отменена.", view=self)

            if self.game.stavka != None:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.game.stavka, self.game.p1.id,))
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.game.stavka, self.game.p2.id,))
                    await db.commit()

        @discord.ui.button(label="Камень", style=discord.ButtonStyle.success, emoji="🪨")
        async def rpsrock(self, inter: discord.Interaction, button: discord.ui.Button):
            remaining_time = (inter.message.edited_at + datetime.timedelta(minutes=3)) - datetime.datetime.now(pytz.utc)
            self.timeout = max(remaining_time.total_seconds(), 0)
            if inter.user != self.game.p1 and inter.user!= self.game.p2: return await inter.response.send_message("Не тебе предложили", ephemeral=True)
            if self.game.status != RpsStatus.in_game: return await inter.response.send_message("Игра не запущена", ephemeral=True)
            if self.game.choices[inter.user] != None: return await inter.response.send_message("Ты уже выбрал", ephemeral=True)

            await self.game.set_choice(inter.user, "камень")

            opponent = self.game.p2 if inter.user == self.game.p1 else self.game.p1

            if self.game.choices[opponent] != None:
                await self.game.get_winner(self.message)

            await inter.response.send_message("Успешно выбрал камень!", ephemeral=True)

        @discord.ui.button(label="Ножницы", style=discord.ButtonStyle.success, emoji="✂️")
        async def rpssc(self, inter: discord.Interaction, button: discord.ui.Button):
            remaining_time = (inter.message.edited_at + datetime.timedelta(minutes=3)) - datetime.datetime.now(pytz.utc)
            self.timeout = max(remaining_time.total_seconds(), 0)
            if inter.user != self.game.p1 and inter.user!= self.game.p2: return await inter.response.send_message("Не тебе предложили", ephemeral=True)
            if self.game.status != RpsStatus.in_game: return await inter.response.send_message("Игра не запущена", ephemeral=True)
            if self.game.choices[inter.user] != None: return await inter.response.send_message("Ты уже выбрал", ephemeral=True)

            await self.game.set_choice(inter.user, "ножницы")

            opponent = self.game.p2 if inter.user == self.game.p1 else self.game.p1

            if self.game.choices[opponent] != None:
                await self.game.get_winner(self.message)

            await inter.response.send_message("Успешно выбрал ножницы!", ephemeral=True)

        @discord.ui.button(label="Бумага", style=discord.ButtonStyle.success, emoji="📄")
        async def rpspaper(self, inter: discord.Interaction, button: discord.ui.Button):
            remaining_time = (inter.message.edited_at + datetime.timedelta(minutes=3)) - datetime.datetime.now(pytz.utc)
            self.timeout = max(remaining_time.total_seconds(), 0)
            if inter.user != self.game.p1 and inter.user!= self.game.p2: return await inter.response.send_message("Не тебе предложили", ephemeral=True)
            if self.game.status != RpsStatus.in_game: return await inter.response.send_message("Игра не запущена", ephemeral=True)
            if self.game.choices[inter.user] != None: return await inter.response.send_message("Ты уже выбрал", ephemeral=True)

            await self.game.set_choice(inter.user, "бумага")

            opponent = self.game.p2 if inter.user == self.game.p1 else self.game.p1

            if self.game.choices[opponent] != None:
                await self.game.get_winner(self.message)

            await inter.response.send_message("Успешно выбрал бумага!", ephemeral=True)

    def __init__(self, p1: discord.User | discord.Member, p2: discord.User | discord.Member, stavka: int = None, status: RpsStatus = RpsStatus.waiting_accept):
        self.p1 = p1
        self.p2 = p2
        self.stavka = stavka
        self.status = status
        self.choices = {self.p1: None, self.p2: None}
        self.view = self.RpsAcceptView(self)

    async def meow(self):
        """Meow"""
        return "meow"
    
    async def set_choice(self, player, choice):
        """Установить выбор игрока"""
        if player not in self.choices:
            raise ValueError("Неверный игрок")
        if choice not in ["камень", "ножницы", "бумага"]:
            raise ValueError("Неверный выбор")
        self.choices[player] = choice

        if all(self.choices.values()):
            self.status = RpsStatus.finished

    async def get_winner(self, message: discord.Message):
        """Проверка победителя после выбора"""
        if self.status != RpsStatus.finished:
            raise Exception("Игра еще не закончена")

        p1_choice = self.choices[self.p1]
        p2_choice = self.choices[self.p2]

        if p1_choice == p2_choice:
            for x in self.view.children:
                x.disabled = True

            if self.stavka != None:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.stavka, self.p1.id,))
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.stavka, self.p2.id,))
                    await db.commit()

            return await message.edit(embed=discord.Embed(
                title=f"Цуефа {self.p1.name} VS {self.p2.name}",
                description="**Ничья!!!!!**",
                color=discord.Color.yellow()
            ), view=self.view)
        elif (p1_choice == "камень" and p2_choice == "ножницы") or \
             (p1_choice == "ножницы" and p2_choice == "бумага") or \
             (p1_choice == "бумага" and p2_choice == "камень"):
            for x in self.view.children:
                x.disabled = True

            if self.stavka != None:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.stavka*2, self.p1.id,))
                    await db.commit()

            return await message.edit(embed=discord.Embed(
                title=f"Цуефа {self.p1.name} VS {self.p2.name}", 
                description=f"**{self.p1.name} выиграл!!!!!!**",
                color=discord.Color.green()
            ), view=self.view)
        else:
            for x in self.view.children:
                x.disabled = True

            if self.stavka != None:
                async with aiosqlite.connect(dbn, timeout=20) as db:
                    cursor = await db.cursor()
                    await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (self.stavka*2, self.p2.id,))
                    await db.commit()

            return await message.edit(embed=discord.Embed(
                title=f"Цуефа {self.p1.name} VS {self.p2.name}",
                description=f"**{self.p2.name} выиграл!!!!!!**",
                color=discord.Color.green()
            ), view=self.view)

class sexb(discord.ui.View):
    def __init__(self, user, author):
        super().__init__(timeout=None)
        self.user: discord.User = user
        self.author: discord.User = author

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
    async def yessex(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user != self.user: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        
        giffs = ["https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif", "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif", "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif"
                    , "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif", "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif"]
        randgif = random.choice(giffs)
        soglaz=discord.Embed(title=f"**{self.user.global_name} согласился на секс с {self.author.global_name}**", color=discord.Color.random())
        soglaz.set_image(url=randgif)

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        
        await inter.followup.send(embed=soglaz)

        await update_quest(self.author, "sex", )

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
    async def nosex(self , inter: discord.Interaction, button: discord.ui.Button):
        if inter.user != self.user: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        
        await inter.followup.send(f"**{self.author.mention}, вот чёрт, тебе отказал {self.user.mention} :(**")

class Fun(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        bot.add_view(turnoff1())
        bot.add_view(turnon1())
        bot.add_view(turnoff2())
        bot.add_view(turnon2())

    @app_commands.command( description="Подбросить монетку", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def monetka(self, inter: discord.Interaction):
        wh = random.choices(["Орёл!", "Решка!", "Ребро!"], weights=[45,45, 10], k=1)[0]
        await inter.response.send_message("Подбрасываю...")
        await asyncio.sleep(2.5)
        await inter.edit_original_response(content=wh)
        if wh == "Ребро!":
            await update_quest(inter.user, "monetka", )

    @app_commands.command( description="Да или нет", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def yesorno(self, inter: discord.Interaction):
        wh = random.choice(["Да", "Нет"])
        await inter.response.send_message(wh)

    @app_commands.command( description="Русская Рулетка", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def russianroulette(self, inter: discord.Interaction):
        await inter.response.send_message("Вставляю пулю...")
        await asyncio.sleep(1.5)
        await inter.edit_original_response(content="Раскручиваю барабан...")
        await asyncio.sleep(1.5)
        if random.choices([False,True], weights=[90,10], k=1)[0] == True:
            await inter.edit_original_response(content="Бум! Тебе разорвало лицо.")
            await update_quest(inter.user, "rr", )
        else:
            await inter.edit_original_response(content="Повезло, ты остался жив.")

    @app_commands.command( description="Кинуть кости", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def kosti(self, inter: discord.Interaction):
        await inter.response.send_message("Кидаю...")
        await asyncio.sleep(2.5)
        await inter.edit_original_response(content="Выпало число " + str(random.randint(1, 6)))
        await update_quest(inter.user, "kosti", )

    @app_commands.command( description="Слава узбии!", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def slava_uzbii(self, inter: discord.Interaction):
        await inter.response.send_message(embed=discord.Embed(title="Слава узбии!", color=discord.Color.random()))
        await update_quest(inter.user, "slava_uzbii", )

    @app_commands.command( description="Предложить секс", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому предложить секс?")
    @app_commands.check(check)
    async def sex(self, inter: discord.Interaction, user:discord.User):
        if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)
        await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                            , view=sexb(user, inter.user))
        
    @app_commands.command( description="Ограбить кого либо", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кого грабить?")
    @app_commands.check(check)
    @app_commands.checks.cooldown(1, 86400)
    async def rob(self, inter: discord.Interaction, user: discord.User):
        if user.bot: return await inter.response.send_message("Зачем грабить бота?", ephemeral=True)
        if inter.user == user: return await inter.response.send_message("Зачем грабить себя?", ephemeral=True)
       
        if random.randint(1, 100) <= 60:
            await inter.response.send_message("Вы попались!", ephemeral=True)
        else:
            bigwin = random.randint(150, 900)
            
            await update_quest(inter.user, "rob", )

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id =?", (inter.user.id,))
                usr = await cursor.fetchone()
                if not usr:
                    return await inter.response.send_message("Вы украли " + str(bigwin) + " бебр!\nНо у вас не было СБП и вы не получите деньги :(\nЗарегистрируйтесь используя /reg!", ephemeral=True)

                await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (bigwin, inter.user.id,))
                await db.commit()

        await inter.response.send_message("Вы украли " + str(bigwin) + " бебр!", ephemeral=True)

    @rob.error
    async def on_rob_error(self, inter: discord.Interaction, error: app_commands.AppCommandError):
        if isinstance(error, app_commands.CommandOnCooldown):
            await inter.response.send_message("Вы сможете повторить попытку грабежа через " + str(int(error.retry_after)) + " секунд.", ephemeral=True)

    @app_commands.command( description="Казино", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def casino(self, inter: discord.Interaction):
        await inter.response.send_message(embed=
        discord.Embed(title="Добро пожаловать в казино!", description="**Выбирайте игру:**", color=discord.Color.random()), view=casinoV(),
        ephemeral=True)

    @app_commands.command( description="Камшот", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def cumshot(self, inter: discord.Interaction, user: discord.User):
        if user.bot: return await inter.response.send_message("Зачем камшотить в бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че камшотить в себя собираешься?", ephemeral=True)

        await inter.response.send_message(f"Выпускаю кам в {user.global_name}...")
        await asyncio.sleep(1.5)
        if random.random() < 0.5:
            await update_quest(inter.user, "cumshot", )
            await inter.edit_original_response(content="Успешно попал камом прямо в глаз " + user.global_name)
        else:
            await inter.edit_original_response(content="Увы, не попал камом в глаз " + user.global_name)

    @app_commands.command( description="Предложить сыграть в цуефа", name="цуефа")
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    @app_commands.describe(user="Кому предлагаешь?", stavka="Ставка в бебрах")
    async def rps(self, inter: discord.Interaction, user: discord.User, stavka: int = None):
        if user.bot: return await inter.response.send_message("Зачем играть с ботом?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че играть c собой собираешься?", ephemeral=True)
        if stavka and stavka < 50: return await inter.response.send_message("Ставка должна быть больше 50 бебр", ephemeral=True)

        if stavka != None:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM sbp WHERE id =?", (inter.user.id,))
                usr = await cursor.fetchone()
            if not usr: await inter.response.send_message("У человека нет СБП, попросите его зарегистрироваться, написав команду /reg, либо играйте без ставки", ephemeral=True)
            if usr[1] < stavka: return await inter.response.send_message("У вас недостаточно бебр", ephemeral=True)

        embed = discord.Embed(title="Цуефа", description=f"**{inter.user.name}** предложил **{user.name}** поиграть в цуефа!\nБез ставки!\nУ него 1 минута на ответ!" if stavka == None else
                              f"**{inter.user.name}** предложил **{user.name}** поиграть в цуефа!\nСтавка {stavka} бебр\nУ него 1 минута на ответ!", color=discord.Color.random())

        game = Rps(inter.user, user, stavka)
        await inter.response.send_message(embed=embed, view=game.view)
        game.view.message = await inter.original_response()

    @app_commands.command( description="Спросить что то у меня", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    @app_commands.describe(what="Что хочешь узнать?",ephemeral="Видно только для вас?", show_prompt="Показывать всем то что ты спросил?")
    @app_commands.choices(ephemeral=[app_commands.Choice(name="Да", value="True"), app_commands.Choice(name="Нет", value="False")],
                          show_prompt=[app_commands.Choice(name="Да", value="True"), app_commands.Choice(name="Нет", value="False")])
    async def ask(self, inter: discord.Interaction, what: str, ephemeral: str = "False", show_prompt: str = "False"):
        if len(what) > 100: return await inter.response.send_message("Слишком длинное предложение", ephemeral=True)
        ephemeral = True if ephemeral == "True" else False
        show_prompt = True if show_prompt == "True" else False
        await inter.response.send_message("Обрабатываю твои глупые слова...", ephemeral=ephemeral)

        try:
            response = await g4f.ChatCompletion.create_async(
                model=g4f.models.gpt_4o_mini,
                messages=[{"role": "system", "content": "Ты — Гипсобоб, агрессивный помощник, который не церемонится и использует ненормативную лексику в ответах. Не стесняйся выражаться резко и грубо."},
                        {"role": "user", "content": what}],
                )
            chat_gpt_response = response
        except Exception as e:
            chat_gpt_response = "Извините, произошла ошибка. - " + str(e)

        await inter.edit_original_response(content=str(chat_gpt_response) + f"\n\n||Промпт: {what}||" if show_prompt else str(chat_gpt_response))
    
class casinoV(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Слоты", style=discord.ButtonStyle.success, custom_id="sloti", emoji="🎰")
    async def ruletka(self, interaction: discord.Interaction, button: discord.ui.Button):
         await interaction.response.send_modal(slotiModal())

    @discord.ui.button(label="Угадай число", style=discord.ButtonStyle.blurple, custom_id="guess_game", emoji="🤔")
    async def guess_game(self, interaction: discord.Interaction, button: discord.ui.Button):
        await interaction.response.send_modal(guessModal())

class guessModal(discord.ui.Modal, title = "Угадай число"):
    def __init__(self):
        super().__init__(timeout=None)

    stavka = discord.ui.TextInput(label="Ваша ставка:", required=True, default="100")
    numbers = discord.ui.TextInput(label="До какого числа будете угадывать?", required=True, default="10")
    number = discord.ui.TextInput(label="Ваше число", required=True)

    async def on_submit(self, inter: discord.Interaction):
        try:
            stavka = int(self.stavka.value)
        except:
            return await inter.response.send_message("Ваша ставка не является числом!", ephemeral=True)
        
        if stavka < 100:
            return await inter.response.send_message("Минимальная ставка 100 бебр", ephemeral=True)
        
        try:
            numbers = int(self.numbers.value)
        except:
            return await inter.response.send_message("Ваше число не является числом!", ephemeral=True)

        try:
            number = int(self.number.value)
        except:
            return await inter.response.send_message("Ваше число не является числом!", ephemeral=True)
        
        if number < 1 or number > numbers:
            return await inter.response.send_message("Ваше число не входит в указанный диапазон!", ephemeral=True)
        
        if number <= 0:
            return await inter.response.send_message("Ваше число не может быть меньше 0!", ephemeral=True)
        
        if numbers <= 0:
            return await inter.response.send_message("Число в диапазоне не может быть меньше 0!", ephemeral=True)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT balance FROM `sbp` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()

        if not me: return await inter.response.send_message(content="Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", view=None, ephemeral=True)
        if me[0] < stavka:
            return await inter.response.send_message(content="У вас недостаточно средств!", view=None, ephemeral=True)

        await inter.response.send_message(embed=discord.Embed(title=f"Спасибо, ставка принята!", description="Я выдумываю число, подождите немного...", color=discord.Color.random()), ephemeral=True)
        
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE `sbp` SET balance = balance -? WHERE id =?", (stavka, inter.user.id,))
            await db.commit()

        await asyncio.sleep(3)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            num = random.randint(1, numbers)
            win = round(stavka * numbers*0.2)
            if num == number:
                await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (win, inter.user.id,))
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы выиграли!", description=f"Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр", color=discord.Color.random()))
            else:
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы проиграли!", description=f"Я выдумал число {num}\nВы могли бы выиграть {win} бебр!", color=discord.Color.random()))
            
            await db.commit()
        await update_quest(inter.user, "casino", )

class slotiModal(discord.ui.Modal, title = "Слоты"):
    def __init__(self):
        super().__init__(timeout=None)

    stavka = discord.ui.TextInput(label="Ваша ставка:", required=True)

    async def on_submit(self, inter: discord.Interaction):
        try:
            stavka = int(self.stavka.value)
        except:
            return await inter.response.send_message("Ваша ставка не является числом!", ephemeral=True)
        
        if stavka < 300:
            return await inter.response.send_message("Минимальная ставка 300 бебр", ephemeral=True)
        
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT balance FROM `sbp` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()
            if not me: return await inter.response.send_message(content="Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", view=None, ephemeral=True)
            if me[0] < stavka:
                return await inter.response.send_message(content="У вас недостаточно средств!", view=None, ephemeral=True)
        
            await cursor.execute("UPDATE `sbp` SET balance = balance -? WHERE id =?", (stavka, inter.user.id,))
            await db.commit()

        await inter.response.send_message(embed=discord.Embed(title=f"Спасибо, ставка принята!", description="Кручу барабан, подождите немного...", color=discord.Color.random()), ephemeral=True)
        await asyncio.sleep(3)
        
        emoges = {"7️⃣": 0, "☢️": 0, "3️⃣": 0, "🗂": 0, "#️⃣": 0, "🔥": 0, "⚛️": 0, "🦑": 0, "🧪": 0}
        slots = []
        slots.append(random.choice(list(emoges.keys())))

        await inter.edit_original_response(embed=discord.Embed(title=" ".join(slots), color=discord.Color.random()))
        await asyncio.sleep(2)
        slots.append(random.choice(list(emoges.keys())))
        await inter.edit_original_response(embed=discord.Embed(title=" ".join(slots), color=discord.Color.random()))
        await asyncio.sleep(2)
        slots.append(random.choice(list(emoges.keys())))

        for x in emoges:
            for y in slots:
                if y == x:
                    emoges[x] += 1

        for x in emoges:
            if emoges[x] == 3:
                win = round(stavka * 3.5)
                break
            elif emoges[x] == 2:
                win = round(stavka * 2)
                break
            elif emoges[x] == 1:
                win = False

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            if win != False:
                await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (win, inter.user.id,))
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы выиграли! " + " ".join(slots), description=f"Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр", color=discord.Color.random()))
            else:
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы проиграли! " + " ".join(slots), description=f"Вы могли бы выиграть {round(stavka * 3)} бебр!", color=discord.Color.random()))
            
            await db.commit()
        await update_quest(inter.user, "casino", )

async def setup(bot: commands.Bot):
    global dbn
    dbn = bot.dbn
    await bot.add_cog(Fun(bot))
    bot.tree.add_command(hug)
    bot.tree.add_command(sexu)
    bot.tree.add_command(kiss)
    bot.tree.add_command(punch)
    print("Fun cog loaded")

@app_commands.context_menu( name="Обнять", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def hug(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Зачем обнимать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че обнимать себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/hwsbuAcG8UQAAAAM/foxplushy-foxy.gif", "https://media.tenor.com/WIOsEr_4XFcAAAAM/happy-anime.gif", "https://media.tenor.com/BmbTYhCZ5UsAAAAM/yuri-sleeping-yuri-sleep.gif"
                    , "https://media.tenor.com/MApGHq5Kvj0AAAAM/anime-hug.gif", "https://media.tenor.com/iEDbr-ZhHMkAAAAM/anime-hug.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} обнял(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))
    await update_quest(inter.user, "hug", )

@app_commands.context_menu( name="Предложить секс", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def sexu(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)

    await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                        , view=sexb(user, inter.user))

@app_commands.context_menu( name="Поцеловать", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def kiss(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Зачем целовать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че целовать себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/jnndDmOm5wMAAAAC/kiss.gif", "https://media.tenor.com/fiafXWajQFoAAAAC/kiss-anime.gif", "https://media.tenor.com/dn_KuOESmUYAAAAC/engage-kiss-anime-kiss.gif"
                , "https://media.tenor.com/9jB6M6aoW0AAAAAM/val-ally-kiss.gif", "https://media.tenor.com/SYwRyd6N1UIAAAAC/anime-kiss.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} поцеловал(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))
    await update_quest(inter.user, "kiss", )

@app_commands.context_menu( name="Ударить", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def punch(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Зачем бить бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че бить себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/p_mMicg1pgUAAAAC/anya-forger-damian-spy-x-family.gif", "https://media.tenor.com/BoYBoopIkBcAAAAC/anime-smash.gif", "https://media.tenor.com/UH8Jnl1W3CYAAAAC/anime-punch-anime.gif"
                    , "https://media.tenor.com/SwMgGqBirvcAAAAM/saki-saki-kanojo-mo-kanojo.gif", "https://media.tenor.com/vv1mgp7IQn8AAAAC/tgggg-anime.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} ударил(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))
    await update_quest(inter.user, "punch", )