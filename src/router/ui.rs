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
    // /transaction
    // /blocks (with paging and search)
    // /transaction/block/:blockHash (with paging and search)
    // /transaction/hash/:hash
    // /block/hash/:hash
    // /block/height/:height
    let ui_routes = Router::new()
        // / -> block-explorer
        .route("/", get(block_explorer));

    let ui_group = Router::new().nest("/", ui_routes);

    Ok(ui_group)
}

// get block explorer
async fn block_explorer() -> Html<&'static str> {
    Html("<main><h1>blockchain explorer</main></h1>")
}
