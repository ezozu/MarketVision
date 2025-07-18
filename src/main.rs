// src/main.rs
/*
 * Main executable for MarketVision
 */

use clap::Parser;
use marketvision::{Result, run};

#[derive(Parser)]
#[command(version, about = "MarketVision - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
