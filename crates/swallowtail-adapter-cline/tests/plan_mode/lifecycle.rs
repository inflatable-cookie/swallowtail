use crate::helpers::{config_sets, driver, plan_open_request, wire_methods};
use crate::support::{FixtureHost, Scenario, plan_selection};
use futures_executor::block_on;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OperationContent, RuntimeTurnId, TerminalStatus,
    TurnRequest, WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};

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
fn plan_disconnect_during_negotiation_joins_owned_work() {
    let host_id = ExecutionHostId::new("fixture.host.plan.disconnect").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::PlanDisconnect);
    let error = block_on(driver().open_session(
        selected.plan,
        plan_open_request("cline-plan-disconnect", selected.resource),
        host.services(host_id),
    ))
    .err()
    .expect("disconnect rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.cline.acp.connection_ended"
    );
    assert_eq!(config_sets(&host).len(), 1);
    assert_eq!(host.releases(), 1);
}

#[test]
fn same_session_second_turn_does_not_reselect_mode() {
    let host_id = ExecutionHostId::new("fixture.host.plan.second-turn").expect("valid host id");
    let selected = plan_selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let mut session = block_on(driver().open_session(
        selected.plan,
        plan_open_request("cline-plan-second-turn", selected.resource),
        services.clone(),
    ))
    .expect("plan session opens");
    assert_eq!(config_sets(&host).len(), 1);
    for turn_name in ["cline-plan-turn-a", "cline-plan-turn-b"] {
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(turn_name).expect("valid turn"),
                OperationContent::new("private fixture prompt").expect("valid prompt"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let outcome = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    }
    assert_eq!(config_sets(&host).len(), 1);
    assert_eq!(
        wire_methods(&host)
            .into_iter()
            .filter(|method| method == "session/prompt")
            .count(),
        2
    );
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
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
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services.clone())),
        CleanupOutcome::Clean
    );

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
    let outcome = block_on(restoration.restore(services.clone())).expect("replacement restores");
    match outcome {
        WorkingStateRestorationOutcome::SessionReplaced(replaced) => {
            let (interrupted, session) = replaced.into_parts();
            assert_eq!(interrupted.as_str(), "cline-plan-interrupted");
            assert_eq!(
                block_on(session.close(host.cleanup_request(), services)),
                CleanupOutcome::Clean
            );
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
        services.clone(),
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
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}
