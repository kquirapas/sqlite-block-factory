use crate::blockchain::Transaction;

use anyhow::Result;
use axum::{
    extract::State,
    response::Json,
    routing::{get, put},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Configuration;

pub fn router(shared_config: Arc<Mutex<Configuration>>) -> Result<Router> {
    // /api/transaction
    // /api/blocks (with paging and search)
    // /api/transaction/block/:blockHash (with paging and search)
    // /api/transaction/hash/:hash
    // /api/block/hash/:hash
    // /api/block/height/:height
    let api_routes = Router::new()
        // /api/hello
        .route("/hello", get(hello_world))
        .route("/transaction", put(transaction));
    // .route("/transaction/pool", get(tx_pool));

    let api_group = Router::new()
        .nest("/api", api_routes)
        .with_state(shared_config);

    Ok(api_group)
}

// GET /hello
async fn hello_world() -> Json<Value> {
    Json(json!({
        "message": "hello, world!",
        "status": "OK",
    }))
}

// PUT /transaction
async fn transaction(
    State(config): State<Arc<Mutex<Configuration>>>,
    Json(payload): Json<Transaction>,
) -> Json<Value> {
    let tx = payload;
    let config = Arc::clone(&config);

    let mut configuration = config.lock().await;
    configuration.node.tx_pool.push(tx);

    Json(json!({
        "message": "successfully added transaction to pool",
        "status": "OK",
    }))
}

// GET /transaction/pool
// async fn tx_pool(State(config): State<Arc<Mutex<Configuration>>>) -> Json<Value> {
//     let config = Arc::clone(&config);
//
//     Json(json!({
//         "data": config.node,
//         "status": "OK",
//     }))
// }
