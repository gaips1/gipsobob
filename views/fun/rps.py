import datetime
import enum
from db.database_instance import db
import discord
import pytz

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
                balance = await db.sbp.get_balance(inter.user.id)
                if balance < self.game.stavka: return await inter.response.send_message("У тебя недостаточно денег", ephemeral=True)

                await db.sbp.decrease_balance(inter.user.id, self.game.stavka)
                await db.sbp.decrease_balance(self.game.p1.id, self.game.stavka)

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
                await db.sbp.increase_balance(self.game.p1.id, self.game.stavka)
                await db.sbp.increase_balance(self.game.p2.id, self.game.stavka)

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
                await db.sbp.increase_balance(self.p1.id, self.stavka)
                await db.sbp.increase_balance(self.p2.id, self.stavka)

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
                await db.sbp.increase_balance(self.p1.id, self.stavka*2)

            return await message.edit(embed=discord.Embed(
                title=f"Цуефа {self.p1.name} VS {self.p2.name}", 
                description=f"**{self.p1.name} выиграл!!!!!!**",
                color=discord.Color.green()
            ), view=self.view)
        else:
            for x in self.view.children:
                x.disabled = True

            if self.stavka != None:
                await db.sbp.increase_balance(self.p2.id, self.stavka*2)

            return await message.edit(embed=discord.Embed(
                title=f"Цуефа {self.p1.name} VS {self.p2.name}",
                description=f"**{self.p2.name} выиграл!!!!!!**",
                color=discord.Color.green()
            ), view=self.view)
        
async def offer_rps(inter: discord.Interaction, user: discord.User, stavka: int = None):
    if user.bot: return await inter.response.send_message("Зачем играть с ботом?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че играть c собой собираешься?", ephemeral=True)
    if stavka and stavka < 50: return await inter.response.send_message("Ставка должна быть больше 50 бебр", ephemeral=True)

    if stavka != None:
        me = await db.sbp.get_user(inter.user.id)
        
        from cogs.sbp import AUTHOR_UNATHORIZED_ERROR, USER_UNATHORIZED_ERROR
        
        if not me: return await inter.response.send_message(AUTHOR_UNATHORIZED_ERROR, ephemeral=True)
        if me.balance < stavka: return await inter.response.send_message("У тебя нет столько бебр.", ephemeral=True)

        usr = await db.sbp.get_user(user.id)
        if not usr: return await inter.response.send_message(USER_UNATHORIZED_ERROR, ephemeral=True)
        
    embed = discord.Embed(title="Цуефа", description=f"**{inter.user.name}** предложил **{user.name}** поиграть в цуефа!\nБез ставки!\nУ него 1 минута на ответ!" if stavka == None else
                            f"**{inter.user.name}** предложил **{user.name}** поиграть в цуефа!\nСтавка {stavka} бебр\nУ него 1 минута на ответ!", color=discord.Color.random())

    game = Rps(inter.user, user, stavka)
    await inter.response.send_message(embed=embed, view=game.view)
    game.view.message = await inter.original_response()