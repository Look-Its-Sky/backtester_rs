import backtester_rs
import pandas as pd 
import numpy as np
import pandas_ta as ta
from pprint import pprint

df = pd.read_feather('/home/jude/Nextcloud/Fin_data/crypto/ETH_USDT_USDT-1m-futures.feather')
# df.drop(columns={'date'}, inplace=True)
# df.rename(columns={'date': 'time'}, inplace=True)
df.drop(columns={'date'}, inplace=True)

# df = np.log(df)
# df = df[-100:]

rsi = ta.rsi(df['close'], length=14)
df['signal'] = np.where(
    (rsi > 70),
    1,
    np.where(
        (rsi < 30),
        -1,
        0
    )
)

df.dropna(inplace=True)

# Convert
df = df.to_dict()
df_dict = {}
for key in df:
    df_dict[key] = list(df[key].values())
    
results = backtester_rs.backtest(
    ohlcv=df_dict,
    signal_column='signal',
    starting_cash=1_000,
)

# print(f'''
# ===========
# Results
# Total Trades: {results['total_trades']}
# Winning Trades: {results['winning_trades']}
# Losing Trades: {results['losing_trades']}
# Win Rate: {results['win_rate']}
# Profit Factor: {results['profit_factor']}
# ''')

print(results)
