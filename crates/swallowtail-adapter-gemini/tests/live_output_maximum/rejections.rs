#[test]
fn unsupported_values_reject_before_access_or_connection() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let failure = prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("output-max-rejected").expect("request id is valid"),
                None,
            )
            .with_maximum_output_tokens(maximum(65_537)),
        )
        .expect_err("above-limit maximum is rejected");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.gemini.live_preparation.output_limit_invalid"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn request_plan_and_value_drift_reject_before_endpoint_or_credential_work() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let base = OpenRealtimeMediaSessionRequest::new(
        RequestId::new("output-max-drift").expect("request id is valid"),
        config(),
        None,
    )
    .with_planned_connection_rollover(rollover_policy());
    let drifted: [(PreflightPlan, OpenRealtimeMediaSessionRequest); 3] = [
        (
            fixture.plan(),
            base.clone().with_maximum_output_tokens(maximum(1_024)),
        ),
        (plan_with_maximum(&fixture, 1_024), base.clone()),
        (
            plan_with_maximum(&fixture, 1),
            base.with_maximum_output_tokens(maximum(1_024)),
        ),
    ];
    for (plan, request) in drifted {
        let failure = block_on(GeminiLiveDriver::new().open_realtime_media_session(
            plan,
            request,
            fixture.services(),
        ))
        .err()
        .expect("drifted output maximum is rejected");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.gemini.live_preflight_rejected"
        );
    }
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn agreed_out_of_domain_maximum_rejects_before_endpoint_or_credential_work() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let failure = block_on(
        GeminiLiveDriver::new().open_realtime_media_session(
            plan_with_maximum(&fixture, 65_537),
            OpenRealtimeMediaSessionRequest::new(
                RequestId::new("output-max-out-of-domain").expect("request id is valid"),
                config(),
                None,
            )
            .with_planned_connection_rollover(rollover_policy())
            .with_maximum_output_tokens(maximum(65_537)),
            fixture.services(),
        ),
    )
    .err()
    .expect("agreed out-of-domain maximum is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.gemini.live_preflight_rejected"
    );
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}
