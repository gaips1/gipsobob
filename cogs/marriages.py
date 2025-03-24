from discord.ext import commands
import discord
import os
from discord import app_commands
import random
import ext
from db.database_instance import db
from views.marriages.offer import MarriageOfferView
from views.marriages.menu import MarriageMenuView, get_info

class MG(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    @app_commands.command(description="Сделать предложение брака", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    @app_commands.describe(user="Кому предложить брак?")
    async def marriage(self, inter: discord.Interaction, user: discord.User):
        if user.bot: return await inter.response.send_message("Зачем предлагать брак боту?", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Даже не пробуй.", ephemeral=True)

        marriage = await db.marriages.get_marriage(inter.user.id)
        if marriage: return await inter.response.send_message("Ты чё, изменщик? Куснись.", ephemeral=True)
        if await db.marriages.get_marriage(user.id): return await inter.response.send_message("Этот пользователь уже состоит в браке.", ephemeral=True)

        embed = discord.Embed(
            title="Предложение брака",
            description=f"**{inter.user.global_name}** предлагает брак **{user.global_name}**",
            color=discord.Color.random()
        )

        await inter.response.send_message(embed=embed, view=MarriageOfferView(inter.user, user))

    @app_commands.command(description="Посмотреть информацию о своём браке", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def my_marriage(self, inter: discord.Interaction):
        marriage = await db.marriages.get_marriage(inter.user.id)
        if not marriage: return await inter.response.send_message("Вы не состоите в браке", ephemeral=True)

        await inter.response.send_message(embed=await get_info(inter.user.id), view=MarriageMenuView(), ephemeral=True)

    @app_commands.command(description="Показать топ браков по времени", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def marriages(self, inter: discord.Interaction):
        marriages = await db.marriages.get_marriages()
        if not marriages: return await inter.response.send_message("Нет ни одного брака", ephemeral=True)
        
        marriages.sort(key=lambda x: x.created_at)
        marriages = marriages[:10]
        
        description = ""
        for i, marriage in enumerate(marriages, 1):
            description += f"**{i}.** <@{marriage.user_id}> и <@{marriage.partner_id}> - <t:{marriage.created_at}:R>\n"
            
        embed = discord.Embed(
            title="Топ браков по времени",
            description=description,
            color=discord.Color.random()
        )
        
        await inter.response.send_message(embed=embed, ephemeral=True)

async def setup(bot: commands.Bot):
    await bot.add_cog(MG(bot))
    print("Marriages cog loaded")