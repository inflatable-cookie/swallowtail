mod realtime_support;

use futures_executor::block_on;
use futures_util::StreamExt;
use realtime_support::{
    Call, RealtimeFixture, RealtimeScenario, TimeMode, complete, config, input_chunk, open,
    start_one, turn_id,
};
use std::num::NonZeroU64;
use swallowtail_adapter_openai::OpenAiRealtimeDriver;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, MediaInputCommit, MediaStreamId,
    MonotonicInstant, OpenRealtimeMediaSessionRequest, ProviderCancellationOutcome,
    RealtimeMediaEventKind, RealtimeMediaResponseStatus, RealtimeMediaSessionDriver, RequestId,
    TerminalStatus,
};

#[test]
fn native_cancellation_is_single_confirmed_and_session_ending() {
    let fixture = RealtimeFixture::new(RealtimeScenario::Cancel, TimeMode::Pending);
    let mut session = open(&fixture, "cancel", None);
    let session_id = session.session_id().clone();
    let stream_id = MediaStreamId::new("cancel-input").expect("stream is valid");
    block_on(session.append_input(
        input_chunk(&session_id, stream_id.clone(), 1),
        fixture.services(),
    ))
    .expect("input appends");
    let mut response = block_on(session.commit_input(
        MediaInputCommit::new(turn_id(1), stream_id),
        fixture.services(),
    ))
    .expect("input commits");
    let mut events = response.take_events().expect("events exist");
    let terminal = response
        .take_terminal_outcome()
        .expect("terminal outcome exists");
    fixture.server.wait_for_frames(4);
    assert_eq!(
        block_on(response.cancellation().request()).expect("cancel is accepted"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(response.cancellation().request()).expect("duplicate cancel is accepted"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let (seen, outcome) = block_on(async {
        let mut seen = Vec::new();
        while let Some(event) = events.next().await {
            seen.push(event.expect("event succeeds"));
        }
        (seen, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert!(matches!(
        seen.last().expect("terminal event exists").kind(),
        RealtimeMediaEventKind::ResponseTerminal(RealtimeMediaResponseStatus::Cancelled(
            ProviderCancellationOutcome::Confirmed
        ))
    ));
    let cancel_frames: Vec<_> = fixture
        .server
        .frames()
        .into_iter()
        .filter(|frame| frame.contains("response.cancel"))
        .collect();
    assert_eq!(cancel_frames.len(), 1);
    assert!(!cancel_frames[0].contains("response_id"));
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    let frame_count = fixture.server.frames().len();
    let error = block_on(session.append_input(
        input_chunk(
            &session_id,
            MediaStreamId::new("after-cancel").expect("stream is valid"),
            1,
        ),
        fixture.services(),
    ))
    .expect_err("cancelled session is closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.realtime_session_closed"
    );
    assert_eq!(fixture.server.frames().len(), frame_count);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn deadline_uses_native_cancel_and_unknown_semantics_fail_closed() {
    let deadline = RealtimeFixture::new(RealtimeScenario::Cancel, TimeMode::Delayed);
    let mut session = open(
        &deadline,
        "deadline",
        Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX))),
    );
    let response = start_one(&mut session, &deadline, "deadline-input");
    let (response, _events, outcome) = complete(response);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let calls = deadline.calls.calls();
    let timer_complete = calls
        .iter()
        .position(|call| *call == Call::TimerComplete)
        .expect("deadline timer completed");
    let timer_drop = calls
        .iter()
        .position(|call| *call == Call::TimerDrop)
        .expect("deadline timer dropped");
    let task_join = calls
        .iter()
        .position(|call| *call == Call::TaskJoin)
        .expect("response task joined");
    let credential_release = calls
        .iter()
        .position(|call| *call == Call::CredentialRelease)
        .expect("credential released");
    assert!(timer_complete <= timer_drop);
    assert!(timer_drop < task_join);
    assert!(task_join < credential_release);

    let unknown = RealtimeFixture::new(RealtimeScenario::Unknown, TimeMode::Pending);
    let mut session = open(&unknown, "unknown", None);
    let response = start_one(&mut session, &unknown, "unknown-input");
    let (response, _events, outcome) = complete(response);
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("unknown semantics must fail safely");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.openai.realtime_event_unknown"
    );
    assert!(!format!("{outcome:?}").contains("hidden"));
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn invalid_media_request_rejects_before_access_or_connection() {
    let fixture = RealtimeFixture::new(RealtimeScenario::TwoTurns, TimeMode::Pending);
    let base = config();
    let wrong = swallowtail_core::RealtimeMediaConfig::new(
        base.input_format(),
        base.output_format(),
        NonZeroU64::new(16_384).expect("bound is nonzero"),
        base.maximum_turns(),
    );
    let error = block_on(OpenAiRealtimeDriver::new().open_realtime_media_session(
        fixture.plan(),
        OpenRealtimeMediaSessionRequest::new(
            RequestId::new("invalid").expect("request id is valid"),
            wrong,
            None,
        ),
        fixture.services(),
    ))
    .err()
    .expect("drift is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.realtime_preflight_rejected"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
