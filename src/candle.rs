use std::collections::HashMap;

pub struct OHLCV {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub signal: f64,
    pub tp: f64,
    pub sl: f64,
    pub size: f64
}

impl OHLCV { 
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        signal: f64,
        tp: f64,
        sl: f64,
        size: f64
    ) -> OHLCV {
        
        OHLCV {
            open: open,
            high: high,
            low: low,
            close: close,
            volume: volume,
            signal: signal,
            tp: tp,
            sl: sl,
            size: size 
        }
    }
}

pub struct OHLCVViews<'a> {
    pub open: &'a [f64],
    pub high: &'a [f64],
    pub low: &'a [f64],
    pub close: &'a [f64],
    pub volume: &'a [f64],
    pub tp: &'a [f64],
    pub sl: &'a [f64],
    pub size: &'a [f64]
}
