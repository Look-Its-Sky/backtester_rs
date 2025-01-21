import pandas as pd
import numpy as np
# import backtest_rs
import pandas_ta as ta
from pprint import pprint
from binding import pybacktest

df = pd.read_feather('/home/jude/Nextcloud/Fin_data/crypto/BTC_USDT-1m.feather')
print(f'Loaded df with {len(df)} rows')

df.drop(columns=['date'], inplace=True)

# Simple RSI strategy with Stop, TP and sizing
df['rsi'] = ta.rsi(df['close'], 14)
df['long'] = (df['rsi'] < 30).astype(int)

df['short'] = 0 # Disable shorting

def sizing_func(df):
    return 1.0

# EXAMPLE: tp at +0.1% and sl at -0.1% 

def tp(df, is_long):
    close = df['close']
    side = np.where(df['long'], 1, -1)

    return close + side * 1.001

def sl(df, is_long):
    close = df['close'] 
    side = np.where(df['long'], 1, -1) 

    return close * side * 0.999

result = pybacktest(
    df=df,
    starting_cash = 10_000,
    starting_candles = 0,
    sizing_func = sizing_func,
    tp_func = tp,
    sl_func = sl,
    long_column = 'long',
    short_column = 'short',
    do_log = True
)

pprint(result)
