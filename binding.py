import pandas as pd
import numpy as np
import time
import backtest_rs as btrs

def pybacktest(
    df: pd.DataFrame,
    sizing_func: callable,
    tp_func: callable, 
    sl_func: callable,
    long_column: str = 'long',
    short_column: str = 'short',
    starting_cash: float = 10_000,
    starting_candles: int = 0,
    do_log: bool = False,
    exclusive: bool = True
) -> dict:
    start = time.time()
    print('Running backtest')

    # Prepare
    if 'timestamp' in df.columns:
        df.drop(columns={'timestamp'}, inplace=True)

    if 'date' in df.columns:
        df.drop(columns={'date'}, inplace=True)

    if do_log:
        df['open'] = pd.Series(np.log(df['open']), dtype='float64')
        df['high'] = pd.Series(np.log(df['high']), dtype='float64')
        df['low'] = pd.Series(np.log(df['low']), dtype='float64') 
        df['close'] = pd.Series(np.log(df['close']), dtype='float64') 
    
    cols = ['open', 'high', 'low', 'close', long_column, short_column, 'tp', 'sl', 'size']
    if 'volume' in df.columns:
        cols.append('volume')

    # Embed tpsl / size into dataframe
    df['tp'] = 0
    df['sl'] = 0
    df['size'] = 0

    # NOTE: this might not work this is a dogshit implementation imo
    long_cond = (df[long_column] == True) & (df[short_column] == False)
    short_cond = (df[short_column] == True) & (df[long_column] == False)

    df.loc[long_cond, 'sl'] = sl_func(df, df[long_column].astype(bool))
    df.loc[short_cond, 'sl'] = sl_func(df, ~df[short_column].astype(bool))

    df.loc[long_cond, 'tp'] = tp_func(df, df[long_column].astype(bool)) 
    df.loc[short_cond, 'tp'] = tp_func(df, ~df[short_column].astype(bool))
    df.loc[long_cond ^ short_cond, 'size'] = sizing_func(df)

    # Convert dataframe to primitive
    df = df[cols]
    df = df.to_dict()  
    df_dict = {} 

    for key in df:
        df_dict[key] = list(df[key].values())

    # Actual backtest
    result = btrs.rust_backtest(
        ohlcv = df_dict,
        starting_cash=starting_cash,
        starting_candles=starting_candles,
        long_signal_column_p=long_column,
        short_signal_column_p=short_column,
        exclusive_p=exclusive
    )

    end = time.time()
    print('Took' , round(end - start, 3), 'seconds')

    return result
