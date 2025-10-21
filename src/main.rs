use std::sync::{Arc, Mutex};

use project::infrastructure::repositories::in_memory::InMemoryTodoRepository;
use project::presentation::http::{build_router, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = AppState { repo: Arc::new(Mutex::new(InMemoryTodoRepository::default())) };
    let app = build_router(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr).await.expect("bind addr");
    println!("Starting server on {}", addr);
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("server error: {}", e);
    }
}
