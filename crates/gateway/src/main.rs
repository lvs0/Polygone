use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use reqwest::Client;
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct AppState {
    http_client: Client,
    backend_url: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn get_resource(
    Path(cid): Path<String>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    let s = state.read().await;
    let url = format!("{}/p2p/get/{}", s.backend_url, cid);

    match s.http_client.get(&url).send().await {
        Ok(resp) => {
            if resp.status() == 404 {
                let err = ErrorResponse { error: format!("Resource not found: {}", cid) };
                return (axum::http::StatusCode::NOT_FOUND, axum::Json(err)).into_response();
            }
            match resp.text().await {
                Ok(text) => Html(text).into_response(),
                Err(_) => (axum::http::StatusCode::BAD_GATEWAY, "Bad response from backend").into_response(),
            }
        }
        Err(_) => (axum::http::StatusCode::BAD_GATEWAY, "Cannot reach backend").into_response(),
    }
}

async fn index() -> &'static str {
    "Polygone Gateway: HTTP -> P2P bridge (connected)\n"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());

    let state = Arc::new(RwLock::new(AppState {
        http_client: Client::new(),
        backend_url: backend_url.clone(),
    }));

    let app = Router::new()
        .route("/", get(index))
        .route("/sites/:cid", get(get_resource))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Polygone Gateway listening on http://{}", addr);
    println!("Backend URL: {}", backend_url);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app.into_make_service()).await?;

    Ok(())
}
