mod input;
mod plan;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use input::GeminiSessionProfileInput;
pub use plan::GeminiPreparedEvidence;
pub use session::GeminiPreparedSession;

pub type GeminiPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
