from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(env_prefix="DECOMPROOF_", env_file=".env", extra="ignore")

    database_url: str = "sqlite:///./decomproof.db"
    artifact_root: str = ".decomproof/artifacts"
    max_proof_bytes: int = 5_000_000


settings = Settings()
