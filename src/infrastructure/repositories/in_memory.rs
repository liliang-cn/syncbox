use std::collections::HashMap;

use crate::domain::{repositories::{RepoError, TodoRepository}, todo::Todo};
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryTodoRepository {
    store: HashMap<Uuid, Todo>,
}

impl TodoRepository for InMemoryTodoRepository {
    fn save(&mut self, todo: Todo) -> Result<(), RepoError> {
        self.store.insert(todo.id(), todo);
        Ok(())
    }

    fn find_by_id(&self, id: Uuid) -> Result<Option<Todo>, RepoError> {
        Ok(self.store.get(&id).cloned())
    }

    fn list(&self) -> Result<Vec<Todo>, RepoError> {
        Ok(self.store.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_find() {
        let mut repo = InMemoryTodoRepository::default();
        let todo = Todo::new("Task 1").unwrap();
        let id = todo.id();
        repo.save(todo).unwrap();
        let fetched = repo.find_by_id(id).unwrap().unwrap();
        assert_eq!(fetched.id(), id);
    }

    #[test]
    fn list_returns_all() {
        let mut repo = InMemoryTodoRepository::default();
        repo.save(Todo::new("A").unwrap()).unwrap();
        repo.save(Todo::new("B").unwrap()).unwrap();
        let list = repo.list().unwrap();
        assert_eq!(list.len(), 2);
    }
}
