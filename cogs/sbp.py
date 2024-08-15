import inspect
from discord.ext import commands
import discord
import aiosqlite
import os
import string
from PIL import Image, ImageDraw, ImageFont
import random

class Sbp(commands.Cog):
    def __init__(self, bot: discord.Bot):
        self.bot: discord.Bot = bot

    class turnon(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)
        @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success, emoji="✅")
        async def tunroff11(self, button: discord.ui.Button, inter: discord.Interaction):
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
        async def perevod1(self, button: discord.ui.Button, inter: discord.Interaction):
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
        async def tunrofdff(self, button: discord.ui.Button, inter: discord.Interaction):
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
        async def perevod(self, button: discord.ui.Button, inter: discord.Interaction):
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
        async def accept_plat(self, button: discord.ui.Button, inter: discord.Interaction):
            user = await inter.client.get_or_fetch_user(int(inter.message.content.split()[6].replace("[", "").replace("]","")))
            await inter.response.send_modal(transferm(title="Перевод " + str(user.global_name), user=user, messag=True))

        @discord.ui.button(label="Нет", style=discord.ButtonStyle.danger)
        async def dontaccept_plat(self, button: discord.ui.Button, inter: discord.Interaction):
            await inter.response.edit_message(content="Отменено", view=None)

    class turnon1(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Включить уведомления", style=discord.ButtonStyle.success)
        async def gkrejgkerhlg(self, button: discord.ui.Button, inter: discord.Interaction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 1 WHERE id = {inter.user.id}")
                await db.commit()
            await inter.response.edit_message(view=Sbp.turnoff1())

    class turnoff1(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Выключить уведомления", style=discord.ButtonStyle.danger)
        async def gkrejgkerhlg(self, button: discord.ui.Button, inter: discord.Interaction):
            async with aiosqlite.connect(dbn, timeout=20) as db:
                cursor = await db.cursor()
                await cursor.execute(f"UPDATE sbp SET notif = 0 WHERE id = {inter.user.id}")
                await db.commit()
            await inter.response.edit_message(view=Sbp.turnon1())

    @commands.slash_command(
        description="Твой личный кабинет Системы Быстрых Платежей!", 
        integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
        contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm, discord.InteractionContextType.guild}
    )
    async def account(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT * FROM `sbp` WHERE id = ?", (inter.user.id,))
            user = await cursor.fetchone()
        if not user:
            return await inter.respond("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)
        await inter.response.send_message(embed=discord.Embed(
            title=f"Личный кабинет: {inter.user.global_name}",
            description=f"**Добро пожаловать в Систему Быстрых Платежей, {inter.user.global_name}!\nБаланс: {user[1]} бебр\nУведомления: {'включены' if user[2]==1 else 'выключены'}**"
        ), ephemeral=True,view=Sbp.turnoff() if user[2]==1 else Sbp.turnon())

    @commands.slash_command(description="Зарегистрироваться в Системе Быстрых Платежей", integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
                   contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild})
    async def reg(self, inter: discord.ApplicationContext):
        if await self.bot.check(inter) == 1: return
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

    @commands.slash_command(description="Изменить количество денег в СБП", integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
                   contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild})
    @discord.option("amount", description="На сколько изменять?", required=True, input_type=discord.SlashCommandOptionType.integer)  
    @discord.option("user", description="Кому изменять?", required=False, input_type=discord.SlashCommandOptionType.user)   
    async def setbal(self, inter: discord.ApplicationContext, amount:int, user:discord.User):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
        async with aiosqlite.connect(dbn, timeout=20) as db:
            cursor = await db.cursor()
            await cursor.execute(f'UPDATE sbp SET balance = {amount} WHERE id = {inter.user.id}') if not user else await cursor.execute(f'UPDATE sbp SET balance = {amount} WHERE id = {user.id}')
            await db.commit()
        await inter.response.send_message("Успешно!", ephemeral=True)

    @commands.user_command(name="Перевод", integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
                   contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild})
    async def transferu(self, inter: discord.ApplicationContext, user: discord.User):
        if await self.bot.check(inter) == 1: return
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

    @commands.slash_command(description="Перевод бебр пользователю Системы Быстрых Платежей", integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
                   contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild})
    @discord.option("user", description="Кому переводить?", required=True, input_type=discord.SlashCommandOptionType.user)
    @discord.option("amount", description="Сколько переводить?", required=True, input_type=discord.SlashCommandOptionType.integer)  
    @discord.option("comment", str, description="Комментарий к переводу", required=False, max_length=50)
    async def transfer(self, inter: discord.ApplicationContext, user:discord.User, amount:int, comment:str):
        if await self.bot.check(inter) == 1: return
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

        if usr[2] == 1:
            if comment:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", description=
                                        f"Комментарий от отправителя: **{comment}**", color=discord.Color.green())
            else:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", color=discord.Color.green())
            
            await user.send(embed=embed, view=Sbp.turnoff1())

    @commands.slash_command(description="Пройти капчу и получить бебры", integration_types={discord.IntegrationType.user_install, discord.IntegrationType.guild_install},
                   contexts={discord.InteractionContextType.private_channel, discord.InteractionContextType.bot_dm,
                             discord.InteractionContextType.guild})
    async def captcha(self, inter: discord.ApplicationContext):
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
        await inter.response.send_message("Привет!\nТвоя капча:", ephemeral=True, file=discord.File('random_text.png'), view=captchab(captcha=kap))
        os.remove('random_text.png')

class captchab(discord.ui.View):
    def __init__(self, captcha: string):
        super().__init__(timeout=None)
        self.captcha = captcha

    @discord.ui.button(label="Ввести капчу", style=discord.ButtonStyle.blurple)
    async def vvdod(self, button: discord.ui.Button, inter: discord.ApplicationContext):
        await inter.response.send_modal(captcham(captcha=self.captcha))

class captcham(discord.ui.Modal):
    def __init__(self, captcha, *args, **kwargs) -> None:
        super().__init__(title="Капча", *args, **kwargs)
        self.add_item(discord.ui.InputText(label="Введите капчу:", required=True)),
        self.captcha: string  = captcha

    async def callback(self, inter: discord.ApplicationContext):
        if self.children[0].value == self.captcha:
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
        else:
            await inter.response.edit_message(content="Капча введена неверно! Попробуйте ещё раз", view=None)

class transferm(discord.ui.Modal):
    def __init__(self, title, user, messag=False, *args, **kwargs):
        super().__init__(title=title,*args, **kwargs)
        self.add_item(discord.ui.InputText(label="Сумма перевода", required=True)),
        self.add_item(discord.ui.InputText(label="Комментарий к переводу",required=False, max_length=50))
        self.user: discord.User = user
        self.messag = messag

    async def callback(self, inter: discord.ApplicationContext):
        try:
            amount = int(self.children[0].value)
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

        if usr[2] == 1:
            if self.children[1].value:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", description=
                                    f"Комментарий от отправителя: **{self.children[1].value}**", color=discord.Color.green())
            else:
                embed = discord.Embed(title=f"Получен перевод от {inter.user.global_name} суммой {amount} бебр.", color=discord.Color.green())
            
            await self.user.send(embed=embed, view=Sbp.turnoff1())

def setup(bot: discord.Bot):
    bot.add_cog(Sbp(bot))
    global dbn
    dbn = bot.dbn
    print("Sbp cog loaded")
