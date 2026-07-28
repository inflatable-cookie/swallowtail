use super::support::selection::{open_request, selection};
use super::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_BASELINE_VERSION, CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION,
    ClaudeAgentAcpDriver, claude_agent_acp_claim, claude_agent_acp_descriptor,
};
use swallowtail_core::{
    Capability, CredentialMechanism, ExecutionHostId, ExternalNetworkPolicy, ExternalSearchPolicy,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    InterfaceCompatibilityAssessment, InterfaceVersion, ProviderApprovalPolicy, ResourceAccess,
};
use swallowtail_runtime::{EnvironmentRef, InteractiveSessionDriver};
use swallowtail_testkit::{
    ClosedSemanticWindowCase, ConformanceAssertion, SyntheticProfile,
    assert_closed_semantic_compatibility_window, assert_unverified_newer_execution,
    run_acp_single_turn_projection_assertions, run_long_lived_acp_profile,
};

#[test]
fn unchanged_long_lived_acp_profile_covers_the_portable_subset() {
    let report = run_long_lived_acp_profile();
    assert_eq!(report.profile(), SyntheticProfile::LongLivedAcpHarness);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::StalePlanRejected,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::ProcessLifecycle,
        ConformanceAssertion::WorkingResourceCallback,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn provider_neutral_acp_projection_keeps_retention_callbacks_and_close_exact() {
    let report = run_acp_single_turn_projection_assertions();
    assert_eq!(report.profile(), SyntheticProfile::LongLivedAcpHarness);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::CallbackExchange,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn qualified_range_and_unverified_newer_keep_distinct_support_truth() {
    let claim = claude_agent_acp_claim();
    let case = ClosedSemanticWindowCase::new(
        version(CLAUDE_AGENT_ACP_BASELINE_VERSION),
        version(CLAUDE_AGENT_ACP_LATEST_QUALIFIED_VERSION),
    )
    .with_accepted([
        version("0.54.0"),
        version("0.54.1"),
        version("0.59.0"),
        version("0.60.0"),
    ])
    .with_rejected([
        version("0.52.0"),
        version("0.58.0"),
        version("0.61.0-rc.1"),
        version("0.62.0"),
    ]);
    assert_closed_semantic_compatibility_window(&claim, &case);
    assert_unverified_newer_execution(&claim, &version("0.62.0"));
    assert_eq!(claim.milestones().len(), 4);
}

#[test]
fn both_topologies_bind_public_api_ambient_read_only_authority() {
    let descriptor = claude_agent_acp_descriptor();
    let session_services = descriptor
        .required_host_services(swallowtail_core::DriverRole::InteractiveSession)
        .collect::<Vec<_>>();
    assert!(!session_services.contains(&HostServiceKind::Credential));
    assert!(session_services.contains(&HostServiceKind::WorkingResourceIo));
    assert!(!session_services.contains(&HostServiceKind::Network));

    for host in ["fixture.host.local", "fixture.host.remote-authoritative"] {
        let host = ExecutionHostId::new(host).expect("valid host");
        let selected = selection(host.clone(), "0.61.0");
        let plan = selected.plan;
        assert_eq!(plan.execution_host_id(), &host);
        assert_eq!(plan.credential_mechanism(), &CredentialMechanism::ApiKey);
        assert!(
            plan.requirements()
                .host_services()
                .any(|service| service == HostServiceKind::Credential)
        );
        assert_eq!(plan.endpoint_audience().as_str(), "api.anthropic.com");
        assert_eq!(
            plan.model_id().expect("model is bound").as_str(),
            "claude-sonnet-4-6"
        );
        assert_eq!(
            plan.harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            plan.requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        let access = plan
            .requirements()
            .session_access_policy()
            .expect("session access is bound");
        assert_eq!(access.resource_access(), Some(ResourceAccess::Read));
        assert_eq!(access.filesystem_boundary(), None);
        assert_eq!(
            access.harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(access.approval_policy(), ProviderApprovalPolicy::Never);
        assert_eq!(
            access.external_network(),
            ExternalNetworkPolicy::AmbientHost
        );
        assert_eq!(access.external_search(), ExternalSearchPolicy::Disabled);

        let capabilities = plan
            .requirements()
            .capabilities()
            .map(|requirement| requirement.capability())
            .collect::<Vec<_>>();
        for unavailable in [
            Capability::WorkingResourceTextWrite,
            Capability::LoadSession,
            Capability::Resume,
            Capability::ToolCalls,
            Capability::ProviderExternalNetwork,
            Capability::ExternalSearch,
        ] {
            assert!(!capabilities.contains(&unavailable));
        }
    }
}

#[test]
fn terminal_auth_advertisement_fails_and_releases_both_leases() {
    let host_id = ExecutionHostId::new("fixture.host.auth-drift").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::AuthDrift, "0.61.0");
    let driver = ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        selected.credential,
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        open_request("auth-drift-open", selected.resource),
        host.services(host_id),
    )) {
        Ok(_) => panic!("terminal auth advertisement must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.terminal_auth_rejected"
    );
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(host.credential_releases(), 1);
}

#[test]
fn missing_delete_capability_stops_before_session_or_management_effects() {
    let host_id = ExecutionHostId::new("fixture.host.lifecycle-drift").expect("valid host");
    let selected = selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::LifecycleDrift, "0.61.0");
    let driver = ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("valid environment"),
        selected.credential,
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        open_request("lifecycle-drift-open", selected.resource),
        host.services(host_id),
    )) {
        Ok(_) => panic!("missing negotiated deletion capability must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.lifecycle_capability_drift"
    );
    let writes = host.writes();
    assert!(!writes.iter().any(|message| {
        matches!(
            message.get("method").and_then(serde_json::Value::as_str),
            Some("session/new" | "session/close" | "session/delete")
        )
    }));
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(host.credential_releases(), 1);
}

#[test]
fn newer_plan_stays_unverified_after_preflight() {
    let selected = selection(
        ExecutionHostId::new("fixture.host.newer").expect("valid host"),
        "0.62.0",
    );
    let binding = selected
        .plan
        .interface_versions()
        .next()
        .expect("version is bound");
    assert!(selected.plan.classify_interface_version(binding).is_none());
    assert!(matches!(
        selected.plan.assess_interface_version(binding),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
