from __future__ import annotations


class RandomForestRegimeClassifier:
    regimes = (
        "BULL_TRENDING",
        "BULL_VOLATILE",
        "SIDEWAYS_CALM",
        "SIDEWAYS_CHOPPY",
        "BEAR_VOLATILE",
        "BEAR_TRENDING",
        "CRISIS",
    )

    def __init__(self, deployed_accuracy: float = 0.79) -> None:
        self.deployed_accuracy = deployed_accuracy

    def predict(self, sector_strength: float, fii_flow: float, vix: float) -> str:
        if vix > 23:
            return "CRISIS"
        if vix > 18 and fii_flow < 0:
            return "BEAR_VOLATILE"
        if sector_strength > 0.62 and fii_flow > 300 and vix < 16:
            return "BULL_TRENDING"
        if sector_strength > 0.55:
            return "BULL_VOLATILE"
        if vix < 14:
            return "SIDEWAYS_CALM"
        return "SIDEWAYS_CHOPPY"
