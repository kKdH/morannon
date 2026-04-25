pub mod get_models;

use axum::Router;
use axum::routing::get;

use crate::daemon::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/models", get(get_models::get_models))
}
