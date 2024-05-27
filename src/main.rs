use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use std::env;

// defaults
const PORT_DEFAULT: &str = "8080";
const DATABASE_NAME_DEFAULT: &str = "blocks.db";
const BLOCK_TIME_DEFAULT: &str = "1"; // in seconds

/// Program for producing blocks and persisting
/// them into a database.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// port to use for block-factory (e.g. 8080)
    #[clap(short, long, default_value = PORT_DEFAULT)]
    port: Option<String>,

    /// name to use for your sqlite DB file name
    #[arg(short, long, default_value = DATABASE_NAME_DEFAULT)]
    database: Option<String>,

    /// set how long (in seconds) the factory will wait before it
    /// wrangles blocks in the pool and creates a block
    #[arg(short, long, default_value = BLOCK_TIME_DEFAULT)]
    block_time: Option<u32>,
}

fn main() {
    dotenv().ok(); // Load the .env file

    let args = Args::parse();
    println!("{}", format!("{:?}", args).green().bold());

    println!(
        "{}",
        env::var("BLOCK_TIME")
            .unwrap_or(String::from(BLOCK_TIME_DEFAULT))
            .to_string()
            .yellow()
            .bold()
    );
}
