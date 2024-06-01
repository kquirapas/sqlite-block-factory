use anyhow::{Context, Result};
use askama::Template;
use axum::{routing::get, Router};
use clap::{value_parser, Arg, Command, ValueEnum};
use sha256::digest;
use std::hash;
use tokio::time;

mod router;
use router::{api, ui};

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Full,
    Generation,
    Query,
}

struct Configuration {
    port: String,
    block_time: u32,
}

struct Transaction<'a> {
    from: String,
    to: String,
    instruction: &'a [u8],
}

struct Block {
    transactions: Vec<String>,
    prev_block_hash: String,
    timestamp: u32,
    nonce: u32,
}

struct Node<'a> {
    transaction_pool: Vec<Transaction<'a>>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn process_tx_pool() -> Result<()> {
    // let duration = time::Duration::from_secs(u32)
    //
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // requires `cargo` feature, reading name, version, author, and description from `Cargo.toml`
    let matches = Command::new("block-factory")
        .version("0.1.0")
        .about("Generate blocks from incoming transactions")
        .arg(
            Arg::new("PORT")
                .help("Port to use in serving the factory")
                .long("port")
                .short('p')
                .value_parser(value_parser!(u32).range(1..))
                .default_value("8080"),
        )
        .arg(
            Arg::new("BLOCKTIME")
                .help("Amount of seconds to wait before creating a block")
                .long("block-time")
                .value_parser(value_parser!(u32).range(1..))
                .default_value("1"),
        )
        .arg(
            Arg::new("MODE")
                .help("Mode for block factory")
                .long("mode")
                .short('m')
                .value_parser(value_parser!(Mode))
                .default_value("full"),
        )
        .get_matches();

    // parse arguments and flags
    let port = matches.get_one::<u32>("PORT").unwrap();
    let block_time = matches.get_one::<u32>("BLOCKTIME").unwrap();
    let mode = matches.get_one::<Mode>("MODE").unwrap();

    println!("----- Configuration -----");
    println!("PORT: {}", port);
    println!("BLOCK TIME: {}", block_time);
    println!("MODE: {:?}", mode);
    println!("-------------------------");

    // get routes
    let app = Router::new().merge(api::router()?).merge(ui::router()?);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
