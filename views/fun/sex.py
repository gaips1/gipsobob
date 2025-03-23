import random
import discord
from ext import update_quest

async def offer_sex(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Зачем ебать бота?", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Ты че ебать себя собираешься?", ephemeral=True)

    await inter.response.send_message(
        embed=discord.Embed(
            title=f"{user.global_name}, {inter.user.global_name} предложил Вам секс, Вы согласны?",
            color=discord.Color.random()
        ),
        view=SexView(inter.user, user)
    )

class SexView(discord.ui.View):
    def __init__(self, author: discord.User, user: discord.User):
        super().__init__(timeout=None)
        self.author = author
        self.user = user

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
    async def yessex(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user.id != self.user.id: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        
        giffs = ["https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif", "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif", "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif"
                    , "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif", "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif"]
        randgif = random.choice(giffs)
        soglaz=discord.Embed(title=f"**{self.user.global_name} согласился на секс с {self.author.global_name}**", color=discord.Color.random())
        soglaz.set_image(url=randgif)

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        await inter.followup.send(embed=soglaz)

        await update_quest(self.author, "sex", used_user=self.user)

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
    async def nosex(self , inter: discord.Interaction, button: discord.ui.Button):
        if inter.user.id != self.user.id: return await inter.response.send_message("Завидуй молча, это не тебе секс предлагали", ephemeral=True)
        
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        await inter.followup.send(f"**{self.author.mention}, вот чёрт, тебе отказал {self.user.mention} :(**")