use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::Server;
use project::presentation::http::{build_router, AppState};
use project::infrastructure::repositories::in_memory::InMemoryTodoRepository;

#[tokio::main]
async fn main() {
    let state = AppState { repo: Arc::new(Mutex::new(InMemoryTodoRepository::default())) };
    let app = build_router(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("Starting server on {}", addr);
    if let Err(e) = Server::bind(&addr).serve(app.into_make_service()).await {
        eprintln!("server error: {}", e);
    }
}
