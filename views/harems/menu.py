import asyncio
import discord
from db.database_instance import db
from db.models.harems import Harem
import ext

class DeleteHaremView(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success, emoji="✅")
    async def delete_harem_yes(self, inter: discord.Interaction, button: discord.ui.Button):
        harem = await db.harems.get_harem(inter.user.id)
        if not harem: return await inter.response.send_message("У вас нет гарема", ephemeral=True)

        await harem.delete()

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self, embed=None, content="Вы успешно удалили свой гарем :(")

        for user in harem.users:
            try:
                user = await ext.get_or_fetch_user(inter.client, user)
                if not user: continue

                await user.send(f"**{inter.user.global_name}** удалил свой гарем. Вы автоматически были удалены из него.")
            except:
                continue

            await asyncio.sleep(1.5)

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger, emoji="❌")
    async def delete_harem_no(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.edit_message(embed=None, view=None, content="Молодец что одумался")

class LeaveHaremView(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Да", style=discord.ButtonStyle.success, emoji="✅")
    async def leave_harem_yes(self, inter: discord.Interaction, button: discord.ui.Button):
        harem = await db.harems.get_user_harem(inter.user.id)
        if not harem: return await inter.response.send_message("Вы не в гареме.", ephemeral=True)

        await harem.remove_user(inter.user.id)

        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self, embed=None, content="Вы успешно покинули гарем :(")

        user = await ext.get_or_fetch_user(inter.client, harem.user_id)
        if not user: return

        await user.send(f"**{inter.user.global_name}** покинул ваш гарем.")

    @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger, emoji="❌")
    async def leave_harem_no(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.edit_message(embed=None, view=None, content="Молодец что одумался")

class HaremMenuView(discord.ui.View):
    def __init__(self, user_id: int, harem: Harem):
        super().__init__(timeout=None)
        self.user_id = user_id
        self.harem = harem

        button = discord.ui.Button(
            label="Удалить гарем",
            style=discord.ButtonStyle.danger,
            emoji="💀"
        ) if self.harem.user_id == self.user_id else discord.ui.Button(
            label="Покинуть гарем",
            style=discord.ButtonStyle.danger,
            emoji="👋"
        )
        button.callback = self.delete_harem if self.harem.user_id == self.user_id else self.leave_harem
        self.add_item(button)

    async def delete_harem(self, inter: discord.Interaction):
        embed = discord.Embed(
            title="Удалить гарем",
            description=f"Вы уверены, что хотите удалить свой гарем? Все пользователи будут автоматически удалены из него.",
            color=discord.Color.random()
        )

        await inter.response.edit_message(embed=embed, view=DeleteHaremView())

    async def leave_harem(self, inter: discord.Interaction):
        embed = discord.Embed(
            title="Покинуть гарем",
            description=f"Вы уверены, что хотите покинуть гарем?",
            color=discord.Color.random()
        )

        await inter.response.edit_message(embed=embed, view=LeaveHaremView())
