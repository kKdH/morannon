use axum::body::Bytes;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum_extra::{headers, TypedHeader};

#[axum::debug_handler]
pub async fn fallback(
    uri: Uri,
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
    body: Bytes,
) -> impl IntoResponse {
    tracing::debug!("`{}` requested unknown endpoint '{}': {:?}", user_agent.as_str(), uri, String::from_utf8_lossy(&body));
    (StatusCode::NOT_FOUND, "Not found")
}
