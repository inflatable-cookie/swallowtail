use super::*;

#[path = "agent/lifecycle.rs"]
mod lifecycle;
mod process;
mod reasoning;
mod responses;

pub(super) use process::FixtureProcessHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Scenario {
    Success,
    LargeToolUpdate,
    MalformedUsage,
    DeleteMissing,
    DeleteProviderFailure,
    DeleteDisconnect,
    RunDeleteDisconnect,
    DeleteMalformed,
    DeletePending,
    Permission,
    Elicitation,
    Cancellation,
    ClosePending,
    Disconnect,
    ModelDrift,
    AuthDrift,
    LifecycleDrift,
    ReasoningMismatchAdvertised,
    ReasoningMismatchUnadvertised,
    ReasoningMismatchUnqualified,
    ReasoningConfirmationMissing,
    ReasoningConfirmationMalformed,
    ReasoningConfirmationDuplicate,
    ReasoningConfirmationUnbounded,
    ModelMalformed,
    ModelDuplicate,
    ModelUnadvertised,
    ModelUnbounded,
    Version,
}

#[derive(Clone, Debug)]
pub struct ObservedProcess {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environment_count: usize,
    pub working_resource: Option<WorkingResourceRef>,
}

#[derive(Default)]
pub(super) struct AgentState {
    pub(super) output: VecDeque<ProcessOutputChunk>,
    pub(super) writes: Vec<Value>,
    prompt_id: Option<u64>,
    requested_model: Option<String>,
    current_model: Option<String>,
    effort: Option<String>,
    mode: Option<String>,
    stopped: bool,
}

pub(super) struct SharedAgent {
    pub(super) state: Mutex<AgentState>,
    changed: Condvar,
    scenario: Scenario,
    version: String,
}

include!("agent/session.rs");
include!("agent/prompt.rs");
