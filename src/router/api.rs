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

pub fn router(shared_config: Arc<Configuration>) -> Result<Router> {
    // /api/blocks (with paging and search)
    // /api/transaction/block/:blockHash (with paging and search)
    // /api/transaction/hash/:hash
    // /api/block/hash/:hash
    // /api/block/height/:height
    let api_routes = Router::new()
        // /api/hello
        .route("/hello", get(hello_world))
        .route("/transaction", put(transaction))
        .route("/transaction/pool", get(tx_pool));

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
    State(config): State<Arc<Configuration>>,
    Json(payload): Json<Transaction>,
) -> Json<Value> {
    let config = Arc::clone(&config);
    let tx = payload;
    let pool_arc = Arc::clone(&config.node.tx_pool);

    let mut pool = pool_arc.lock().await;
    pool.push(tx);

    Json(json!({
        "message": "successfully added transaction to pool",
        "status": "OK",
    }))
}

// GET /transaction/pool
async fn tx_pool(State(config): State<Arc<Configuration>>) -> Json<Value> {
    let config = Arc::clone(&config);
    let pool_arc = Arc::clone(&config.node.tx_pool);

    let pool = pool_arc.lock().await;

    Json(json!({
        "data": *pool,
        "status": "OK",
    }))
}
