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
    /// One mediated Bash admission request carrying a bounded command view.
    BashAdmission,
    /// An admission request for a tool outside the read-only set.
    UnadmittedToolAdmission,
    /// More admission requests than the bounded exchange accepts.
    ToolAdmissionOverflow,
    /// Interrupt reports a receipt the runtime never advertised.
    UnadvertisedInterruptReceipt,
    /// Open reports first-party without the subscription evidence field.
    AccountNotSubscription,
    /// Open reports a delegated cloud provider rather than first party.
    AccountNotFirstParty,
    /// Open leaks an account identity field.
    AccountIdentityLeak,
    /// Open reports a version outside the bound one-point claim.
    IdentityMismatch,
    /// Open reports a cwd other than the leased resource root.
    CwdMismatch,
    /// Open reports a canonical effective model for the selected alias.
    CanonicalModel,
    /// The requested alias is listed, but the canonical effective model is not.
    AliasOnly,
    /// Only the canonical effective model is listed.
    CanonicalOnly,
    /// Both the requested alias and canonical effective model are listed.
    BothIds,
    /// Neither the requested alias nor canonical effective model is listed.
    NeitherIds,
    /// Open reports no effective model.
    MissingModel,
    /// Open reports an effective model outside its supported-model list.
    UnsupportedModel,
    /// Open reports an empty supported-model list, which is unavailable.
    EmptySupportedModels,
    /// `set_model` returns the requested model as explicit confirmation.
    ModelChangeConfirmed,
    /// `set_model` resolves without a model confirmation.
    ModelChangeUnconfirmed,
    /// `set_model` is rejected by the fake SDK.
    ModelChangeRejected,
    /// The first-turn init reports the requested effort.
    EffortConfirmed,
    /// The first query response reports that no system/init was yielded.
    InitMissing,
    /// The SDK failed while yielding the first-turn init message.
    InitializationFailed,
    /// The sidecar rejects open and reports its fixed construction code.
    OpenRejected,
    /// The sidecar rejects open and reports its fixed initialization code.
    OpenInitializationRejected,
    /// The sidecar rejects open because it cannot read the account evidence.
    OpenAccountRejected,
    /// The sidecar rejects query and reports its fixed turn-active code.
    QueryRejected,
    /// The first query succeeds, the second rejects, and later queries succeed.
    PostInitRejected,
    /// The sidecar rejects interrupt and reports its fixed interrupt code.
    InterruptRejected,
    /// The sidecar rejects close and reports its fixed invalid-command code.
    CloseRejected,
    /// Open reports a Node runtime newer than the qualified point.
    NewerNode,
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
    /// The sidecar reports an invalid command terminal failure.
    TerminalInvalidCommand,
    /// The sidecar reports a reused command id terminal failure.
    TerminalCommandIdReused,
    /// The sidecar reports too many pending commands terminal failure.
    TerminalTooManyPending,
    /// The sidecar reports an unknown callback terminal failure.
    TerminalCallbackUnknown,
    /// The sidecar reports an invalid callback terminal failure.
    TerminalCallbackInvalid,
    /// The sidecar reports an oversized record terminal failure.
    TerminalRecordTooLarge,
    /// The sidecar reports an empty record terminal failure.
    TerminalEmptyRecord,
    /// The sidecar reports malformed JSON terminal failure.
    TerminalMalformedJson,
    /// The sidecar reports a missing type terminal failure.
    TerminalMissingType,
    /// The sidecar reports an unknown record terminal failure.
    TerminalUnknownRecord,
    /// The sidecar reports an internal terminal failure.
    TerminalInternalError,
    /// The sidecar reports that it could not map an SDK message.
    TerminalUnknownMessage,
    /// The sidecar reports a failed SDK result with all sanitized fields.
    TurnEndedError,
    /// A tool ends without ever starting.
    ToolOrderingDrift,
    /// The sidecar writes an admission request that the turn's own end raced.
    AdmissionAfterResult,
    /// A persisted session resumes and reports the bound session id.
    ResumeComplete,
    /// Resume init reports a different working directory.
    ResumeCwdMismatch,
    /// Resume init reports non-first-party account provenance.
    ResumeAccountMismatch,
    /// Resume init reports a different provider session id.
    ResumeSessionUnknown,
    /// Resume rejects the supplied message boundary.
    ResumeBoundaryRejected,
    /// The fake SDK returns bounded provider session metadata.
    SessionListing,
    /// Open reports connected status for declared MCP servers.
    McpConnected,
    /// Open rejects because a required MCP server failed to connect.
    McpRequiredFail,
    /// Open records a failed optional MCP server and still succeeds.
    McpOptionalFail,
    /// One mediated MCP tool admission request during the turn.
    McpAdmission,
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
