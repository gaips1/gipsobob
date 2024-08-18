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
    async def account(self, inter: discord.Interaction):
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

    @app_commands.command( description="Зарегистрироваться в Системе Быстрых Платежей", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def reg(self, inter: discord.Interaction):
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

    @app_commands.command( description="Изменить деньги СБП", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(user="Кому переводить?", amount="Сколько переводить?")
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
    @app_commands.describe(user="Кому переводить?", amount="Сколько переводить?", comment="Комментарий к переводу")
    async def transfer(self, inter: discord.Interaction, user:discord.User, amount:int, comment:str = None):
        if await self.bot.check(inter) == 1: return
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
    async def captcha(self, inter: discord.Interaction):
        if await self.bot.check(inter) == 1: return
        letters = string.ascii_letters + string.digits
        kap = ''.join(random.choice(letters) for i in range(10))
        image = Image.new('RGB', (200, 50), (255, 255, 255))

        font = ImageFont.truetype('arial.ttf', 28)

        draw = ImageDraw.Draw(image)

        draw.text((10, 10), kap, font=font, fill=(0, 0, 0))
        image.save('random_text.png')

        await inter.response.send_message("Привет!\nТвоя капча:", ephemeral=True, file=discord.File('random_text.png'), view=captchab(captcha=kap))
        os.remove('random_text.png')

    @app_commands.command( description="Магазин", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def shop(self, ctx: discord.Interaction):
        await ctx.response.send_message("Добро пожаловать в круглосуточный магазин <<**У легенды**>>\n||Внимание! После завершения операции ваша душа будет\nавтоматически передана в вечное пользование Uzbia Inc.||", ephemeral=True, view=shop())

    @app_commands.command( description="Инвентарь", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    async def inventory(self, inter: discord.Interaction):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT inv FROM sbp WHERE id = ?", (inter.user.id,))
            usr = await cursor.fetchone()
            
            if usr:
                if not usr[0]:
                    return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
                inv: dict = json.loads(usr[0])
                if len(inv) == 0:
                    return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
            else:
                return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)

            inv_items = []

            for i in inv.keys():
                inv_items.append(f"**{i}** - {inv[i]}")
            
            await inter.response.send_message("Название предмета - количество у Вас в инвентаре:\n\n" + ",\n".join(inv_items) , ephemeral=True)

    def zov():
        return [app_commands.Choice(name="Талон на секс", value="талон на секс")]

    @app_commands.command( description="Использовать предмет из инвентаря", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(what="Что использовать?", user="На ком использовать?")
    @app_commands.choices(what=zov())
    async def use(self, ctx: discord.Interaction, what: str, user: discord.User = None):
        if await self.bot.check(ctx) == 1: return
        if what == "талон на секс":
            if user == None: return await ctx.response.send_message("Укажите пользователя", ephemeral=True)
            await use_sex_talon(ctx, user)
        
async def use_sex_talon(inter: discord.Interaction, user: discord.User):
    if user.bot: return await inter.response.send_message("Нельзя использовать на боте", ephemeral=True)
    if user == inter.user: return await inter.response.send_message("Нельзя использовать на себе", ephemeral=True)
    async with aiosqlite.connect(dbn) as db:
        cursor = await db.cursor()
        await cursor.execute("SELECT inv FROM sbp WHERE id = ?", (inter.user.id,))
        usr = await cursor.fetchone()
        
        if usr:
            if not usr[0]:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
            inv: dict = json.loads(usr[0])
            if len(inv) == 0:
                return await inter.response.send_message("У вас пустой инвентарь", ephemeral=True)
        else:
            return await inter.response.send_message("Вы не зарегистрированы в Системе Быстрых платежей! Сделайте это, написав **/reg**", ephemeral=True)

        for i in inv.keys():
            if i == "Талон на секс":
                giffs = ["https://media.tenor.com/pn5xTq0WtqcAAAAC/anime-girl.gif", "https://media.tenor.com/9G1zsVIiV6UAAAAC/anime-bed.gif", "https://media.tenor.com/tdK59AzAWZgAAAAC/pokemon-anime.gif"
                            , "https://media.tenor.com/i7S2Taae5H8AAAAC/sex-anime.gif", "https://media.tenor.com/eq-B2_glw0sAAAAC/ver-anime.gif"]
                randgif = random.choice(giffs)
                soglaz=discord.Embed(title=f"**{inter.user.name} использует талон на секс и ебётся с {user.name}**", color=discord.Color.random())
                soglaz.set_image(url=randgif)
                await inter.response.send_message(embed=soglaz)
                inv[i] -= 1
                if inv[i] <= 0:
                    await cursor.execute("UPDATE sbp SET inv =? WHERE id =?", (None, inter.user.id))
                else:
                    await cursor.execute("UPDATE sbp SET inv =? WHERE id =?", (json.dumps(inv), inter.user.id))
                await db.commit()
                return
        
    await inter.response.send_message("У вас нет талонов на секс", ephemeral=True)

class shop(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="Талон на секс [2200]", style=discord.ButtonStyle.success, custom_id="talon_na_sex")
    async def talon_na_sex(self, interaction: discord.Interaction, button: discord.ui.Button):
        await interaction.response.send_message("Талон на секс позволяет занятся сексом с любым пользователем дискорда, без его согласия.\nВы подтверждаете покупку?", ephemeral=True, view=confirm_buy(item="Талон на секс"))

class confirm_buy(discord.ui.View):
    def __init__(self, item: string):
        super().__init__(timeout=None)
        self.item = item
    
    @discord.ui.button(label="Приобрести", style=discord.ButtonStyle.success, custom_id="buy")
    async def buy(self, interaction: discord.Interaction, button: discord.ui.Button):
        async with aiosqlite.connect(dbn) as db:
            cursor = await db.cursor()
            await cursor.execute("SELECT balance, inv FROM sbp WHERE id = ?", (interaction.user.id,))
            usr = await cursor.fetchone()

            if usr[0] < 2200:
                return await interaction.response.send_message("Недостаточно средств!", ephemeral=True)
            
            if usr[1]:
                inv = json.loads(usr[1]) 
                inv[self.item] = inv.get(self.item, 0) + 1
            else:
                inv = {self.item: 1}

            inv_json = json.dumps(inv)

            await cursor.execute("UPDATE sbp SET balance = balance - 2200, inv = ? WHERE id = ?", (inv_json, interaction.user.id))
            await db.commit()

        await interaction.response.edit_message(content="Вы успешно приобрели " + self.item, view=None)

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
async def transferu(inter: discord.Interaction, user: discord.User):
    if await inter.client.check(inter) == 1: return
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