mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{DriverCall, DriverFixture, ServerScenario, turn_request};
use swallowtail_adapter_xai::{USD_TICKS_PER_USD, XaiWebSocketDriver, xai_websocket_descriptor};
use swallowtail_core::{CancellationScope, SessionAccessPolicy};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Currency, Deadline, DriverRegistration,
    InteractiveSessionDriver, InteractiveSessionHandle, MonotonicInstant, OpenSessionRequest,
    OperationContent, OperationPolicy, ProviderObservation, RequestId, RunHandle, RuntimeEvent,
    RuntimeEventKind, SchemaDocument, SessionPlanAgreement, StructuredRunDriver,
    StructuredRunRequest, TerminalOutcome, TerminalStatus, ToolDeclaration, TurnHandle,
    WorkingResourceRef,
};

#[test]
fn descriptor_registers_interactive_and_structured_roles_independently() {
    let driver = Arc::new(XaiWebSocketDriver::new());
    let registration = DriverRegistration::new(xai_websocket_descriptor())
        .with_interactive_session(Arc::clone(&driver) as Arc<dyn InteractiveSessionDriver>)
        .expect("interactive role registers")
        .with_structured_run(driver as Arc<dyn StructuredRunDriver>)
        .expect("structured role registers");
    assert!(registration.interactive_session().is_some());
    assert!(registration.structured_run().is_some());
}

#[test]
fn one_response_run_closes_without_exposing_connection_continuation() {
    let fixture = DriverFixture::new(ServerScenario::OneResponse);
    let request = structured_request("one-response");
    let mut run = block_on(XaiWebSocketDriver::new().start_run(
        fixture.run_plan(),
        request,
        fixture.services(),
    ))
    .expect("structured run starts");
    assert!(run.provider_run_ref().is_none());
    assert_eq!(run.cancellation().scope(), CancellationScope::StructuredRun);
    let (events, outcome) = complete_run(&mut run);
    assert_completed(&outcome, "First response.");
    assert_turn_evidence(&events, 125_000, "xai-structured:one-response");
    assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let frames = fixture.server.frames();
    assert_eq!(frames.len(), 1);
    assert!(frames[0].contains(r#""store":false"#));
    assert!(!frames[0].contains("previous_response_id"));
}

#[test]
fn structured_run_cancellation_closes_and_joins_the_connection() {
    let fixture = DriverFixture::new(ServerScenario::WaitForClientClose);
    let mut run = block_on(XaiWebSocketDriver::new().start_run(
        fixture.run_plan(),
        structured_request("cancel-run"),
        fixture.services(),
    ))
    .expect("structured run starts");
    fixture.server.wait_for_frames(1);
    block_on(run.cancellation().request()).expect("cancellation requests");
    let (_, outcome) = complete_run(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
}

#[test]
fn structured_run_tools_reject_before_connection_or_credential_effects() {
    let fixture = DriverFixture::new(ServerScenario::OneResponse);
    let request = structured_request("tool-run").with_tools([ToolDeclaration::new(
        "unexpected_tool",
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024).expect("schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool")]);
    let error = block_on(XaiWebSocketDriver::new().start_run(
        fixture.run_plan(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("tools reject");
    assert_eq!(error.diagnostic().code(), "swallowtail.xai.unsupported");
    assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn serial_session_streams_two_private_continuation_turns() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let mut session = open_session(&fixture, "serial-session");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());

    let (first, first_events, first_outcome) = complete_turn(&mut session, &fixture, "turn-1");
    assert_completed(&first_outcome, "First response.");
    assert_turn_evidence(&first_events, 125_000, "turn-1");

    let second = start_turn(&mut session, &fixture, "turn-2");
    assert_eq!(
        block_on(first.cancellation().request()).expect("late cancellation is accepted"),
        CancellationAcknowledgement::AlreadyRequested
    );
    assert_eq!(block_on(first.close()), CleanupOutcome::NotApplicable);
    let (second, second_events, second_outcome) = complete_handle(second);
    assert_completed(&second_outcome, "Second response.");
    assert_turn_evidence(&second_events, 175_000, "turn-2");
    assert_eq!(block_on(second.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 0);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let frames = fixture.server.frames();
    assert_eq!(frames.len(), 2);
    assert!(frames[1].contains("previous_response_id"));
    assert_eq!(
        fixture.server.handshake(),
        Some((
            "/v1/responses".to_owned(),
            Some("Bearer fixture-secret".to_owned())
        ))
    );
    assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 1);
    assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
    let calls = fixture.calls.calls();
    assert!(
        calls.iter().rposition(|call| *call == DriverCall::TaskJoin)
            < calls
                .iter()
                .rposition(|call| *call == DriverCall::CredentialRelease)
    );
}

#[test]
fn active_turn_rejects_parallel_work_and_cancellation_closes_session_chain() {
    let fixture = DriverFixture::new(ServerScenario::WaitForClientClose);
    let mut session = open_session(&fixture, "cancel-session");
    let mut turn = start_turn(&mut session, &fixture, "turn-active");
    let _events = turn.take_events().expect("events are available");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    fixture.server.wait_for_frames(1);

    let error = block_on(session.start_turn(turn_request("turn-rejected"), fixture.services()))
        .err()
        .expect("parallel turn is rejected");
    assert_eq!(error.diagnostic().code(), "swallowtail.xai.turn_active");
    assert_eq!(fixture.server.frames().len(), 1);

    block_on(turn.cancellation().request()).expect("cancellation is accepted");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    let error = block_on(session.start_turn(turn_request("turn-after-cancel"), fixture.services()))
        .err()
        .expect("closed continuation rejects later turns");
    assert_eq!(error.diagnostic().code(), "swallowtail.xai.session_closed");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn disconnect_fails_once_and_invalidates_private_continuation() {
    let fixture = DriverFixture::new(ServerScenario::Disconnect);
    let mut session = open_session(&fixture, "disconnect-session");
    let (turn, _events, outcome) = complete_turn(&mut session, &fixture, "turn-disconnect");
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);

    let error = block_on(session.start_turn(turn_request("turn-after-loss"), fixture.services()))
        .err()
        .expect("invalid continuation is rejected");
    assert_eq!(error.diagnostic().code(), "swallowtail.xai.session_closed");
    assert_eq!(fixture.server.frames().len(), 1);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn provider_continuation_and_connection_limit_failures_remain_distinct() {
    for (scenario, expected) in [
        (
            ServerScenario::PreviousResponseNotFound,
            "swallowtail.xai.previous_response_not_found",
        ),
        (
            ServerScenario::ConnectionLimit,
            "swallowtail.xai.connection_limit_reached",
        ),
    ] {
        let fixture = DriverFixture::new(scenario);
        let mut session = open_session(&fixture, "provider-failure-session");
        let (turn, _events, outcome) = complete_turn(&mut session, &fixture, "failed-turn");
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("provider failure should remain provider-owned");
        };
        assert_eq!(diagnostic.code(), expected);
        assert!(!format!("{outcome:?}").contains("raw provider"));
        assert!(!format!("{outcome:?}").contains("raw response id"));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn deadline_closes_the_connection_and_reports_timeout_after_join() {
    let fixture = DriverFixture::new(ServerScenario::WaitForClientClose);
    let mut session = open_session(&fixture, "deadline-session");
    let request = turn_request("deadline-turn")
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(u64::MAX)));
    let mut turn = block_on(session.start_turn(request, fixture.services())).expect("turn starts");
    let _events = turn.take_events().expect("events are available");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    assert_eq!(block_on(terminal).status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    let error =
        block_on(session.start_turn(turn_request("turn-after-deadline"), fixture.services()))
            .err()
            .expect("timed-out continuation rejects later turns");
    assert_eq!(error.diagnostic().code(), "swallowtail.xai.session_closed");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.calls.count(DriverCall::TaskJoin), 1);
}

#[test]
fn resource_bound_open_fails_before_network_or_credential_effects() {
    let fixture = DriverFixture::new(ServerScenario::WaitForClientClose);
    let request = OpenSessionRequest::new(
        RequestId::new("resource-bound-session").expect("request id is valid"),
        WorkingResourceRef::new("unexpected-workspace").expect("resource is valid"),
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::read_only(),
            Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
            None,
        ),
    );
    let error = block_on(XaiWebSocketDriver::new().open_session(
        fixture.plan(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("resource-bound session is rejected");
    assert!(matches!(
        error.diagnostic().code(),
        "swallowtail.session_access.plan_mismatch" | "swallowtail.xai.unsupported"
    ));
    assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
}

fn open_session(fixture: &DriverFixture, request_id: &str) -> Box<dyn InteractiveSessionHandle> {
    let plan = fixture.plan();
    let request = OpenSessionRequest::resource_free_from_plan(
        &plan,
        RequestId::new(request_id).expect("request id is valid"),
        None,
    )
    .expect("session request derives from plan");
    block_on(XaiWebSocketDriver::new().open_session(plan, request, fixture.services()))
        .expect("session opens")
}

fn start_turn(
    session: &mut Box<dyn InteractiveSessionHandle>,
    fixture: &DriverFixture,
    turn_id: &str,
) -> Box<dyn TurnHandle> {
    block_on(session.start_turn(turn_request(turn_id), fixture.services())).expect("turn starts")
}

fn complete_turn(
    session: &mut Box<dyn InteractiveSessionHandle>,
    fixture: &DriverFixture,
    turn_id: &str,
) -> (Box<dyn TurnHandle>, Vec<RuntimeEvent>, TerminalOutcome) {
    complete_handle(start_turn(session, fixture, turn_id))
}

fn complete_handle(
    mut turn: Box<dyn TurnHandle>,
) -> (Box<dyn TurnHandle>, Vec<RuntimeEvent>, TerminalOutcome) {
    let mut stream = turn.take_events().expect("events are available");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    (turn, events, outcome)
}

fn complete_run(run: &mut Box<dyn RunHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut stream = run.take_events().expect("events are available");
    let terminal = run
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    })
}

fn structured_request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("fixture input").expect("content is valid"),
        OperationPolicy::offline(),
    )
}

fn assert_completed(outcome: &TerminalOutcome, output: &str) {
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(outcome.output().expect("output exists").as_str(), output);
}

fn assert_turn_evidence(events: &[RuntimeEvent], cost: u64, turn_id: &str) {
    assert_eq!(events[0].kind(), &RuntimeEventKind::Started);
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::OutputDelta)
    );
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    let billed = events
        .iter()
        .find_map(|event| match event.kind() {
            RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(cost)) => {
                Some(cost)
            }
            _ => None,
        })
        .expect("billed cost evidence exists");
    assert_eq!(billed.amount(), cost);
    assert_eq!(billed.currency(), Currency::Usd);
    assert_eq!(billed.units_per_currency().get(), USD_TICKS_PER_USD);
    assert_eq!(billed.turn_id().as_str(), turn_id);
    assert_eq!(billed.model_route_id().as_str(), "xai-grok-fixture");
    assert_eq!(billed.access_profile_id().as_str(), "access.xai.public");
    assert_eq!(billed.provider_attempt().get(), 1);
}
