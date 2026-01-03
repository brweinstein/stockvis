# Statistical Features in StockVis

## Overview

The StockVis statistical analysis module computes key metrics for financial time series data, providing insights into stock performance, risk, and return characteristics. All statistics are computed on daily returns.

## Data Preparation

Given a time series of closing prices $\{P_t\}_{t=1}^{n}$, we compute the daily returns as:

$$r_t = \frac{P_t - P_{t-1}}{P_{t-1}} = \frac{P_t}{P_{t-1}} - 1$$

where $r_t$ represents the simple return at time $t$.

## Statistical Metrics

### Mean Return

The arithmetic mean of daily returns provides the average daily performance:

$$\mu = \frac{1}{n} \sum_{t=1}^{n} r_t$$

This metric indicates the expected daily return. A positive $\mu$ suggests upward price momentum over the period, while a negative value indicates declining prices.

### Volatility

Volatility is measured as the standard deviation of returns, quantifying the dispersion of returns around the mean:

$$\sigma = \sqrt{\frac{1}{n} \sum_{t=1}^{n} (r_t - \mu)^2}$$

Higher volatility indicates greater price variability and is often used as a proxy for investment risk. The daily volatility can be annualized using:

$$\sigma_{\text{annual}} = \sigma \times \sqrt{252}$$

where 252 represents the typical number of trading days in a year.

### Minimum Return

The minimum return identifies the worst single-day performance:

$$r_{\min} = \min_{t \in \{1, \ldots, n\}} r_t$$

This metric is useful for understanding downside risk and the magnitude of potential losses during adverse market conditions.

### Maximum Return

The maximum return identifies the best single-day performance:

$$r_{\max} = \max_{t \in \{1, \ldots, n\}} r_t$$

This provides insight into the upside potential and the magnitude of favorable price movements.

## Implementation

The `compute_stats` function in the StockVis Python module implements these calculations using NumPy for efficient numerical computation:

```python
def compute_stats(df: pd.DataFrame) -> dict:
    r = df["returns"].values
    return {
        "mean": float(np.mean(r)),
        "volatility": float(np.std(r)),
        "min": float(np.min(r)),
        "max": float(np.max(r)),
    }
```

All return values are converted to Python floats for JSON serialization and display purposes.
