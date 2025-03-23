import discord
from db.database_instance import db

class MarriageOfferView(discord.ui.View):
    def __init__(self, user1: discord.User, user2: discord.User):
        super().__init__(timeout=None)
        self.user1 = user1
        self.user2 = user2

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
    async def yesmg(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user.id != self.user2.id: return await inter.response.send_message("Завидуй молча, это не тебе брак предлагали", ephemeral=True)

        await db.marriages.create_marriage(self.user1.id, self.user2.id)

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)

        await inter.followup.send(f"**Поздравим молодожёнов!\n{self.user1.mention} и {self.user2.mention} теперь официально вместе!**")
        
    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
    async def nomg(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user.id != self.user2.id: return await inter.response.send_message("Завидуй молча, это не тебе брак предлагали", ephemeral=True)
        
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        await inter.followup.send(f"**{self.user1.mention}, вот чёрт, тебе отказал {self.user2.mention} :(**")