use std::net::SocketAddr;
use std::sync::Arc;

use axum::{middleware, Router};
use tower_http::trace::TraceLayer;

use morannon_config::DaemonConfiguration;

use crate::{auth::authentication_middleware, endpoints};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<DaemonConfiguration>,
}

pub async fn run(config: DaemonConfiguration) -> Result<(), Box<dyn std::error::Error>> {

    let listen_addr = SocketAddr::new(config.bind_address, config.bind_port);
    let state = AppState {
        config: Arc::new(config),
    };

    let app = Router::new()
        .merge(endpoints::openai::routes())
        .merge(endpoints::proxy::routes())
        .fallback(endpoints::fallback)
        .route_layer(middleware::from_fn_with_state(
            Arc::clone(&state.config),
            authentication_middleware,
        ))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(listen_addr).await?;
    tracing::info!("listening on {listen_addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl-C handler");
    tracing::info!("shutdown signal received");
}
