use axum::{
    extract::Json,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::fs;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct PublicKeyMessage {
    public_key: String,
}

static mut PUBLIC_KEY: String = String::new();

pub fn get_public_key() -> &'static str {
    unsafe { &PUBLIC_KEY }
}

pub async fn start_server() {
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/", post(handle_message));

    let listener = TcpListener::bind("0.0.0.0:50009")
        .await
        .expect("Failed to bind listener");

    axum::serve(listener, app).await;
}

async fn serve_index() -> impl IntoResponse {
    match fs::read_to_string("src/server/index.html") {
        Ok(contents) => Html(contents),
        Err(_) => Html("<h1>Erro ao carregar a página</h1>".to_string()),
    }
}

async fn handle_message(Json(payload): Json<PublicKeyMessage>) {
    unsafe {
        PUBLIC_KEY = payload.public_key;
    }
}
