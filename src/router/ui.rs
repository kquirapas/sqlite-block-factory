use anyhow::Result;

use askama::Template;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub fn router() -> Result<Router> {
    let ui_routes = Router::new()
        // / -> block-explorer
        .route("/", get(block_explorer));

    let ui_group = Router::new().nest("/", ui_routes);

    Ok(ui_group)
}

//---- HANDLERS -----

// get block explorer
async fn block_explorer() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

//---- ASKAMA TEMPLATING -----

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
