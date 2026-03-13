from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import os


@dataclass(frozen=True)
class Settings:
    model_dir: Path = Path(os.getenv("MODEL_DIR", "models"))
    timescaledb_url: str = os.getenv(
        "TIMESCALEDB_URL", "postgresql://neuraltrade:neuraltrade@localhost:5432/neuraltrade"
    )
    redis_url: str = os.getenv("REDIS_URL", "redis://localhost:6379/0")
    training_lookback_years: int = int(os.getenv("TRAINING_LOOKBACK_YEARS", "3"))
    feature_count: int = int(os.getenv("FEATURE_COUNT", "47"))
    lstm_sequence_length: int = int(os.getenv("LSTM_SEQUENCE_LENGTH", "60"))
    batch_size: int = int(os.getenv("BATCH_SIZE", "64"))
    min_accuracy_to_save: float = float(os.getenv("MIN_ACCURACY_TO_SAVE", "0.62"))


settings = Settings()
