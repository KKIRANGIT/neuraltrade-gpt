from __future__ import annotations


class LightGBMSignalQualityScorer:
    def __init__(self, deployed_accuracy: float = 0.69) -> None:
        self.deployed_accuracy = deployed_accuracy

    def predict(self, features: list[float], signal_type: str = "SCR10") -> float:
        screener_bonus = 0.08 if signal_type in {"SCR10", "SCR30", "SCR13"} else 0.03
        quality = 0.28 + features[0] * 0.3 + features[10] * 0.22 + features[20] * 0.14 + screener_bonus
        return round(min(max(quality, 0.05), 0.98), 4)
