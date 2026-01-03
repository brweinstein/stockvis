import argparse
from data import load_data
from stats import compute_stats
from plots import plot

def main():
    parser = argparse.ArgumentParser()

    parser.add_argument("ticker")
    parser.add_argument("--range", default="1y")
    parser.add_argument("--metric", default="all")
    parser.add_argument("--plot")
    args = parser.parse_args()

    df = load_data(args.ticker, args.range)
    stats = compute_stats(df)

    if args.metric == "all":
        for k, v in stats.items():
            print(f"{k}: {v}")
    else:
        print(f"{args.metric}: {stats[args.metric]}")

    if args.plot:
        plot(df, args.plot)

if __name__ == "__main__":
    main()
