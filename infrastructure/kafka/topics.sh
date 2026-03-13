#!/bin/bash
# Kafka topic creation script
# Topics:
#   raw.ticks — from MDG, partitions=10 (by instrument_id % 10)
#   candles.1m candles.5m candles.15m candles.1h candles.4h candles.1d — partitions=5
#   indicators.snapshot — from Indicator Engine, partitions=10
#   signals.raw — from Screener/Scoreboard Engine, partitions=3
#   signals.validated — from AI Validation, partitions=3
#   signals.updates — lifecycle updates, partitions=3
#   stories.generated — from Story Engine, partitions=3
#   alerts.pending — from Signal Service to Alert Service, partitions=3
# Retention: raw.ticks=1h, candles=7d, signals=30d, stories=7d
