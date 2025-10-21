use std::sync::{Arc, Mutex};

use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{application::use_cases::{CreateTodoError, CreateTodoInput, CreateTodoUseCase},
            infrastructure::repositories::in_memory::InMemoryTodoRepository};

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<Mutex<InMemoryTodoRepository>>, // simple in-memory store for now
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest { pub title: String }

#[derive(Debug, Serialize)]
pub struct TodoResponse {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/todos", post(create_todo))
        .with_state(state)
}

async fn health() -> &'static str { "ok" }

async fn create_todo(State(state): State<AppState>, Json(payload): Json<CreateTodoRequest>) -> Result<(StatusCode, Json<TodoResponse>), (StatusCode, String)> {
    let mut repo = state.repo.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "repo lock".into()))?;
    let todo = CreateTodoUseCase::execute(&mut *repo, CreateTodoInput { title: payload.title })
        .map_err(|e| match e {
            CreateTodoError::Domain(_) => (StatusCode::BAD_REQUEST, e.to_string()),
            CreateTodoError::Repository(_) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        })?;

    let resp = TodoResponse {
        id: todo.id().to_string(),
        title: todo.title().to_string(),
        completed: todo.is_completed(),
        created_at: todo.created_at().to_rfc3339(),
    };
    Ok((StatusCode::CREATED, Json(resp)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn health_works() {
        let app = build_router(AppState { repo: Arc::new(Mutex::new(InMemoryTodoRepository::default())) });
        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_todo_endpoint_works() {
        let app = build_router(AppState { repo: Arc::new(Mutex::new(InMemoryTodoRepository::default())) });

        let payload = serde_json::json!({"title": "Buy milk"});
        let req = Request::builder()
            .method("POST")
            .uri("/todos")
            .header("content-type", "application/json")
            .body(Body::from(payload.to_string()))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);

        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let v: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(v.get("title").unwrap(), "Buy milk");
        assert_eq!(v.get("completed").unwrap(), false);
    }
}
