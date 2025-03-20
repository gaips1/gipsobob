import os
from dotenv import load_dotenv
from .database import DatabaseManager

load_dotenv()

db = DatabaseManager(os.environ.get("DATABASE_URL"))