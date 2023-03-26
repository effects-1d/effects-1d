use thiserror::Error;

/// Any error that can happen during rendering
#[derive(Error, Debug)]
pub enum RenderError {
    /// Any error that isn't in the predefined list of errors
    #[error("custom error: {0}")]
    Other(String),
}
