import discord
from db.database_instance import db
import ext

async def get_info(user_id: int):
    marriage = await db.marriages.get_marriage(user_id)
    if not marriage: return None

    return discord.Embed(
        title="Информация о браке",
        description=f"**Партнёр:** <@{marriage.partner_id if user_id == marriage.user_id else marriage.user_id}>\n**Вы вступили в брак** <t:{marriage.created_at}:R>",
        color=discord.Color.random()
    )

class DivorceView(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success, emoji="✅")
    async def divorce_yes(self, inter: discord.Interaction, button: discord.ui.Button):
        marriage = await db.marriages.get_marriage(inter.user.id)
        if not marriage: return await inter.response.send_message("Вы не состоите в браке", ephemeral=True)

        await marriage.delete()

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self, embed=None, content="Вы успешно развелись :(")

        partner = await ext.get_or_fetch_user(inter.client, marriage.partner_id if inter.user.id == marriage.user_id else marriage.user_id)
        if not partner: return

        await partner.send(f"**{inter.user.global_name}** развёлся с вами")

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger, emoji="❌")
    async def divorce_no(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.edit_message(view=MarriageMenuView(), embed=await get_info(inter.user.id))

class MarriageMenuView(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Развестись", style=discord.ButtonStyle.danger, emoji="🖤")
    async def divorce_mg(self, inter: discord.Interaction, button: discord.ui.Button):
        embed = discord.Embed(
            title="Развод",
            description=f"Вы уверены, что хотите развестись?",
            color=discord.Color.random()
        )

        await inter.response.edit_message(embed=embed, view=DivorceView())