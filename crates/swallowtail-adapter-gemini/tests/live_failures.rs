mod live_support;

use futures_executor::block_on;
use live_support::{
    Call, LiveFixture, LiveScenario, TimeMode, complete, config, input_chunk, open,
    rollover_policy, start_turn,
};
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::GeminiLiveDriver;
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, MediaStreamId, MonotonicInstant,
    OpenRealtimeMediaSessionRequest, ProviderCancellationOutcome, RealtimeMediaSessionDriver,
    RequestId, TerminalStatus,
};

#[test]
fn invalid_media_rejects_before_access_or_connection() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let base = config();
    let wrong = swallowtail_core::RealtimeMediaConfig::new(
        base.input_format(),
        base.output_format(),
        NonZeroU64::new(16_384).unwrap(),
        base.maximum_turns(),
    );
    let request =
        OpenRealtimeMediaSessionRequest::new(RequestId::new("invalid").unwrap(), wrong, None)
            .with_planned_connection_rollover(rollover_policy());
    let error = block_on(GeminiLiveDriver::new().open_realtime_media_session(
        fixture.plan(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("invalid request rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.gemini.live_preflight_rejected"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn missing_handle_and_replacement_failure_end_the_session_without_replay() {
    for (scenario, request) in [
        (LiveScenario::MissingHandle, "missing-handle"),
        (LiveScenario::ReplacementFailure, "replacement-failure"),
    ] {
        let fixture = LiveFixture::new(scenario, TimeMode::Pending);
        let mut session = open(&fixture, request, None);
        let response = start_turn(&mut session, &fixture, 1);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);

        let error = block_on(session.append_input(
            input_chunk(
                session.session_id(),
                MediaStreamId::new("second-input").unwrap(),
                1,
            ),
            fixture.services(),
        ))
        .expect_err("rollover failure rejects next input");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.gemini.live_rollover_failed"
        );
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(
            fixture
                .server
                .frames()
                .iter()
                .filter(|frame| frame.contains("\"audio\""))
                .count(),
            1
        );
        assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    }
}

#[test]
fn exhausted_rollover_and_disconnect_preserve_distinct_terminal_truth() {
    let exhausted = LiveFixture::new(LiveScenario::SecondGoAway, TimeMode::Pending);
    let mut session = open(&exhausted, "exhausted", None);
    let first = start_turn(&mut session, &exhausted, 1);
    let (first, _, outcome) = complete(first);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(first.close()), CleanupOutcome::Clean);
    let second = start_turn(&mut session, &exhausted, 2);
    let (second, _, outcome) = complete(second);
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("second GoAway must exhaust the bounded rollover");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.gemini.live_rollover_exhausted"
    );
    assert_eq!(block_on(second.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let disconnected = LiveFixture::new(LiveScenario::Disconnect, TimeMode::Pending);
    let mut session = open(&disconnected, "disconnect", None);
    let response = start_turn(&mut session, &disconnected, 1);
    let (response, _, outcome) = complete(response);
    let TerminalStatus::RuntimeFailed(diagnostic) = outcome.status() else {
        panic!("disconnect must retain runtime-failure truth");
    };
    assert_eq!(diagnostic.code(), "swallowtail.gemini.live_disconnected");
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    let error = block_on(session.append_input(
        input_chunk(
            session.session_id(),
            MediaStreamId::new("after-disconnect").unwrap(),
            1,
        ),
        disconnected.services(),
    ))
    .expect_err("disconnected session is not reusable");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.gemini.live_session_closed"
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn provider_failure_unknown_event_and_format_drift_are_safe_in_both_topologies() {
    for host in ["host.local", "host.remote-authoritative"] {
        for (scenario, code) in [
            (
                LiveScenario::ProviderFailure,
                "swallowtail.gemini.live_provider_failed",
            ),
            (
                LiveScenario::UnknownEvent,
                "swallowtail.gemini.live_event_unknown",
            ),
            (
                LiveScenario::FormatDrift,
                "swallowtail.gemini.live_format_drift",
            ),
        ] {
            let fixture = LiveFixture::for_host(scenario, TimeMode::Pending, host);
            let mut session = open(&fixture, &format!("failure-{host}-{code}"), None);
            let response = start_turn(&mut session, &fixture, 1);
            let (response, _, outcome) = complete(response);
            let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
                panic!("provider or protocol failure must remain distinct");
            };
            assert_eq!(diagnostic.code(), code);
            let rendered = format!("{outcome:?}");
            assert!(!rendered.contains("fixture private"));
            assert!(!rendered.contains("fixture-secret"));
            assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
            assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
            assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
        }
    }
}

#[test]
fn cancellation_and_deadline_are_local_session_ending_and_unconfirmed() {
    let cancelled = LiveFixture::new(LiveScenario::Wait, TimeMode::Pending);
    let mut session = open(&cancelled, "cancel", None);
    let response = start_turn(&mut session, &cancelled, 1);
    cancelled.server.wait_for_frames(4);
    assert_eq!(
        block_on(response.cancellation().request()).unwrap(),
        CancellationAcknowledgement::Requested
    );
    let (response, _, outcome) = complete(response);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
    assert_eq!(cancelled.server.frames().len(), 4);
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let deadline = LiveFixture::new(LiveScenario::Wait, TimeMode::Delayed);
    let mut session = open(
        &deadline,
        "deadline",
        Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX))),
    );
    let response = start_turn(&mut session, &deadline, 1);
    let (response, _, outcome) = complete(response);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
    assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let calls = deadline.calls.calls();
    let timer_drop = calls
        .iter()
        .position(|call| *call == Call::TimerDrop)
        .unwrap();
    let task_join = calls
        .iter()
        .position(|call| *call == Call::TaskJoin)
        .unwrap();
    let release = calls
        .iter()
        .position(|call| *call == Call::CredentialRelease)
        .unwrap();
    assert!(timer_drop < task_join && task_join < release);
}

#[test]
fn cleanup_failure_remains_visible_after_both_connection_generations_join() {
    let fixture =
        LiveFixture::with_release_failure(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let mut session = open(&fixture, "cleanup-failure", None);
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, _, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    let CleanupOutcome::Failed(diagnostic) = block_on(session.close()) else {
        panic!("credential cleanup failure must remain visible");
    };
    assert_eq!(diagnostic.code(), "fixture.credential_release_failed");

    let calls = fixture.calls.calls();
    let release = calls
        .iter()
        .rposition(|call| *call == Call::CredentialRelease)
        .unwrap();
    assert_eq!(calls.last(), Some(&Call::CredentialRelease));
    assert_eq!(fixture.calls.count(Call::BlockingJoin), 4);
    assert!(
        calls
            .iter()
            .rposition(|call| *call == Call::BlockingJoin)
            .unwrap()
            < release
    );
    assert!(
        calls
            .iter()
            .rposition(|call| *call == Call::TaskJoin)
            .unwrap()
            < release
    );
}
