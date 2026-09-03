mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{DriverFixture, ServerScenario, cleanup_request};
use swallowtail_adapter_alibaba_model_studio::AlibabaModelStudioDriver;
use swallowtail_core::{ConfiguredInstanceId, SessionProviderStatePolicy, SessionRef};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, LoadSessionRequest, OpenSessionRequest,
    OperationContent, RequestId, RuntimeTurnId, SessionAccessPolicy, SessionPlanAgreement,
    SessionResumeBinding, TerminalStatus, TurnRequest,
};

#[test]
fn retained_open_exposes_exact_bindings_and_preserves_on_close() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let plan = fixture.retained_plan();
    let request = OpenSessionRequest::resource_free_from_plan(
        &plan,
        RequestId::new("retained-open").expect("request id"),
        None,
    )
    .expect("retained request follows plan");
    let mut session =
        block_on(AlibabaModelStudioDriver::new().open_session(plan, request, fixture.services()))
            .expect("retained session opens");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider identity")
            .as_provider_value(),
        "conv_fixture_01"
    );
    assert!(
        session
            .resume_binding()
            .expect("resume binding")
            .is_resource_free()
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("retained-turn").expect("turn id"),
            OperationContent::new("continue retained conversation").expect("content"),
        ),
        fixture.services(),
    ))
    .expect("retained turn starts");
    let mut events = turn.take_events().expect("events");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(session.close(cleanup_request(&fixture), fixture.services())),
        CleanupOutcome::Clean
    );

    let requests = fixture.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].method, "POST");
    assert_eq!(requests[1].target, "/compatible-mode/v1/responses");
    assert!(requests.iter().all(|request| request.method != "DELETE"));
    assert_eq!(fixture.releases(), 1);
}

#[test]
fn retained_load_retrieves_complete_ordered_replay_before_continuation() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let plan = fixture.retained_plan();
    let binding = binding(&plan);
    let request = LoadSessionRequest::resource_free_from_plan(
        &plan,
        RequestId::new("retained-load").expect("request id"),
        binding.clone(),
        None,
    )
    .expect("load follows retained plan");
    let loaded =
        block_on(AlibabaModelStudioDriver::new().load_session(plan, request, fixture.services()))
            .expect("retained conversation loads");
    assert_eq!(
        loaded
            .replay()
            .map(|item| item.content().expect("message content").as_str().to_owned())
            .collect::<Vec<_>>(),
        [
            "First fixture input",
            "Hello world.",
            "Second fixture input",
            "Second output."
        ]
    );
    let (_, mut session) = loaded.into_parts();
    assert_eq!(session.resume_binding(), Some(&binding));
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("loaded-turn").expect("turn id"),
            OperationContent::new("continue after replay").expect("content"),
        ),
        fixture.services(),
    ))
    .expect("loaded session continues");
    let mut events = turn.take_events().expect("events");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        assert_eq!(terminal.await.status(), &TerminalStatus::Completed);
    });
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(session.close(cleanup_request(&fixture), fixture.services())),
        CleanupOutcome::Clean
    );
    let requests = fixture.requests();
    assert_eq!(requests[0].method, "GET");
    assert_eq!(
        requests[1].target,
        "/compatible-mode/v1/conversations/conv_fixture_01/items?limit=100&order=asc"
    );
    assert!(requests[2].target.ends_with("after=msg_output_01"));
    assert!(requests.iter().all(|request| request.method != "DELETE"));
}

#[test]
fn retained_load_rejects_drift_and_provider_failures_without_a_handle() {
    let fixture = DriverFixture::new(ServerScenario::RetainedSuccess);
    let plan = fixture.retained_plan();
    let stale = SessionResumeBinding::resource_free(
        SessionRef::new("conv_fixture_01").expect("session ref"),
        ConfiguredInstanceId::new("foreign.instance").expect("instance id"),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route").clone(),
        plan.model_id().expect("model").clone(),
        SessionAccessPolicy::resource_free(),
    );
    let request = LoadSessionRequest::resource_free(
        RequestId::new("stale-load").expect("request id"),
        stale,
        None,
        retained_agreement(),
    );
    let error =
        block_on(AlibabaModelStudioDriver::new().load_session(plan, request, fixture.services()))
            .err()
            .expect("stale binding rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.alibaba_model_studio.load_binding_mismatch"
    );
    assert!(fixture.requests().is_empty());

    for scenario in [
        ServerScenario::RetainedMissing,
        ServerScenario::RetainedForeign,
        ServerScenario::RetainedMalformed,
        ServerScenario::RetainedOversized,
    ] {
        let fixture = DriverFixture::new(scenario);
        let plan = fixture.retained_plan();
        let request = LoadSessionRequest::resource_free_from_plan(
            &plan,
            RequestId::new("failed-load").expect("request id"),
            binding(&plan),
            None,
        )
        .expect("load request");
        let error = block_on(AlibabaModelStudioDriver::new().load_session(
            plan,
            request,
            fixture.services(),
        ))
        .err()
        .expect("failed retrieval returns no handle");
        if scenario == ServerScenario::RetainedMissing {
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.alibaba_model_studio.provider_resource_missing"
            );
        }
        assert_eq!(fixture.releases(), 1);
        assert!(
            fixture
                .requests()
                .iter()
                .all(|request| request.method != "POST")
        );
    }
}

#[test]
fn retained_load_deadline_joins_transport_before_releasing_access() {
    let fixture = DriverFixture::new(ServerScenario::RetainedWaitForDeadline);
    let plan = fixture.retained_plan();
    let request = LoadSessionRequest::resource_free_from_plan(
        &plan,
        RequestId::new("timed-load").expect("request id"),
        binding(&plan),
        Some(fixture.deadline_after(500)),
    )
    .expect("load request");
    let error =
        block_on(AlibabaModelStudioDriver::new().load_session(plan, request, fixture.services()))
            .err()
            .expect("deadline rejects load");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.alibaba_model_studio.timed_out"
    );
    assert_eq!(fixture.releases(), 1);
    assert_eq!(fixture.release_after_blocking(), [1]);
}

fn binding(plan: &swallowtail_core::PreflightPlan) -> SessionResumeBinding {
    SessionResumeBinding::resource_free(
        SessionRef::new("conv_fixture_01").expect("session ref"),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().expect("route").clone(),
        plan.model_id().expect("model").clone(),
        SessionAccessPolicy::resource_free(),
    )
}

fn retained_agreement() -> SessionPlanAgreement {
    SessionPlanAgreement::explicit(
        SessionAccessPolicy::resource_free(),
        Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
        None,
    )
}
