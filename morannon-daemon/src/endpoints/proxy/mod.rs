mod handler;

use crate::daemon::AppState;
use crate::endpoints::proxy::handler::proxy_handler;
use axum::routing::any;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/{*path}", any(proxy_handler))
}
