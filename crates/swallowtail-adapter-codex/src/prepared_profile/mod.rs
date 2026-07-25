mod exec;
mod input;
mod plan;
mod session;
mod session_capabilities;

use swallowtail_runtime::{BoxFuture, InteractiveSessionHandle, RuntimeFailure};

pub use exec::CodexPreparedExec;
pub use input::{CodexExecProfileInput, CodexModelSelection, CodexSessionProfileInput};
pub use plan::CodexPreparedEvidence;
pub use session::{CodexPreparedCatalogue, CodexPreparedSession};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodexPreparedSessionKind {
    ReadOnly,
    BoundedWorkspace,
}

pub type CodexPreparedSessionFuture =
    BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
