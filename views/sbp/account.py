from decimal import Decimal
import discord
from db.models.sbp import User
from db.database_instance import db

class ToAccount(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="В личный кабинет", style=discord.ButtonStyle.success)
    async def toaccount(self, inter: discord.Interaction, button: discord.ui.Button):
        from cogs.sbp import show_account
        return await show_account(inter, True)
    
class Account(discord.ui.View):
    async def change_notifications(self, inter: discord.Interaction):
        await self.user.set_notifications(not self.user.notifications)

        from cogs.sbp import show_account
        return await show_account(inter, True)

    def __init__(self, user: User):
        super().__init__(timeout=None)
        self.user = user

        self.add_item(
            discord.ui.Button(label="Выключить уведомления", style=discord.ButtonStyle.danger, emoji="✖") 
            if self.user.notifications else 
            discord.ui.Button(label="Включить уведомления", style=discord.ButtonStyle.success, emoji="✅")
        )

        self.children[0].callback = self.change_notifications

class TransferModal(discord.ui.Modal):
    def __init__(self, title: str, user: discord.User):
        super().__init__(title=title)
        self.user: discord.User = user

    amount = discord.ui.TextInput(label="Сумма перевода", required=True)
    comment = discord.ui.TextInput(label="Комментарий к переводу", required=False, max_length=50)

    async def on_submit(self, inter: discord.Interaction):
        try:
            amount = float(self.amount.value)
            if amount <= 0:
                return await inter.response.send_message("Сумма перевода должна быть больше нуля.", ephemeral=True)
        except ValueError:
            return await inter.response.send_message("Введите корректную сумму перевода.", ephemeral=True)

        from cogs.sbp import transfer
        return await transfer(inter, self.user, amount, self.comment.value)
    
class AcceptInvite(discord.ui.View):
    def __init__(self, author):
        super().__init__(timeout=None)
        self.author: discord.User = author

    @discord.ui.button(label="Принять приглашение", style=discord.ButtonStyle.blurple, custom_id="accept_invite")
    async def accept_invite_btn(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user == self.author: return await inter.response.send_message("Вы являетесь автором приглашения", ephemeral=True)

        usr = await db.sbp.get_user(inter.user.id)
        if usr: return await inter.response.send_message("Вы уже зарегистрированы в Системе Быстрых Платежей!", ephemeral=True)

        await db.sbp.create_invite(self.author.id, inter.user.id)
        await db.sbp.create_user(inter.user.id)
        await db.sbp.increase_balance(self.author.id, 200)

        await inter.response.send_message(embed=discord.Embed(title="Приглашение принято", description="Вы зарегистрировались в СБП по ссылке от " + self.author.name), ephemeral=True)
        await self.author.send(f"{inter.user.name} зарегистрировался в СБП по вашей ссылке!")