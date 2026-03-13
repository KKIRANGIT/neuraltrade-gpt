// StockController — REST endpoints for frontend
// GET /api/stocks — list of all 500 stocks with basic scoreboard
// GET /api/stocks/{symbol} — full stock data (scoreboard + story + signal)
// GET /api/stocks/{symbol}/story — generated story (rule-based or AI)
// GET /api/stocks/{symbol}/story/deep — on-demand Claude deep analysis
// GET /api/stocks/{symbol}/indicators — full indicator snapshot
// GET /api/stocks/{symbol}/sr-levels — support/resistance zones
// GET /api/stocks/screener/{id} — stocks matching specific screener
// GET /api/stocks/top/bull — top 20 bull scores
// GET /api/stocks/top/bear — top 20 bear scores
