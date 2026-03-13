from __future__ import annotations

from dataclasses import dataclass
from datetime import date, timedelta
from math import sin
from typing import Iterable

from config.config import settings


@dataclass(frozen=True)
class FeatureBundle:
    instrument_id: int
    features: list[float]
    sequence: list[list[float]]
    sector_strength: float
    fii_flow: float
    vix: float


class FeatureBuilder:
    """Builds deterministic feature vectors for the seeded reference platform."""

    def __init__(self) -> None:
        self.feature_count = settings.feature_count
        self.sequence_length = settings.lstm_sequence_length

    def fetch_features_for_stock(self, instrument_id: int) -> list[float]:
        base = instrument_id % 100
        feature_vector = [
            round(((base + index * 7) % 101) / 100.0, 4)
            for index in range(self.feature_count)
        ]
        feature_vector[0] = round(0.38 + (base / 200.0), 4)
        feature_vector[1] = round(0.62 - (base / 350.0), 4)
        feature_vector[2] = round(0.45 + sin(instrument_id / 50.0) * 0.1, 4)
        return feature_vector

    def build_sequence_for_lstm(self, instrument_id: int) -> list[list[float]]:
        base = (instrument_id % 37) / 100.0
        sequence: list[list[float]] = []
        for offset in range(self.sequence_length):
            sequence.append(
                [
                  round(base + offset * 0.002, 4),
                  round(0.55 + sin(offset / 7.0) * 0.1, 4),
                  round(0.48 + sin(offset / 4.0) * 0.15, 4),
                  round(0.35 + sin(offset / 8.0) * 0.14, 4),
                  round(0.5 + offset / 180.0, 4),
                  round(0.42 + sin(offset / 11.0) * 0.08, 4),
                  round(0.38 + sin(offset / 9.0) * 0.11, 4),
                  round(0.27 + sin(offset / 13.0) * 0.12, 4),
                ]
            )
        return sequence

    def build_feature_bundle(self, instrument_id: int) -> FeatureBundle:
        features = self.fetch_features_for_stock(instrument_id)
        sequence = self.build_sequence_for_lstm(instrument_id)
        sector_strength = round(0.4 + (instrument_id % 19) / 30.0, 4)
        fii_flow = round(250 + (instrument_id % 11) * 65.0, 2)
        vix = round(12.5 + (instrument_id % 9) * 0.6, 2)
        return FeatureBundle(
            instrument_id=instrument_id,
            features=features,
            sequence=sequence,
            sector_strength=sector_strength,
            fii_flow=fii_flow,
            vix=vix,
        )

    def build_training_dataset(
        self, start_date: date, end_date: date
    ) -> tuple[list[list[float]], list[int]]:
        samples: list[list[float]] = []
        labels: list[int] = []
        current = start_date
        while current <= end_date:
            instrument_id = 1000 + current.toordinal() % 500
            features = self.fetch_features_for_stock(instrument_id)
            samples.append(features)
            future_bias = (current.toordinal() % 9) / 20.0
            labels.append(1 if features[0] + future_bias > 0.62 else 0)
            current += timedelta(days=1)
        return samples, labels

    def batch_bundles(self, instrument_ids: Iterable[int]) -> list[FeatureBundle]:
        return [self.build_feature_bundle(instrument_id) for instrument_id in instrument_ids]
