mod input;
mod plan;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use input::{ClaudeAgentModelSelection, ClaudeAgentSessionProfileInput};
pub use plan::ClaudeAgentPreparedEvidence;
pub use session::ClaudeAgentPreparedSession;

pub type ClaudeAgentPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
