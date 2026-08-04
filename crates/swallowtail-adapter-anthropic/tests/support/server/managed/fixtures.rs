const MANAGED_ROOT: &str = "../../../fixtures/managed-agents-2026-04-01";
const MANAGED_AGENT: &str =
    include_str!(concat!("../../../fixtures/managed-agents-2026-04-01/agent.json"));
const MANAGED_ENVIRONMENT_CREATE: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/environment-create.json"
));
const MANAGED_ENVIRONMENT: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/environment.json"
));
const MANAGED_SESSION_CREATE: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/session-create.json"
));
const MANAGED_SESSION: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/session.json"
));
const MANAGED_MESSAGE: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/user-message.json"
));
const MANAGED_TOOL_RESULT: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/custom-tool-result.json"
));
const MANAGED_INTERRUPT: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/interrupt.json"
));
const MANAGED_SUCCESS: &str =
    include_str!(concat!("../../../fixtures/managed-agents-2026-04-01/success.sse"));
const MANAGED_REQUIRES_ACTION: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/requires-action.sse"
));
const MANAGED_DISCONNECT: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/disconnect.sse"
));
const MANAGED_RESCHEDULING: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/rescheduling.sse"
));
const MANAGED_PROVIDER_FAILURE: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/provider-failure.sse"
));
const MANAGED_HISTORY: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/history.json"
));
const MANAGED_RECOVERY_HISTORY: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/recovery-history.json"
));
const MANAGED_RECOVERY_HISTORY_PAGE_1: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/recovery-history-page-1.json"
));
const MANAGED_RECOVERY_HISTORY_PAGE_2: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/recovery-history-page-2.json"
));
const MANAGED_RECOVERY_ACTIVE_HISTORY: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/recovery-active-history.json"
));
const MANAGED_DELETE_SESSION: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/delete-session.json"
));
const MANAGED_DELETE_ENVIRONMENT: &str = include_str!(concat!(
    "../../../fixtures/managed-agents-2026-04-01/delete-environment.json"
));

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ManagedFixtureState {
    pub environment_created: bool,
    pub session_creations: usize,
    pub stream_attachments: usize,
    pub session_deleted: bool,
    pub environment_deleted: bool,
    pub tool_results: usize,
    pub interrupts: usize,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ManagedStreamFixture {
    #[default]
    Success,
    RequiresActionThenSuccess,
    DisconnectThenSuccess,
    Rescheduling,
    ProviderFailure,
    WaitForInterrupt,
    SessionDeleteFailure,
    Recovered,
    RecoveredPaginated,
    RecoveredActive,
    RecoveredSessionDeleteFailure,
}
