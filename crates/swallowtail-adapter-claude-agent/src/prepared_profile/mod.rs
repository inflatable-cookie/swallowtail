mod activity_profile;
mod input;
mod management;
mod plan;
mod run;
mod session;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, LoadedSession, RuntimeFailure};

pub use input::{
    ClaudeAgentModelSelection, ClaudeAgentPermissionHandling, ClaudeAgentRunProfileInput,
    ClaudeAgentRunRetention, ClaudeAgentSessionManagementInput, ClaudeAgentSessionProfileInput,
};
pub use management::ClaudeAgentPreparedDelete;
pub use plan::ClaudeAgentPreparedEvidence;
pub use run::ClaudeAgentPreparedRun;
pub use session::ClaudeAgentPreparedSession;

/// Future returned when a prepared Claude Agent session opens or resumes.
pub type ClaudeAgentPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
/// Future returned when a prepared Claude Agent session loads with replay.
pub type ClaudeAgentPreparedSessionLoadFuture =
    BoxFuture<'static, Result<LoadedSession, RuntimeFailure>>;
