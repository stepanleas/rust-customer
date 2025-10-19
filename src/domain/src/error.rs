use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("item with id {id} not found")]
    NotFound { id: Uuid },

    #[error("internal error: {message}")]
    InternalError { message: String },
}
