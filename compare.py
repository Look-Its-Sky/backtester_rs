from backtesting import Strategy, Backtest
from backtesting.lib import SignalStrategy 
from numba import njit
import backtest_rs
import pandas as pd
import numpy as np
import pandas_ta as ta
import time 

def run_backtesting_py(df: pd.DataFrame):
    # Backtesting py 
    class Strat(SignalStrategy):
        def init(self):
            super().init()
            self.signal = self.I(lambda: self.data.df.signal)
            self.set_signal(entry_size=self.signal)

    df_copy = df.copy()
    df_copy.rename(columns={
        'open': 'Open',
        'high': 'High',
        'low': 'Low',
        'close': 'Close',
        'volume': 'Volume'
    }, inplace=True)

    print('Running Backtesting py')
    start = time.time()
    Backtest(df_copy, Strat).run()
    end = time.time()

    result = round(end - start, 3)
    print(f'Took {result}s')

    return result 

def run_backtest_rs(df: pd.DataFrame):
    # @njit
    def get_size(o, h, l, c, v):
        return 1.0

    # @njit
    def get_tp_sl(o, h, l, c, v, is_long):
        close = c[-1]
        side = 1 if is_long else -1 

        return {
            'tp': close + side * 0.1,
            'sl': close + side * 0.1
        }

    df.loc[df['signal'] == 1, 'enter_long'] = 1
    df.loc[df['signal'] == -1, 'enter_short'] = 0 #1

    # Convert
    df = df.to_dict()
    df_dict = {}
    for key in df:
        df_dict[key] = list(df[key].values())

    print('Running backtester_rs')
    start = time.time()
    stats = backtester_rs.backtest(
        ohlcv=df_dict,
        starting_cash=1_000,
        starting_candles=100,
        sizing_func=get_size,
        tpsl_func=get_tp_sl,
        long_signal_column_p='enter_long',
        short_signal_column_p='enter_short'
    )
    print(stats)
    end = time.time()

    result = round(end - start, 3)
    print(f'Took {result}s')

    return result 

# Prep
print('Preparing dataframe')
df = pd.read_feather('/home/jude/Nextcloud/Fin_data/crypto/ETH_USDT_USDT-1m-futures.feather')
length = len(df)
rsi = ta.rsi(df['close'], length=14)
df['signal'] = np.where(
    rsi >= 70,
    -1,
    np.where(
        rsi <= 30,
        1,
        0
    )
)
df.dropna(inplace=True)
df.drop(columns={'date'}, inplace=True)

backtest_rs_result = run_backtest_rs(df)
backtesting_py_result = run_backtesting_py(df)

print(f'''
================================================
Over {length:,} rows
Backtester_rs Time: {backtest_rs_result}s
Backtesting.py Time: {backtesting_py_result}s
================================================
''')
