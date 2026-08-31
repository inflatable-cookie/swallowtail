#[path = "fixtures/code.rs"]
mod code;
#[path = "fixtures/profiles.rs"]
mod profiles;

use crate::support::{FixtureHost, Scenario};
pub(super) use code::{code_run, response_run};
use futures_executor::block_on;
pub(super) use profiles::{observed_dispositions, profile_contributions};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, ClaudeAgentModelSelection, ClaudeAgentPreparationInput,
    ClaudeAgentPreparationProbe, ClaudeAgentPreparedDelete, ClaudeAgentPreparedIntegration,
    ClaudeAgentPreparedRun, ClaudeAgentPreparedSession, ClaudeAgentRunProfileInput,
    ClaudeAgentSessionManagementInput, ClaudeAgentSessionProfileInput, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, ReasoningMode, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, SessionOptions, WorkingResourceRef,
};

pub(super) const AGENT_HOST: &str = "fixture.projection.agent";

pub(super) fn agent_prepared(revision: &str) -> ClaudeAgentPreparedIntegration {
    agent_prepared_instance("projection.agent.instance", revision)
}

pub(super) fn agent_prepared_instance(
    instance_id: &str,
    revision: &str,
) -> ClaudeAgentPreparedIntegration {
    let host = ExecutionHostId::new(AGENT_HOST).expect("host is valid");
    let fixture = FixtureHost::new(Scenario::Version, "0.61.0");
    block_on(prepare_claude_agent(
        agent_preparation_input(host.clone(), instance_id, revision),
        agent_probe(),
        fixture.services(host),
    ))
    .expect("Claude Agent prepares")
}

pub(super) fn agent_run(
    reasoning: Option<&str>,
    mediated: bool,
    temporary: bool,
) -> ClaudeAgentPreparedRun {
    agent_run_with(agent_prepared("1"), reasoning, mediated, temporary)
}

pub(super) fn agent_run_at_revision(revision: &str) -> ClaudeAgentPreparedRun {
    agent_run_with(agent_prepared(revision), None, false, false)
}

pub(super) fn agent_run_at_instance(instance_id: &str) -> ClaudeAgentPreparedRun {
    agent_run_with(
        agent_prepared_instance(instance_id, "1"),
        None,
        false,
        false,
    )
}

pub(super) fn agent_run_with(
    prepared: ClaudeAgentPreparedIntegration,
    reasoning: Option<&str>,
    mediated: bool,
    temporary: bool,
) -> ClaudeAgentPreparedRun {
    let mut input = ClaudeAgentRunProfileInput::new(
        RequestId::new("projection-agent-run").expect("request is valid"),
        agent_model("run"),
        OperationContent::new("projection fixture").expect("content is valid"),
        resource(),
        None,
    );
    if let Some(reasoning) = reasoning {
        input = input.with_reasoning_mode(mode(reasoning));
    }
    if mediated {
        input = input.with_consumer_mediated_permissions();
    }
    if temporary {
        input = input.with_owned_session_cleanup();
    }
    prepared.prepare_run(input).expect("agent run prepares")
}

pub(super) fn agent_session(reasoning: Option<&str>, mediated: bool) -> ClaudeAgentPreparedSession {
    let prepared = agent_prepared("1");
    let mut options = SessionOptions::default();
    if let Some(reasoning) = reasoning {
        options = options.with_reasoning_mode(mode(reasoning));
    }
    let mut input = ClaudeAgentSessionProfileInput::new(
        RequestId::new("projection-agent-session").expect("request is valid"),
        agent_model("session"),
        resource(),
        options,
    );
    if mediated {
        input = input.with_consumer_mediated_permissions();
    }
    prepared
        .prepare_session(input)
        .expect("agent session prepares")
}

pub(super) fn agent_delete() -> ClaudeAgentPreparedDelete {
    let prepared = agent_prepared("1");
    let session = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("projection-agent-binding").expect("request is valid"),
            agent_model("binding"),
            resource(),
            SessionOptions::default(),
        ))
        .expect("binding session prepares");
    let host_id = ExecutionHostId::new(AGENT_HOST).expect("host is valid");
    let fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let opened =
        block_on(session.open_session(fixture.services(host_id))).expect("binding session opens");
    let binding = opened
        .management_binding()
        .expect("management binding exists")
        .clone();
    assert_eq!(block_on(opened.close()), CleanupOutcome::Clean);
    prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            RequestId::new("projection-agent-delete").expect("request is valid"),
            binding,
        ))
        .expect("delete prepares")
}

fn agent_model(suffix: &str) -> ClaudeAgentModelSelection {
    ClaudeAgentModelSelection::new(
        ModelRouteId::new(format!("projection.agent.{suffix}")).expect("route is valid"),
        ModelRouteRevision::new("1").expect("revision is valid"),
        ModelId::new("claude-sonnet-4-6").expect("model is valid"),
    )
}

fn resource() -> WorkingResourceRef {
    WorkingResourceRef::new("projection.agent.workspace").expect("resource is valid")
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("reasoning mode is valid")
}

fn agent_preparation_input(
    host: ExecutionHostId,
    instance_id: &str,
    revision: &str,
) -> ClaudeAgentPreparationInput {
    let access = AccessProfile::new(
        AccessProfileId::new("projection.agent.access").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("api.anthropic.com").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(
        CredentialRef::new("projection.agent.credential").expect("credential is valid"),
    );
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    ClaudeAgentPreparationInput::new(
        ConfiguredInstanceId::new(instance_id).expect("instance is valid"),
        InstanceRevision::new(revision).expect("revision is valid"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("projection.agent.executable").expect("executable is valid"),
            InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("projection.agent.environment").expect("environment is valid"),
        access,
        PreparedAccessEvidence::caller_asserted(status),
    )
}

fn agent_probe() -> ClaudeAgentPreparationProbe {
    ClaudeAgentPreparationProbe::new(
        RequestId::new("projection-agent-probe").expect("request is valid"),
        ScopeId::new("projection-agent-probe").expect("scope is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}
