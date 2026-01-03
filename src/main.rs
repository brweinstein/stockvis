use clap::Parser;
use std::process::Command;

mod cli;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    let mut cmd = Command::new("python");
    cmd.arg("python/analyze.py")
        .arg(&cli.ticker)
        .arg("--range")
        .arg(cli.range.as_str())
        .arg("--metric")
        .arg(cli.metric.as_str());

    if let Some(plot) = &cli.plot {
        cmd.arg("--plot").arg(plot.as_str());
    }

    let status = cmd.status().expect("failed to run python analysis");

    if !status.success() {
        eprintln!("Analysis failed");
    }
}
