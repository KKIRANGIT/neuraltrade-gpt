pub fn classify_tier(bull_total: u8, bear_total: u8, daily_bull: u8, hourly_bull: u8, volume_ratio: f32, close_above_ema200: bool) -> String {
    if daily_bull >= 50
        && hourly_bull >= 45
        && bull_total >= 80
        && bull_total >= bear_total.saturating_add(15)
        && volume_ratio >= 1.0
        && close_above_ema200
    {
        return "ELITE".to_string();
    }
    if daily_bull >= 50
        && hourly_bull >= 45
        && bull_total >= 70
        && bull_total >= bear_total.saturating_add(15)
        && volume_ratio >= 1.0
        && close_above_ema200
    {
        return "HIGH".to_string();
    }
    if bull_total >= 60 {
        return "MEDIUM".to_string();
    }
    "WATCHLIST".to_string()
}
