use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

// Simulateur de stockage local tres basique
#[derive(Clone)]
pub struct LocalStorage {
    files: HashMap<String, Vec<u8>>,
}

impl LocalStorage {
    pub fn new() -> Self {
        let mut files = HashMap::new();
        // Fichier de demonstration
        files.insert(
            "demo.txt".to_string(),
            b"Hello from Polygone P2P network!".to_vec(),
        );
        Self { files }
    }

    pub fn get(&self, cid: &str) -> Option<&Vec<u8>> {
        self.files.get(cid)
    }
}

async fn get_file(
    Path(cid): Path<String>,
    storage: axum::extract::State<Arc<LocalStorage>>,
) -> impl IntoResponse {
    match storage.get(&cid) {
        Some(data) => Html(data.clone()).into_response(),
        None => (axum::http::StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

pub async fn run_http_api(addr: SocketAddr, storage: Arc<LocalStorage>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new()
        .route("/p2p/get/:cid", get(get_file))
        .with_state(storage);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app.into_make_service()).await?;
    Ok(())
}
