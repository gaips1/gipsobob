import discord
import asyncio
from dotenv import load_dotenv
import aiosqlite
import inspect, os.path
import random
load_dotenv()
TOKEN = os.environ.get("TOKEN")
intents = discord.Intents.default()
intents.members = True
intents.message_content = True
filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
dbn = path + "/gipsobob.sql"

async def check(inter: discord.Interaction):
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM `users` WHERE id = ?", (inter.user.id,))
        me = await cursor.fetchone()
    if not me:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("INSERT INTO `users` (id) VALUES (?)", (inter.user.id,))
            await db.commit()
            return 0

    if me[1] == 1:
        await inter.response.send_message("Вы забанены в боте.", ephemeral=True)
        return 1
    return 0

bot = discord.Bot(intents=intents)
bot.dbn = dbn
bot.check = check

@bot.event
async def on_ready():
    print(f"{bot.user} is ready and online!")
    await bot.change_presence(status=discord.Status.dnd, activity=discord.Game("Visual Studio Code"))

@bot.slash_command(description="Сказать что-то от имени бота", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild],)
@discord.option("what", description="Что сказать?", required=True, input_type=discord.SlashCommandOptionType.string)  
@discord.option("inchat", description="Будет ли отправляться в чате?", required=False, input_type=discord.SlashCommandOptionType.boolean)  
async def say(inter: discord.Interaction, what:str, inchat:bool):
    if await bot.check(inter) == 1: return
    try:
        await inter.response.send_message(what) if not inchat else await inter.channel.send(what)
        await inter.response.defer()
    except(discord.errors.Forbidden):
         await inter.response.send_message("Не удалось написать туда", ephemeral=True)
    except(discord.errors.InteractionResponded):
        pass

@bot.slash_command(description="Пинг?", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
async def ping(inter: discord.Interaction):
    if await bot.check(inter) == 1: return
    await inter.response.send_message("Понг!", ephemeral=True)

@bot.message_command(name="Инфо о сообщении", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
async def get_messag12e_id(inter: discord.Interaction, message: discord.Message):
    if await bot.check(inter) == 1: return
    await inter.response.send_message(f"Message ID: `{message.id}`, Message author: '{message.author.mention}', Message author ID: `{message.author.id}`, Message content: `{message.content}`", ephemeral=True)

@bot.message_command(name="(Раз)Забанить автора", integration_types=[discord.IntegrationType.user_install, discord.IntegrationType.guild_install],
                   contexts=[discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild])
async def banauthor(inter: discord.Interaction, message: discord.Message):
    if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM `users` WHERE id = ?", (message.author.id,))
        me = await cursor.fetchone()
    if not me:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("INSERT INTO `users` (id, banned) VALUES (?, ?)", (message.author.id, 1))
            await db.commit()
        return await inter.response.send_message("Забанил!", ephemeral=True)

    if me[1] == 1:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute('UPDATE users SET banned = 0 WHERE id = ?', (message.author.id,))
            await db.commit()
        return await inter.response.send_message("Разбанил!", ephemeral=True)
    
    if me[1] == 0:
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute('UPDATE users SET banned = 1 WHERE id = ?', (message.author.id,))
            await db.commit()
        return await inter.response.send_message("Забанил!", ephemeral=True)

@bot.event
async def on_member_join(member):
    if member.guild.id == 621378615174758421:
        gifs = ['https://media.tenor.com/ZvSSenCwxEcAAAAC/hello.gif', 'https://media.tenor.com/3o2hRDX8vw0AAAAC/hello-cute.gif', 'https://media.tenor.com/J_JT8JsNDlUAAAAC/hello-anime.gif'
                , 'https://media.tenor.com/mIteh_Sas9QAAAAd/anime-hello.gif', 'https://media.tenor.com/Q1dW7INg5ioAAAAC/hello-anime.gif']
        randgifs = random.choice(gifs)
        embed=discord.Embed(title=f"**{member.global_name}, привет! возможно мы рады тебя видеть...**", color=discord.Color.random())
        embed.set_image(url=randgifs)
        channel = await bot.fetch_channel(807651258520436736)
        await channel.send(embed=embed)

@bot.event
async def on_member_remove(member):
    if member.guild.id == 621378615174758421:
        gifs = ['https://media.tenor.com/m0MabzE7tLIAAAAC/bye-anime-girl.gif', 'https://media.tenor.com/lOMogKtB3E8AAAAC/goodbye-bye.gif', 'https://media.tenor.com/4NHXeITTdKcAAAAC/anime-wave.gif'
                , 'https://media.tenor.com/KasGopE0HIsAAAAC/bye-bye-anime.gif', 'https://media.tenor.com/oiYL8iyWwmkAAAAC/anime-jujutsu-kaisen.gif']
        randgifs = random.choice(gifs)
        embed=discord.Embed(title=f"**{member.global_name}, надеемся ты к нам ещё придёшь**", color=discord.Color.random())
        embed.set_image(url=randgifs)
        channel = await bot.fetch_channel(807651258520436736)
        await channel.send(embed=embed)

"""
    @bot.listen()
    async def on_message(message: discord.Message):
        if message.author.bot: return
        if not message.content.startswith("."): return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT num FROM `counter`")
            last_num = await cursor.fetchone()
            last_num  = last_num[0]

        try:
            num = int(message.content.replace(".", ""))
        except:
            return

        if num != (last_num+1): return 
"""
bot.load_extension("cogs.sbp")
bot.load_extension("cogs.fun")
# bot.load_extension("cogs.xp") deprecated. do not use.
bot.load_extension("cogs.giveaways")
bot.load_extension("cogs.dl")
bot.run(TOKEN)
