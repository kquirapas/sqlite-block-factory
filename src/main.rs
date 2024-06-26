use anyhow::Result;
use axum::Router;
use std::sync::Arc;

mod blockchain;
mod cli;
mod config;
mod error;
mod persistence;
mod router;
mod service;
mod utils;

use blockchain::{Chain, Node};
use cli::Cli;
use config::{Configuration, Mode};
use router::{api, ui};

#[tokio::main]
async fn main() -> Result<()> {
    // get CLI matches
    let matches = Cli::get_matches();

    // parse arguments and flags
    let port = *matches.get_one::<u32>("PORT").unwrap();
    let block_time = *matches.get_one::<u32>("BLOCKTIME").unwrap();
    let mode = matches.get_one::<Mode>("MODE").unwrap();

    // store in config struct
    let shared_config = Arc::new(Configuration {
        port: port.to_owned(),
        block_time: block_time.to_owned(),
        mode: mode.to_owned(),
        chain: Chain::new(),
    });

    // display config with beautiful table
    utils::display_configuration(&port, &block_time, mode);

    // run the Chain in a task with Node runner
    let config = Arc::clone(&shared_config);
    let chain_handle = tokio::spawn(async move {
        println!("Spawning node runner...");

        // get ownership of BLOCK TIME
        let owned_block_time = block_time.to_owned();

        // run the node
        let config = config;
        let node = Node::new().await?;

        // create genesis block
        node.store_genesis_block().await?;

        node.run(&config.chain, owned_block_time).await
    });

    // get routes and merge under one App route
    let app = Router::new()
        // route /assets (serve static files from /assets)
        .merge(service::service()?)
        // route /api/
        .merge(api::router(shared_config.clone())?)
        // route / (for rendering templates)
        .merge(ui::router()?);

    // add global 404 page
    let app = app.fallback(ui::not_found);

    // serve block factory in a task
    let server_handle = tokio::spawn(async move {
        println!("Spawning server...");
        // get ownership of PORT
        let owned_port = port.to_owned();
        // run our app with hyper, listening globally on {--port}
        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{owned_port}")).await?;
        // run the server
        axum::serve(listener, app).await
    });

    chain_handle.await??;
    server_handle.await??;

    Ok(())
}
