use anyhow::Result;
use axum::Router;
use clap::{
    builder::{EnumValueParser, RangedU64ValueParser},
    Arg, Command,
};
use std::sync::Arc;
use tokio::sync::Mutex;

mod blockchain;
mod config;
mod errors;
mod persistence;
mod router;
mod service;
mod utils;

use blockchain::Node;
use config::{Configuration, Mode};
use router::{api, ui};

async fn process_tx_pool() -> Result<()> {
    // let duration = time::Duration::from_secs(u32)
    //
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("block-factory")
        .version("0.1.0")
        .about("Generate blocks from incoming transactions")
        .arg(
            Arg::new("PORT")
                .help("Port to use in serving the factory")
                .long("port")
                .short('p')
                .value_parser(RangedU64ValueParser::<u32>::new().range(1..))
                .default_value("8080"),
        )
        .arg(
            Arg::new("BLOCKTIME")
                .help("Amount of seconds to wait before creating a block")
                .long("block-time")
                .value_parser(RangedU64ValueParser::<u32>::new().range(1..))
                .default_value("1"),
        )
        .arg(
            Arg::new("MODE")
                .help("Mode for block factory")
                .long("mode")
                .short('m')
                .value_parser(EnumValueParser::<Mode>::new())
                .default_value("full"),
        )
        .get_matches();

    // parse arguments and flags
    let port = matches.get_one::<u32>("PORT").unwrap();
    let block_time = *matches.get_one::<u32>("BLOCKTIME").unwrap();
    let mode = matches.get_one::<Mode>("MODE").unwrap();

    // store in config struct
    let shared_config = Arc::new(Mutex::new(Configuration {
        port: port.to_owned(),
        block_time: block_time.to_owned(),
        mode: mode.to_owned(),
        node: Node::new(),
    }));

    // display config with beautiful table
    utils::display_configuration(port, &block_time, mode);

    let config = Arc::clone(&shared_config);
    let node_handle = tokio::spawn(async move {
        // run the node
        let config = config.lock().await;
        config.node.run(block_time.to_owned()).await
    });

    // get routes
    let app = Router::new()
        // serve static files from /assets
        .merge(service::service()?)
        .merge(api::router(shared_config.clone())?)
        .merge(ui::router()?);

    // add global 404
    let app = app.fallback(ui::not_found);

    // run our app with hyper, listening globally on {--port}
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    // serve block factory
    axum::serve(listener, app).await?;

    Ok(())
}
