mod realtime_support;

use futures_executor::block_on;
use futures_util::StreamExt;
use realtime_support::{
    Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, input_chunk, open, start_one,
    start_turn,
};
use swallowtail_adapter_openai::OpenAiRealtimeDriver;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, MediaStreamId, OpenRealtimeMediaSessionRequest,
    ProviderCancellationOutcome, RealtimeMediaSessionDriver, RequestId, TerminalStatus,
};

#[test]
fn active_response_rejects_parallel_input_without_transport_side_effect() {
    let fixture = RealtimeFixture::new(RealtimeScenario::Cancel, TimeMode::Pending);
    let mut session = open(&fixture, "parallel", None);
    let response = start_one(&mut session, &fixture, "parallel-one");
    fixture.server.wait_for_frames(4);

    let before = fixture.server.frames().len();
    let error = block_on(session.append_input(
        input_chunk(
            session.session_id(),
            MediaStreamId::new("parallel-two").expect("stream id is valid"),
            1,
        ),
        fixture.services(),
    ))
    .expect_err("parallel input is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.realtime_media_rejected"
    );
    assert_eq!(fixture.server.frames().len(), before);

    assert_eq!(
        block_on(response.cancellation().request()).expect("cancel is accepted"),
        CancellationAcknowledgement::Requested
    );
    let (response, _events, outcome) = complete(response);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn provider_failure_disconnect_and_format_drift_fail_closed_and_redacted() {
    let provider = RealtimeFixture::new(RealtimeScenario::ProviderFailed, TimeMode::Pending);
    let mut session = open(&provider, "provider-failure", None);
    let response = start_one(&mut session, &provider, "provider-input");
    let (response, _events, outcome) = complete(response);
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("provider error must remain a provider failure");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.openai.realtime_provider_failed"
    );
    assert!(!format!("{outcome:?}").contains("synthetic private provider detail"));
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let disconnected = RealtimeFixture::new(RealtimeScenario::Disconnect, TimeMode::Pending);
    let mut session = open(&disconnected, "disconnect", None);
    let session_id = session.session_id().clone();
    let response = start_one(&mut session, &disconnected, "disconnect-input");
    let (response, _events, outcome) = complete(response);
    let TerminalStatus::RuntimeFailed(diagnostic) = outcome.status() else {
        panic!("connection loss must retain runtime-failure truth");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.openai.realtime_disconnected"
    );
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    let error = block_on(session.append_input(
        input_chunk(
            &session_id,
            MediaStreamId::new("after-disconnect").expect("stream id is valid"),
            1,
        ),
        disconnected.services(),
    ))
    .expect_err("disconnected session is not reusable");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.realtime_session_closed"
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let drift = RealtimeFixture::new(RealtimeScenario::FormatDrift, TimeMode::Pending);
    let error = block_on(OpenAiRealtimeDriver::new().open_realtime_media_session(
        drift.plan(),
        OpenRealtimeMediaSessionRequest::new(
            RequestId::new("format-drift").expect("request id is valid"),
            realtime_support::config(),
            None,
        ),
        drift.services(),
    ))
    .err()
    .expect("format drift rejects session open");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.realtime_format_drift"
    );
    assert_eq!(drift.calls.count(Call::CredentialAcquire), 1);
    assert_eq!(drift.calls.count(Call::CredentialRelease), 1);
    let calls = drift.calls.calls();
    assert!(position(&calls, Call::BlockingJoin) < position(&calls, Call::CredentialRelease));
}

#[test]
fn connection_loss_after_cancel_preserves_unconfirmed_provider_truth() {
    let fixture = RealtimeFixture::new(RealtimeScenario::CancelDisconnect, TimeMode::Pending);
    let mut session = open(&fixture, "cancel-disconnect", None);
    let mut response = start_one(&mut session, &fixture, "cancel-disconnect-input");
    let mut events = response.take_events().expect("events exist");
    let terminal = response
        .take_terminal_outcome()
        .expect("terminal outcome exists");
    fixture.server.wait_for_frames(4);
    assert_eq!(
        block_on(response.cancellation().request()).expect("cancel is accepted"),
        CancellationAcknowledgement::Requested
    );
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
    assert_eq!(
        fixture
            .server
            .frames()
            .iter()
            .filter(|frame| frame.contains("response.cancel"))
            .count(),
        1
    );
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn cleanup_failure_stays_visible_after_all_owned_work_joins() {
    let fixture =
        RealtimeFixture::with_release_failure(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let mut session = open(
        &fixture,
        "cleanup-failure",
        Some(swallowtail_runtime::Deadline::at(
            swallowtail_runtime::MonotonicInstant::from_ticks(u64::MAX),
        )),
    );
    for turn in 1..=2 {
        let stream = format!("cleanup-input-{turn}");
        let response = start_turn(&mut session, &fixture, &stream, turn);
        let (response, _events, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    let cleanup = block_on(session.close());
    let CleanupOutcome::Failed(diagnostic) = cleanup else {
        panic!("credential cleanup failure must remain visible");
    };
    assert_eq!(diagnostic.code(), "fixture.credential_release_failed");

    let calls = fixture.calls.calls();
    let release = position(&calls, Call::CredentialRelease);
    assert!(last_position(&calls, Call::TimerDrop) < release);
    assert!(last_position(&calls, Call::TaskJoin) < release);
    assert!(last_position(&calls, Call::BlockingJoin) < release);
    assert_eq!(calls.last(), Some(&Call::CredentialRelease));
}

fn position(calls: &[Call], target: Call) -> usize {
    calls
        .iter()
        .position(|call| *call == target)
        .expect("call is recorded")
}

fn last_position(calls: &[Call], target: Call) -> usize {
    calls
        .iter()
        .rposition(|call| *call == target)
        .expect("call is recorded")
}
