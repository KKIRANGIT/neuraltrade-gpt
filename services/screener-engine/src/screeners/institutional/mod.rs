// SCR22: FII + Price Action Confluence
//   fii_net_buyers_3d = true, fii_net_amount_cr > 500
//   Price trigger: dc_upper_20 OR supertrend flip OR ema_20 cross
//   Sector performance positive, market_cap_cr > 5000
//   cooldown: 24h | valid: 5 days
//
// SCR24: Sector Rotation Leader Entry
//   sector_performance > 1.5% today, sector was underperforming last week
//   Stock is sector leader (roc_12 > sector average)
//   volume >= 2.0x, close > ema_50
//   cooldown: 24h | valid: 3 days
//
// SCR25: Earnings Gap Hold and Continuation
//   Gap up 3%+ on earnings, gap held 3+ days (low > gap_low * 0.99)
//   Consolidation after gap (tight BB), continuation trigger
//   volume >= 1.8x on continuation bar
//   cooldown: 48h | valid: 5 days
