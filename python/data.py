import yfinance as yf
import pandas as pd

def load_data(ticker: str, range: str) -> pd.DataFrame:
    df = yf.Ticker(ticker).history(period=range)

    if df.empty:
        raise ValueError(f"No data returned for {ticker}")

    df = df[["Close"]].copy()
    df["returns"] = df["Close"].pct_change()
    df = df.dropna()

    return df
