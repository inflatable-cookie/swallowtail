mod support;

use futures_executor::block_on;
use support::{FixtureHost, Scenario, plan_selection, selection};
use swallowtail_adapter_cline::ClineAcpDriver;
use swallowtail_core::{ExecutionHostId, HarnessMode, ProviderRequestHandling, ResourceAccess};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent, RequestId,
    RuntimeTurnId, SessionOptions, SessionPlanAgreement, TerminalStatus, TurnRequest,
    WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

#[test]
fn plan_mode_dispatches_one_set_config_before_prompt() {
    let host_id = ExecutionHostId::new("fixture.host.plan.open").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let mut session = block_on(driver().open_session(
        selected.plan,
        plan_open_request("cline-plan-open", selected.resource),
        services.clone(),
    ))
    .expect("plan-mode session opens");
    assert_eq!(
        wire_methods(&host),
        ["initialize", "session/new", "session/set_config_option"]
    );
    let set = config_sets(&host);
    assert_eq!(set.len(), 1);
    assert_eq!(set[0]["params"]["configId"], "mode");
    assert_eq!(set[0]["params"]["value"], "plan");
    assert_eq!(set[0]["params"]["sessionId"], "opaque-fixture-session");

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("cline-plan-turn").expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services,
    ))
    .expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        wire_methods(&host),
        [
            "initialize",
            "session/new",
            "session/set_config_option",
            "session/prompt"
        ]
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(host.releases(), 1);
}

#[test]
fn omitted_plan_mode_keeps_initialize_and_session_new_only() {
    let host_id = ExecutionHostId::new("fixture.host.plan.omit").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let session = block_on(driver().open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("cline-plan-omitted").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        host.services(host_id),
    ))
    .expect("omitted plan mode opens");
    assert_eq!(wire_methods(&host), ["initialize", "session/new"]);
    assert!(config_sets(&host).is_empty());
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn plan_shape_rejections_abort_and_join_owned_work() {
    for (scenario, expected_code, expected_sets) in [
        (
            Scenario::PlanMissing,
            "swallowtail.cline.acp.harness_mode_option_missing",
            0,
        ),
        (
            Scenario::PlanAmbiguous,
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            0,
        ),
        (
            Scenario::PlanMalformed,
            "swallowtail.cline.acp.harness_mode_option_malformed",
            0,
        ),
        (
            Scenario::PlanConfirmationMissing,
            "swallowtail.cline.acp.harness_mode_confirmation_missing",
            1,
        ),
        (
            Scenario::PlanDrift,
            "swallowtail.cline.acp.harness_mode_mismatch",
            1,
        ),
        (
            Scenario::PlanRejected,
            "swallowtail.cline.acp.request_rejected",
            1,
        ),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.host.plan.reject.{expected_code}"))
            .expect("valid host id");
        let selected = plan_selection(host_id.clone());
        let host = FixtureHost::new(scenario);
        let error = block_on(driver().open_session(
            selected.plan,
            plan_open_request("cline-plan-rejected", selected.resource),
            host.services(host_id),
        ))
        .err()
        .expect("plan mismatch rejects");
        assert_eq!(error.diagnostic().code(), expected_code);
        assert_eq!(config_sets(&host).len(), expected_sets);
        let diagnostic = format!("{error:?}");
        assert!(!diagnostic.contains("configOptions"));
        assert!(!diagnostic.contains("configId"));
        assert_eq!(host.releases(), 1);
        assert!(!host.process_started() || host.releases() == 1);
    }
}

#[test]
fn plan_provider_rejection_aborts_joined_work() {
    let host_id = ExecutionHostId::new("fixture.host.plan.provider-reject").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::PlanRejected);
    assert!(
        block_on(driver().open_session(
            selected.plan,
            plan_open_request("cline-plan-provider-rejected", selected.resource),
            host.services(host_id),
        ))
        .is_err()
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn plan_mode_mismatch_rejects_before_spawn() {
    let host_id = ExecutionHostId::new("fixture.host.plan.mismatch").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let error = block_on(
        driver().open_session(
            selected.plan,
            OpenSessionRequest::new(
                RequestId::new("cline-plan-mismatch").expect("valid request"),
                selected.resource,
                None,
                SessionPlanAgreement::explicit(
                    swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                    Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                    Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
                ),
            )
            .with_options(SessionOptions::default().with_harness_mode(HarnessMode::Plan)),
            host.services(host_id),
        ),
    )
    .err()
    .expect("request/plan mismatch rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cline.acp.harness_mode_mismatch"
    );
    assert!(!host.process_started());
    assert_eq!(host.releases(), 0);
}

#[test]
fn plan_mode_fresh_replacement_renegotiates_immutable_selection() {
    let host_id = ExecutionHostId::new("fixture.host.plan.replace").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let request = plan_open_request("cline-plan-replace", selected.resource);
    let session =
        block_on(driver().open_session(selected.plan.clone(), request.clone(), services.clone()))
            .expect("first plan session opens");
    assert_eq!(config_sets(&host).len(), 1);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let restoration =
        swallowtail_runtime::PreparedWorkingStateRestoration::fresh_session_replacement(
            RuntimeTurnId::new("cline-plan-interrupted").expect("valid turn"),
            driver(),
            selected.plan,
            request,
        );
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );
    let outcome = block_on(restoration.restore(services)).expect("replacement restores");
    match outcome {
        WorkingStateRestorationOutcome::SessionReplaced(replaced) => {
            let (interrupted, session) = replaced.into_parts();
            assert_eq!(interrupted.as_str(), "cline-plan-interrupted");
            assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        }
        _ => panic!("expected SessionReplaced"),
    }
    assert_eq!(config_sets(&host).len(), 2);
    assert!(
        config_sets(&host)
            .iter()
            .all(|message| message["params"]["value"] == "plan")
    );
    assert_eq!(host.releases(), 2);
}

#[test]
fn plan_mode_does_not_select_allow_always_on_permission() {
    let host_id = ExecutionHostId::new("fixture.host.plan.permission").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Permission);
    let services = host.services(host_id);
    let mut session = block_on(driver().open_session(
        selected.plan,
        plan_open_request("cline-plan-permission", selected.resource),
        services.clone(),
    ))
    .expect("plan session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("cline-plan-permission-turn").expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services,
    ))
    .expect("turn starts");
    let outcome = block_on(turn.take_terminal_outcome().expect("terminal"));
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    assert!(host.writes().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["outcome"] == "cancelled"
            && message["result"]["outcome"].get("optionId")
                != Some(&serde_json::json!("allow_always"))
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

fn driver() -> ClineAcpDriver {
    ClineAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("cline.fixture.isolated")
            .expect("valid environment"),
    )
}

fn plan_open_request(
    id: &str,
    resource: swallowtail_runtime::WorkingResourceRef,
) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new(id).expect("valid request"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
            Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
            Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
        ),
    )
    .with_options(SessionOptions::default().with_harness_mode(HarnessMode::Plan))
}

fn wire_methods(host: &FixtureHost) -> Vec<String> {
    host.writes()
        .iter()
        .filter_map(|message| {
            message
                .get("method")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .collect()
}

fn config_sets(host: &FixtureHost) -> Vec<serde_json::Value> {
    host.writes()
        .into_iter()
        .filter(|message| {
            message.get("method").and_then(serde_json::Value::as_str)
                == Some("session/set_config_option")
        })
        .collect()
}

#[allow(dead_code)]
fn _provider_request_handling(_: ProviderRequestHandling) {}
