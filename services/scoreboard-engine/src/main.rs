use scoreboard_engine::aggregator::aggregate_scores;
use scoreboard_engine::eligibility::classify_tier;
use scoreboard_engine::models::{ScoreInput, ScoreboardEntry};
use scoreboard_engine::scorer::score_timeframe;

fn main() {
    let inputs = vec![
        ScoreInput {
            timeframe: "15M".to_string(),
            close: 2980.0,
            ema_9: 2970.0,
            ema_20: 2961.0,
            ema_50: 2930.0,
            ema_200: 2748.0,
            supertrend_direction: 1,
            rsi_14: 62.0,
            rsi_7: 66.0,
            macd_histogram: 1.8,
            prev_macd_histogram: 1.2,
            roc_12: 3.1,
            volume_ratio: 2.0,
            obv_slope_5: 1.4,
            cmf_20: 0.11,
            vwap_side: 1,
            bb_squeeze: false,
            squeeze_released_up: true,
            dc_position_20: 0.81,
            adx: 28.0,
            plus_di: 32.0,
            minus_di: 14.0,
        },
        ScoreInput {
            timeframe: "1H".to_string(),
            close: 2980.0,
            ema_9: 2968.0,
            ema_20: 2955.0,
            ema_50: 2922.0,
            ema_200: 2760.0,
            supertrend_direction: 1,
            rsi_14: 61.0,
            rsi_7: 64.0,
            macd_histogram: 1.4,
            prev_macd_histogram: 1.0,
            roc_12: 2.6,
            volume_ratio: 1.8,
            obv_slope_5: 1.1,
            cmf_20: 0.09,
            vwap_side: 1,
            bb_squeeze: false,
            squeeze_released_up: false,
            dc_position_20: 0.76,
            adx: 26.0,
            plus_di: 29.0,
            minus_di: 16.0,
        },
        ScoreInput {
            timeframe: "1D".to_string(),
            close: 2980.0,
            ema_9: 2950.0,
            ema_20: 2898.0,
            ema_50: 2802.0,
            ema_200: 2620.0,
            supertrend_direction: 1,
            rsi_14: 59.0,
            rsi_7: 62.0,
            macd_histogram: 1.0,
            prev_macd_histogram: 0.7,
            roc_12: 5.2,
            volume_ratio: 1.6,
            obv_slope_5: 1.8,
            cmf_20: 0.15,
            vwap_side: 1,
            bb_squeeze: false,
            squeeze_released_up: false,
            dc_position_20: 0.88,
            adx: 31.0,
            plus_di: 34.0,
            minus_di: 12.0,
        },
    ];

    let timeframe_scores = inputs.iter().map(score_timeframe).collect::<Vec<_>>();
    let (bull_total, bear_total) = aggregate_scores(&timeframe_scores);
    let tier = classify_tier(
        bull_total,
        bear_total,
        timeframe_scores[2].bull,
        timeframe_scores[1].bull,
        1.8,
        true,
    );
    let entry = ScoreboardEntry {
        instrument_id: 3045,
        symbol: "RELIANCE".to_string(),
        bull_total,
        bear_total,
        tier,
        timeframe_scores,
    };

    println!(
        "scoreboard-engine ranked {} with bull score {} and tier {}",
        entry.symbol, entry.bull_total, entry.tier
    );
}
