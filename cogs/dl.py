from discord.ext import commands
import discord
from ext import check, update_quest
from datetime import datetime
import random
import asyncio
import pytz
from discord import app_commands
from db.database_instance import db

class donateBtns(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)

    @discord.ui.button(label="ДО в СБП", style=discord.ButtonStyle.success, row=1)
    async def dovsbp(self, inter: discord.Interaction, button: discord.ui.Button):
        me = await db.sbp.get_user(inter.user.id)

        from cogs.sbp import AUTHOR_UNATHORIZED_ERROR
        if not me: return await inter.response.send_message(content=AUTHOR_UNATHORIZED_ERROR, view=None, embed=None, ephemeral=True)

        await inter.response.send_modal(dovsbpModal())

    @discord.ui.button(label="СБП в ДО", style=discord.ButtonStyle.success, row=1)
    async def sbpvdo(self, inter: discord.Interaction, button: discord.ui.Button):
        me = await db.sbp.get_user(inter.user.id)

        from cogs.sbp import AUTHOR_UNATHORIZED_ERROR
        if not me: return await inter.response.send_message(content=AUTHOR_UNATHORIZED_ERROR, view=None, embed=None, ephemeral=True)

        await inter.response.send_modal(sbpvdoModal())
        
    @discord.ui.button(label="Назад", style=discord.ButtonStyle.success, row=1)
    async def backTomenu(self, inter: discord.Interaction, button: discord.ui.Button):
        await inter.response.edit_message(embed=None, view=DL.mn(), content="Главное меню")

class dovsbpModal(discord.ui.Modal):
    def __init__(self, *args, **kwargs):
        super().__init__(title="ДО в СБП", *args, **kwargs)

    num = discord.ui.TextInput(label="Сколько переводить монет ДО? (300 к 1)", required=True)
    async def on_submit(self, inter: discord.Interaction):
        try:
            num = int(self.num.value)
        except:
            return await inter.response.edit_message(content="Не является числом!", view=donateBtns())
        if num < 300:
            return await inter.response.edit_message(content="Минимальный перевод 300 монет", view=donateBtns())

        me = await db.dl.get_user(inter.user.id)

        if me.balance < num:
            return await inter.response.edit_message(content="Недостаточно средств!", view=donateBtns())

        await me.decrease_balance(num)
        await db.sbp.increase_balance(inter.user.id, round(num/300, 1))

        await inter.response.edit_message(content=f"Успешно перевёл {round(num/300, 1)} бебр!", view=DL.mn(), embed=None)

class sbpvdoModal(discord.ui.Modal):
    def __init__(self, *args, **kwargs):
        super().__init__(title="СБП в ДО", *args, **kwargs)
        
    num = discord.ui.TextInput(label="Сколько переводить бебр? (1 к 0.5)", required=True)
    async def on_submit(self, inter: discord.Interaction):
        try:
            num = int(self.num.value)
        except:
            return await inter.response.edit_message(content="Не является числом!", view=donateBtns())
        if num < 300:
            return await inter.response.edit_message(content="Минимальный перевод 300 бебр", view=donateBtns())

        me = await db.sbp.get_user(inter.user.id)

        if me.balance < num:
            return await inter.response.edit_message(content="Недостаточно средств!", view=donateBtns())

        await db.dl.increase_stat(inter.user.id, "balance", round(num*0.5, 1))
        await me.decrease_balance(num)

        await inter.response.edit_message(content=f"Успешно перевёл {round(num*0.5, 1)} монет ДО!", view=DL.mn(), embed=None)

class DL(commands.Cog):
    def __init__(self, bot):
        self.bot: commands.Bot = bot

    class confdel(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Да", style=discord.ButtonStyle.danger)
        async def yesdel_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            await db.dl.delete_user(inter.user.id)

            await inter.response.edit_message(content="Пока, путник!", view=None, embed=None)

        @discord.ui.button(label="Нет", style=discord.ButtonStyle.success)
        async def nodel_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            await inter.response.edit_message(content="Молодец, что одумался!", view=DL.mn(), embed=None)

    class seller(discord.ui.Modal):
        def __init__(self, *args, **kwargs):
            super().__init__(title="Что желаешь купить, путник?", *args, **kwargs)

        item = discord.ui.TextInput(label="Всё по 399 монет! (хп/урон/мана)", required=True)

        async def on_submit(self, inter: discord.Interaction):
            user = await db.dl.get_user(inter.user.id)

            if user.balance < 399: return await inter.response.send_message("Недостаточно средств", ephemeral=True)

            if self.item.value.lower() == "хп":
                await user.decrease_balance(399)
                await user.increase_health(10)

            elif self.item.value.lower() == "урон":
                await user.decrease_balance(399)
                await user.increase_damage(10)

            elif self.item.value.lower() == "мана":
                await user.decrease_balance(399)
                await user.increase_health(10)

            else:
                return await inter.response.send_message("Такого товара нет в наличии. Попробуйте ещё раз", ephemeral=True)

            await inter.response.send_message("Успешно!", ephemeral=True)

    class mn(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Отправиться в лабиринт", style=discord.ButtonStyle.danger, row=1)
        async def labirint(self, inter: discord.Interaction, button: discord.ui.Button):
            user = await db.dl.get_user(inter.user.id)
                
            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            
            if user.in_game:
                return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)
            else:
                await user.set_in_game(True)

            await inter.response.edit_message(content="Вы вошли в лабиринт.", embed=None, view=None)
            await asyncio.sleep(1.5)

            monsters = await db.dl.get_monsters()
            monster = random.choice(monsters)
            r_name = monster.name

            embed=discord.Embed(title="Лабиринт", description=f"**Вы наткнулись на {r_name}!**", color=discord.Color.random())
            msg: discord.Interaction = await inter.original_response()

            await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
            await asyncio.sleep(1.75)

            player_hp = user.health
            monster_hp = monster.health

            while player_hp > 0 and monster_hp > 0:
                monster_hp -= user.damage
                await asyncio.sleep(2)

                embed=discord.Embed(title=r_name, description=f"**Вы атакуете...**", color=discord.Color.random())
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)

                await asyncio.sleep(2)

                if monster_hp <= 0:
                    embed=discord.Embed(title=r_name, description=f"**Ты победил {r_name}!\nЗа победу тебе выдали {monster.reward} монет!**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())

                    await update_quest(inter.user, "dromlyandia")

                    await user.increase_balance(monster.reward)
                    break

                else:
                    embed=discord.Embed(title=r_name, description=f"**У {r_name} осталось {monster_hp} хп!**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)

                await asyncio.sleep(2)

                embed=discord.Embed(title=r_name, description=f"**{r_name} атакует...**", color=discord.Color.random())
                await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)

                await asyncio.sleep(2)

                player_hp -= monster.damage

                if player_hp <= 0:
                    slivm = random.randint(1,99)
                    
                    embed=discord.Embed(title=r_name, description=f"**Вы проиграли ): {r_name} спиздил у вас {slivm} монет**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=DL.mn())
                    
                    await user.decrease_balance(slivm)
                    break

                else:
                    embed=discord.Embed(title=r_name, description=f"**У вас осталось {player_hp} хп**", color=discord.Color.random())
                    await inter.followup.edit_message(message_id=msg.id, content="", embed=embed, view=None)
                    continue

            await user.set_in_game(False)

        @discord.ui.button(label="Информация об игроке", style=discord.ButtonStyle.success, row=1)
        async def playerinfo(self, inter: discord.Interaction, button: discord.ui.Button,):
            user = await db.dl.get_user(inter.user.id)

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user.in_game: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            embed = discord.Embed(title=f"Ваше имя: {user.name}\nКласс: {user.className}\nБаланс: {user.balance}\nХП: {user.health}\nМана: {user.mana}\nУрон: {user.damage}", color=discord.Color.random())
            now = datetime.now(pytz.timezone('Europe/Moscow'))
            embed.set_footer(text=f'Дромляндия: Онлайн • Сегодня в {now.strftime("%H:%M")}')

            await inter.response.edit_message(content="", embed=embed, view=self)

        @discord.ui.button(label="Удалить персонажа", style=discord.ButtonStyle.danger, row=2)
        async def deleteplayer(self, inter: discord.Interaction, button: discord.ui.Button,):
            user = await db.dl.get_user(inter.user.id)

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user.in_game: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.edit_message(content=f"Вы действительно хотите удалить персонажа?\nВы потеряете {user.balance} монет!", embed=None, view=DL.confdel())

        @discord.ui.button(label="Магазин", style=discord.ButtonStyle.success, row=2)
        async def shop_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            user = await db.dl.get_user(inter.user.id)

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user.in_game: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)

            await inter.response.send_modal(DL.seller())

        @discord.ui.button(label="Донат", style=discord.ButtonStyle.danger, row=3)
        async def donate_dl(self, inter: discord.Interaction, button: discord.ui.Button,):
            user = await db.dl.get_user(inter.user.id)

            if not user: return await inter.response.send_message("Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже", view=DL.regb(), ephemeral=True)
            if user.in_game: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)
            
            embed = embed=discord.Embed(
                color=discord.Color.random(), title="Донатик", description=
                "Добро пожаловать в меню доната!\nЧтобы перевести деньги из баланса **Дромляндии: Онлайн** на свой счёт СБП нажмите кнопку 'ДО в СБП'.\nЧтобы перевести деньги из баланса **СБП** на свой счёт Дромляндии: Онлайн нажмите кнопку 'СБП в ДО'\nКурс перевода из ДО в СБП - 300 к 1\nКурс перевода из СБП в ДО - 1 к 1.5")
            
            embed.set_footer(text="При поддержке Системы Быстрых Платежей")
            
            await inter.response.edit_message(embed=embed,
                view=donateBtns(), content="")

    class regmodal(discord.ui.Modal):
        def __init__(self):
            super().__init__(title="Анкета ввода данных")

        name = discord.ui.TextInput(label="Твоё имя", required=True, max_length=50)
        clas = discord.ui.TextInput(label="Твой класс (маг/воин/танк)",required=True)

        async def on_submit(self, inter: discord.Interaction):
            if self.clas.value.lower() not in ["маг", "воин", "танк"]:
                return await inter.response.send_message("Неверный класс", ephemeral=True)

            await db.dl.create_user(inter.user.id, self.name.value, self.clas.value)
            await inter.response.edit_message(content=f"Добро пожаловать, {self.clas.value} {self.name.value}", view=DL.mn())

    class regb(discord.ui.View):
        def __init__(self):
            super().__init__(timeout=None)

        @discord.ui.button(label="Ввести данные", style=discord.ButtonStyle.success)
        async def vvestidannie(self, inter: discord.Interaction, button: discord.ui.Button,):
            await inter.response.send_modal(DL.regmodal())

    @app_commands.command( description="Войти в Дромляндия: Онлайн", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(check)
    async def game(self, inter: discord.Interaction):
        user = await db.dl.get_user(inter.user.id)

        if not user: return await inter.response.send_message(
            "Добро пожаловать в края Дромляндии, путник! Введи свои данные, нажав на кнопку ниже",
            view=self.regb(),
            ephemeral=True
        )
        
        if user.in_game: return await inter.response.send_message("Вы в данный момент в лабиринте", ephemeral=True)
        
        await inter.response.send_message(f"Добро пожаловать обратно, {user.className} {user.name}", view=self.mn(), ephemeral=True)

async def setup(bot: commands.Bot):
    await bot.add_cog(DL(bot))
    print("DL cog loaded")