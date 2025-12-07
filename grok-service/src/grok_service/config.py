"""Configuration settings for Grok service."""

from functools import lru_cache

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""

    # xAI API key (no prefix, used directly by xai-sdk)
    xai_api_key: str = Field(default="", description="xAI API key for Grok")

    # Server settings (with GROK_ prefix)
    host: str = Field(default="0.0.0.0", description="Server host")
    port: int = Field(default=8001, description="Server port")
    debug: bool = Field(default=False, description="Enable debug mode")

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
        # Map XAI_API_KEY and GROK_* prefixed vars
        env_prefix="GROK_",
    )

    def __init__(self, **kwargs):
        """Initialize settings, checking for XAI_API_KEY without prefix."""
        super().__init__(**kwargs)
        # xai-sdk looks for XAI_API_KEY directly, so we handle both
        import os

        if not self.xai_api_key:
            self.xai_api_key = os.getenv("XAI_API_KEY", "")


@lru_cache
def get_settings() -> Settings:
    """Get cached settings instance."""
    return Settings()
