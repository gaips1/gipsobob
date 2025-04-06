from datetime import datetime, timedelta
import os
from typing import Dict, List
from discord.ext import commands, tasks
import discord
import asyncio
from discord import app_commands
import pytz
from db.database_instance import db
from db.models.quests import Quest
import ext
import tempfile

class Quests(commands.Cog):
    def __init__(self, bot: commands.Bot):
        self.bot: commands.Bot = bot
        self.add_quest.start()
        self.check_expired_quests.start()
        bot.add_view(ext.turnoff1())
        bot.add_view(ext.turnon1())
        bot.add_view(ext.turnoff2())
        bot.add_view(ext.turnon2())

    @app_commands.command(description="Квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def quests(self, inter: discord.Interaction):
        quests = await db.quests.get_user_quests(inter.user.id)

        if not quests:
            embed = discord.Embed(
                title="Квесты",
                color=discord.Color.random(),
                description="У вас нет доступных квестов."
            )

        else:
            embed = discord.Embed(title="Квесты", color=discord.Color.random())

            for quest in quests:
                embed.add_field(
                    name=quest.name,
                    value=
                    f"{quest.description}\nВыполнено - {quest.progress}/{quest.progress_max}\n" \
                    f"Награда: {quest.reward} бебр\nИстекает: {"<t:" + str(quest.ends) + ":R>" if quest.ends else "Никогда"}",
                    inline=False
                )

        await inter.response.send_message(embed=embed, ephemeral=True)

    @app_commands.command(description="Выполненные квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def completed_quests(self, inter: discord.Interaction):
        quests = await db.quests.get_user_quests(inter.user.id, "completed")
        
        if not quests:
            return await inter.response.send_message(
                "У вас нет выполненных квестов.",
                ephemeral=True
            )

        temp_file = None
        try:
            temp_file = tempfile.NamedTemporaryFile(mode='w', encoding='utf-8', suffix='.txt', delete=False)
            temp_file_path = temp_file.name
            temp_file.close()

            with open(temp_file_path, 'w', encoding='utf-8') as f:
                f.write(f"=== Выполненные квесты ===\nВсего: {len(quests)}\n\n")
                
                for quest in quests:
                    f.write(f"Квест: {quest.name}\n")
                    f.write(f"Описание: {quest.description}\n")
                    f.write(f"Награда: {quest.reward} бебр\n")
                    f.write(f"Завершён: {quest.datetime.strftime('%d.%m.%Y %H:%M') if quest.ends else 'Нет даты'}\n")
                    f.write("-" * 40 + "\n")

            await inter.response.send_message(
                content="Ваши выполненные квесты:", 
                ephemeral=True, 
                file=discord.File(temp_file_path, filename="completed_quests.txt")
            )
        finally:
            if temp_file and os.path.exists(temp_file_path):
                try:
                    os.unlink(temp_file_path)
                except:
                    pass

    @app_commands.command(description="Просроченные квесты", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def expired_quests(self, inter: discord.Interaction):
        quests = await db.quests.get_user_quests(inter.user.id, "expired")
        
        if not quests:
            return await inter.response.send_message(
                "У вас нет просроченных квестов.",
                ephemeral=True
            )

        temp_file = None
        try:
            temp_file = tempfile.NamedTemporaryFile(mode='w', encoding='utf-8', suffix='.txt', delete=False)
            temp_file_path = temp_file.name
            temp_file.close()

            with open(temp_file_path, 'w', encoding='utf-8') as f:
                f.write(f"=== Просроченные квесты ===\nВсего: {len(quests)}\n\n")
                
                for quest in quests:
                    f.write(f"Квест: {quest.name}\n")
                    f.write(f"Описание: {quest.description}\n")
                    f.write(f"Награда: {quest.reward} бебр\n")
                    f.write(f"Истёк: {quest.datetime.strftime('%d.%m.%Y %H:%M') if quest.ends else 'Нет даты'}\n")
                    f.write("-" * 40 + "\n")

            await inter.response.send_message(
                content="Ваши просроченные квесты:", 
                ephemeral=True, 
                file=discord.File(temp_file_path, filename="expired_quests.txt")
            )
        finally:
            if temp_file and os.path.exists(temp_file_path):
                try:
                    os.unlink(temp_file_path)
                except:
                    pass

    @app_commands.command(description="Дать рандомный квест", )
    @app_commands.allowed_contexts(guilds=True, dms=True, private_channels=True)
    @app_commands.allowed_installs(guilds=True, users=True)
    @app_commands.check(ext.check)
    async def add_random_quest(self, inter: discord.Interaction, user: discord.User = None):
        if inter.user.id != 449882524697493515: return await inter.response.send_message("Недостаточно прав", ephemeral=True)
        await inter.response.send_message("Начинаю выдавать...", ephemeral=True)
        await ext.add_random_quest(user.id)
        await inter.edit_original_response(content="Успешно!")

    @tasks.loop(minutes=1)
    async def check_expired_quests(self):
        quests = await db.quests.get_quests("active")
        now = datetime.now(pytz.timezone('Europe/Moscow'))
        
        quests_by_user: Dict[str, List[Quest]] = dict()

        for quest in quests:
            if quest.ends and now >= quest.datetime:
                if quest.user_id not in quests_by_user:
                    quests_by_user[quest.user_id] = []
                quests_by_user[quest.user_id].append(quest)
        
        for user_id, user_quests in quests_by_user.items():
            for quest in user_quests:
                if quest.type == "active":
                    try:
                        db_user = await db.users.get_user(user_id)

                        await quest.mark_as_expired()

                        if db_user.ended_quests_notifications:
                            user = await ext.get_or_fetch_user(user_id)
                            await user.send(embed=discord.Embed(
                                title=f"Квест '{quest.name}' истёк",
                                description="Увы, время выполнения квеста истекло.",
                                color=discord.Color.random()
                            ), view=ext.turnoff1())
                    except:
                        break

    @tasks.loop()
    async def add_quest(self):
        now = datetime.now(pytz.timezone('Europe/Moscow'))
        
        target_hour = 12
        target_minute = 0
        
        target_time = now.replace(
            hour=target_hour,
            minute=target_minute,
            second=0,
            microsecond=0
        )

        if now > target_time:
            target_time = target_time + timedelta(days=1)

        wait_seconds = (target_time - now).total_seconds()
        await asyncio.sleep(wait_seconds)

        await ext.add_random_quest()

async def setup(bot: commands.Bot):
    await bot.add_cog(Quests(bot))
    print("Quests cog loaded")
