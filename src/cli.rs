use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Cli {
    pub ticker: String,

    #[arg(long, default_value = "1y")]
    pub range: TimeRange,

    #[arg(long, default_value = "all")]
    pub metric: Metric,

    #[arg(long)]
    pub plot: Option<PlotType>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum TimeRange {
    #[value(name = "1d")]
    OneDay,
    #[value(name = "1w")]
    OneWeek,
    #[value(name = "1m")]
    OneMonth,
    #[value(name = "1y")]
    OneYear,
    #[value(name = "max")]
    Max,
}

impl TimeRange {
    pub fn as_str(&self) -> &str {
        match self {
            TimeRange::OneDay => "1d",
            TimeRange::OneWeek => "1w",
            TimeRange::OneMonth => "1m",
            TimeRange::OneYear => "1y",
            TimeRange::Max => "max",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Metric {
    Mean,
    Volatility,
    Returns,
    All,
}

impl Metric {
    pub fn as_str(&self) -> &str {
        match self {
            Metric::Mean => "mean",
            Metric::Volatility => "volatility",
            Metric::Returns => "returns",
            Metric::All => "all",
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum PlotType {
    Price,
    Returns,
    Hist,
}

impl PlotType {
    pub fn as_str(&self) -> &str {
        match self {
            PlotType::Price => "price",
            PlotType::Returns => "returns",
            PlotType::Hist => "hist",
        }
    }
}
