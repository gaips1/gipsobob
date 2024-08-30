import inspect
import json
from typing import List
from discord.ext import commands
import discord
import aiosqlite
import os
from discord import app_commands
import string
from PIL import Image, ImageDraw, ImageFont
import random
import ext

class Sbp(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot

    class turnon(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, emoji="✅")
        async def tunroff11(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 1 WHERE id = {inter.user.id}")
                await db.commit()

            await inter.response.edit_message(embed=discord.Embed(
                title=f"Личный кабинет: {inter.user.global_name}",
                description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {user[1]} бебр\nУведомления: включены**"
            ), view=Sbp.turnoff())

        @discord.ui.button(label="Перевести бебры", style=discord.ButtonStyle.blurple, emoji="💸")
        async def perevod1(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.send_message("Отправьте айди получателя мне в личные сообщения", ephemeral=True)
            def check(m):
                return m.author == inter.user

            msg: discord.Message = await inter.client.wait_for('message', check=check)
            try:
                user = await inter.client.fetch_user(int(msg.content))
            except:
                return await msg.reply("Не удалось найти пользователя")
            
            if user.bot: return await msg.reply("Нельзя перевести бебры боту")
            if user == inter.user: return await msg.reply("Нельзя перевести бебры себе")

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
                use = await cursor.fetchone()

            if not use: return await msg.reply("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**")

            await msg.reply(f"Вы собираетесь перевести бебры пользователю **{user.global_name}** [{user.id}] \nПодтверждаете платёж?", view=Sbp.yesornoH())

    class turnoff(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger, emoji="✖")
        async def tunrofdff(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
                user = await cursor.fetchone()

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 0 WHERE id = {inter.user.id}")
                await db.commit()

            await inter.response.edit_message(embed=discord.Embed(
                title=f"Личный кабинет: {inter.user.global_name}",
                description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {user[1]} бебр\nУведомления: выключены**"
            ), view=Sbp.turnon())

        @discord.ui.button(label="Перевести бебры", style=discord.ButtonStyle.blurple, emoji="💸")
        async def perevod(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.send_message("Отправьте айди получателя мне в личные сообщения", ephemeral=True)
            def check(m):
                return m.author == inter.user

            msg: discord.Message = await inter.client.wait_for('message', check=check)
            try:
                user = await inter.client.fetch_user(int(msg.content))
            except:
                return await msg.reply("Не удалось найти пользователя")
            
            if user.bot: return await msg.reply("Нельзя перевести бебры боту")
            if user == inter.user: return await msg.reply("Нельзя перевести бебры себе")

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
                use = await cursor.fetchone()

            if not use: return await msg.reply("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**")

            await msg.reply(f"Вы собираетесь перевести бебры пользователю **{user.global_name}** [{user.id}] \nПодтверждаете платёж?", view=Sbp.yesornoH())

    class yesornoH(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Да", style=discord.ButtonStyle.success)
        async def accept_plat(self, inter: discord.Interaction, button: discord.ui.Button):
            user = await inter.client.fetch_user(int(inter.message.content.split()[6].replace("[", "").replace("]","")))
            await inter.response.send_modal(transferm(title="Перевод " + str(user.global_name), user=user, messag=True))

        @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
        async def dontaccept_plat(self, inter: discord.Interaction, button: discord.ui.Button):
            await inter.response.edit_message(content="Отменено", view=None)

    class turnon1(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success)
        async def gkrejgkerhlg(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 1 WHERE id = {inter.user.id}")
                await db.commit()
            await inter.response.edit_message(view=Sbp.turnoff1())

    class turnoff1(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger)
        async def gkrejgkerhlg(self, inter: discord.Interaction, button: discord.ui.Button):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 0 WHERE id = {inter.user.id}")
                await db.commit()
            await inter.response.edit_message(view=Sbp.turnon1())

    @app_commands.command( description="Твой личный кабинет Системы Быстрых Платежей!", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def account(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            user = await cursor.fetchone()
        if not user:
            return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        await inter.response.send_message(embed=discord.Embed(
            title=f"Личный кабинет: {inter.user.global_name}",
            description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {round(user[1], 1)} бебр\nУведомления: {'включены' if user[2]==1 else 'выключены'}**"
        ), ephemeral=True,view=Sbp.turnoff() if user[2]==1 else Sbp.turnon())

    @app_commands.command( description="Зарегистрироваться в Системе Быстрых Платежей", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def reg(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            user = await cursor.fetchone()
        if not user:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("INSERT INTO `sbp` (id) VALUES (?)", (inter.user.id,))
                await db.commit()
            return await inter.response.send_message("Успешно!", ephemeral=True)
        await inter.response.send_message("Вы уже зарегистрированы!", ephemeral=True)

    @app_commands.command( description="Изменить деньги СБП", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому переводить?", amount="Сколько переводить?")
    @app_commands.check(ext.check)
    async def setbal(self, inter: discord.Interaction, amount:int, user:discord.User = None):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute(f'UPDATE sbp SET balance = {amount} WHERE id = {inter.user.id}') if not user else await cursor.execute(f'UPDATE sbp SET balance = {amount} WHERE id = {user.id}')
            await db.commit()
        await inter.response.send_message("Успешно!", ephemeral=True)

    @app_commands.command( description="Перевод бебр пользователю Системы Быстрых Платежей", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    @app_commands.describe(user="Кому переводить?", amount="Сколько переводить?", comment="Комментарий к переводу")
    async def transfer(self, inter: discord.Interaction, user:discord.User, amount:int, comment:str = None):
        if comment and len(comment) > 50:
            await inter.response.send_message(
                "Комментарий не может превышать 50 символов.", 
                ephemeral=True
            )
            return
        if amount <= 0: return await inter.response.send_message("Пожалуйста, введите положительное или не нулевое число", ephemeral=True)
        if user.bot: return await inter.response.send_message("Нельзя перевести бебры боту", ephemeral=True)
        if user == inter.user: return await inter.response.send_message("Нельзя перевести бебры себе", ephemeral=True)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
            usr = await cursor.fetchone()

        if not me: return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        if not usr: return await inter.response.send_message("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**", ephemeral=True)

        if me[1] < amount: return await inter.response.send_message("У вас не хватает денег", ephemeral=True)

        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()  
            await cursor.execute(f"UPDATE sbp SET balance = balance - {amount} WHERE id = {inter.user.id}")
            await cursor.execute(f"UPDATE sbp SET balance = balance + {amount} WHERE id = {user.id}")
            await db.commit()

        await inter.response.send_message("Успешно!", ephemeral=True)
        await ext.update_quest(inter.user, "transfer", amount)
        if usr[2] == 1:
            if comment:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", description=
                                        f"Комментарий от отправителя: **{comment}**", color=discord.Color.green())
            else:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", color=discord.Color.green())
            
            await user.send(embed=embed, view=Sbp.turnoff1())

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

        await inter.response.send_message("Привет!\nТвоя капча:", ephemeral=True, file=discord.File('random_text.png'), view=captchab(captcha=kap))
        os.remove('random_text.png')

    @app_commands.command( description="Пригласить друга в СБП и получить деньги", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def invite(self, inter: discord.Interaction):
        await inter.response.send_message(embed=discord.Embed(title="Приглашение зарегистрироваться в СБП",
                                                              description="Чтобы принять, нажмите на кнопку ниже"),
                                                              view=AcceptInvite(author=inter.user))
        
class AcceptInvite(discord.ui.View):
    def __init__(self, author):
        super().__init__(timeout=None)
        self.author: discord.User = author

    @discord.ui.button(label="Принять приглашение", style=discord.ButtonStyle.blurple, custom_id="accept_invite")
    async def accept_invite_handler(self, inter: discord.Interaction, button: discord.ui.Button):
        if inter.user == self.author: return await inter.response.send_message("Вы являетесь автором приглашения", ephemeral=True)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            user_db = await cursor.fetchone()        

            if user_db: return await inter.response.send_message("Вы уже зарегистрированы в Системе Быстрых платежей!", ephemeral=True)

            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (self.author.id,))
            invites = await cursor.fetchone()
            invites: list = json.loads(invites[3])
            invites.append(inter.user.id)

            await cursor.execute("INSERT INTO `sbp` (id) VALUES (?)", (inter.user.id,))
            await cursor.execute("UPDATE sbp SET (invites, balance) = (?, balance+?) WHERE id = ?", (json.dumps(invites), 200, self.author.id))
            await db.commit()

        await inter.response.send_message(embed=discord.Embed(title="Приглашение принято", description="Вы зарегистрировались в СБП по ссылке от " + self.author.name), ephemeral=True)
        await self.author.send(f"{inter.user.name} зарегистрировался в СБП по вашей ссылке!")

class captchab(discord.ui.View):
    def __init__(self, captcha: string):
        super().__init__(timeout=None)
        self.captcha = captcha

    @discord.ui.button(label="Ввести капчу", style=discord.ButtonStyle.blurple)
    async def vvdod(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.send_modal(captcham(captcha=self.captcha))

class captcham(discord.ui.Modal, title = "Капча"):
    def __init__(self, captcha):
        super().__init__()
        self.captcha: string  = captcha

    capt = discord.ui.TextInput(label="Введите капчу:", required=True)

    async def on_submit(self, inter: discord.Interaction):
        if self.capt.value == self.captcha:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
                me = await cursor.fetchone()

            if not me: return await inter.response.edit_message(content="Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", view=None)

            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()  
                await cursor.execute(f"UPDATE sbp SET balance = balance + 5 WHERE id = {inter.user.id}")
                await db.commit()

            await inter.response.edit_message(content="Капча успешно введена! Вам было добавлено 5 бебр", view=None)
            await ext.update_quest(inter.user, "captcha", )
        else:
            await inter.response.edit_message(content="Капча введена неверно! Попробуйте ещё раз", view=None)

class transferm(discord.ui.Modal):
    def __init__(self, title, user, messag=False, *args, **kwargs):
        super().__init__(title=title,*args, **kwargs)
        self.user: discord.User = user
        self.messag = messag

    amount = discord.ui.TextInput(label="Сумма перевода", required=True)
    comment = discord.ui.TextInput(label="Комментарий к переводу",required=False, max_length=50)

    async def on_submit(self, inter: discord.Interaction):
        try:
            amount = int(self.amount.value)
        except:
            return await inter.response.send_message("Пожалуйста, введите целое число, а не буковки", ephemeral=True)
        if amount <= 0: return await inter.response.send_message("Пожалуйста, введите положительное или не нулевое число", ephemeral=True)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            me = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (self.user.id,))
            usr = await cursor.fetchone()

        if me[1] < amount: return await inter.response.send_message("У вас не хватает денег", ephemeral=True)

        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()  
            await cursor.execute(f"UPDATE sbp SET balance = balance - {amount} WHERE id = {inter.user.id}")
            await cursor.execute(f"UPDATE sbp SET balance = balance + {amount} WHERE id = {self.user.id}")
            await db.commit()

        if not self.messag:
            await inter.response.send_message(content="Успешно!", ephemeral=True)
        else:
            await inter.response.edit_message(content="Успешно!", view=None)

        await ext.update_quest(inter.user, "transfer", amount)

        if usr[2] == 1:
            if self.comment.value:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", description=
                                    f"Комментарий от отправителя: **{self.comment.value}**", color=discord.Color.green())
            else:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", color=discord.Color.green())
            
            await self.user.send(embed=embed, view=Sbp.turnoff1())

async def setup(bot: commands.Bot):
    await bot.add_cog(Sbp(bot))
    bot.tree.add_command(transferu)
    global dbn
    dbn = bot.dbn
    print("Sbp cog loaded")

@app_commands.context_menu( name="Перевод", )
@app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
@app_commands.allowed_installs(guilds=True, users=True)
@app_commands.check(ext.check)
async def transferu(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Нельзя перевести бебры боту", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Нельзя перевести бебры себе", ephemeral=True)

    async with aiosqlite.connect(dbn, timeout=20) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
        me = await cursor.fetchone()
        await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
        use = await cursor.fetchone()

    if not me: return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
    if not use: return await inter.response.send_message("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**", ephemeral=True)

    await inter.response.send_modal(transferm(title="Перевод " + str(user.global_name), user=user))