use crate::domain::todo::Todo;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("not found")] NotFound,
    #[error("unknown repository error: {0}")] Unknown(String),
}

pub trait TodoRepository {
    fn save(&mut self, todo: Todo) -> Result<(), RepoError>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, RepoError>;
    fn list(&self) -> Result<Vec<Todo>, RepoError>;
}
