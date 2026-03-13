use crate::models::TimeframeScore;

pub fn aggregate_scores(scores: &[TimeframeScore]) -> (u8, u8) {
    let weight = |timeframe: &str| -> f32 {
        match timeframe {
            "1M" => 0.05,
            "5M" => 0.10,
            "15M" => 0.20,
            "1H" => 0.30,
            "1D" => 0.35,
            _ => 0.0,
        }
    };

    let mut bull_total = scores
        .iter()
        .map(|score| score.bull as f32 * weight(&score.timeframe))
        .sum::<f32>();
    let bear_total = scores
        .iter()
        .map(|score| score.bear as f32 * weight(&score.timeframe))
        .sum::<f32>();

    let aligned = scores.iter().filter(|score| score.bull >= 50).count();
    bull_total += match aligned {
        5 => 15.0,
        4 => 10.0,
        3 => 5.0,
        _ => 0.0,
    };

    (bull_total.min(100.0) as u8, bear_total.min(100.0) as u8)
}
