#[test]
fn every_admitted_bound_prepares_with_exact_capability_plan_and_request() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for value in ADMITTED {
        let operation = prepared
            .prepare_live_session(
                GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                    RequestId::new(format!("output-max-{value}")).expect("request id is valid"),
                    None,
                )
                .with_maximum_output_tokens(maximum(value)),
            )
            .expect("Gemini Live session prepares");
        assert_eq!(
            operation.request().maximum_output_tokens(),
            Some(maximum(value)),
            "request carries the exact selection"
        );
        assert_eq!(
            planned_maximum(operation.plan()),
            Some(vec![CapabilityConstraint::OutputTokenMaximum(value)]),
            "plan carries exactly one exact output-maximum constraint"
        );
        assert_eq!(
            planned_maximum(operation.evidence().plan()),
            planned_maximum(operation.plan())
        );
        assert_eq!(
            operation.plan().protocol_facade_id().as_str(),
            GEMINI_LIVE_FACADE_REVISION
        );
        assert_eq!(
            operation.plan().model_id().expect("model bound").as_str(),
            "gemini-3.1-flash-live-preview"
        );
        assert_eq!(operation.request().config(), &config());
        assert_eq!(
            operation.request().planned_connection_rollover(),
            rollover_policy()
        );
    }
    assert_eq!(GEMINI_LIVE_MAX_OUTPUT_TOKENS, 65_536);
    assert_eq!(fixture.calls.count(Call::NetworkAuthorize), 0);
    assert_eq!(fixture.calls.count(Call::CredentialAcquire), 0);
    assert!(fixture.server.frames().is_empty());
}

#[test]
fn omitted_maximum_claims_no_output_token_capability() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    let operation = prepared
        .prepare_live_session(GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
            RequestId::new("output-max-omitted").expect("request id is valid"),
            None,
        ))
        .expect("Gemini Live session prepares");
    assert_eq!(operation.request().maximum_output_tokens(), None);
    assert_eq!(planned_maximum(operation.plan()), None);
    assert_eq!(planned_maximum(operation.evidence().plan()), None);
}

#[test]
fn maximum_composes_with_every_admitted_thinking_level() {
    let fixture = LiveFixture::new(LiveScenario::TwoTurnsRollover, TimeMode::Pending);
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Gemini Live integration prepares");
    for portable in ["minimal", "low", "medium", "high"] {
        let operation = prepared
            .prepare_live_session(
                GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                    RequestId::new(format!("output-max-compose-{portable}"))
                        .expect("request id is valid"),
                    None,
                )
                .with_reasoning_mode(mode(portable))
                .with_maximum_output_tokens(maximum(1_024)),
            )
            .expect("composed selection prepares");
        assert_eq!(operation.request().reasoning_mode(), Some(&mode(portable)));
        assert_eq!(
            operation.request().maximum_output_tokens(),
            Some(maximum(1_024))
        );
        assert_eq!(
            planned_maximum(operation.plan()),
            Some(vec![CapabilityConstraint::OutputTokenMaximum(1_024)])
        );
    }
}
