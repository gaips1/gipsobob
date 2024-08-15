from discord.ext import commands
import discord
import aiosqlite
import asyncio
import random

class sexb(discord.ui.View):
    def __init__(self, user, author):
        super().__init__(timeout=None)
        self.user: discord.User = user
        self.author: discord.User = author

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
    async def yessex(self, button: discord.ui.Button, inter: discord.MessageInteraction):
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
    async def nosex(self, button: discord.ui.Button, inter: discord.MessageInteraction):
        if inter.user != self.user: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        
        await inter.followup.send(f"**{self.author.mention}, вот чёрт, тебе отказал {self.user.mention} :(**")

class Fun(commands.Cog):
    def __init__(self, bot):
        self.bot = bot

    @commands.user_command(name="Предложить секс", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def sexu(self, inter: discord.ApplicationContext, user: discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)

        await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                          , view=sexb(user, inter.user))

    @commands.slash_command(description="Предложить секс", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    @discord.option("user", description="Кому предложить секс?", required=True, input_type=discord.SlashCommandOptionType.user)  
    async def sex(self, inter: discord.ApplicationContext, user:discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)
        await inter.response.send_message(embed=discord.Embed(title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?", color=discord.Color.random())
                                          , view=sexb(user, inter.user))
        
    @commands.user_command(name="Поцеловать", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def kiss(self, inter: discord.ApplicationContext, user: discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем целовать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че целовать себя собираешься?", ephemeral=True)
        giffs = ["https://media.tenor.com/jnndDmOm5wMAAAAC/kiss.gif", "https://media.tenor.com/fiafXWajQFoAAAAC/kiss-anime.gif", "https://media.tenor.com/dn_KuOESmUYAAAAC/engage-kiss-anime-kiss.gif"
                    , "https://media.tenor.com/9jB6M6aoW0AAAAAM/val-ally-kiss.gif", "https://media.tenor.com/SYwRyd6N1UIAAAAC/anime-kiss.gif"]
        randgif = random.choice(giffs)
        await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} поцеловал(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))

    @commands.user_command(name="Ударить", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def punch(self, inter: discord.ApplicationContext, user: discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем бить бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че бить себя собираешься?", ephemeral=True)
        giffs = ["https://media.tenor.com/p_mMicg1pgUAAAAC/anya-forger-damian-spy-x-family.gif", "https://media.tenor.com/BoYBoopIkBcAAAAC/anime-smash.gif", "https://media.tenor.com/UH8Jnl1W3CYAAAAC/anime-punch-anime.gif"
                     , "https://media.tenor.com/SwMgGqBirvcAAAAM/saki-saki-kanojo-mo-kanojo.gif", "https://media.tenor.com/vv1mgp7IQn8AAAAC/tgggg-anime.gif"]
        randgif = random.choice(giffs)
        await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} ударил(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))

    @commands.user_command(name="Обнять", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def hug(self, inter: discord.ApplicationContext, user: discord.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Зачем обнимать бота?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Ты че обнимать себя собираешься?", ephemeral=True)
        giffs = ["https://media.tenor.com/hwsbuAcG8UQAAAAM/foxplushy-foxy.gif", "https://media.tenor.com/WIOsEr_4XFcAAAAM/happy-anime.gif", "https://media.tenor.com/BmbTYhCZ5UsAAAAM/yuri-sleeping-yuri-sleep.gif"
                     , "https://media.tenor.com/MApGHq5Kvj0AAAAM/anime-hug.gif", "https://media.tenor.com/iEDbr-ZhHMkAAAAM/anime-hug.gif"]
        randgif = random.choice(giffs)
        await inter.response.send_message(embed=discord.Embed(title=f"{inter.user.global_name} обнял(а) {user.global_name}", color=discord.Color.random()).set_image(url=randgif))

    @commands.slash_command(description="Подбросить монетку", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def monetka(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
        wh = random.choices(["Орёл!", "Решка!", "Ребро!"], weights=[45,45, 10], k=1)[0]
        await inter.response.send_message("Подбрасываю...")
        await asyncio.sleep(2.5)
        await inter.edit(content=wh)

    @commands.slash_command(description="Да или нет", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def yesorno(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
        wh = random.choice(["Да", "Нет"])
        await inter.response.send_message(wh)

    @commands.slash_command(description="Русская Рулетка", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def russianroulette(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
        await inter.response.send_message("Вставляю пулю...")
        await asyncio.sleep(1.5)
        await inter.edit(content="Раскручиваю барабан...")
        await asyncio.sleep(1.5)
        if random.choices([False,True], weights=[90,10], k=1)[0] == True:
            await inter.edit(content="Бум! Тебе разорвало лицо.")
        else:
            await inter.edit(content="Повезло, ты остался жив.")

    @commands.slash_command(description="Кидаю кости", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
    async def kosti(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
        await inter.response.send_message("Кидаю...")
        await asyncio.sleep(2.5)
        await inter.edit(content="Выпало число " + str(random.randint(1, 6)))

def setup(bot):
    bot.add_cog(Fun(bot))
    global dbn
    dbn = bot.dbn
    print("Fun cog loaded")