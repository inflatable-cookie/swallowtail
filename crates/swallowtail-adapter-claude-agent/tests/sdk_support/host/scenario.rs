//! Exactly what the fixture sidecar does on the wire, and which single host
//! service refuses to answer.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SdkScenario {
    /// Open, one streamed turn, and a sidecar-joined graceful close.
    Complete,
    /// The sidecar observes its native child still running at close.
    NativeChildSurvives,
    /// The sidecar claims an exit it never observed.
    NativeJoinWithoutObservation,
    /// One `canUseTool` admission request during the turn.
    ToolAdmission,
    /// An admission request for a tool outside the read-only set.
    UnadmittedToolAdmission,
    /// More admission requests than the bounded exchange accepts.
    ToolAdmissionOverflow,
    /// Interrupt reports a receipt the runtime never advertised.
    UnadvertisedInterruptReceipt,
    /// Open reports a non-subscription access profile.
    AccountApiKeySource,
    /// Open reports a delegated cloud provider rather than first party.
    AccountNotFirstParty,
    /// Open leaks an account identity field.
    AccountIdentityLeak,
    /// Open reports a version outside the bound one-point claim.
    IdentityMismatch,
    /// Open reports a cwd other than the leased resource root.
    CwdMismatch,
    /// Open reports an effective model other than the selected one.
    ModelMismatch,
    /// The sidecar accepts open and never answers it.
    OpenHold,
    /// The sidecar accepts the query and never answers it.
    QueryHold,
    /// Open advertises tools beyond the admitted set.
    ToolsWidened,
    /// Open confirms a permission mode other than the one requested.
    PermissionModeDrift,
    /// The sidecar refuses a mid-session permission-mode change.
    PermissionModeRejected,
    /// The sidecar answers a mode change with a different mode.
    PermissionModeUnconfirmed,
    /// The stream carries an unqualified event name.
    UnknownEvent,
    /// The stream carries invalid JSON.
    Malformed,
    /// The stream ends mid-record.
    Disconnect,
    /// The sidecar reports a terminal failure.
    TerminalRecord,
    /// A tool ends without ever starting.
    ToolOrderingDrift,
    /// The sidecar writes an admission request that the turn's own end raced.
    AdmissionAfterResult,
}

/// One host service that never answers, so a caller bound is the only thing
/// that can end the wait.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stall {
    CredentialAcquire,
    ResourceResolve,
    ProcessStart,
    ProcessWrite,
    ForceStop,
    /// The pump's own read never ends, so the pump task outlives process exit
    /// until the test releases it.
    PumpRead,
}
