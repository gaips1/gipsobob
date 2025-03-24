import asyncio
import discord
from discord.ext import commands
from discord import app_commands
import discord.ext.commands
from dotenv import load_dotenv
import discord.ext
from ext import check, turnoff1, turnon1, get_or_fetch_user
import os
import random
from db.database_instance import db

load_dotenv()

intents = discord.Intents.default()
intents.members = True
intents.message_content = True

bot = commands.Bot(intents=intents, help_command=None, command_prefix="$")

@bot.tree.error
async def on_command_error(ctx: discord.Interaction, error):
    if isinstance(error, app_commands.errors.CheckFailure):
        await ctx.response.send_message("Вы забанены в боте!", ephemeral=True)
    else:
        raise error
    
@bot.event
async def on_ready():
    print(f"{bot.user} is ready and online!")

    await db.connect()

    await bot.load_extension("cogs.fun")
    await bot.load_extension("cogs.sbp")
    await bot.load_extension("cogs.dl")
    await bot.load_extension("cogs.giveaways")
    await bot.load_extension("cogs.quests")
    await bot.load_extension("cogs.marriages")
    await bot.load_extension("cogs.harems")
    
    # DEPRECATED await bot.load_extension("cogs.farm") DEPRECATED

    await bot.tree.sync()

    bot.add_view(turnon1())
    bot.add_view(turnoff1())

    await bot.change_presence(
        status=discord.Status.dnd,
        activity=discord.Activity(
            name="Visual Studio Code",
            type=discord.ActivityType.playing,
            state="The best bot in the world!"
        )
    )

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

@bot.tree.command(description="Отправить всем людям в боте",)
@app_commands.describe(what="Что писать?")
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(check)
async def say_to_all(inter: discord.Interaction, what:str):
    if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)

    await inter.response.send_message("Отправляю...", ephemeral=True)

    users = await db.users.get_users()
    for user in users:
        discord_user = await get_or_fetch_user(bot, user.id)
        if discord_user:
            try:
                await discord_user.send(what)
            except:
                continue

            await asyncio.sleep(1)
                
    await inter.user.send("Сообщение было отправлено всем пользователям!")

@bot.tree.command(name="ping", description="Пинг?",)
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

    user = await db.users.get_user(message.author.id, True)
    await user.set_banned(False if user.is_banned else True)

    return await inter.response.send_message("Разбанен" if user.is_banned else "Забанен", ephemeral=True)

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

bot.run(os.environ.get("TOKEN"))
#meow