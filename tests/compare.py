from backtesting import Strategy, Backtest
from backtesting.lib import SignalStrategy 
import backtester_rs
import pandas as pd
import numpy as np
import pandas_ta as ta
import time 

# Prep
df = pd.read_feather('/home/jude/Nextcloud/Fin_data/crypto/ETH_USDT-1m.feather')
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

start = time.time()
Backtest(df_copy, Strat).run()
end = time.time()

backtesting_result = end - start

# Convert
df = df.to_dict()
df_dict = {}
for key in df:
    df_dict[key] = list(df[key].values())

start = time.time()
backtester_rs.backtest(
    ohlcv=df_dict,
    signal_column='signal',
    starting_cash=1_000,
)
end = time.time()

rs_time = end - start

print(f'''
Running on {len(df_copy):,} rows
Backtesting.py Time: {round(backtesting_result, 3)}
Backtester_rs Time: {round(rs_time, 3)}
''')
