use axum::extract::{Request, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use morannon_config::ClientConfiguration;

use crate::daemon::AppState;


pub async fn get_models(
    State(state): State<AppState>,
    request: Request,
) -> Response {

    let client = request
        .extensions()
        .get::<ClientConfiguration>()
        .expect("There must be a `ClientConfiguration` in the request extensions");

    let models = client
        .models
        .iter()
        .map(|model_id| {
            let model = state.config.models.get(model_id)
                .expect("There must be a model with the given id.");
            json!({
                "id": model.downstream_name,
                "object": "model",
                "owned_by": "morannon",
            })
        })
        .collect::<Vec<_>>();

    Json(json!({
        "object": "list",
        "data": models,
    }))
        .into_response()
}
