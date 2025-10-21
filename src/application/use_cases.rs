use crate::domain::{errors::DomainError, repositories::TodoRepository, todo::Todo};

pub struct CreateTodoInput {
    pub title: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTodoError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("repository error: {0}")]
    Repository(String),
}

pub struct CreateTodoUseCase;

impl CreateTodoUseCase {
    pub fn execute<R: TodoRepository>(repo: &mut R, input: CreateTodoInput) -> Result<Todo, CreateTodoError> {
        let todo = Todo::new(input.title)?;
        repo.save(todo.clone()).map_err(|e| CreateTodoError::Repository(e.to_string()))?;
        Ok(todo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::in_memory::InMemoryTodoRepository;

    #[test]
    fn create_todo_succeeds_and_persists() {
        let mut repo = InMemoryTodoRepository::default();
        let result = CreateTodoUseCase::execute(&mut repo, CreateTodoInput { title: "Buy milk".into() });
        assert!(result.is_ok());
        let todo = result.unwrap();
        let fetched = repo.find_by_id(todo.id()).unwrap().unwrap();
        assert_eq!(fetched.title(), "Buy milk");
        assert!(!fetched.is_completed());
    }

    #[test]
    fn create_todo_rejects_empty_title() {
        let mut repo = InMemoryTodoRepository::default();
        let result = CreateTodoUseCase::execute(&mut repo, CreateTodoInput { title: "   ".into() });
        assert!(matches!(result.unwrap_err(), CreateTodoError::Domain(DomainError::EmptyTitle)));
    }
}
