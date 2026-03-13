use crate::indicators::trend::sma;
use crate::models::Candle;

#[derive(Debug, Clone, Default)]
pub struct VolumeIndicators {
    pub obv: f32,
    pub obv_sma_20: f32,
    pub obv_slope_5: f32,
    pub volume_ratio_20: f32,
    pub cmf_20: f32,
}

pub fn calculate(candles: &[Candle]) -> VolumeIndicators {
    let mut obv_series = Vec::with_capacity(candles.len());
    let mut current_obv = 0.0;
    for window in candles.windows(2) {
        let prev = &window[0];
        let current = &window[1];
        if current.close > prev.close {
            current_obv += current.volume;
        } else if current.close < prev.close {
            current_obv -= current.volume;
        }
        obv_series.push(current_obv);
    }
    let obv = *obv_series.last().unwrap_or(&0.0);
    let obv_sma_20 = sma(&obv_series, 20);
    let obv_slope_5 = linear_slope(&obv_series[obv_series.len().saturating_sub(5)..]);
    let volumes: Vec<f32> = candles.iter().map(|candle| candle.volume).collect();
    let volume_ratio_20 = volumes
        .last()
        .copied()
        .unwrap_or(0.0)
        / sma(&volumes, 20).max(0.0001);
    let cmf_20 = chaikin_money_flow(candles, 20);

    VolumeIndicators {
        obv,
        obv_sma_20,
        obv_slope_5,
        volume_ratio_20,
        cmf_20,
    }
}

fn linear_slope(values: &[f32]) -> f32 {
    if values.len() < 2 {
        return 0.0;
    }
    let mut numerator = 0.0;
    let mut denominator = 0.0;
    let mean_x = (values.len() as f32 - 1.0) / 2.0;
    let mean_y = values.iter().sum::<f32>() / values.len() as f32;
    for (index, value) in values.iter().enumerate() {
        let x = index as f32;
        numerator += (x - mean_x) * (value - mean_y);
        denominator += (x - mean_x).powi(2);
    }
    numerator / denominator.max(0.0001)
}

fn chaikin_money_flow(candles: &[Candle], period: usize) -> f32 {
    let slice = if candles.len() > period {
        &candles[candles.len() - period..]
    } else {
        candles
    };
    let mut mfv_sum = 0.0;
    let mut volume_sum = 0.0;
    for candle in slice {
        let range = (candle.high - candle.low).max(0.0001);
        let multiplier = ((candle.close - candle.low) - (candle.high - candle.close)) / range;
        mfv_sum += multiplier * candle.volume;
        volume_sum += candle.volume;
    }
    mfv_sum / volume_sum.max(0.0001)
}
