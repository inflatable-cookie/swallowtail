mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_xai::{
    XaiRunProfileInput, XaiSessionProfileInput, prepare_xai_responses_websocket,
};
use swallowtail_core::{
    DriverRole, ExecutionHostId, InterfaceCompatibilityAssessment, ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, ProviderObservation, RequestId, RuntimeEventKind,
    RuntimeTurnId, TerminalStatus, WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};
use swallowtail_testkit::assert_observable_activity_trace;

use support::{
    DriverCall, DriverFixture, ServerScenario, assert_wire_controls, complete_turn, model,
    qualified_model,
};

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
        assert_eq!(
            block_on(fixture.close_session(session)),
            CleanupOutcome::Clean
        );
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
    assert_eq!(
        block_on(fixture.close_session(replacement)),
        CleanupOutcome::Clean
    );
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
