use crate::models::Candle;

#[derive(Debug, Clone, Default)]
pub struct TrendIndicators {
    pub ema_9: f32,
    pub ema_20: f32,
    pub ema_50: f32,
    pub ema_200: f32,
    pub sma_20: f32,
    pub sma_50: f32,
    pub rsi_14: f32,
    pub rsi_7: f32,
    pub ema_stack_score: f32,
    pub consecutive_green: f32,
    pub consecutive_red: f32,
    pub adx: f32,
    pub plus_di: f32,
    pub minus_di: f32,
    pub adx_ready: bool,
    pub ema_200_ready: bool,
}

pub fn calculate(candles: &[Candle]) -> TrendIndicators {
    let closes: Vec<f32> = candles.iter().map(|candle| candle.close).collect();
    let ema_9 = ema(&closes, 9);
    let ema_20 = ema(&closes, 20);
    let ema_50 = ema(&closes, 50);
    let ema_200 = ema(&closes, 200);
    let sma_20 = sma(&closes, 20);
    let sma_50 = sma(&closes, 50);
    let rsi_14 = rsi(&closes, 14);
    let rsi_7 = rsi(&closes, 7);
    let current = *closes.last().unwrap_or(&0.0);
    let ema_stack_score = [
        current > ema_9,
        current > ema_20,
        current > ema_50,
        current > ema_200,
    ]
    .iter()
    .filter(|value| **value)
    .count() as f32;
    let consecutive_green = consecutive_bars(candles, true) as f32;
    let consecutive_red = consecutive_bars(candles, false) as f32;
    let (adx, plus_di, minus_di) = adx(candles, 14);

    TrendIndicators {
        ema_9,
        ema_20,
        ema_50,
        ema_200,
        sma_20,
        sma_50,
        rsi_14,
        rsi_7,
        ema_stack_score,
        consecutive_green,
        consecutive_red,
        adx,
        plus_di,
        minus_di,
        adx_ready: candles.len() >= 28,
        ema_200_ready: candles.len() >= 250,
    }
}

pub fn ema(values: &[f32], period: usize) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let k = 2.0 / (period as f32 + 1.0);
    values
        .iter()
        .copied()
        .skip(1)
        .fold(values[0], |ema_value, value| value * k + ema_value * (1.0 - k))
}

pub fn sma(values: &[f32], period: usize) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let slice = if values.len() > period {
        &values[values.len() - period..]
    } else {
        values
    };
    slice.iter().sum::<f32>() / slice.len() as f32
}

pub fn rsi(values: &[f32], period: usize) -> f32 {
    if values.len() <= period {
        return 50.0;
    }
    let mut gains = 0.0;
    let mut losses = 0.0;
    for window in values.windows(2).rev().take(period) {
        let change = window[1] - window[0];
        if change >= 0.0 {
            gains += change;
        } else {
            losses += change.abs();
        }
    }
    if losses == 0.0 {
        return 100.0;
    }
    let rs = gains / losses.max(0.0001);
    100.0 - (100.0 / (1.0 + rs))
}

fn consecutive_bars(candles: &[Candle], green: bool) -> usize {
    let mut count = 0;
    for candle in candles.iter().rev() {
        let is_green = candle.close >= candle.open;
        if is_green == green {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn adx(candles: &[Candle], period: usize) -> (f32, f32, f32) {
    if candles.len() <= period {
        return (0.0, 0.0, 0.0);
    }
    let mut tr_sum = 0.0;
    let mut plus_dm_sum = 0.0;
    let mut minus_dm_sum = 0.0;
    for window in candles.windows(2).rev().take(period) {
        let prev = &window[0];
        let current = &window[1];
        let plus_dm = (current.high - prev.high).max(0.0);
        let minus_dm = (prev.low - current.low).max(0.0);
        let tr = (current.high - current.low)
            .max((current.high - prev.close).abs())
            .max((current.low - prev.close).abs());
        tr_sum += tr;
        plus_dm_sum += plus_dm;
        minus_dm_sum += minus_dm;
    }
    if tr_sum == 0.0 {
        return (0.0, 0.0, 0.0);
    }
    let plus_di = 100.0 * plus_dm_sum / tr_sum;
    let minus_di = 100.0 * minus_dm_sum / tr_sum;
    let dx = 100.0 * ((plus_di - minus_di).abs() / (plus_di + minus_di).max(0.0001));
    (dx, plus_di, minus_di)
}
