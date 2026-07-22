mod live_support;

use futures_executor::block_on;
use live_support::{
    Call, LiveFixture, LiveScenario, TimeMode, complete, config, open, rollover_policy, start_turn,
};
use swallowtail_adapter_gemini::gemini_live_descriptor;
use swallowtail_core::{CredentialMechanism, DriverRole};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ProviderObservation, RealtimeMediaEventKind,
    RealtimeMediaResponseStatus, TerminalStatus,
};

#[test]
fn two_turns_roll_over_once_without_replay_and_join_before_credential_release() {
    let descriptor = gemini_live_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.gemini.live"
    );
    assert_eq!(
        descriptor.transport_family().as_str(),
        "gemini-live-raw-websocket"
    );
    assert!(descriptor.supports_role(DriverRole::RealtimeMediaSession));

    for host in ["host.local", "host.remote-authoritative"] {
        assert_topology(host);
    }
}

fn assert_topology(host: &str) {
    let fixture = LiveFixture::for_host(LiveScenario::TwoTurnsRollover, TimeMode::Pending, host);
    let plan = fixture.plan();
    assert_eq!(plan.execution_host_id().as_str(), host);
    assert_eq!(plan.instance_id().as_str(), "gemini.public.live-preview");
    assert_eq!(
        plan.instance_target_ref().as_host_value(),
        "gemini-live-fixture-endpoint"
    );
    assert_eq!(
        plan.access_profile_id().as_str(),
        "gemini.authorization-api-key.project"
    );
    assert_eq!(
        plan.endpoint_audience().as_str(),
        "generativelanguage.googleapis.com"
    );
    assert_eq!(plan.credential_mechanism(), &CredentialMechanism::ApiKey);
    assert_eq!(
        plan.model_route_id().expect("route bound").as_str(),
        "gemini-3-1-flash-live-preview"
    );
    assert_eq!(
        plan.model_id().expect("model bound").as_str(),
        "gemini-3.1-flash-live-preview"
    );
    assert_eq!(
        plan.provider_id().expect("provider bound").as_str(),
        "gemini"
    );
    assert_eq!(
        plan.requirements().planned_connection_rollover(),
        rollover_policy()
    );
    assert_eq!(
        plan.requirements()
            .realtime_media()
            .expect("media bound")
            .config(),
        &config()
    );
    assert_eq!(
        plan.protocol_facade_id().as_str(),
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent"
    );
    let mut session = open(
        &fixture,
        &format!("two-turns-{host}"),
        Some(Deadline::at(MonotonicInstant::from_ticks(u64::MAX))),
    );
    for turn in 1..=2 {
        let response = start_turn(&mut session, &fixture, turn);
        let (response, events, outcome) = complete(response);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_evidence(&events);
        assert_eq!(block_on(response.close()), CleanupOutcome::Clean);
    }
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let handshakes = fixture.server.handshakes();
    assert_eq!(handshakes.len(), 2);
    assert!(
        handshakes
            .iter()
            .all(|target| target.ends_with("?key=fixture-secret"))
    );
    let frames = fixture.server.frames();
    let setups: Vec<_> = frames
        .iter()
        .filter(|frame| frame.contains("\"setup\""))
        .collect();
    assert_eq!(setups.len(), 2);
    assert!(!setups[0].contains("fixture-private-handle"));
    assert!(setups[1].contains("fixture-private-handle-2"));
    assert!(!setups[1].contains("fixture-private-stale-handle"));
    assert_eq!(
        frames
            .iter()
            .filter(|frame| frame.contains("\"audio\""))
            .count(),
        2
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 1);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 1);
    assert_eq!(fixture.calls.count(Call::CredentialRelease), 1);
    assert_eq!(fixture.calls.count(Call::BlockingStart), 4);
    assert_eq!(fixture.calls.count(Call::TimerStart), 2);
    assert_eq!(fixture.calls.count(Call::TimerDrop), 2);
    let calls = fixture.calls.calls();
    let release = calls
        .iter()
        .rposition(|call| *call == Call::CredentialRelease)
        .unwrap();
    assert!(
        calls
            .iter()
            .rposition(|call| *call == Call::TaskJoin)
            .unwrap()
            < release
    );
    assert!(
        calls
            .iter()
            .rposition(|call| *call == Call::TimerDrop)
            .unwrap()
            < release
    );
    assert!(
        calls
            .iter()
            .rposition(|call| *call == Call::BlockingJoin)
            .unwrap()
            < release
    );
}

fn assert_evidence(events: &[swallowtail_runtime::RealtimeMediaEvent]) {
    assert!(matches!(
        events.first().expect("event exists").kind(),
        RealtimeMediaEventKind::ResponseStarted
    ));
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RealtimeMediaEventKind::OutputAudio(_)))
    );
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RealtimeMediaEventKind::TranscriptCompleted(_)))
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RealtimeMediaEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert!(matches!(
        events.last().expect("terminal exists").kind(),
        RealtimeMediaEventKind::ResponseTerminal(RealtimeMediaResponseStatus::Completed)
    ));
    let rendered = format!("{events:?}");
    assert!(!rendered.contains("fixture transcript"));
    assert!(!rendered.contains("fixture-private-handle"));
    assert!(!rendered.contains("fixture-secret"));
}
