#[test]
fn all_historical_facade_points_are_named_and_no_longer_executable() {
    assert_eq!(
        GEMINI_LIVE_SUPERSEDED_FACADE_REVISION,
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent",
        "the pre-thinking proof keeps its exact historical point"
    );
    assert_eq!(
        GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION,
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-2026-08-23",
        "the thinking-capable proof keeps its exact historical point"
    );
    assert_eq!(
        GEMINI_LIVE_OUTPUT_MAXIMUM_SUPERSEDED_FACADE_REVISION,
        "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent.thinking-output-max-2026-08-23",
        "the output-maximum proof keeps its exact historical point"
    );
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    for (label, facade) in [
        ("pre-thinking", GEMINI_LIVE_SUPERSEDED_FACADE_REVISION),
        ("thinking", GEMINI_LIVE_THINKING_SUPERSEDED_FACADE_REVISION),
        (
            "output-maximum",
            GEMINI_LIVE_OUTPUT_MAXIMUM_SUPERSEDED_FACADE_REVISION,
        ),
    ] {
        let failure = block_on(
            GeminiLiveDriver::new().open_realtime_media_session(
                fixture.plan_with_facade(facade),
                OpenRealtimeMediaSessionRequest::new(
                    RequestId::new(format!("historical-facade-{label}"))
                        .expect("request id is valid"),
                    config(),
                    None,
                )
                .with_planned_connection_rollover(rollover_policy()),
                fixture.services(),
            ),
        )
        .err()
        .expect("a plan on a historical facade point is rejected");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.gemini.live_preflight_rejected",
            "{label} historical point rejects before effects"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
