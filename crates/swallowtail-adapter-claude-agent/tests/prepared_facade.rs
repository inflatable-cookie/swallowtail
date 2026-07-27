#![allow(dead_code)]

#[path = "prepared_facade/session_management.rs"]
mod session_management;
mod support;

use futures_executor::block_on;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, ClaudeAgentModelSelection, ClaudeAgentPreparationInput,
    ClaudeAgentPreparationProbe, ClaudeAgentSessionManagementInput, ClaudeAgentSessionProfileInput,
    prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, HarnessConfigurationPosture,
    HarnessIsolation, InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId,
    ModelRouteRevision, ProviderSessionAffectedScope, ProviderSessionDeletionStrength,
    ProviderSessionEffectTruth, ResourceAccess, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, RequestId, ScopeId,
    SessionOptions, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_sessions_bind_version_access_model_and_ambient_read_policy() {
    for host_value in ["fixture.prepared.local", "fixture.prepared.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_session(ClaudeAgentSessionProfileInput::new(
                RequestId::new("claude-agent-prepared-open").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                SessionOptions::default(),
            ))
            .expect("session profile prepares");

        assert_eq!(
            profile
                .evidence()
                .observation()
                .version()
                .version()
                .as_str(),
            "0.61.0"
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert_eq!(
            profile.plan().model_id().map(ModelId::as_str),
            Some("claude-sonnet-4-6")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let session = block_on(profile.open_session(operation_host.services(host_id.clone())))
            .expect("prepared session opens");
        let binding = session
            .management_binding()
            .expect("prepared session returns lifecycle binding")
            .clone();
        assert!(binding.supports(Capability::ProviderNativeSessionClose));
        assert!(binding.supports(Capability::ProviderSessionDelete));
        assert!(session.resume_binding().is_none());
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert!(operation_host.writes().iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
        }));
        assert_eq!(operation_host.credential_acquires(), 1);
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);

        let delete = prepared
            .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
                RequestId::new(format!("claude-agent-delete-{host_value}"))
                    .expect("valid delete request"),
                binding,
            ))
            .expect("qualified Claude Agent delete prepares");
        assert_prepared_operation_evidence_matches_plan(
            delete.evidence().operation(),
            delete.plan().preflight(),
        );
        let delete_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let outcome = block_on(delete.execute(delete_host.services(host_id)))
            .expect("prepared Claude Agent delete executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            outcome.effect().confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
        );
        assert_eq!(
            outcome.effect().affected_scope(),
            Some(ProviderSessionAffectedScope::ProviderDefinedDescendants)
        );
        let writes = delete_host.writes();
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
        }));
        assert!(!writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/new")
        }));
        assert_eq!(delete_host.credential_acquires(), 1);
        assert_eq!(delete_host.credential_releases(), 1);
        assert_eq!(delete_host.resource_releases(), 1);
    }
}

#[test]
fn unsupported_options_fail_before_session_process_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.options").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id),
    ))
    .expect("Claude Agent prepares");
    let result = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("claude-agent-options").expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_reasoning_mode(
            swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
        ),
    ));
    assert!(result.is_err());
}

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
