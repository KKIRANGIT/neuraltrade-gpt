from __future__ import annotations

from fastapi import FastAPI, HTTPException

from inference.batch_inference import BatchInferenceRunner
from training.trainer import ModelTrainer

app = FastAPI(title="NeuralTrade ML Service", version="0.1.0")

runner = BatchInferenceRunner()
trainer = ModelTrainer()
latest_batch = runner.run_all_500_stocks()


@app.get("/ml/health")
def health() -> dict[str, str]:
    return {"status": "ok"}


@app.post("/ml/inference/{instrument_id}")
def run_inference(instrument_id: int) -> dict[str, float | str]:
    if instrument_id <= 0:
        raise HTTPException(status_code=400, detail="instrument_id must be positive")
    return runner.inference_for_stock(instrument_id)


@app.get("/ml/regime")
def get_regime() -> dict[str, str]:
    return {"regime": latest_batch.regime}


@app.get("/ml/accuracy")
def get_accuracy() -> dict[str, object]:
    report = trainer.generate_training_report()
    return {
        "sample_count": report.sample_count,
        "model_accuracy": report.model_accuracy,
        "promoted_models": report.promoted_models,
    }


@app.post("/ml/retrain")
def retrain_models() -> dict[str, object]:
    report = trainer.generate_training_report()
    return {
        "status": "retrained",
        "promoted_models": report.promoted_models,
        "model_accuracy": report.model_accuracy,
    }
