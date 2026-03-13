from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class LSTMPrediction:
    pattern_score: float
    next_5d_return_prediction: float


class LSTMPatternMatcher:
    def __init__(self, deployed_accuracy: float = 0.66) -> None:
        self.deployed_accuracy = deployed_accuracy

    def predict(self, sequence: list[list[float]]) -> LSTMPrediction:
        momentum = sum(row[2] + row[3] for row in sequence[-10:]) / 20.0
        trend = sum(row[0] for row in sequence[-15:]) / 15.0
        pattern_score = min(max((momentum + trend) * 70.0, 15.0), 99.0)
        return_prediction = round((momentum - 0.45) * 12.0, 4)
        return LSTMPrediction(pattern_score=round(pattern_score, 2), next_5d_return_prediction=return_prediction)
