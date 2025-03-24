import discord
from db.database_instance import db

class HaremInviteView(discord.ui.View):
    def __init__(self, author: discord.User):
        super().__init__(timeout=None)
        self.author = author
        
    @discord.ui.button(label="Присоединиться", style=discord.ButtonStyle.success, emoji="👋")
    async def accept_invite_to_harem(self, inter: discord.Interaction, button: discord.ui.Button):
        harem = await db.harems.get_user_harem(inter.user.id) or await db.harems.get_harem(inter.user.id)
        if harem: return await inter.response.send_message("Вы уже состоите в гареме", ephemeral=True)

        harem = await db.harems.get_harem(self.author.id)
        if not harem: return await inter.response.send_message("Автор гарема не найден", ephemeral=True)

        await harem.add_user(inter.user.id)

        await inter.response.send_message("Вы успешно присоединились к гарему", ephemeral=True)

        await self.author.send(f"**{inter.user.global_name}** присоединился к вашему гарему")

