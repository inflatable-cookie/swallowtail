use super::client::ClientFrame;
use super::handle::{ProviderSessionHandle, ResumptionState};
use super::server::{ServerEvent, parse_server_frame};
use serde_json::Value;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AudioEncoding, CredentialMechanism, EndpointAudience,
    EntitlementMetering, MediaFormat, ModelId, ProtocolFacadeId, RealtimeMediaConfig,
    SupportAuthority,
};

const CORPUS: &str = "../../tests/fixtures/gemini-live-2026-07-22";

macro_rules! fixture {
    ($name:literal) => {
        include_bytes!(concat!(
            "../../tests/fixtures/gemini-live-2026-07-22/",
            $name
        ))
    };
}

fn expected(name: &str) -> Value {
    let bytes: &[u8] = match name {
        "client-setup-initial.json" => fixture!("client-setup-initial.json"),
        "client-setup-resume.json" => fixture!("client-setup-resume.json"),
        "client-activity-start.json" => fixture!("client-activity-start.json"),
        "client-audio.json" => fixture!("client-audio.json"),
        "client-activity-end.json" => fixture!("client-activity-end.json"),
        _ => panic!("unknown fixture in {CORPUS}: {name}"),
    };
    serde_json::from_slice(bytes).expect("frozen client fixture is valid JSON")
}

#[test]
fn exact_preview_route_access_and_asymmetric_media_posture_is_frozen() {
    let profile = AccessProfile::new(
        AccessProfileId::new("gemini.authorization-api-key.project").unwrap(),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("generativelanguage.googleapis.com").unwrap(),
        SupportAuthority::ProviderSupported,
    );
    let facade = ProtocolFacadeId::new(
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent",
    )
    .unwrap();
    let model = ModelId::new("gemini-3.1-flash-live-preview").unwrap();
    let input = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(16_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    let output = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    let media = RealtimeMediaConfig::new(
        input,
        output,
        NonZeroU64::new(32_768).unwrap(),
        NonZeroU32::new(2).unwrap(),
    );

    assert_eq!(profile.credential_mechanism(), &CredentialMechanism::ApiKey);
    assert_eq!(
        profile.endpoint_audience().as_str(),
        "generativelanguage.googleapis.com"
    );
    assert!(facade.as_str().contains("v1beta"));
    assert_eq!(model.as_str(), "gemini-3.1-flash-live-preview");
    assert_eq!(media.input_format().sample_rate_hz().get(), 16_000);
    assert_eq!(media.output_format().sample_rate_hz().get(), 24_000);
    assert_eq!(media.maximum_turns().get(), 2);
}

#[test]
fn exact_client_setup_activity_and_audio_frames_match_the_frozen_corpus() {
    assert_eq!(
        ClientFrame::Setup { handle: None }.to_json(),
        expected("client-setup-initial.json")
    );
    let handle = ProviderSessionHandle::new("fixture-private-handle-2".to_owned());
    assert_eq!(
        ClientFrame::Setup {
            handle: Some(&handle)
        }
        .to_json(),
        expected("client-setup-resume.json")
    );
    assert_eq!(
        ClientFrame::ActivityStart.to_json(),
        expected("client-activity-start.json")
    );
    assert_eq!(
        ClientFrame::Audio(&[1, 2, 3, 4]).to_json(),
        expected("client-audio.json")
    );
    assert_eq!(
        ClientFrame::ActivityEnd.to_json(),
        expected("client-activity-end.json")
    );
}

#[test]
fn server_setup_audio_transcript_usage_and_turn_boundaries_parse_exactly() {
    assert!(matches!(
        one("server-setup-complete.json"),
        ServerEvent::SetupComplete
    ));
    let ServerEvent::Audio(audio) = one("server-audio.json") else {
        panic!("audio event expected");
    };
    assert_eq!(audio.bytes(), &[1, 2, 3, 4]);
    let ServerEvent::Transcript(transcript) = one("server-transcript.json") else {
        panic!("transcript event expected");
    };
    assert_eq!(transcript, "fixture transcript");
    let ServerEvent::Usage(usage) = one("server-usage.json") else {
        panic!("usage event expected");
    };
    assert_eq!(usage.input_tokens(), Some(12));
    assert_eq!(usage.output_tokens(), Some(7));
    assert!(matches!(
        one("server-generation-complete.json"),
        ServerEvent::GenerationComplete
    ));
    assert!(matches!(
        one("server-turn-complete.json"),
        ServerEvent::TurnComplete
    ));
}

#[test]
fn latest_resumable_handle_replaces_clears_and_stays_redacted() {
    let ServerEvent::ResumptionUpdate { resumable, handle } = one("server-resumption-update.json")
    else {
        panic!("resumption update expected");
    };
    assert!(resumable);
    let mut state = ResumptionState::default();
    state.update(resumable, handle);
    assert_eq!(state.latest().unwrap().expose(), "fixture-private-handle-2");
    assert!(!format!("{:?}", state.latest().unwrap()).contains("fixture-private"));

    let ServerEvent::ResumptionUpdate { resumable, handle } =
        one("server-resumption-not-ready.json")
    else {
        panic!("non-resumable update expected");
    };
    state.update(resumable, handle);
    assert!(state.latest().is_none());
    state.update(
        true,
        Some(ProviderSessionHandle::new("replacement".to_owned())),
    );
    state.clear();
    assert!(state.latest().is_none());
}

#[test]
fn rollover_warning_is_private_and_clears_at_handoff() {
    let mut state = super::handle::RolloverState::default();
    state.update(
        true,
        Some(ProviderSessionHandle::new(
            "fixture-private-handle-2".to_owned(),
        )),
    );
    state.warn("15s-private-warning".to_owned());
    assert!(state.pending());
    assert!(!state.warning_debug().unwrap().contains("15s-private"));
    state.complete();
    assert!(!state.pending());
    assert!(state.warning_debug().is_none());
}

#[test]
fn warning_failure_unknown_malformed_and_format_drift_remain_distinct_and_safe() {
    let ServerEvent::GoAway(time_left) = one("server-go-away.json") else {
        panic!("GoAway expected");
    };
    assert_eq!(time_left, "15s");
    assert!(matches!(
        one("server-failure.json"),
        ServerEvent::ProviderFailed
    ));

    for (name, code) in [
        (
            "server-unknown.json",
            "swallowtail.gemini.live_event_unknown",
        ),
        (
            "server-malformed.json",
            "swallowtail.gemini.live_protocol_malformed",
        ),
        (
            "server-format-drift.json",
            "swallowtail.gemini.live_format_drift",
        ),
    ] {
        let error = parse_server_frame(server_fixture(name)).expect_err("fixture must fail closed");
        assert_eq!(error.diagnostic().code(), code);
        let rendered = error.to_string();
        assert!(!rendered.contains("fixture raw payload"));
        assert!(!rendered.contains("RESOURCE_EXHAUSTED"));
    }
}

fn one(name: &str) -> ServerEvent {
    let mut events = parse_server_frame(server_fixture(name)).expect("server fixture parses");
    assert_eq!(events.len(), 1);
    events.pop().unwrap()
}

fn server_fixture(name: &str) -> &'static [u8] {
    match name {
        "server-setup-complete.json" => fixture!("server-setup-complete.json"),
        "server-audio.json" => fixture!("server-audio.json"),
        "server-transcript.json" => fixture!("server-transcript.json"),
        "server-usage.json" => fixture!("server-usage.json"),
        "server-resumption-update.json" => fixture!("server-resumption-update.json"),
        "server-resumption-not-ready.json" => fixture!("server-resumption-not-ready.json"),
        "server-go-away.json" => fixture!("server-go-away.json"),
        "server-generation-complete.json" => fixture!("server-generation-complete.json"),
        "server-turn-complete.json" => fixture!("server-turn-complete.json"),
        "server-failure.json" => fixture!("server-failure.json"),
        "server-unknown.json" => fixture!("server-unknown.json"),
        "server-malformed.json" => fixture!("server-malformed.json"),
        "server-format-drift.json" => fixture!("server-format-drift.json"),
        _ => panic!("unknown fixture in {CORPUS}: {name}"),
    }
}
