from discord.ext import commands
import discord
import os
from discord import app_commands
import string
from PIL import Image, ImageDraw, ImageFont
import random
import ext
from db.database_instance import db
import views.sbp.account as account
import views.sbp.сaptcha as captcha

AUTHOR_UNATHORIZED_ERROR = "Вы не зарегистрированы в Системе Быстрых Платежей! Сделайте это, написав **/reg**."
USER_UNATHORIZED_ERROR = "Пользователь не зарегистрирован в Системе Быстрых Платежей! Скажите ему, чтобы он сделал это, написав **/reg**."

async def show_account(inter: discord.Interaction, edit: bool = False):
    user = await db.sbp.get_user(inter.user.id)
        
    if not user:
        return await inter.response.send_message(AUTHOR_UNATHORIZED_ERROR, ephemeral=True)
    
    if edit:
        await inter.response.edit_message(embed=discord.Embed(
            title=f"Личный кабинет: {inter.user.global_name}",
            description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {user.balance_text} бебр(ы)\nУведомления: {"включены" if user.notifications else "выключены"}**",
        ), view=account.Account(user))
    else:
        await inter.response.send_message(embed=discord.Embed(
            title=f"Личный кабинет: {inter.user.global_name}",
            description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {user.balance_text} бебр(ы)\nУведомления: {'включены' if user.notifications else 'выключены'}**"
        ), ephemeral=True,view=account.Account(user))

async def transfer(inter: discord.Interaction, user: discord.User, amount: float, comment: str = None):
    if comment and len(comment) > 50:
        await inter.response.send_message(
            "Комментарий не может превышать 50 символов.", 
            ephemeral=True
        )
        return
    
    if amount <= 0: return await inter.response.send_message("Пожалуйста, введите положительное или не нулевое число", ephemeral=True)
    if user.bot: return await inter.response.send_message("Нельзя перевести бебры боту", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Нельзя перевести бебры себе", ephemeral=True)
    
    me = await db.sbp.get_user(inter.user.id)
    usr = await db.sbp.get_user(user.id)

    if not me: return await inter.response.send_message(AUTHOR_UNATHORIZED_ERROR, ephemeral=True)
    if not usr: return await inter.response.send_message(USER_UNATHORIZED_ERROR, ephemeral=True)

    if me.balance < amount: return await inter.response.send_message("У вас не хватает денег", ephemeral=True)

    await me.decrease_balance(amount)
    await usr.increase_balance(amount)

    await inter.response.send_message(f"Успешно перевёл `{amount}` бебр {user.global_name}.", ephemeral=True)

    await ext.update_quest(inter.user, "transfer", amount)

    if usr.notifications:
        if comment:
            embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", description=
                                    f"Комментарий от отправителя: **{comment}**", color=discord.Color.green())
        else:
            embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", color=discord.Color.green())
        
        await user.send(embed=embed, view=account.Account(usr))

class Sbp(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    @app_commands.command( description="Твой личный кабинет Системы Быстрых Платежей!", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def reg(self, inter: discord.Interaction):
        user = await db.sbp.get_user(inter.user.id)

        if user: 
            return await inter.response.send_message("Вы уже зарегистрированы в Системе Быстрых Платежей!", ephemeral=True)
        
        await db.sbp.create_user(inter.user.id)

        await inter.response.send_message(
            "Вы успешно зарегистрировались в Системе Быстрых Платежей!",
            ephemeral=True,
            view=account.ToAccount()
        )

    @app_commands.command( description="Твой личный кабинет Системы Быстрых Платежей!", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def account(self, inter: discord.Interaction):
        return await show_account(inter)
    
    @app_commands.command( description="Изменить деньги СБП", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому изменять?", amount="На сколько изменять?")
    @app_commands.check(ext.check)
    async def setbal(self, inter: discord.Interaction, amount:int, user:discord.User = None):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)

        usr = await db.sbp.get_user(user.id if user else inter.user.id)
        await usr.set_balance(amount)

        await inter.response.send_message("Успешно!", ephemeral=True)

    @app_commands.command( description="Перевод бебр пользователю Системы Быстрых Платежей", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    @app_commands.describe(user="Кому переводить?", amount="Сколько переводить?", comment="Комментарий к переводу")
    async def transfer(self, inter: discord.Interaction, user:discord.User, amount: float, comment:str = None):
        return await transfer(inter, user, amount, comment)

    @app_commands.command( description="Пройти капчу и получить бебры", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def captcha(self, inter: discord.Interaction):
        letters = string.ascii_letters + string.digits
        kap = ''.join(random.choice(letters) for i in range(10))
        image = Image.new('RGB', (200, 50), (255, 255, 255))

        font = ImageFont.truetype('arial.ttf', 28)

        draw = ImageDraw.Draw(image)
        draw.text((10, 10), kap, font=font, fill=(0, 0, 0))

        image.save('random_text.png')

        await inter.response.send_message(
            "Привет!\nТвоя капча:",
            ephemeral=True,
            file=discord.File('random_text.png'),
            view=captcha.СaptchaButton(kap)
        )

        os.remove('random_text.png')

    @app_commands.command( description="Пригласить друга в СБП и получить деньги", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def invite(self, inter: discord.Interaction):
        await inter.response.send_message(
            embed=discord.Embed(
            title="Приглашение зарегистрироваться в СБП",
            description="Чтобы принять, нажмите на кнопку ниже"),
            view=account.AcceptInvite(author=inter.user)
        )

@app_commands.context_menu( name="Перевод", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(ext.check)
async def transferCM(inter: discord.Interaction, user: discord.User):
    await inter.response.send_modal(account.TransferModal(title="Перевод " + str(user.global_name), user=user))

async def setup(bot: commands.Bot):
    await bot.add_cog(Sbp(bot))
    bot.tree.add_command(transferCM)
    print("Sbp cog loaded")