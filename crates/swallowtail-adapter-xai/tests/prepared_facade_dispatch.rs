mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_xai::{
    XaiRunProfileInput, XaiSessionProfileInput, prepare_xai_responses_websocket,
};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{CleanupOutcome, OperationContent, RequestId, TerminalStatus};

use support::{DriverFixture, ServerScenario, assert_wire_controls, qualified_model, turn_request};

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
    assert_eq!(
        block_on(session_fixture.close_session(session)),
        CleanupOutcome::Clean
    );
    let frames = session_fixture.server.frames();
    assert_eq!(frames.len(), 2);
    assert_wire_controls(&frames[0], Some("xhigh"), Some(512), false);
    assert_wire_controls(&frames[1], Some("xhigh"), Some(512), true);
}
