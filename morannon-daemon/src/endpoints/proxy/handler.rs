use crate::daemon::AppState;
use axum::extract::{Request, State};
use axum::response::Response;

pub async fn proxy_handler(
    State(_state): State<AppState>,
    _request: Request,
) -> Response {
    todo!()
}
