import csv
from binance.client import Client

SYMBOL = 'BTCUSDT'

# No API key/secret needed for this type of call
client = Client()

columns = [
    'open_time', 'open', 'high', 'low', 'close', 'volume',
    'close_time', 'quote_asset_volume', 'number_of_trades',
    'taker_buy_base_asset_volume', 'taker_buy_quote_asset_volume',
    'ignore'
]

klines = client.get_historical_klines(SYMBOL, Client.KLINE_INTERVAL_30MINUTE, "1 Jan, 2022")

with open('output.csv', 'w') as f:
    write = csv.writer(f)
    write.writerow(columns)
    write.writerows(klines)