from disnake.ext import commands
import disnake
import aiosqlite
import os
import random

class XP(commands.Cog):
    def __init__(self, bot):
        self.bot: commands.InteractionBot = bot

    @commands.Cog.listener()
    async def on_message(self, message: disnake.Message):
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

    @commands.slash_command(description="Посмотреть свой уровень", integration_types=[0,1], contexts=[0,1,2])
    async def rank(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `xp` WHERE id = ?", (inter.author.id,))
            me = await cursor.fetchone()
            if not me:
                await cursor.execute("INSERT INTO `xp` (id) VALUES (?)", (inter.author.id,))
                await db.commit()
                me = [inter.author.id, 0, 150, 1]
        await inter.response.send_message(ephemeral=True, embed=disnake.Embed(
            title="Система Быстрых Уровней", description=f"**Количество опыта: {me[1]}/{me[2]}\nУровень: {me[3]}**", color=disnake.Color.random()
        ))

    @commands.slash_command(description="Топ людей по опыту", integration_types=[0,1], contexts=[0,1,2])
    async def top(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `xp`")
            clanst = await cursor.fetchall()
        spis = []
        sorted_players = sorted(clanst, key=lambda x: int(x[1]), reverse=True)
        for i, player in enumerate(sorted_players[:10]):
            usr = await self.bot.getch_user(player[0])
            spis.append(f"{i+1}.  **{usr.display_name}: {player[1]} XP**")

        await inter.response.send_message(ephemeral=True, embed=disnake.Embed(
            title="Топ 10 людей по уровню", description='\n'.join(spis), color=disnake.Color.random()
        ))

def setup(bot):
    bot.add_cog(XP(bot))
    global dbn
    dbn = bot.dbn
    print("XP cog loaded")