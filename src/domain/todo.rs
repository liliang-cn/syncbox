use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::errors::DomainError;

const MAX_TITLE_LEN: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

impl Todo {
    pub fn new(title: impl Into<String>) -> Result<Self, DomainError> {
        let title = title.into().trim().to_string();
        if title.is_empty() {
            return Err(DomainError::EmptyTitle);
        }
        if title.len() > MAX_TITLE_LEN {
            return Err(DomainError::TitleTooLong(MAX_TITLE_LEN));
        }
        Ok(Self {
            id: Uuid::new_v4(),
            title,
            completed: false,
            created_at: Utc::now(),
        })
    }

    pub fn id(&self) -> Uuid { self.id }
    pub fn title(&self) -> &str { &self.title }
    pub fn is_completed(&self) -> bool { self.completed }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }

    pub fn rename(&mut self, new_title: impl Into<String>) -> Result<(), DomainError> {
        let t = new_title.into().trim().to_string();
        if t.is_empty() { return Err(DomainError::EmptyTitle); }
        if t.len() > MAX_TITLE_LEN { return Err(DomainError::TitleTooLong(MAX_TITLE_LEN)); }
        self.title = t;
        Ok(())
    }

    pub fn toggle(&mut self) {
        self.completed = !self.completed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_todo() {
        let todo = Todo::new("Buy milk").unwrap();
        assert_eq!(todo.title(), "Buy milk");
        assert!(!todo.is_completed());
    }

    #[test]
    fn empty_title_is_error() {
        let err = Todo::new("   ").unwrap_err();
        assert!(matches!(err, DomainError::EmptyTitle));
    }

    #[test]
    fn too_long_title_is_error() {
        let long = "a".repeat(300);
        let err = Todo::new(long).unwrap_err();
        assert!(matches!(err, DomainError::TitleTooLong(_)));
    }

    #[test]
    fn rename_and_toggle() {
        let mut todo = Todo::new("Task").unwrap();
        todo.toggle();
        assert!(todo.is_completed());
        todo.rename("New Title").unwrap();
        assert_eq!(todo.title(), "New Title");
    }
}
