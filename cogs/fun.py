from discord.ext import commands
import discord
import asyncio
import random
from discord import app_commands
from db.database_instance import db
from ext import *
import views.fun.sex as sex
import views.fun.rps as rps
import views.fun.casino as casino

class Fun(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

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

        if random.random() <= 0.1:
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

    @app_commands.command( description="Ограбить кого либо", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кого грабить?")
    @app_commands.check(check)
    @app_commands.checks.cooldown(1, 86400)
    async def rob(self, inter: discord.Interaction, user: discord.User):
        if user.bot: return await inter.response.send_message("Зачем грабить бота?", ephemeral=True)
        if inter.user == user: return await inter.response.send_message("Зачем грабить себя?", ephemeral=True)
       
        if random.random() <= 0.6:
            return await inter.response.send_message("Вы попались!", ephemeral=True)
        
        bigwin = random.randint(150, 900)
        
        await update_quest(inter.user, "rob", )

        usr = await db.sbp.get_user(inter.user.id)
        if not usr:
            return await inter.response.send_message(
                "Вы успешно украли " + str(bigwin) + " бебр!\nНо у вас не было СБП и вы не получите деньги :(\nЗарегистрируйтесь используя **/reg**!",
                ephemeral=True
            )
        
        await usr.increase_balance(bigwin)

        await inter.response.send_message(
            "Вы успешно украли " + str(bigwin) + " бебр!",
            ephemeral=True
        )

    @rob.error
    async def on_rob_error(self, inter: discord.Interaction, error: app_commands.AppCommandError):
        if isinstance(error, app_commands.CommandOnCooldown):
            await inter.response.send_message("Вы сможете повторить попытку грабежа через " + str(int(error.retry_after)) + " секунд.", ephemeral=True)
    
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
            await update_quest(inter.user, "cumshot", used_user=user)
            await inter.edit_original_response(content="Успешно попал камом прямо в глаз " + user.global_name)
        else:
            await inter.edit_original_response(content="Увы, не попал камом в глаз " + user.global_name)

    @app_commands.command( description="Предложить секс", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому предложить секс?")
    @app_commands.check(check)
    async def sex(self, inter: discord.Interaction, user: discord.User):
        return await sex.offer_sex(inter, user)
    
    @app_commands.command( description="Предложить сыграть в цуефа", name="цуефа")
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    @app_commands.describe(user="Кому предлагаешь?", stavka="Ставка в бебрах")
    async def rps(self, inter: discord.Interaction, user: discord.User, stavka: int = None):
        return await rps.offer_rps(inter, user, stavka)
    
    @app_commands.command( description="Казино", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def casino(self, inter: discord.Interaction):
        await inter.response.send_message(
            embed=discord.Embed(
                title="Добро пожаловать в казино!",
                description="**Выбирайте игру:**",
                color=discord.Color.random()
            ),
            view=casino.casinoView(),
            ephemeral=True
        )

    @app_commands.command(name="минет", description="Сделать минет пользователю", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    @app_commands.describe(user="Кому делать минет?")
    async def minet(self, inter: discord.Interaction, user: discord.User):
        if user.bot: return await inter.response.send_message("Зачем делать минет боту?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че делать минет себе собираешься?", ephemeral=True)

        await inter.response.send_message(f"Вы сосёте {user.global_name}...")
        await asyncio.sleep(3.5)

        if random.random() < 0.5:
            await update_quest(inter.user, "minet", used_user=user)
            await inter.edit_original_response(content=f"Вы успешно довели до оргазма {user.global_name}!")
        else:
            await inter.edit_original_response(content=f"Вы не смогли заставить кончить {user.global_name} :(")

    @app_commands.command( description="KEEP YOURSELF SAFE", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def kys(self, inter: discord.Interaction):
        with open("views/fun/dies.json", "r", encoding="utf-8") as f:
            choices = json.load(f)

        await inter.response.send_message(f"Вы {random.choice(choices)}. Поздравляю со смертью!", ephemeral=True)

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
    await update_quest(inter.user, "hug", used_user=user)

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
    await update_quest(inter.user, "kiss", used_user=user)

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
    await update_quest(inter.user, "punch", used_user=user)

@app_commands.context_menu( name="Предложить секс", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def sexCM(inter: discord.Interaction, user: discord.User):
    return await sex.offer_sex(inter, user)

async def setup(bot: commands.Bot):
    await bot.add_cog(Fun(bot))
    bot.tree.add_command(hug)
    bot.tree.add_command(sexCM)
    bot.tree.add_command(kiss)
    bot.tree.add_command(punch)
    print("Fun cog loaded")