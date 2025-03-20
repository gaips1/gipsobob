from discord.ext import commands, tasks
from discord import app_commands
import discord
from datetime import datetime, timedelta
from db.models.ga import Giveaway
from ext import check
from discord.ui import Button, View
import random
import asyncio
import pytz
from db.database_instance import db

class dm(discord.ui.View):
    def __init__(self, id):
        super().__init__(timeout=None)
        self.id = id

    @discord.ui.button(label="Не хочу участвовать", style=discord.ButtonStyle.danger)
    async def delete_ga(self, inter: discord.Interaction, button: discord.ui.Button):
        giveaway = await db.ga.get_giveaway(self.id)
        
        await giveaway.remove_user(inter.user.id)
        
        for x in self.children:
            x.disabled = True

        await inter.response.edit_message(view=self)
        await inter.followup.send("Успешно!", ephemeral=True)

class gab(discord.ui.View):
    def __init__(self):
        super().__init__(timeout=None)
        
    @discord.ui.button(label="Принять участие", style=discord.ButtonStyle.success, custom_id="prinyat_uchastie")
    async def join_ga(self, inter: discord.Interaction, button: discord.ui.Button):
        giveaway = await db.ga.get_giveaway(inter.message.id)

        if inter.user.id in giveaway.users:
            return await inter.response.send_message("Вы уже участвуете в розыгрыше!", ephemeral=True, view=dm(inter.message.id))
        
        await giveaway.add_user(inter.user.id)
        await inter.response.send_message("Успешно!", ephemeral=True)

class view_giv(discord.ui.View):
    def __init__(self, url):
        super().__init__(timeout=None)
        self.url = url

        self.add_item(discord.ui.Button(label="Розыгрыш", style=discord.ButtonStyle.url, url=self.url))

class GA(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        self.bot.add_view(gab())
        bot.loop.create_task(self.start_giveaway())
        self.runs.start()

    @tasks.loop(seconds=1)
    async def runs(self):
        now = datetime.now(pytz.timezone('Europe/Moscow'))
        target_hour = 12
        target_minute = 0
        if now.hour == target_hour and now.minute == target_minute:
            if 0 <= now.second < 1:
                await self.evday()
                await asyncio.sleep(60 - now.second)

    async def evday(self):
        await self.bot.wait_until_ready()
        now = datetime.now(pytz.timezone('Europe/Moscow')).replace(minute=0, second=0, microsecond=0)

        ends = now + timedelta(hours=12)
        #ends = now + timedelta(minutes=38)

        ends = int(ends.timestamp())
        embed = discord.Embed(title=f"Ежедневный розыгрыш 100 бебр", description=
                              f"**Чтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{ends}:R>**",
                              color=discord.Color.random())
        
        channel = await self.bot.fetch_channel(843475272107163648)
        #channel = await self.bot.fetch_channel(1057539983927410709)

        serv: discord.Guild = self.bot.get_guild(621378615174758421)
        role = serv.get_role(968467508724138014)

        msg = await channel.send(content=role.mention, embed=embed, view=gab())

        giveaway = await db.ga.create_giveaway(msg.id, msg.channel.id, 100, ends)
        await self.start_giveaway(giveaway)

    async def start_giveaway(self, giveaway: Giveaway = False):
        async def timer(giveaway: Giveaway):
            if datetime.now(pytz.timezone('Europe/Moscow')) < giveaway.datetime:
                td = giveaway.datetime - datetime.now(pytz.timezone('Europe/Moscow'))
                for i in range(int(td.total_seconds()), 0, -1):
                    await asyncio.sleep(1)

            users = await giveaway.update_users_list()
            await giveaway.delete()

            channel: discord.TextChannel = await self.bot.fetch_channel(giveaway.channel_id)
            msg: discord.Message = await channel.fetch_message(giveaway.id)

            new_gab = View()
            new_gab.add_item(Button(label="Конкурс обкончен", disabled=True, style=discord.ButtonStyle.success))
            await msg.edit(view=new_gab)

            if len(users) == 0:
                return await msg.reply(embed=discord.Embed(title="К сожалению, никто не поучаствовал в розыгрыше :(", colour=0xf50000))
            
            winner = random.choice(users)
            winner: discord.User = await self.bot.fetch_user(winner)

            await msg.reply(embed=discord.Embed(title="Ура! У нас есть победитель",
                    description=f"Поздравим {winner.mention} с победой, приз уже на его счету!",
                        colour=0x00f51d))
            
            winner_sbp = await db.sbp.get_user(winner.id, True)
            await winner_sbp.increase_balance(giveaway.prize)

            await winner.send("Поздравляю с победой в розыгрыше!", view=view_giv(msg.jump_url))

        if not giveaway:
            await self.bot.wait_until_ready()

            giveaways = await db.ga.get_giveaways()
            if len(giveaways)  ==  0: return

            for x in giveaways:
                self.bot.loop.create_task(timer(x))

        else:
            self.bot.loop.create_task(timer(giveaway))

    @app_commands.command( description="Создать розыгрыш", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.describe(amount="На сколько бебр розыгрыш?", opis="Описание розыгрыша", ends="Когда заканчивается розыгрыш?")
    @app_commands.check(check)
    async def create_giveaway(self, inter: discord.Interaction, amount: int, opis: str, ends: str):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)

        embed = discord.Embed(title=f"Розыгрыш {amount} бебр", description=
                              f"**{opis}\nЧтобы участвовать, нажми кнопку ниже.\nЗаканчивается <t:{ends}:R>**",
                              color=discord.Color.random())
        
        msg = await inter.channel.send(embed=embed, view=gab())

        giveaway = await db.ga.create_giveaway(msg.id, msg.channel.id, amount, int(ends))

        await inter.response.send_message("Успешно!", ephemeral=True)
        await self.start_giveaway(giveaway)

async def setup(bot: commands.Bot):
    await bot.add_cog(GA(bot))
    print("GA cog loaded")