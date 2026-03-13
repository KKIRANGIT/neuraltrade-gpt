from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class AnomalyPrediction:
    anomaly_score: float
    anomaly_type: str


class IsolationForestAnomalyDetector:
    def __init__(self, deployed_accuracy: float = 0.64) -> None:
        self.deployed_accuracy = deployed_accuracy

    def predict(self, features: list[float]) -> AnomalyPrediction:
        volume_component = features[15]
        price_component = features[5]
        divergence = abs(volume_component - price_component)
        anomaly_type = "NORMAL"
        if divergence > 0.35 and volume_component > price_component:
            anomaly_type = "VOLUME"
        elif divergence > 0.35:
            anomaly_type = "DIVERGENCE"
        elif price_component > 0.82:
            anomaly_type = "PRICE"
        return AnomalyPrediction(anomaly_score=round(min(divergence + 0.12, 0.98), 4), anomaly_type=anomaly_type)
