use anyhow::Result;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use clap::{value_parser, Arg, Command, ValueEnum};
use comfy_table::{presets::UTF8_FULL, *};
use sha256::digest;
use std::hash;
use tokio::time;

mod router;
use router::{api, ui};

mod service;

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Full,
    FactoryOnly,
    QueryOnly,
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

    // display configuration
    let mut table = Table::new();
    // resolve temporary borrow error
    let table = table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .set_header(vec![
            Cell::new("Configuration").add_attribute(Attribute::Bold)
        ])
        .set_header(vec![
            Cell::new("Name").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
        ]);

    table.add_row(vec![Cell::new("Port"), Cell::new(port)]);
    table.add_row(vec![Cell::new("Block Time"), Cell::new(block_time)]);
    table.add_row(vec![
        Cell::new("Mode"),
        Cell::new(match mode {
            Mode::Full => "Full",
            Mode::FactoryOnly => "Factory Only",
            Mode::QueryOnly => "Query Only",
        }),
    ]);

    println!("{table}");

    // get routes
    let app = Router::new()
        // service to come before the ui routes
        .merge(service::service()?)
        .merge(api::router()?)
        .merge(ui::router()?);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
