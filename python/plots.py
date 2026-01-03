import matplotlib.pyplot as plt

def plot(df, kind: str):
    if kind == "price":
        df["Close"].plot(title="Price over Time")
    elif kind == "returns":
        df["returns"].plot(title="Daily Returns")
    elif kind == "hist":
        df["returns"].hist(bins=50)
    else:
        raise ValueError(f"Unknown plot type: {kind}")

    plt.tight_layout()
    plt.show()
