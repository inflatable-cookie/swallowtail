use crate::helpers::{config_sets, driver, plan_open_request, wire_methods};
use crate::support::{FixtureHost, Scenario, plan_selection, selection};
use futures_executor::block_on;
use swallowtail_core::{ExecutionHostId, HarnessMode, ResourceAccess};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OpenSessionRequest, OperationContent, RequestId,
    RuntimeTurnId, SessionOptions, SessionPlanAgreement, TerminalStatus, TurnRequest,
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
            Scenario::PlanMissingModes,
            "swallowtail.cline.acp.harness_mode_option_missing",
            0,
        ),
        (
            Scenario::PlanMissingConfig,
            "swallowtail.cline.acp.harness_mode_option_missing",
            0,
        ),
        (
            Scenario::PlanAmbiguousModes,
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            0,
        ),
        (
            Scenario::PlanAmbiguousConfig,
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            0,
        ),
        (
            Scenario::PlanMalformedModes,
            "swallowtail.cline.acp.harness_mode_option_malformed",
            0,
        ),
        (
            Scenario::PlanMalformedConfig,
            "swallowtail.cline.acp.harness_mode_option_malformed",
            0,
        ),
        (
            Scenario::PlanCurrentContradiction,
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            0,
        ),
        (
            Scenario::PlanConfirmationMissing,
            "swallowtail.cline.acp.harness_mode_confirmation_missing",
            1,
        ),
        (
            Scenario::PlanConfirmationAmbiguous,
            "swallowtail.cline.acp.harness_mode_option_ambiguous",
            1,
        ),
        (
            Scenario::PlanConfirmationMalformed,
            "swallowtail.cline.acp.harness_mode_option_malformed",
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
    }
}

#[test]
fn blank_session_id_rejects_before_set_config() {
    let host_id = ExecutionHostId::new("fixture.host.plan.blank-session").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::PlanBlankSessionId);
    let error = block_on(driver().open_session(
        selected.plan,
        plan_open_request("cline-plan-blank-session", selected.resource),
        host.services(host_id),
    ))
    .err()
    .expect("blank session id rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cline.acp.malformed_message"
    );
    assert!(config_sets(&host).is_empty());
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
