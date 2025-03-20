import discord
from db.database_instance import db
import ext
    
class СaptchaButton(discord.ui.View):
    def __init__(self, captcha: str):
        super().__init__(timeout=None)
        self.captcha = captcha

    @discord.ui.button(label="Ввести капчу", style=discord.ButtonStyle.blurple)
    async def entercaptcha(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.send_modal(CaptchaModal(captcha=self.captcha))

class CaptchaModal(discord.ui.Modal, title = "Капча"):
    def __init__(self, captcha):
        super().__init__()
        self.captcha: str  = captcha

    capt = discord.ui.TextInput(label="Введите капчу:", required=True)

    async def on_submit(self, inter: discord.Interaction):
        if self.capt.value == self.captcha:
            me = await db.sbp.get_user(inter.user.id)
            
            from cogs.sbp import AUTHOR_UNATHORIZED_ERROR
            if not me: return await inter.response.edit_message(content=AUTHOR_UNATHORIZED_ERROR, view=None)

            await me.increase_balance(5)

            await inter.response.edit_message(content="Капча успешно введена! Вам было добавлено 5 бебр", view=None)
            await ext.update_quest(inter.user, "captcha", )
            
        else:
            await inter.response.edit_message(content="Капча введена неверно! Попробуйте ещё раз", view=None)