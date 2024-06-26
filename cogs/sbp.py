from disnake.ext import commands
import disnake
import aiosqlite
import os
import string
from PIL import Image, ImageDraw, ImageFont
import random

class Sbp(commands.Cog):

    def __init__(self, bot):
        self.bot = bot

    @commands.Cog.listener("on_button_click")
    async def btnclicks(self, inter: disnake.MessageInteraction):
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            user = await cursor.fetchone()

        if inter.component.custom_id == "turnoff":
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 0 WHERE id = {inter.author.id}")
                await db.commit()
            await inter.response.edit_message(embed=disnake.Embed(
                title=f"Личный кабинет: {inter.author.global_name}",
                description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.author.global_name}!\nБаланс: {user[1]} бебр\nУведомления: выключены**"
            ), components=[disnake.ui.Button(label="Включить уведомления", style=disnake.ButtonStyle.success, custom_id="turnon", emoji="✅"),
                        disnake.ui.Button(label="Перевести бебры", style=disnake.ButtonStyle.blurple, custom_id="transferb", emoji="💸")])

        elif inter.component.custom_id == "turnon":
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 1 WHERE id = {inter.author.id}")
                await db.commit()
            await inter.response.edit_message(embed=disnake.Embed(
                title=f"Личный кабинет: {inter.author.global_name}",
                description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.author.global_name}!\nБаланс: {user[1]} бебр\nУведомления: включены**"
            ), components=[disnake.ui.Button(label="Выключить уведомления", style=disnake.ButtonStyle.danger, custom_id="turnoff", emoji="✖"),
                        disnake.ui.Button(label="Перевести бебры", style=disnake.ButtonStyle.blurple, custom_id="transferb", emoji="💸")])

        elif inter.component.custom_id == "turnon1":
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 1 WHERE id = {inter.author.id}")
                await db.commit()
            await inter.response.edit_message(components=[disnake.ui.Button(label="Выключить уведомления", style=disnake.ButtonStyle.danger, custom_id="turnoff1", emoji="✖")])

        elif inter.component.custom_id == "turnoff1":
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 0 WHERE id = {inter.author.id}")
                await db.commit()
            await inter.response.edit_message(components=[disnake.ui.Button(label="Включить уведомления", style=disnake.ButtonStyle.success, custom_id="turnon1", emoji="✅")])

        elif inter.component.custom_id == "transferb":
            await inter.response.send_message("Отправьте айди получателя мне в личные сообщения", ephemeral=True)
            def check(m):
                return m.author == inter.author

            msg: disnake.Message = await self.bot.wait_for('message', check=check)
            try:
                user = await self.bot.fetch_user(int(msg.content))
            except:
                return await msg.reply("Не удалось найти пользователя")
            
            if user.bot: return await msg.reply("Нельзя перевести бебры боту")
            if user == inter.author: return await msg.reply("Нельзя перевести бебры себе")

            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
                use = await cursor.fetchone()

            if not use: return await msg.reply("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**")

            await msg.reply(f"Вы собираетесь перевести бебры пользователю **{user.global_name}** [{user.id}] \nПодтверждаете платёж?", components=[
                disnake.ui.Button(label="Да", style=disnake.ButtonStyle.success, custom_id="yes-transfer", emoji="✅"),
                disnake.ui.Button(label="Нет", style=disnake.ButtonStyle.danger, custom_id="no-transfer", emoji="✖")])
            
        elif inter.component.custom_id == "yes-transfer":
            user = await self.bot.getch_user(int(inter.message.content.split()[6].replace("[", "").replace("]","")))
            await inter.response.send_modal(transferm(title="Перевод " + str(user.global_name), user=user, messag=True))

        elif inter.component.custom_id == "no-transfer":
            await inter.response.edit_message("Отменено", components=[])

    @commands.slash_command(description="Твой личный кабинет Системы Быстрых Платежей!", integration_types=[0,1], contexts=[0,1,2])
    async def account(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            user = await cursor.fetchone()
        if not user:
            return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        await inter.response.send_message(embed=disnake.Embed(
            title=f"Личный кабинет: {inter.author.global_name}",
            description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.author.global_name}!\nБаланс: {user[1]} бебр\nУведомления: {'включены' if user[2]==1 else 'выключены'}**"
        ), ephemeral=True,
        components=[disnake.ui.Button(label="Выключить уведомления", style=disnake.ButtonStyle.danger, custom_id="turnoff", emoji="✖") if user[2]==1 else disnake.ui.Button(label="Включить уведомления", style=disnake.ButtonStyle.success, custom_id="turnon", emoji="✅"),
                    disnake.ui.Button(label="Перевести бебры", style=disnake.ButtonStyle.blurple, custom_id="transferb", emoji="💸")])

    @commands.slash_command(description="Зарегистрироваться в Системе Быстрых Платежей", integration_types=[0,1], contexts=[0,1,2])
    async def reg(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            user = await cursor.fetchone()
        if not user:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("INSERT INTO `sbp` (id) VALUES (?)", (inter.author.id,))
                await db.commit()
            return await inter.response.send_message("Успешно!", ephemeral=True)
        await inter.response.send_message("Вы уже зарегистрированы!", ephemeral=True)

    @commands.slash_command(description="Изменить количество денег в СБП", integration_types=[0,1], contexts=[0,1,2], options=[
        disnake.Option(name="amount", description="На сколько изменять?", required=True, type=disnake.OptionType.integer),
        disnake.Option(name="user", description="Кому изменять?", required=False, type=disnake.OptionType.user)
    ])
    async def setbal(self, inter: disnake.ApplicationCommandInteraction):
        if inter.author.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute(f'UPDATE sbp SET balance = {inter.options["amount"]} WHERE id = {inter.author.id}') if not inter.options.get("user") else await cursor.execute(f'UPDATE sbp SET balance = {inter.options["amount"]} WHERE id = {inter.options.get("user").id}')
            await db.commit()
        await inter.response.send_message("Успешно!", ephemeral=True)

    @commands.user_command(name="Перевод", integration_types=[0,1], contexts=[0,1,2])
    async def transferu(self, inter: disnake.UserCommandInteraction, user: disnake.User):
        if await self.bot.check(inter) == 1: return
        if user.bot: return await inter.response.send_message("Нельзя перевести бебры боту", ephemeral=True)
        if user == inter.author: return await inter.response.send_message("Нельзя перевести бебры себе", ephemeral=True)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            me = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
            use = await cursor.fetchone()

        if not me: return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        if not use: return await inter.response.send_message("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**", ephemeral=True)

        await inter.response.send_modal(transferm(title="Перевод " + str(user.global_name), user=user))

    @commands.slash_command(description="Перевод бебр пользователю Системы Быстрых Платежей", integration_types=[0,1], contexts=[0,1,2], options=[
        disnake.Option(name="user", description="Кому переводить?", required=True, type=disnake.OptionType.user),
        disnake.Option(name="amount", description="Сколько переводить?", required=True, type=disnake.OptionType.string),
        disnake.Option(name="comment", description="Комментарий к переводу?", required=False, type=disnake.OptionType.string, max_length=50)
    ])
    async def transfer(self, inter: disnake.ApplicationCommandInteraction):
        if await self.bot.check(inter) == 1: return
        try:
            amount = int(inter.options.get("amount"))
        except:
            return await inter.response.send_message("Пожалуйста, введите целое число, а не буковки", ephemeral=True)
        if amount <= 0: return await inter.response.send_message("Пожалуйста, введите положительное или не нулевое число", ephemeral=True)
        user = inter.options.get("user")
        if user.bot: return await inter.response.send_message("Нельзя перевести бебры боту", ephemeral=True)
        if user == inter.author: return await inter.response.send_message("Нельзя перевести бебры себе", ephemeral=True)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            me = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (user.id,))
            usr = await cursor.fetchone()

        if not me: return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        if not usr: return await inter.response.send_message("Пользователь не зарегистрирован в Системе Быстрых платежей! Скажите ему чтобы он сделал это, написав **/reg**", ephemeral=True)

        if me[1] < amount: return await inter.response.send_message("У вас не хватает денег", ephemeral=True)

        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()  
            await cursor.execute(f"UPDATE sbp SET balance = balance - {amount} WHERE id = {inter.author.id}")
            await cursor.execute(f"UPDATE sbp SET balance = balance + {amount} WHERE id = {user.id}")
            await db.commit()

        await inter.response.send_message("Успешно!", ephemeral=True)

        if usr[2] == 1:
            if inter.options.get("comment"):
                embed = disnake.Embed(title=f"Получен перевод от {inter.author.global_name} суммой {amount} бебр.", description=
                                        f"Комментарий от отправителя: **{inter.options.get("comment")}**", color=disnake.Color.green())
            else:
                embed = disnake.Embed(title=f"Получен перевод от {inter.author.global_name} суммой {amount} бебр.", color=disnake.Color.green())
            
            await user.send(embed=embed, components=[disnake.ui.Button(label="Выключить уведомления", style=disnake.ButtonStyle.danger, custom_id="turnoff1", emoji="✖")])

    @commands.slash_command(description="Пройти капчу и получить бебры", integration_types=[0,1], contexts=[0,1,2])
    async def captcha(self, inter: disnake.UserCommandInteraction):
        if await self.bot.check(inter) == 1: return
        letters = string.ascii_letters + string.digits
        kap = ''.join(random.choice(letters) for i in range(10))
        image = Image.new('RGB', (200, 50), (255, 255, 255))

        # Open a font file
        font = ImageFont.truetype('arial.ttf', 28)

        # Create a draw object
        draw = ImageDraw.Draw(image)

        # Draw the text on the image
        draw.text((10, 10), kap, font=font, fill=(0, 0, 0))

        # Save the image to a file
        image.save('random_text.png')
        #await ctx.respond(f'Привет!\nУ тебя 5 секунд на ввод капчи!', view=kapch(), ephemeral=True, file=discord.File('random_text.png'))
        await inter.response.send_message("Привет!\nТвоя капча:", ephemeral=True, file=disnake.File('random_text.png'), view=captchab(captcha=kap))
        os.remove('random_text.png')

class captchab(disnake.ui.View):
    def __init__(self, captcha: string):
        super().__init__(timeout=None)
        self.captcha = captcha

    @disnake.ui.button(label="Ввести капчу", style=disnake.ButtonStyle.blurple)
    async def vvdod(self, button: disnake.ui.Button, inter: disnake.MessageInteraction):
        await inter.response.send_modal(captcham(captcha=self.captcha))

class captcham(disnake.ui.Modal):
    def __init__(self, captcha: string):
        components = [
            disnake.ui.TextInput(
                label="Капча",
                custom_id="captcha",
                style=disnake.TextInputStyle.short,
                required=True,
                max_length=11,
            ),
        ]
        super().__init__(title="Капча", components=components)
        self.captcha: string  = captcha

    async def callback(self, inter: disnake.ModalInteraction):
        if inter.text_values["captcha"] == self.captcha:
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
                me = await cursor.fetchone()

            if not me: return await inter.response.edit_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", view=None)

            async with aiosqlite.connect(dbn) as db:
                cursor = await db.cursor()  
                await cursor.execute(f"UPDATE sbp SET balance = balance + 5 WHERE id = {inter.author.id}")
                await db.commit()

            await inter.response.edit_message("Капча успешно введена! Вам было добавлено 5 бебр", view=None)
        else:
            await inter.response.edit_message("Капча введена неверно! Попробуйте ещё раз", view=None)

class transferm(disnake.ui.Modal):
    def __init__(self, title, user, messag=False):
        components = [
            disnake.ui.TextInput(
                label="Сумма перевода",
                custom_id="amount",
                style=disnake.TextInputStyle.short,
                required=True,
            ),
            disnake.ui.TextInput(
                label="Комментарий к переводу",
                required=False,
                custom_id="comment",
                style=disnake.TextInputStyle.short,
                max_length=50
            )
        ]
        super().__init__(title=title, components=components)
        self.user: disnake.User = user
        self.messag = messag

    async def callback(self, inter: disnake.ModalInteraction):
        try:
            amount = int(inter.text_values["amount"])
        except:
            return await inter.response.send_message("Пожалуйста, введите целое число, а не буковки", ephemeral=True)
        if amount <= 0: return await inter.response.send_message("Пожалуйста, введите положительное или не нулевое число", ephemeral=True)

        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.author.id,))
            me = await cursor.fetchone()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (self.user.id,))
            usr = await cursor.fetchone()

        if me[1] < amount: return await inter.response.send_message("У вас не хватает денег", ephemeral=True)

        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()  
            await cursor.execute(f"UPDATE sbp SET balance = balance - {amount} WHERE id = {inter.author.id}")
            await cursor.execute(f"UPDATE sbp SET balance = balance + {amount} WHERE id = {self.user.id}")
            await db.commit()

        if not self.messag:
            await inter.response.send_message("Успешно!", ephemeral=True)
        else:
            await inter.response.edit_message("Успешно!", components=[])

        if usr[2] == 1:
            if inter.text_values["comment"]:
                embed = disnake.Embed(title=f"Получен перевод от {inter.author.global_name} суммой {amount} бебр.", description=
                                    f"Комментарий от отправителя: **{inter.text_values['comment']}**", color=disnake.Color.green())
            else:
                embed = disnake.Embed(title=f"Получен перевод от {inter.author.global_name} суммой {amount} бебр.", color=disnake.Color.green())
            
            await self.user.send(embed=embed, components=[disnake.ui.Button(label="Выключить уведомления", style=disnake.ButtonStyle.danger, custom_id="turnoff1", emoji="✖")])

def setup(bot):
    bot.add_cog(Sbp(bot))
    global dbn
    dbn = bot.dbn
    print("Sbp cog loaded")
