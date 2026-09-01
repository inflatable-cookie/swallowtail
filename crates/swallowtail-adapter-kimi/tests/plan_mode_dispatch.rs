use crate::support;

use futures_executor::block_on;
use support::{
    CleanupEvent, FixtureHost, Scenario, plan_reasoning_selection, plan_selection,
    version_selection,
};
use swallowtail_adapter_kimi::KimiAcpDriver;
use swallowtail_core::{
    ExecutionHostId, HarnessMode, InterfaceCompatibilityAssessment, ReasoningMode, ResourceAccess,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, RequestId, SessionAccessPolicy,
    SessionOptions, SessionPlanAgreement, WorkingResourceRef,
};

#[test]
fn qualified_and_unverified_versions_dispatch_one_plan_selection() {
    for topology in [
        swallowtail_testkit::ExecutionTopologyFixture::local(),
        swallowtail_testkit::ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for (version, scenario, qualified) in [
            ("0.28.1", Scenario::PlanLegacySuccess, true),
            ("0.29.0", Scenario::PlanSuccess, true),
            ("0.38.0", Scenario::PlanCeilingSuccess, true),
        ] {
            let host_id = topology.execution_host_id().clone();
            let selected = plan_selection(host_id.clone(), version);
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
                plan_open_request("kimi-plan-open", selected.resource, None),
                host.services(host_id),
            ))
            .expect("plan-mode session opens");
            assert_eq!(
                host.wire_methods(),
                ["initialize", "session/new", "session/set_config_option"]
            );
            let set = config_sets(&host);
            assert_eq!(set.len(), 1);
            assert_eq!(set[0]["params"]["configId"], "mode");
            assert_eq!(set[0]["params"]["value"], "plan");
            assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
            assert_eq!(host.cleanup_events(), joined_cleanup());
        }
    }
}

#[test]
fn omitted_plan_mode_preserves_the_reasoning_only_wire() {
    let host_id = ExecutionHostId::new("fixture.host.plan.omit").expect("valid host id");
    let selected = version_selection(host_id.clone(), "0.29.0");
    let host = FixtureHost::new(Scenario::PlanSuccess);
    let session = block_on(driver(selected.credential).open_session(
        selected.plan,
        empty_open_request("kimi-plan-omitted", selected.resource),
        host.services(host_id),
    ))
    .expect("omitted plan mode opens");
    assert_eq!(host.wire_methods(), ["initialize", "session/new"]);
    assert!(config_sets(&host).is_empty());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn admitted_reasoning_values_compose_before_plan_mode() {
    for (version, mode, scenario) in [
        ("0.28.1", "off", Scenario::ReasoningLegacySuccess),
        ("0.28.1", "on", Scenario::ReasoningLegacySuccess),
        ("0.29.0", "off", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "on", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "low", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "medium", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "high", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "xhigh", Scenario::ReasoningEffortSuccess),
        ("0.29.0", "max", Scenario::ReasoningEffortSuccess),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.host.plan.compose.{version}.{mode}"))
            .expect("valid host id");
        let selected = plan_reasoning_selection(host_id.clone(), version, mode);
        let host = FixtureHost::new(scenario);
        let session = block_on(driver(selected.credential).open_session(
            selected.plan,
            plan_open_request(
                "kimi-plan-compose",
                selected.resource,
                Some(ReasoningMode::new(mode).expect("valid mode")),
            ),
            host.services(host_id),
        ))
        .expect("composed session opens");
        assert_eq!(
            host.wire_methods(),
            [
                "initialize",
                "session/new",
                "session/set_config_option",
                "session/set_config_option"
            ]
        );
        let set = config_sets(&host);
        assert_eq!(set.len(), 2);
        assert_eq!(set[0]["params"]["configId"], "thinking");
        assert_eq!(set[0]["params"]["value"], mode);
        assert_eq!(set[1]["params"]["configId"], "mode");
        assert_eq!(set[1]["params"]["value"], "plan");
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn plan_shape_rejections_abort_and_join_all_attachment_work() {
    for (scenario, expected_code, expected_sets) in [
        (
            Scenario::PlanMissing,
            "swallowtail.kimi.acp.harness_mode_option_missing",
            0,
        ),
        (
            Scenario::PlanAmbiguous,
            "swallowtail.kimi.acp.harness_mode_option_ambiguous",
            0,
        ),
        (
            Scenario::PlanMalformed,
            "swallowtail.kimi.acp.harness_mode_option_malformed",
            0,
        ),
        (
            Scenario::PlanUnknownRow,
            "swallowtail.kimi.acp.harness_mode_option_malformed",
            0,
        ),
        (
            Scenario::PlanConfirmationMissing,
            "swallowtail.kimi.acp.harness_mode_confirmation_missing",
            1,
        ),
        (
            Scenario::PlanDrift,
            "swallowtail.kimi.acp.harness_mode_mismatch",
            1,
        ),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.host.plan.reject.{expected_code}"))
            .expect("valid host id");
        let selected = plan_selection(host_id.clone(), "0.29.0");
        let host = FixtureHost::new(scenario);
        let error = block_on(driver(selected.credential).open_session(
            selected.plan,
            plan_open_request("kimi-plan-rejected", selected.resource, None),
            host.services(host_id),
        ))
        .err()
        .expect("plan mismatch rejects");
        assert_eq!(error.diagnostic().code(), expected_code);
        assert_eq!(config_sets(&host).len(), expected_sets);
        let diagnostic = format!("{error:?}");
        assert!(!diagnostic.contains("configOptions"));
        assert!(!diagnostic.contains("configId"));
        assert_eq!(host.cleanup_counts(), (1, 1));
        assert_eq!(host.cleanup_events(), joined_cleanup());
    }
}

#[test]
fn plan_failure_after_reasoning_still_joins_attachment_work() {
    let host_id = ExecutionHostId::new("fixture.host.plan.after-reasoning").expect("valid host id");
    let selected = plan_reasoning_selection(host_id.clone(), "0.29.0", "high");
    let host = FixtureHost::new(Scenario::PlanRejected);
    let error = block_on(driver(selected.credential).open_session(
        selected.plan,
        plan_open_request(
            "kimi-plan-after-reasoning",
            selected.resource,
            Some(ReasoningMode::new("high").expect("valid mode")),
        ),
        host.services(host_id),
    ))
    .err()
    .expect("later plan rejection aborts");
    let diagnostic = format!("{error:?}");
    assert!(!diagnostic.contains("configOptions"));
    let set = config_sets(&host);
    assert_eq!(set.len(), 2);
    assert_eq!(set[0]["params"]["configId"], "thinking");
    assert_eq!(set[1]["params"]["configId"], "mode");
    assert_eq!(host.cleanup_events(), joined_cleanup());
}

fn driver(credential: swallowtail_core::CredentialRef) -> KimiAcpDriver {
    KimiAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("kimi.fixture.isolated-state")
            .expect("valid environment"),
        credential,
    )
}

fn empty_open_request(id: &str, resource: WorkingResourceRef) -> OpenSessionRequest {
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
}

fn plan_open_request(
    id: &str,
    resource: WorkingResourceRef,
    reasoning: Option<ReasoningMode>,
) -> OpenSessionRequest {
    let mut options = SessionOptions::default().with_harness_mode(HarnessMode::Plan);
    if let Some(mode) = reasoning {
        options = options.with_reasoning_mode(mode);
    }
    empty_open_request(id, resource).with_options(options)
}

fn config_sets(host: &FixtureHost) -> Vec<serde_json::Value> {
    host.wire_messages()
        .into_iter()
        .filter(|message| {
            message.get("method").and_then(serde_json::Value::as_str)
                == Some("session/set_config_option")
        })
        .collect()
}

fn joined_cleanup() -> [CleanupEvent; 3] {
    [
        CleanupEvent::ProcessWait,
        CleanupEvent::ResourceRelease,
        CleanupEvent::CredentialRelease,
    ]
}
