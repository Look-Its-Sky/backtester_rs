use pyo3::prelude::*;
use pyo3::types::PyString;
use std::collections::HashMap;
use polars::prelude::*;
mod candle;

#[pyfunction]
fn backtest(ohlcv: HashMap<String, Vec<f64>>, signal_column: &str, starting_cash: f64) -> PyResult<HashMap<String, f64>> {
    let mut result = HashMap::new();
    let mut cash = starting_cash;

    let mut winning_trades = 0;
    let mut losing_trades = 0;

    for n in 0..ohlcv.get("volume").unwrap().len() {
        // let time = ohlcv.get("time").unwrap()[n] as i64;
        let open = ohlcv.get("open").unwrap()[n];
        let high = ohlcv.get("high").unwrap()[n];
        let low = ohlcv.get("low").unwrap()[n];
        let close = ohlcv.get("close").unwrap()[n];
        let volume = ohlcv.get("volume").unwrap()[n];
        let signal = ohlcv.get(signal_column).unwrap()[n];

        let candle = candle::OHLCV::new(open, high, low, close, volume, signal);

        if candle.signal == 1.0 {
            cash -= candle.close;
        } else if candle.signal == -1.0 {
            cash += candle.close;
        }
    }

    //result.insert()
    result.insert("Ending cash".to_string(), cash);

    Ok(result)
}

#[pymodule]
fn backtester_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(backtest, m)?)?;
    Ok(())
}
