from __future__ import annotations

from dataclasses import asdict, dataclass

from features.feature_builder import FeatureBuilder
from models.anomaly_model import IsolationForestAnomalyDetector
from models.lightgbm_model import LightGBMSignalQualityScorer
from models.lstm_model import LSTMPatternMatcher
from models.regime_model import RandomForestRegimeClassifier
from models.xgboost_model import XGBoostDirectionPredictor


@dataclass(frozen=True)
class BatchInferenceResult:
    regime: str
    scores: dict[int, dict[str, float | str]]


class BatchInferenceRunner:
    def __init__(self) -> None:
        self.builder = FeatureBuilder()
        self.xgb = XGBoostDirectionPredictor()
        self.lgbm = LightGBMSignalQualityScorer()
        self.lstm = LSTMPatternMatcher()
        self.regime = RandomForestRegimeClassifier()
        self.anomaly = IsolationForestAnomalyDetector()
        self.cache: dict[int, dict[str, float | str]] = {}

    def load_all_models(self) -> dict[str, str]:
        return {
            "xgboost": "loaded",
            "lightgbm": "loaded",
            "lstm": "loaded",
            "regime": "loaded",
            "anomaly": "loaded",
        }

    def run_all_500_stocks(self, instrument_ids: list[int] | None = None) -> BatchInferenceResult:
        ids = instrument_ids or list(range(1001, 1501))
        bundles = self.builder.batch_bundles(ids)
        market_regime = self.regime.predict(
            sector_strength=sum(bundle.sector_strength for bundle in bundles[:10]) / min(len(bundles), 10),
            fii_flow=sum(bundle.fii_flow for bundle in bundles[:10]) / min(len(bundles), 10),
            vix=sum(bundle.vix for bundle in bundles[:10]) / min(len(bundles), 10),
        )
        score_map: dict[int, dict[str, float | str]] = {}
        for bundle in bundles:
            xgb_prediction = self.xgb.predict(bundle.features)
            quality_score = self.lgbm.predict(bundle.features)
            lstm_prediction = self.lstm.predict(bundle.sequence)
            anomaly_prediction = self.anomaly.predict(bundle.features)
            score_map[bundle.instrument_id] = {
                "bull_probability": xgb_prediction.bull_probability,
                "bear_probability": xgb_prediction.bear_probability,
                "quality_score": quality_score,
                "pattern_score": lstm_prediction.pattern_score,
                "next_5d_return_prediction": lstm_prediction.next_5d_return_prediction,
                "anomaly_score": anomaly_prediction.anomaly_score,
                "anomaly_type": anomaly_prediction.anomaly_type,
                "regime": market_regime,
            }
        self.cache = score_map
        return BatchInferenceResult(regime=market_regime, scores=score_map)

    def inference_for_stock(self, instrument_id: int) -> dict[str, float | str]:
        if instrument_id in self.cache:
            return self.cache[instrument_id]
        return self.run_all_500_stocks([instrument_id]).scores[instrument_id]

    def store_to_redis(self) -> dict[int, dict[str, float | str]]:
        return self.cache

    def to_serializable(self) -> dict[str, object]:
        return asdict(self.run_all_500_stocks())
