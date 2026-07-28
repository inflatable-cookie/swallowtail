mod input;
mod management;
mod plan;
mod run;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use input::{
    ClaudeAgentModelSelection, ClaudeAgentPermissionHandling, ClaudeAgentRunProfileInput,
    ClaudeAgentSessionManagementInput, ClaudeAgentSessionProfileInput,
};
pub use management::ClaudeAgentPreparedDelete;
pub use plan::ClaudeAgentPreparedEvidence;
pub use run::ClaudeAgentPreparedRun;
pub use session::ClaudeAgentPreparedSession;

pub type ClaudeAgentPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
