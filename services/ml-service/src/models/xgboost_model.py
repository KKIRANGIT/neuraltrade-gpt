from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class XGBoostPrediction:
    bull_probability: float
    bear_probability: float


class XGBoostDirectionPredictor:
    def __init__(self, deployed_accuracy: float = 0.71) -> None:
        self.deployed_accuracy = deployed_accuracy

    def predict(self, features: list[float]) -> XGBoostPrediction:
        bull = min(max(0.15 + features[0] * 0.5 + features[1] * 0.2 + features[2] * 0.1, 0.0), 0.97)
        bear = min(max(1.0 - bull - 0.05, 0.01), 0.89)
        return XGBoostPrediction(bull_probability=round(bull, 4), bear_probability=round(bear, 4))
