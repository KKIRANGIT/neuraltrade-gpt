# HistoricalDataLoader
# Run once at system setup to load historical candles.
# For each of 500 symbols:
#   Fetch 750 daily candles from Fyers API
#   Fetch 750 1H candles
#   Fetch 750 15M candles
#   Fetch 500 5M candles
#   Fetch 750 1M candles (last 3 days only)
#   Upsert into TimescaleDB candles table
# Progress bar, error handling, retry on rate limit
# Estimated time: 20-30 minutes for all 500 stocks
# Run: python load_historical.py --symbols nifty500 --timeframes all
