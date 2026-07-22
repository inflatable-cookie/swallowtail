use crate::{RecordedHostCall, RecordingHostServices, RecordingOutcome, poll_immediate};
use std::num::NonZeroU64;
use swallowtail_core::MediaDirection;
use swallowtail_runtime::{
    CleanupOutcome, CredentialRef, MediaChunk, MediaStreamId, ProviderCancellationOutcome,
    RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaResponseStatus,
    RealtimeMediaSessionState, RuntimeSessionId, RuntimeTurnId, ScopeId,
};

pub(super) fn assert_interruption_ends_session() {
    for status in [
        RealtimeMediaResponseStatus::Cancelled(ProviderCancellationOutcome::Confirmed),
        RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::Unconfirmed),
        RealtimeMediaResponseStatus::Failed(swallowtail_core::SafeDiagnostic::new(
            "fixture.provider_failed",
            "Provider failed",
        )),
        RealtimeMediaResponseStatus::Disconnected,
    ] {
        let config = crate::realtime_media_fixture::realtime_media_config();
        let session_id =
            RuntimeSessionId::new("interrupted-media-session").expect("session id is valid");
        let input = MediaStreamId::new("interrupted-input").expect("stream id is valid");
        let mut state = RealtimeMediaSessionState::new(session_id.clone(), config.clone());
        let chunk = MediaChunk::new(
            session_id,
            input.clone(),
            nonzero(1),
            MediaDirection::Input,
            config.input_format(),
            vec![1, 2],
            &config,
        )
        .expect("input is valid");
        state.append_input(&chunk).expect("input appends");
        let turn = RuntimeTurnId::new("interrupted-turn").expect("turn id is valid");
        state
            .commit_input(turn.clone(), input)
            .expect("input commits");
        for event in [
            RealtimeMediaEvent::new(
                nonzero(1),
                turn.clone(),
                RealtimeMediaEventKind::ResponseStarted,
            ),
            RealtimeMediaEvent::new(
                nonzero(2),
                turn,
                RealtimeMediaEventKind::ResponseTerminal(status),
            ),
        ] {
            state.record_response_event(&event).expect("event applies");
        }
        assert!(!state.is_reusable());
    }
}

pub(super) fn assert_joined_cleanup(plan: &swallowtail_core::PreflightPlan) {
    let recording = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let services = recording.services();
    let scope = ScopeId::new("media-cleanup").expect("scope is valid");
    let credential = poll_immediate(
        services
            .credential()
            .expect("credential service exists")
            .acquire(
                scope.clone(),
                CredentialRef::new("media-credential").expect("credential is valid"),
                plan.endpoint_audience().clone(),
            ),
    )
    .expect("credential is acquired");
    let task = services
        .task()
        .expect("task service exists")
        .spawn(scope.clone(), Box::pin(async {}))
        .expect("reader/writer task starts");
    poll_immediate(task.join()).expect("reader/writer task joins");
    poll_immediate(
        services
            .blocking_work()
            .expect("blocking service exists")
            .run(scope, Box::new(|| Ok(()))),
    )
    .expect("connection closes");
    assert_eq!(
        poll_immediate(
            services
                .credential()
                .expect("credential service exists")
                .release(credential),
        ),
        CleanupOutcome::Clean
    );
    let calls = recording.calls();
    assert!(
        position(&calls, RecordedHostCall::TaskJoin)
            < position(&calls, RecordedHostCall::BlockingWork)
    );
    assert!(
        position(&calls, RecordedHostCall::BlockingWork)
            < position(&calls, RecordedHostCall::CredentialRelease)
    );
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
}

fn position(calls: &[RecordedHostCall], expected: RecordedHostCall) -> usize {
    calls
        .iter()
        .position(|call| *call == expected)
        .expect("host call exists")
}

fn nonzero(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).expect("sequence is nonzero")
}
