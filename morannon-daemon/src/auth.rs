use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use morannon_config::{ClientConfiguration, DaemonConfiguration};

pub async fn authentication_middleware(
    State(config): State<Arc<DaemonConfiguration>>,
    mut request: Request,
    next: Next,
) -> Response {

    let Some(token) = extract_bearer_token(&request) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Missing Authorization header" })),
        ).into_response()
    };

    match resolve_client(&config, token) {
        Some(client) => {
            request.extensions_mut().insert(client.clone());
            next.run(request).await
        }
        None => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid API key" })),
        ).into_response(),
    }
}

fn extract_bearer_token(request: &Request) -> Option<&str> {
    let header = request.headers().get(axum::http::header::AUTHORIZATION)?;
    let value = header.to_str().ok()?;
    value.strip_prefix("Bearer ")
}

fn resolve_client<'a>(config: &'a DaemonConfiguration, key: &str) -> Option<&'a ClientConfiguration> {
    config.clients.values().find(|c| c.key == key)
}
