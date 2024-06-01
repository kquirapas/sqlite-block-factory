use anyhow::{Context, Result};
use axum::Router;
use std::env;
use tower_http::services::ServeDir;

pub fn service() -> Result<Router> {
    let assets_route = Router::new();

    let current_dir = env::current_dir()?;
    let current_dir = current_dir
        .to_str()
        .context("Cannot parse current directory to string")?;
    let assets_group =
        assets_route.nest_service("/assets", ServeDir::new(format!("{current_dir}/assets")));

    Ok(assets_group)
}
