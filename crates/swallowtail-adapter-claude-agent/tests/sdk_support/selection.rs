//! Builds prepared Claude Agent SDK sidecar sessions through the public
//! facade, so every driver case also exercises the exact preparation surface.

use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPreparedSession, ClaudeAgentSdkSessionPreparation,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, CredentialRef, ExecutionHostId, InstanceRevision,
    InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_runtime::{
    Deadline, EnvironmentRef, MonotonicInstant, OperationContent, RequestId, SessionCleanupRequest,
    SessionOptions, TurnRequest, WorkingResourceRef,
};

pub fn prepared_session(host: ExecutionHostId) -> ClaudeAgentSdkPreparedSession {
    prepared_session_with(host, SessionOptions::default())
        .expect("fixture preparation succeeds with no options")
}

pub fn prepared_session_with(
    host: ExecutionHostId,
    options: SessionOptions,
) -> Result<ClaudeAgentSdkPreparedSession, swallowtail_runtime::PreparationFailure> {
    let input = ClaudeAgentSdkSessionPreparation::new(
        ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("valid instance"),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        host,
        InstanceTargetRef::new("claude-agent-sdk.fixture.launch-recipe").expect("valid target"),
        EnvironmentRef::new("claude-agent-sdk.fixture.environment").expect("valid environment"),
        CredentialRef::new("claude-agent-sdk.fixture.delegated-subscription")
            .expect("valid credential"),
        AccessProfileId::new("claude-agent-sdk.fixture.subscription").expect("valid access id"),
        ModelRouteId::new("claude-agent-sdk.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        ModelId::new("claude-sonnet-5").expect("valid model"),
        WorkingResourceRef::new("claude-agent-sdk.fixture.workspace").expect("valid resource"),
        RequestId::new("request-1").expect("valid request"),
        Deadline::at(MonotonicInstant::from_ticks(10_000)),
    );
    prepare_claude_agent_sdk_session(input, options)
}

/// One caller-selected cleanup deadline, far enough ahead that a healthy close
/// completes inside it.
pub fn cleanup_request() -> SessionCleanupRequest {
    SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(10_000)))
}

/// A cleanup deadline the host clock has already passed.
pub fn expired_cleanup_request() -> SessionCleanupRequest {
    SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(1)))
}

pub fn turn_request(id: &str, text: &str) -> TurnRequest {
    TurnRequest::new(
        swallowtail_runtime::RuntimeTurnId::new(id).expect("valid turn id"),
        OperationContent::new(text).expect("valid content"),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10_000)))
}
