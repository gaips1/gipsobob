import asyncio
import random
import discord
from db.database_instance import db
from ext import update_quest

class casinoView(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Слоты", style=discord.ButtonStyle.success, emoji="🎰")
    async def sloti(self, interaction: discord.Interaction, button: discord.ui.Button):
        await interaction.response.send_modal(slotiModal())

    @discord.ui.button(label="Угадай число", style=discord.ButtonStyle.blurple, emoji="🤔")
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

        me = await db.sbp.get_user(inter.user.id)

        from cogs.sbp import AUTHOR_UNATHORIZED_ERROR
        if not me: return await inter.response.send_message(content=AUTHOR_UNATHORIZED_ERROR, view=None, ephemeral=True)
        
        if me.balance < stavka:
            return await inter.response.send_message(content="У вас недостаточно средств!", view=None, ephemeral=True)

        await inter.response.send_message(embed=discord.Embed(title=f"Спасибо, ставка принята!", description="Я выдумываю число, подождите немного...", color=discord.Color.random()), ephemeral=True)
        await me.decrease_balance(stavka)

        await asyncio.sleep(3)

        num = random.randint(1, numbers)
        win = round(stavka * numbers * 0.2)

        if num == number:
            await me.increase_balance(win)
            await inter.edit_original_response(embed=discord.Embed(title=f"Вы выиграли!", description=f"Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр", color=discord.Color.random()))
        else:
            await inter.edit_original_response(embed=discord.Embed(title=f"Вы проиграли!", description=f"Я выдумал число {num}\nВы могли бы выиграть {win} бебр!", color=discord.Color.random()))
        
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
        
        me = await db.sbp.get_user(inter.user.id)

        from cogs.sbp import AUTHOR_UNATHORIZED_ERROR
        if not me: return await inter.response.send_message(content=AUTHOR_UNATHORIZED_ERROR, view=None, ephemeral=True)
        
        if me.balance < stavka:
            return await inter.response.send_message(content="У вас недостаточно средств!", view=None, ephemeral=True)

        await me.decrease_balance(stavka)

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

        if win != False:
            await me.increase_balance(win)
            await inter.edit_original_response(embed=discord.Embed(title=f"Вы выиграли! " + " ".join(slots), description=f"Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр", color=discord.Color.random()))
        else:
            await inter.edit_original_response(embed=discord.Embed(title=f"Вы проиграли! " + " ".join(slots), description=f"Вы могли бы выиграть {round(stavka * 3)} бебр!", color=discord.Color.random()))
            
        await update_quest(inter.user, "casino", )