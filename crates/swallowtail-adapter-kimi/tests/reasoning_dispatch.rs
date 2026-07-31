use crate::support;

use futures_executor::block_on;
use support::{CleanupEvent, FixtureHost, Scenario, reasoning_selection};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{
    ExecutionHostId, InterfaceCompatibilityAssessment, ReasoningMode, ResourceAccess,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, RequestId, SessionAccessPolicy,
    SessionOptions, SessionPlanAgreement, WorkingResourceRef,
};

#[test]
fn qualified_and_unverified_versions_dispatch_one_reasoning_selection() {
    for topology in [
        swallowtail_testkit::ExecutionTopologyFixture::local(),
        swallowtail_testkit::ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for (version, mode, scenario, qualified) in [
            ("0.28.1", "on", Scenario::ReasoningLegacySuccess, true),
            ("0.29.0", "high", Scenario::ReasoningEffortSuccess, true),
            ("0.29.0", "on", Scenario::ReasoningEffortSuccess, true),
            ("0.29.1", "high", Scenario::ReasoningEffort291Success, true),
            ("0.29.2", "high", Scenario::ReasoningEffort292Success, true),
            ("0.30.0", "high", Scenario::ReasoningEffort300Success, true),
            ("0.31.0", "high", Scenario::ReasoningEffort310Success, true),
            ("0.31.1", "high", Scenario::ReasoningEffort311Success, true),
            ("0.32.0", "high", Scenario::ReasoningNewerSuccess, false),
        ] {
            let host_id = topology.execution_host_id().clone();
            let selected = reasoning_selection(host_id.clone(), version, mode);
            let requested = ReasoningMode::new(mode).expect("valid mode");
            swallowtail_testkit::assert_negotiated_reasoning_setup_contract(
                &selected.plan,
                requested.clone(),
                ReasoningMode::new("different").expect("valid different mode"),
            );
            let binding = selected
                .plan
                .interface_versions()
                .next()
                .expect("Kimi binding is planned");
            assert_eq!(
                matches!(
                    selected.plan.assess_interface_version(binding),
                    InterfaceCompatibilityAssessment::Qualified(_)
                ),
                qualified
            );
            let host = FixtureHost::new(scenario);
            let session = block_on(driver(selected.credential).open_session(
                selected.plan,
                open_request("kimi-reasoning-open", selected.resource, requested),
                host.services(host_id),
            ))
            .expect("reasoning session opens");
            assert_eq!(
                host.wire_methods(),
                ["initialize", "session/new", "session/set_config_option"]
            );
            let set = host
                .wire_messages()
                .into_iter()
                .find(|message| {
                    message.get("method").and_then(serde_json::Value::as_str)
                        == Some("session/set_config_option")
                })
                .expect("one reasoning request is present");
            assert_eq!(set["params"]["configId"], "thinking");
            assert_eq!(set["params"]["value"], mode);
            assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
            assert_eq!(host.cleanup_events(), joined_cleanup());
        }
    }
}

#[test]
fn reasoning_shape_rejections_abort_and_join_all_attachment_work() {
    for (scenario, mode, expected_code, expected_sets) in [
        (
            Scenario::ReasoningMissing,
            "high",
            "swallowtail.kimi.acp.reasoning_option_missing",
            0,
        ),
        (
            Scenario::ReasoningAmbiguous,
            "high",
            "swallowtail.kimi.acp.reasoning_option_ambiguous",
            0,
        ),
        (
            Scenario::ReasoningMalformed,
            "high",
            "swallowtail.kimi.acp.reasoning_option_malformed",
            0,
        ),
        (
            Scenario::ReasoningAlwaysThinking,
            "off",
            "swallowtail.kimi.acp.reasoning_value_unsupported",
            0,
        ),
        (
            Scenario::ReasoningConfirmationMissing,
            "high",
            "swallowtail.kimi.acp.reasoning_confirmation_missing",
            1,
        ),
        (
            Scenario::ReasoningDrift,
            "high",
            "swallowtail.negotiated_reasoning.effective_mismatch",
            1,
        ),
    ] {
        let host_id = ExecutionHostId::new(format!(
            "fixture.host.reasoning.reject.{mode}.{expected_code}"
        ))
        .expect("valid host id");
        let selected = reasoning_selection(host_id.clone(), "0.29.0", mode);
        let host = FixtureHost::new(scenario);
        let error = block_on(driver(selected.credential).open_session(
            selected.plan,
            open_request(
                "kimi-reasoning-rejected",
                selected.resource,
                ReasoningMode::new(mode).expect("valid mode"),
            ),
            host.services(host_id),
        ))
        .err()
        .expect("reasoning mismatch rejects");
        assert_eq!(error.diagnostic().code(), expected_code);
        assert_eq!(
            host.wire_methods()
                .iter()
                .filter(|method| method.as_str() == "session/set_config_option")
                .count(),
            expected_sets
        );
        let diagnostic = format!("{error:?}");
        assert!(!diagnostic.contains("thinking"));
        assert!(!diagnostic.contains("configOptions"));
        assert_eq!(host.cleanup_counts(), (1, 1));
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

fn driver(credential: swallowtail_core::CredentialRef) -> KimiAcpDriver {
    KimiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("kimi.fixture.isolated-state")
            .expect("valid environment"),
        credential,
    )
}

fn open_request(id: &str, resource: WorkingResourceRef, mode: ReasoningMode) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("valid request"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
            Some(SessionProviderStatePolicy::Prohibited),
            None,
        ),
    )
    .with_options(SessionOptions::default().with_reasoning_mode(mode))
}

fn joined_cleanup() -> [CleanupEvent; 3] {
    [
        CleanupEvent::ProcessWait,
        CleanupEvent::ResourceRelease,
        CleanupEvent::CredentialRelease,
    ]
}
