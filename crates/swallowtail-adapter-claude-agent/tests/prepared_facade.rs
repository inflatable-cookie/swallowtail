#![allow(dead_code)]

#[path = "prepared_facade/session_management.rs"]
mod session_management;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, ClaudeAgentModelSelection, ClaudeAgentPreparationInput,
    ClaudeAgentPreparationProbe, ClaudeAgentRunProfileInput, ClaudeAgentSessionManagementInput,
    ClaudeAgentSessionProfileInput, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CapabilityConstraint,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId,
    HarnessConfigurationPosture, HarnessIsolation, HarnessMode, HostServiceKind, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, OwnedRemoteResourceKind, ProviderSessionAffectedScope,
    ProviderSessionDeletionStrength, ProviderSessionEffectTruth, ResourceAccess, RuntimeReadiness,
    SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, HarnessQuestionId,
    HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputResponse,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId, RuntimeTurnId, ScopeId,
    SessionOptions, TerminalStatus, TurnRequest, WorkingResourceRef, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};
use swallowtail_testkit::{
    assert_observable_activity_trace, assert_prepared_operation_evidence_matches_plan,
};

include!("prepared_facade/session_cases.rs");
include!("prepared_facade/structured_cases.rs");
include!("prepared_facade/permission_cases.rs");
include!("prepared_facade/access_cases.rs");

fn preparation_input(host: ExecutionHostId) -> ClaudeAgentPreparationInput {
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("claude-agent.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("claude-agent.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("claude-agent.prepared.access").expect("valid access"),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("api.anthropic.com").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(
            CredentialRef::new("claude-agent.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn local_preparation_input(host: ExecutionHostId) -> ClaudeAgentPreparationInput {
    let access_id =
        AccessProfileId::new("claude-agent.prepared.local-access").expect("valid access");
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new("claude-agent.prepared.local").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("claude-agent.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("claude-agent.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::SubscriptionAllowance,
            EndpointAudience::new("api.anthropic.com").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        )),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("claude-agent.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> ClaudeAgentPreparationProbe {
    ClaudeAgentPreparationProbe::new(
        RequestId::new("claude-agent-prepared-probe").expect("valid request"),
        ScopeId::new("claude-agent-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
