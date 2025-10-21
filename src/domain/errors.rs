use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("title is too long (max {0} chars)")]
    TitleTooLong(usize),
}
