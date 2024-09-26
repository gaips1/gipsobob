from discord.ext import commands, tasks
from discord import app_commands
import discord
import aiosqlite
import os
import random
import ext

class XP(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    @commands.Cog.listener()
    async def on_message(self, message: discord.Message):
        if message.author.bot: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `xp` WHERE id = ?", (message.author.id,))
            me = await cursor.fetchone()
            if not me:
                await cursor.execute("INSERT INTO `xp` (id) VALUES (?)", (message.author.id,))
                await db.commit()
                me = [message.author.id, 0, 150, 1]
            
            await cursor.execute('UPDATE xp SET xp = xp+1 WHERE id = ?', (message.author.id,))
            await db.commit()

            if (me[1]+1) >= me[2]:
                await cursor.execute('UPDATE xp SET (lvl, obj) = (lvl+1, obj+150) WHERE id = ?', (message.author.id,))
                await db.commit()
                await message.channel.send(f"{message.author.mention} достиг нового уровня!")

    @app_commands.command( description="Посмотреть свой уровень", )
    @app_commands.allowed_contexts(guilds=True, dms=False, private_channels=False)
    @app_commands.allowed_installs(guilds=True, users=False)
    @app_commands.check(ext.check)
    async def rank(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `xp` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()
            if not me:
                await cursor.execute("INSERT INTO `xp` (id) VALUES (?)", (inter.user.id,))
                await db.commit()
                me = [inter.user.id, 0, 150, 1]

        await inter.response.send_message(ephemeral=True, embed=discord.Embed(
            title="Система Быстрых Уровней", description=f"**Количество опыта: {me[1]}/{me[2]}\nУровень: {me[3]}**", color=discord.Color.random()
        ))

    @app_commands.command( description="Топ людей по опыту", )
    @app_commands.allowed_contexts(guilds=True, dms=False, private_channels=False)
    @app_commands.allowed_installs(guilds=True, users=False)
    @app_commands.check(ext.check)
    async def top(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `xp`")
            clanst = await cursor.fetchall()
        spis = []
        sorted_players = sorted(clanst, key=lambda x: int(x[1]), reverse=True)
        for i, player in enumerate(sorted_players[:10]):
            usr = await ext.get_or_fetch_user(bot=self.bot, id=player[0])
            spis.append(f"{i+1}.  **{usr.display_name}: {player[1]} XP**")

        await inter.response.send_message(ephemeral=True, embed=discord.Embed(
            title="Топ 10 людей по уровню", description='\n'.join(spis), color=discord.Color.random()
        ))

async def setup(bot: commands.Bot):
    global dbn
    dbn = bot.dbn
    await bot.add_cog(XP(bot))
    print("XP cog loaded")