from discord.ext import commands
import discord
import aiosqlite
import asyncio
import random
from discord import app_commands

class sexb(discord.ui.View):
    def __init__(self, user, author):
        super().__init__(timeout=None)
        self.user: discord.User = user
        self.author: discord.User = author

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
    async def yessex(self, inter: discord.MessageInteraction, button: discord.ui.Button):
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

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
    async def nosex(self , inter: discord.MessageInteraction, button: discord.ui.Button):
        if inter.user != self.user: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        
        await inter.followup.send(f"**{self.author.mention}, вот чёрт, тебе отказал {self.user.mention} :(**")

class Fun(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    @app_commands.command( description="Подбросить монетку", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def monetka(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        wh = random.choices(["Орёл!", "Решка!", "Ребро!"], weights=[45,45, 10], k=1)[0]
        await inter.response.send_message("Подбрасываю...")
        await asyncio.sleep(2.5)
        await inter.edit(content=wh)

    @app_commands.command( description="Да или нет", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def yesorno(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        wh = random.choice(["Да", "Нет"])
        await inter.response.send_message(wh)

    @app_commands.command( description="Русская Рулетка", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def russianroulette(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        await inter.response.send_message("Вставляю пулю...")
        await asyncio.sleep(1.5)
        await inter.edit(content="Раскручиваю барабан...")
        await asyncio.sleep(1.5)
        if random.choices([False,True], weights=[90,10], k=1)[0] == True:
            await inter.edit(content="Бум! Тебе разорвало лицо.")
        else:
            await inter.edit(content="Повезло, ты остался жив.")

    @app_commands.command( description="Кинуть кости", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def kosti(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        await inter.response.send_message("Кидаю...")
        await asyncio.sleep(2.5)
        await inter.edit_original_response(content="Выпало число " + str(random.randint(1, 6)))

    @app_commands.command( description="Предложить секс", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому предложить секс?")
    async def sex(self, inter: discord.Interaction, user:discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)
        await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                            , view=sexb(user, inter.user))
        
    @app_commands.command( description="Казино", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def casino(self, inter: discord.Interaction):
        await inter.response.send_message(embed=
        discord.Embed(title="Добро пожаловать в казино!", description="**Выбирайте игру:**", color=discord.Color.random()), view=casinoV(),
        ephemeral=True)
    
class casinoV(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    # @discord.ui.button(label="Рулетка", style=discord.ButtonStyle.success, custom_id="ruletka", emoji="🎰")
    # async def ruletka(self, interaction: discord.Interaction, button: discord.ui.Button):
    #     await interaction.response.send_modal(ruletkaModal())

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
        
        await asyncio.sleep(3)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("UPDATE `sbp` SET balance = balance -? WHERE id =?", (stavka, inter.user.id,))
            num = random.randint(1, numbers)
            win = round(stavka * numbers*0.2)
            if num == number:
                await cursor.execute("UPDATE `sbp` SET balance = balance +? WHERE id =?", (win, inter.user.id,))
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы выиграли!", description=f"Ваша ставка: {stavka} бебр\nВыигрыш: {win} бебр", color=discord.Color.random()))
            else:
                await inter.edit_original_response(embed=discord.Embed(title=f"Вы проиграли!", description=f"Я выдумал число {num}\nВы могли бы выиграть {win} бебр!", color=discord.Color.random()))
            
            await db.commit()

# class ruletkaModal(discord.ui.Modal, title = "Рулетка"):
#     def __init__(self):
#         super().__init__(timeout=None)

#     stavka = discord.ui.TextInput(label="Ваша ставка:", required=True)
#     stavka = discord.ui.TextInput(label="", required=True)

#     async def on_submit(self, inter: discord.Interaction):
#         pass

async def setup(bot: commands.Bot):
    await bot.add_cog(Fun(bot))
    bot.tree.add_command(hug)
    bot.tree.add_command(sexu)
    bot.tree.add_command(kiss)
    bot.tree.add_command(punch)
    global dbn
    dbn = bot.dbn
    print("Fun cog loaded")

@app_commands.context_menu( name="Обнять", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
async def hug(inter: discord.Interaction, user: discord.User):
    if await inter.client.check(inter) == 1: return
    if user.bot: return await inter.response.send_message("Зачем обнимать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че обнимать себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/hwsbuAcG8UQAAAAM/foxplushy-foxy.gif", "https://media.tenor.com/WIOsEr_4XFcAAAAM/happy-anime.gif", "https://media.tenor.com/BmbTYhCZ5UsAAAAM/yuri-sleeping-yuri-sleep.gif"
                    , "https://media.tenor.com/MApGHq5Kvj0AAAAM/anime-hug.gif", "https://media.tenor.com/iEDbr-ZhHMkAAAAM/anime-hug.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} обнял(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))

@app_commands.context_menu( name="Предложить секс", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
async def sexu(inter: discord.Interaction, user: discord.User):
    if await inter.client.check(inter) == 1: return
    if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)

    await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                        , view=sexb(user, inter.user))

@app_commands.context_menu( name="Поцеловать", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
async def kiss(inter: discord.Interaction, user: discord.User):
    if await inter.client.check(inter) == 1: return
    if user.bot: return await inter.response.send_message("Зачем целовать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че целовать себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/jnndDmOm5wMAAAAC/kiss.gif", "https://media.tenor.com/fiafXWajQFoAAAAC/kiss-anime.gif", "https://media.tenor.com/dn_KuOESmUYAAAAC/engage-kiss-anime-kiss.gif"
                , "https://media.tenor.com/9jB6M6aoW0AAAAAM/val-ally-kiss.gif", "https://media.tenor.com/SYwRyd6N1UIAAAAC/anime-kiss.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} поцеловал(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))

@app_commands.context_menu( name="Ударить", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
async def punch(inter: discord.Interaction, user: discord.User):
    if await inter.client.check(inter) == 1: return
    if user.bot: return await inter.response.send_message("Зачем бить бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че бить себя собираешься?", ephemeral=True)
    giffs = ["https://media.tenor.com/p_mMicg1pgUAAAAC/anya-forger-damian-spy-x-family.gif", "https://media.tenor.com/BoYBoopIkBcAAAAC/anime-smash.gif", "https://media.tenor.com/UH8Jnl1W3CYAAAAC/anime-punch-anime.gif"
                    , "https://media.tenor.com/SwMgGqBirvcAAAAM/saki-saki-kanojo-mo-kanojo.gif", "https://media.tenor.com/vv1mgp7IQn8AAAAC/tgggg-anime.gif"]
    randgif = random.choice(giffs)
    await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} ударил(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))
