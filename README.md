# Stock Analysis & Visualization Tool

A hybrid Rust/Python CLI tool for analyzing stock market data with statistical analysis and visualization capabilities.

## Overview

Command-line access to stock market analysis, combining the performance of Rust for the CLI interface with Python's rich data science ecosystem for analysis and visualization. The tool fetches historical stock data, computes statistical metrics, and generates visual reports.

## Project Structure

```
stockvis/
├── Cargo.toml              # Rust dependencies and package configuration
├── README.md               # Project documentation
├── stats_documentation.md # Detailed writeup of stats
├── src/                    # Rust CLI source code
│   ├── main.rs            # Entry point, spawns Python subprocess
│   └── cli.rs             # Command-line argument parser (clap)
├── python/                 # Python analysis modules
│   ├── analyze.py         # Main analysis script
│   ├── data.py            # Data loading from yfinance
│   ├── stats.py           # Statistical computations
│   ├── plots.py           # Visualization functions
│   └── requirements.txt   # Python dependencies
└── notebooks/              # Jupyter notebooks for interactive analysis
    └── stock_analysis.ipynb  # Sample multi-stock comparison notebook
```

## Usage

### CLI Tool

```bash
# Analyze a stock with default settings (1 year)
cargo run -- AAPL

# Specify time range and metric
cargo run -- MSFT --range 6mo --metric volatility

# Generate a plot
cargo run -- GOOGL --range 1y --plot price
```

### Python Modules

```python
from data import load_data
from stats import compute_stats

# Load stock data
df = load_data("AAPL", "1y")

# Compute statistics
stats = compute_stats(df)
print(stats)  
# {'mean': ..., 'volatility': ..., 'min': ..., 'max': ...}
```

## High-Level Architecture

```
User
 ↓
Rust CLI (clap)
 ↓  (subprocess / args)
Python analysis script
 ↓
CSV / stdout / images
 ↓
Terminal output + notebook report
```

## Installation

### Rust CLI
```bash
cargo build --release
```

### Python Environment
```bash
pip install -r python/requirements.txt
```

## Dependencies

- **Rust**: clap 4.5 (CLI parsing)
- **Python**: pandas, numpy, matplotlib, yfinance
