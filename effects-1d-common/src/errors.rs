use snafu::prelude::*;

/// Any error that can happen during rendering
#[derive(Snafu, Debug)]
pub enum RenderError {
    /// Any error that isn't in the predefined list of errors
    #[snafu(display("custom error: {}", msg))]
    Other {
        /// Custom message
        msg: &'static str,
    },
}
