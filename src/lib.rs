use pyo3::prelude::*;
use pyo3::types::PyString;
use std::collections::HashMap;
use pyo3::types::*;
mod candle;
mod trade;

/*
 * Enter signals are positive 
 * Exit signals are negative
 *
 * EX: 
 * short = 1 for enter 
 * short = -1 for exit 
 */

#[pymodule]
fn backtest_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_backtest, m)?)?;
    Ok(())
}

fn get_size(
    func: &Bound<'_, PyAny>,
    views: &candle::OHLCVViews,
    current_idx: usize,
) -> PyResult<f64> {
    let o = &views.open[..=current_idx];
    let h = &views.high[..=current_idx];
    let l = &views.low[..=current_idx];
    let c = &views.close[..=current_idx];
    let v = &views.volume[..=current_idx];

    let args = (o, h, l, c, v);
    func.call1(args)?.extract()
}

#[deprecated(note="This should be broken dumbass")]
fn get_tpsl(
    func: &Bound<'_, PyAny>,
    views: &candle::OHLCVViews,
    current_idx: usize,
    is_long: bool
) -> PyResult<HashMap<String, f64>> {
    let o = &views.open[..=current_idx];
    let h = &views.high[..=current_idx];
    let l = &views.low[..=current_idx];
    let c = &views.close[..=current_idx];
    let v = &views.volume[..=current_idx];

    let args = (o, h, l, c, v, is_long);
    let result: HashMap<String, f64> = func.call1(args)?.extract()?;
    
    // Pre-allocate with capacity
    let mut tpsl_map = HashMap::with_capacity(2);
    
    // Use insert_or_update for better performance
    if let Some(tp) = result.get("tp") {
        tpsl_map.insert("tp".to_string(), *tp);
    }
    if let Some(sl) = result.get("sl") {
        tpsl_map.insert("sl".to_string(), *sl);
    }
    
    Ok(tpsl_map)
}

#[pyfunction]
fn rust_backtest(
    ohlcv: HashMap<String, Vec<f64>>, 
    starting_cash: f64, 
    starting_candles: usize,
    long_signal_column_p: Option<&str>,
    short_signal_column_p: Option<&str>,
    exclusive_p: Option<bool>
) -> PyResult<HashMap<String, f64>> {

    let long_signal_column: &str = long_signal_column_p.unwrap_or("long");
    let short_signal_column: &str = short_signal_column_p.unwrap_or("short");
    let exclusive = exclusive_p.unwrap_or(true);

    let mut result = HashMap::with_capacity(5);  
    let mut trades: Vec<trade::Trade> = Vec::with_capacity(1000000); // About 16mb allocated

    let views = candle::OHLCVViews {
        open: &ohlcv.get("open").expect("The open column should be present"),
        high: &ohlcv.get("high").expect("The high column should be present"),
        low: &ohlcv.get("low").expect("The low column should be present"),
        close: &ohlcv.get("close").expect("The close column should be present"),
        volume: &ohlcv.get("volume").expect("The volume column should be present"),
        tp: &ohlcv.get("tp").expect("The tp column should be present with at least 0s (no NaNs)"),
        sl: &ohlcv.get("sl").expect("The sl column should be present with at least 0s (no NaNs)"),
        size: &ohlcv.get("size").expect("The size column should be present")
    };

    let long_signals = ohlcv.get(long_signal_column)
        .unwrap_or_else(|| panic!("The {} column should be present", long_signal_column));
    let short_signals = ohlcv.get(short_signal_column)
        .unwrap_or_else(|| panic!("The {} column should be present", short_signal_column));

    let mut entry_p = -1.0; // No entry 
    let mut cash = starting_cash.clone();
    let mut is_long = false; 
    let mut position: f64 = 0.0;
    let mut current_tp: f64 = -1.0;
    let mut current_sl: f64 = -1.0;

    for n in starting_candles..views.volume.len() {
        // Current data
        let open = views.open[n];
        let high = views.high[n];
        let low = views.low[n];
        let close = views.close[n];
        let tp = views.tp[n];
        let sl = views.sl[n];
        let size = views.size[n];
        
        // Current signals
        let enter_long = long_signals[n] > 0.0 && !(short_signals[n] > 0.0);
        let enter_short = short_signals[n] > 0.0 && !(long_signals[n] > 0.0);
        let exit_long = long_signals[n] < 0.0 || high >= current_tp || low <= current_sl;
        let exit_short = short_signals[n] < 0.0 || low <= current_tp || high >= current_sl;
        let can_trade = position == 0.0; // || !exclusive; TODO: fix none exclusive trade support 

        // Entries
        if (enter_long || enter_short) && can_trade {
            let size = 10.0; 

            if enter_long {
                current_tp = tp.clone();
                current_sl = sl.clone(); 

                cash -= size * close;
                position += size;
                entry_p = close;
                is_long = true;
            } 

            if enter_short {
                current_tp = tp.clone();
                current_sl = sl.clone(); 

                cash += size * close;
                position -= size;
                entry_p = close;
                is_long = false;
            }

        }

        // Exits
        if position != 0.0 && (exit_long || exit_short) {
            if exit_long && is_long {
                position = 0.0;
                is_long = false;

                trades.push(trade::Trade::new(
                    entry_p,
                    close,
                    is_long,
                ));

            } else if exit_short && !is_long {
                position = 0.0;
                is_long = false;
                
                trades.push(trade::Trade::new(
                    entry_p,
                    close,
                    is_long,
                ));
            }
        }
    }

    for n in trades.iter() {
        cash += n.get_pnl();
    }
    
    result.insert("total_trades".to_string(), trades.len() as f64);
    result.insert("end_cash".to_string(), cash);
    result.insert("end_index".to_string(), views.volume.len() as f64);

    result.insert("first_trade_entry_p".to_string(), trades[0].get_entry_p());
    result.insert("first_trade_exit_p".to_string(), trades[0].get_exit_p());
    result.insert("first_trade_pnl".to_string(), trades[0].get_pnl());

    result.insert("starting_cash".to_string(), starting_cash);
    Ok(result)

    // TODO: also return trades so we can plot a profit curve.... this doesnt have to be done in
    // Rust
}
