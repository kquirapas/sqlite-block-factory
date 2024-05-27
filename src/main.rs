use anyhow::{Context, Result};
use clap::Parser;

/// Program for producing blocks and persisting
/// them into a database.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// port to use for block-factory (e.g. 8080)
    #[arg(short, long)]
    port: Option<String>,

    /// name to use for your sqlite DB file name
    #[arg(short, long)]
    database: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
