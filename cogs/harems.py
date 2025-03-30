from discord.ext import commands
import discord
from discord import app_commands
import ext
from db.database_instance import db
from views.harems.invite import HaremInviteView
from views.harems.menu import HaremMenuView

class Harems(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    @app_commands.command(description="Создать свой гарем", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def create_harem(self, inter: discord.Interaction):
        harem = await db.harems.get_harem(inter.user.id)

        if harem: return await inter.response.send_message("Вы уже создали гарем", ephemeral=True)
        if await db.harems.get_user_harem(inter.user.id): return await inter.response.send_message("Вы уже состоите в чьём-то гареме", ephemeral=True)

        await db.harems.create_harem(inter.user.id)
        await inter.response.send_message("Вы успешно создали свой гарем\nИспользуйте команду `/harem` для управления гаремом", ephemeral=True)

    @app_commands.command(description="Управление своим гаремом", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def harem(self, inter: discord.Interaction):
        harem = await db.harems.get_user_harem(inter.user.id) or await db.harems.get_harem(inter.user.id)
        if harem:
            usr = await ext.get_or_fetch_user(harem.user_id)

            embed = discord.Embed(
                title=f"Гарем пользователя {usr.global_name}",
                description=f"Пользователи гарема: {', '.join([f'<@{user}>' for user in harem.users]) if harem.users else 'Нету'}.\nГарем был создан <t:{harem.created_at}:R>",
                color=discord.Color.random()
            )
            return await inter.response.send_message(embed=embed, view=HaremMenuView(inter.user.id, harem), ephemeral=True)

        await inter.response.send_message("Вы не состоите в гареме", ephemeral=True)

    @app_commands.command(description="Пригласить в гарем", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def invite_to_harem(self, inter: discord.Interaction):
        if not await db.harems.get_harem(inter.user.id):
            return await inter.response.send_message("У вас нет гарема", ephemeral=True)

        embed = discord.Embed(
            title=f"Гарем пользователя {inter.user.global_name}",
            description="Нажмите на кнопку ниже, чтобы присоединиться к его гарему",
            color=discord.Color.random()
        )
        return await inter.response.send_message(embed=embed, view=HaremInviteView(inter.user))
    
    @app_commands.command(description="Топ 10 гаремов по количеству пользователей", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def harems(self, inter: discord.Interaction):
        harems = await db.harems.get_harems()
        
        sorted_harems = sorted(harems, key=lambda h: len(h.users), reverse=True)[:10]
        
        if not sorted_harems:
            return await inter.response.send_message("Пока нет ни одного гарема", ephemeral=True)
            
        description = ""
        for i, harem in enumerate(sorted_harems, 1):
            description += f"**{i}.** <@{harem.user_id}> - {len(harem.users)} пользователей\n"
            
        embed = discord.Embed(
            title="Топ 10 гаремов",
            description=description,
            color=discord.Color.random()
        )
        
        await inter.response.send_message(embed=embed, ephemeral=True)

async def setup(bot: commands.Bot):
    await bot.add_cog(Harems(bot))
    print("Harems cog loaded")