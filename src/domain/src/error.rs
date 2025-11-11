#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("{message}")]
    NotFound { message: String },

    #[error("internal error: {message}")]
    InternalError { message: String },
}
