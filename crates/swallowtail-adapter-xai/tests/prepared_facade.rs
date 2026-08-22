mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::num::NonZeroU64;
use support::{DriverCall, DriverFixture, ServerScenario, turn_request};
use swallowtail_adapter_xai::{
    XaiModelSelection, XaiRunProfileInput, XaiSessionProfileInput, prepare_xai_responses_websocket,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ExecutionHostId,
    InterfaceCompatibilityAssessment, ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, InteractiveSessionHandle, OpenSessionRequest,
    OperationContent, OperationPolicy, ProviderObservation, RequestId, RuntimeEventKind,
    RuntimeTurnId, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
    WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};
use swallowtail_testkit::assert_observable_activity_trace;

#[test]
fn prepared_xai_session_preserves_serial_continuation_cost_and_cleanup_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = DriverFixture::for_host(
            ServerScenario::Success,
            ExecutionHostId::new(host).expect("host id is valid"),
        );
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let operation = prepared
            .prepare_responses_session(XaiSessionProfileInput::new(
                RequestId::new(format!("prepared-{host}")).expect("request id is valid"),
                model(),
                None,
            ))
            .expect("xAI session prepares");
        assert_eq!(operation.plan().execution_host_id().as_str(), host);
        assert!(operation.request().working_resource().is_none());
        assert!(matches!(
            operation
                .evidence()
                .operation()
                .interface_compatibility()
                .next()
                .expect("facade evidence exists")
                .assessment(),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));

        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        for (turn, expected_cost) in [("turn-1", 125_000), ("turn-2", 175_000)] {
            let events = complete_turn(&mut session, &fixture, turn);
            assert_observable_activity_trace(operation.evidence().observable_activity(), &events);
            let cost = events
                .iter()
                .find_map(|event| match event.kind() {
                    RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(
                        cost,
                    )) => Some(cost.amount()),
                    _ => None,
                })
                .expect("billed cost exists");
            assert_eq!(cost, expected_cost);
        }
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
    }
}

#[test]
fn prepared_xai_restoration_opens_a_new_websocket_session() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared =
        prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
            .expect("xAI integration prepares");
    let session = prepared
        .prepare_responses_session(
            XaiSessionProfileInput::new(
                RequestId::new("xai-restoration").expect("request id"),
                qualified_model("grok-4.6"),
                None,
            )
            .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning mode"))
            .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum")),
        )
        .expect("session prepares");
    let interrupted = RuntimeTurnId::new("xai-interrupted").expect("turn id");
    let restoration = session.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, mut replacement) = replacement.into_parts();
    assert!(replacement.provider_session_ref().is_none());
    for turn in ["replacement-turn-1", "replacement-turn-2"] {
        let events = complete_turn(&mut replacement, &fixture, turn);
        assert_observable_activity_trace(session.evidence().observable_activity(), &events);
    }
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.frames().len(), 2);
    let frames = fixture.server.frames();
    assert_wire_controls(&frames[0], Some("high"), Some(512), false);
    assert_wire_controls(&frames[1], Some("high"), Some(512), true);
}

#[test]
fn prepared_xai_binding_drift_rejects_without_provider_effects() {
    let fixture = DriverFixture::new(ServerScenario::Success);
    let prepared =
        prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
            .expect("xAI integration prepares");
    assert!(
        prepared
            .validate_execution_binding(
                &ExecutionHostId::new("host.other").expect("host id is valid"),
                prepared.instance().target_reference(),
            )
            .is_err()
    );
    assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn prepared_one_response_run_preserves_topology_cost_and_cleanup_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = DriverFixture::for_host(
            ServerScenario::OneResponse,
            ExecutionHostId::new(host).expect("host id is valid"),
        );
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let operation = prepared
            .prepare_responses_run(XaiRunProfileInput::new(
                RequestId::new(format!("prepared-run-{host}")).expect("request id is valid"),
                model(),
                OperationContent::new("one prepared response").expect("content is valid"),
                None,
            ))
            .expect("xAI structured run prepares");
        assert_eq!(operation.plan().execution_host_id().as_str(), host);
        assert_eq!(
            operation.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert!(operation.request().working_resource().is_none());

        let mut run = block_on(operation.start_run(fixture.services())).expect("run starts");
        assert!(run.provider_run_ref().is_none());
        let mut events = run.take_events().expect("events exist");
        let terminal = run.take_terminal_outcome().expect("terminal exists");
        let (events, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(operation.evidence().observable_activity(), &events);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
        )));
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(_))
        )));
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.frames().len(), 1);
        assert_eq!(fixture.calls.count(DriverCall::CredentialRelease), 1);
    }
}

#[test]
fn prepared_xai_generation_controls_are_independent_and_exact() {
    for (label, reasoning, maximum) in [
        ("reasoning", Some("high"), None),
        ("output", None, Some(512)),
        ("both", Some("xhigh"), Some(512)),
    ] {
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let mut input = XaiRunProfileInput::new(
            RequestId::new(format!("controls-{label}")).expect("request id is valid"),
            qualified_model("grok-4.6"),
            OperationContent::new("controlled response").expect("content is valid"),
            None,
        );
        if let Some(reasoning) = reasoning {
            input = input.with_reasoning_mode(ReasoningMode::new(reasoning).expect("mode"));
        }
        if let Some(maximum) = maximum {
            input = input.with_maximum_output_tokens(
                NonZeroU64::new(maximum).expect("maximum output is positive"),
            );
        }
        let operation = prepared
            .prepare_responses_run(input)
            .expect("controlled run prepares");
        assert_eq!(
            operation
                .evidence()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            reasoning
        );
        assert_eq!(
            operation
                .evidence()
                .maximum_output_tokens()
                .map(NonZeroU64::get),
            maximum
        );
        assert_eq!(
            operation
                .request()
                .policy()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            reasoning
        );
        assert_eq!(
            operation
                .request()
                .maximum_output_tokens()
                .map(NonZeroU64::get),
            maximum
        );
        assert_generation_requirement(
            operation.plan(),
            Capability::ReasoningSelection,
            reasoning.map(|value| {
                CapabilityConstraint::ReasoningMode(ReasoningMode::new(value).expect("mode"))
            }),
        );
        assert_generation_requirement(
            operation.plan(),
            Capability::OutputTokenLimit,
            maximum.map(CapabilityConstraint::OutputTokenMaximum),
        );
    }
}

#[test]
fn prepared_xai_controls_dispatch_on_run_and_serial_session() {
    let reasoning = ReasoningMode::new("xhigh").expect("reasoning mode is valid");
    let maximum = NonZeroU64::new(512).expect("maximum is positive");

    let run_fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(run_fixture.preparation_input(), &run_fixture.services())
            .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_run(
            XaiRunProfileInput::new(
                RequestId::new("controlled-run").expect("request id"),
                qualified_model("grok-4.6"),
                OperationContent::new("controlled run").expect("content"),
                None,
            )
            .with_reasoning_mode(reasoning.clone())
            .with_maximum_output_tokens(maximum),
        )
        .expect("controlled run prepares");
    let mut run = block_on(operation.start_run(run_fixture.services())).expect("run starts");
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    let outcome = block_on(async {
        while events.next().await.is_some() {}
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let run_frame = run_fixture.server.frames().pop().expect("run frame");
    assert_wire_controls(&run_frame, Some("xhigh"), Some(512), false);

    let session_fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_xai_responses_websocket(
        session_fixture.preparation_input(),
        &session_fixture.services(),
    )
    .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_session(
            XaiSessionProfileInput::new(
                RequestId::new("controlled-session").expect("request id"),
                qualified_model("grok-4.6"),
                None,
            )
            .with_reasoning_mode(reasoning)
            .with_maximum_output_tokens(maximum),
        )
        .expect("controlled session prepares");
    let mut session =
        block_on(operation.open_session(session_fixture.services())).expect("session opens");
    for turn in ["controlled-first", "controlled-second"] {
        let mut handle =
            block_on(session.start_turn(turn_request(turn), session_fixture.services()))
                .expect("turn starts");
        let mut events = handle.take_events().expect("events exist");
        let terminal = handle.take_terminal_outcome().expect("terminal exists");
        block_on(async {
            while events.next().await.is_some() {}
            terminal.await
        });
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let frames = session_fixture.server.frames();
    assert_eq!(frames.len(), 2);
    assert_wire_controls(&frames[0], Some("xhigh"), Some(512), false);
    assert_wire_controls(&frames[1], Some("xhigh"), Some(512), true);
}

#[test]
fn prepared_xai_unqualified_controls_fail_before_provider_effects() {
    for (model_id, reasoning, maximum) in [
        ("grok-4.5-latest", Some("high"), None),
        ("grok-4.5", Some("xhigh"), None),
        ("grok-4.6", None, Some(i32::MAX as u64 + 1)),
    ] {
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let mut input = XaiRunProfileInput::new(
            RequestId::new(format!("rejected-{model_id}")).expect("request id"),
            qualified_model(model_id),
            OperationContent::new("rejected response").expect("content"),
            None,
        );
        if let Some(reasoning) = reasoning {
            input = input.with_reasoning_mode(ReasoningMode::new(reasoning).expect("mode"));
        }
        if let Some(maximum) = maximum {
            input = input
                .with_maximum_output_tokens(NonZeroU64::new(maximum).expect("maximum is positive"));
        }
        let error = prepared
            .prepare_responses_run(input)
            .expect_err("unqualified control is rejected");
        assert!(
            error
                .diagnostic()
                .safe()
                .code()
                .starts_with("swallowtail.xai.preparation.")
        );
        assert_eq!(fixture.calls.count(DriverCall::NetworkAuthorize), 0);
        assert_eq!(fixture.calls.count(DriverCall::CredentialAcquire), 0);
        assert!(fixture.server.frames().is_empty());
    }
}

#[test]
fn prepared_xai_driver_rejects_control_request_drift_before_provider_effects() {
    let run_fixture = DriverFixture::new(ServerScenario::OneResponse);
    let prepared =
        prepare_xai_responses_websocket(run_fixture.preparation_input(), &run_fixture.services())
            .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_run(
            XaiRunProfileInput::new(
                RequestId::new("drift-run").expect("request id"),
                qualified_model("grok-4.6"),
                OperationContent::new("drifted request").expect("content"),
                None,
            )
            .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
            .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum")),
        )
        .expect("controlled run prepares");
    let (_, plan, request) = operation.into_parts();
    let drifted = StructuredRunRequest::new(
        request.request_id().clone(),
        request.content().clone(),
        OperationPolicy::offline(),
    );
    let error = block_on(prepared.low_level_driver().start_run(
        plan,
        drifted,
        run_fixture.services(),
    ))
    .err()
    .expect("request drift is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.xai.generation_control_mismatch"
    );
    assert_eq!(run_fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(run_fixture.calls.count(DriverCall::CredentialAcquire), 0);
    assert!(run_fixture.server.frames().is_empty());

    let session_fixture = DriverFixture::new(ServerScenario::Success);
    let prepared = prepare_xai_responses_websocket(
        session_fixture.preparation_input(),
        &session_fixture.services(),
    )
    .expect("xAI integration prepares");
    let operation = prepared
        .prepare_responses_session(
            XaiSessionProfileInput::new(
                RequestId::new("drift-session").expect("request id"),
                qualified_model("grok-4.6"),
                None,
            )
            .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
            .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum")),
        )
        .expect("controlled session prepares");
    let (_, plan, request) = operation.into_parts();
    let drifted = OpenSessionRequest::resource_free_from_plan(
        &plan,
        request.request_id().clone(),
        request.deadline(),
    )
    .expect("drifted session request derives");
    let error = block_on(prepared.low_level_driver().open_session(
        plan,
        drifted,
        session_fixture.services(),
    ))
    .err()
    .expect("session request drift is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.xai.generation_control_mismatch"
    );
    assert_eq!(session_fixture.calls.count(DriverCall::NetworkAuthorize), 0);
    assert_eq!(
        session_fixture.calls.count(DriverCall::CredentialAcquire),
        0
    );
    assert!(session_fixture.server.frames().is_empty());
}

#[test]
fn prepared_xai_controls_reach_failed_turns_before_chain_invalidation() {
    for scenario in [
        ServerScenario::PreviousResponseNotFound,
        ServerScenario::Disconnect,
    ] {
        let fixture = DriverFixture::new(scenario);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let operation = prepared
            .prepare_responses_session(
                XaiSessionProfileInput::new(
                    RequestId::new("controlled-failure").expect("request id"),
                    qualified_model("grok-4.6"),
                    None,
                )
                .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
                .with_maximum_output_tokens(NonZeroU64::new(512).expect("maximum")),
            )
            .expect("controlled session prepares");
        let mut session =
            block_on(operation.open_session(fixture.services())).expect("session opens");
        let mut turn = block_on(
            session.start_turn(turn_request("controlled-failure-turn"), fixture.services()),
        )
        .expect("turn starts");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let outcome = block_on(async {
            while events.next().await.is_some() {}
            terminal.await
        });
        assert!(matches!(
            outcome.status(),
            TerminalStatus::ProviderFailed(_) | TerminalStatus::RuntimeFailed(_)
        ));
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        let frame = fixture.server.frames().pop().expect("failed turn frame");
        assert_wire_controls(&frame, Some("high"), Some(512), false);
    }
}

fn model() -> XaiModelSelection {
    qualified_model("grok-fixture-exact")
}

fn qualified_model(model_id: &str) -> XaiModelSelection {
    XaiModelSelection::new(
        ModelRouteId::new("xai-grok-fixture").expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("revision is valid"),
        ModelId::new(model_id).expect("model id is valid"),
    )
}

fn assert_generation_requirement(
    plan: &swallowtail_core::PreflightPlan,
    capability: Capability,
    expected: Option<CapabilityConstraint>,
) {
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability);
    match expected {
        Some(expected) => assert_eq!(
            requirement
                .expect("generation capability exists")
                .constraints()
                .collect::<Vec<_>>(),
            vec![&expected]
        ),
        None => assert!(requirement.is_none()),
    }
}

fn assert_wire_controls(frame: &str, reasoning: Option<&str>, maximum: Option<u64>, chained: bool) {
    let value: Value = serde_json::from_str(frame).expect("wire frame parses");
    match reasoning {
        Some(reasoning) => assert_eq!(value["reasoning"]["effort"], reasoning),
        None => assert!(value.get("reasoning").is_none()),
    }
    match maximum {
        Some(maximum) => assert_eq!(value["max_output_tokens"], maximum),
        None => assert!(value.get("max_output_tokens").is_none()),
    }
    if chained {
        assert_eq!(value["previous_response_id"], "resp_fixture_first");
    } else {
        assert!(value.get("previous_response_id").is_none());
    }
}

fn complete_turn(
    session: &mut Box<dyn InteractiveSessionHandle>,
    fixture: &DriverFixture,
    turn: &str,
) -> Vec<swallowtail_runtime::RuntimeEvent> {
    let mut handle =
        block_on(session.start_turn(turn_request(turn), fixture.services())).expect("turn starts");
    let mut stream = handle.take_events().expect("events exist");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    events
}
