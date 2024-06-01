use anyhow::Result;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    response::{Html, Json},
    routing::get,
    Router,
};
use serde_json::{json, Value};

pub fn router() -> Result<Router> {
    // /api/transaction
    // /api/blocks (with paging and search)
    // /api/transaction/block/:blockHash (with paging and search)
    // /api/transaction/hash/:hash
    // /api/block/hash/:hash
    // /api/block/height/:height
    let api_routes = Router::new()
        // /api/hello
        .route("/hello", get(hello_world));

    let api_group = Router::new().nest("/api", api_routes);

    Ok(api_group)
}

// GET hello world
async fn hello_world() -> Json<Value> {
    Json(json!({
        "message": "hello, world!",
        "status": "OK",
    }))
}
