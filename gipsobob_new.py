mode = "PROD"

import sys
import discord
from discord.ext import commands
import asyncio
from discord import app_commands
from dotenv import load_dotenv
import aiosqlite
from ext import check, turnoff1, turnon1
import inspect, os.path
import random
load_dotenv()
if mode == "PROD":
    TOKEN = os.environ.get("TOKEN")
else:
    TOKEN = os.environ.get("DEV_TOKEN")

intents = discord.Intents.default()
intents.members = True
intents.message_content = True
filename = inspect.getframeinfo(inspect.currentframe()).filename
path     = os.path.dirname(os.path.abspath(filename))
dbn = path + "/gipsobob.sql"

bot = commands.Bot(intents=intents, help=None, command_prefix="$")
bot.dbn = dbn
bot.mode = mode

@bot.event
async def on_command_error(ctx: discord.Interaction, error):
    if isinstance(error, commands.CommandNotFound):
        pass
    elif isinstance(error, app_commands.CheckFailure):
        await ctx.response.send_message("Вы забанены в боте!", ephemeral=True)
    else:
        raise error

@bot.event
async def on_ready():
    print(f"{bot.user} is ready and online!")
    await bot.load_extension("cogs.fun")
    await bot.load_extension("cogs.sbp")
    await bot.load_extension("cogs.dl")
    await bot.load_extension("cogs.giveaways")
    await bot.load_extension("cogs.inv")
    await bot.load_extension("cogs.farm")
    await bot.tree.sync()
    bot.add_view(turnon1())
    bot.add_view(turnoff1())
    await bot.change_presence(status=discord.Status.dnd, activity=discord.Game("Visual Studio Code"))

    if sys.platform == 'win32':
        asyncio.set_event_loop_policy(asyncio.WindowsSelectorEventLoopPolicy())

@bot.tree.command(name="say", description="Пинг?",)
@app_commands.describe(what="Что писать?", inchat="Писать ли в чате?")
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def say(inter: discord.Interaction, what:str, inchat:bool = None):
    try:
        await inter.response.send_message(what) if not inchat else await inter.channel.send(what)
        await inter.response.defer()
    except(discord.errors.Forbidden):
         await inter.response.send_message("Не удалось написать туда", ephemeral=True)
    except(discord.errors.InteractionResponded):
        pass

@bot.tree.command(name="ping", description="Пинг?",)
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def ping(inter: discord.Interaction):
    await inter.response.send_message("Понг!\n" + "Задержка: **" + str(round(bot.latency * 1000)) + "** мс", ephemeral=True)

@bot.tree.context_menu(name="Инфо о сообщении",)
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def get_message_id(inter: discord.Interaction, message: discord.Message):
    await inter.response.send_message(f"Message ID: `{message.id}`, Message author: '{message.author.mention}', Message author ID: `{message.author.id}`, Message content: `{message.content}`", ephemeral=True)

@bot.tree.context_menu(name="(Раз)забанить автора сообщения",)
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
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
async def on_member_join(member: discord.Member):
    if member.guild.id == 621378615174758421:
        gifs = ['https://media.tenor.com/ZvSSenCwxEcAAAAC/hello.gif', 'https://media.tenor.com/3o2hRDX8vw0AAAAC/hello-cute.gif', 'https://media.tenor.com/J_JT8JsNDlUAAAAC/hello-anime.gif'
                , 'https://media.tenor.com/mIteh_Sas9QAAAAd/anime-hello.gif', 'https://media.tenor.com/Q1dW7INg5ioAAAAC/hello-anime.gif']
        randgifs = random.choice(gifs)
        embed=discord.Embed(title=f"**{member.global_name}, привет! возможно мы рады тебя видеть...**", color=discord.Color.random())
        embed.set_image(url=randgifs)
        channel = await bot.fetch_channel(807651258520436736)
        await channel.send(embed=embed)

@bot.event
async def on_member_remove(member: discord.Member):
    if member.guild.id == 621378615174758421:
        gifs = ['https://media.tenor.com/m0MabzE7tLIAAAAC/bye-anime-girl.gif', 'https://media.tenor.com/lOMogKtB3E8AAAAC/goodbye-bye.gif', 'https://media.tenor.com/4NHXeITTdKcAAAAC/anime-wave.gif'
                , 'https://media.tenor.com/KasGopE0HIsAAAAC/bye-bye-anime.gif', 'https://media.tenor.com/oiYL8iyWwmkAAAAC/anime-jujutsu-kaisen.gif']
        randgifs = random.choice(gifs)
        embed=discord.Embed(title=f"**{member.global_name}, надеемся ты к нам ещё придёшь**", color=discord.Color.random())
        embed.set_image(url=randgifs)
        channel = await bot.fetch_channel(807651258520436736)
        await channel.send(embed=embed)

#bot.load_extension("cogs.xp") deprecated. do not use.
bot.run(TOKEN)
