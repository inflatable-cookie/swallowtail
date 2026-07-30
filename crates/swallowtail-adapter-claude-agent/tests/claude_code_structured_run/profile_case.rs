#[test]
fn provider_neutral_one_shot_and_native_profiles_cover_headless_boundaries() {
    let one_shot = run_one_shot_structured_cli_profile();
    assert_eq!(one_shot.profile(), SyntheticProfile::OneShotStructuredCli);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::ProcessLifecycle,
    ] {
        assert!(one_shot.covers(assertion), "missing {assertion:?}");
    }

    let native = run_structured_harness_native_boundary_assertions();
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(native.covers(assertion), "missing {assertion:?}");
    }
}

