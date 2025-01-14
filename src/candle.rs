pub struct OHLCV {
    // pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub signal: f64,
}

impl OHLCV { 
    pub fn new(
        // time: i64,
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        signal: f64
    ) -> OHLCV {
        
        OHLCV {
            // time,
            open,
            high,
            low,
            close,
            volume,
            signal
        }
    }
}