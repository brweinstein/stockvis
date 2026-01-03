import numpy as np
import pandas as pd

def compute_stats(df: pd.DataFrame) -> dict:
    r = df["returns"].values

    return {
        "mean": float(np.mean(r)),
        "volatility": float(np.std(r)),
        "min": float(np.min(r)),
        "max": float(np.max(r)),
    }
