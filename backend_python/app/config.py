from collections.abc import Sequence
from functools import lru_cache

from pydantic import Field, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    app_host: str = Field(default="0.0.0.0", alias="APP_HOST")
    app_port: int = Field(default=8000, alias="APP_PORT")
    postgres_host: str = Field(default="localhost", alias="POSTGRES_HOST")
    postgres_port: int = Field(default=5432, alias="POSTGRES_PORT")
    postgres_db: str = Field(default="ocr_bill2sheet", alias="POSTGRES_DB")
    postgres_user: str = Field(default="postgres", alias="POSTGRES_USER")
    postgres_password: str = Field(default="postgres", alias="POSTGRES_PASSWORD")
    database_url: str | None = Field(default=None, alias="DATABASE_URL")

    gemini_api_key: str | None = Field(default=None, alias="GEMINI_API_KEY")
    gemini_api_keys: list[str] = Field(default_factory=list, alias="GEMINI_API_KEYS")
    gemini_model: str = Field(default="gemini-1.5-flash-latest", alias="GEMINI_MODEL")
    gemini_api_url: str = Field(
        default="https://generativelanguage.googleapis.com/v1beta/models",
        alias="GEMINI_API_URL",
    )

    @field_validator("gemini_api_keys", mode="before")
    @classmethod
    def _normalize_gemini_api_keys(cls, value):
        if value in (None, ""):
            return []
        if isinstance(value, str):
            raw_items = value.split(",")
        elif isinstance(value, Sequence):
            raw_items = list(value)
        else:
            raise TypeError("GEMINI_API_KEYS must be a comma-separated string or a sequence of strings")

        keys: list[str] = []
        for item in raw_items:
            candidate = str(item).strip()
            if candidate and candidate not in keys:
                keys.append(candidate)
        return keys

    @property
    def resolved_gemini_api_keys(self) -> list[str]:
        if self.gemini_api_keys:
            return list(self.gemini_api_keys)
        if self.gemini_api_key:
            return [self.gemini_api_key]
        return []

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )

    @property
    def sqlalchemy_database_uri(self) -> str:
        if self.database_url:
            return self.database_url
        return (
            f"postgresql+asyncpg://{self.postgres_user}:{self.postgres_password}"
            f"@{self.postgres_host}:{self.postgres_port}/{self.postgres_db}"
        )


@lru_cache
def get_settings() -> Settings:
    return Settings()
