use super::*;

#[test]
fn unknown_events_are_observed_without_exposing_provider_payloads() {
    let (process, state) = FakeProcessService::completed(&fixture("unknown-event.jsonl"));
    let (events, terminal, cleanup) = run_completed(process, state, "unknown");

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::Progress)
    );
    let public = format!("{events:?}{terminal:?}");
    assert!(!public.contains("fixture-provider-secret-never-diagnose"));
    assert!(!public.contains("fixture-private-prompt"));
}

#[test]
fn provider_and_protocol_failures_remain_distinct_and_redacted() {
    let (process, state) = FakeProcessService::completed(&fixture("provider-failure.jsonl"));
    let (_, provider, cleanup) = run_completed(process, state, "provider-failure");
    assert_status_code(&provider, "swallowtail.qwen.headless.provider_failed", true);
    assert_eq!(cleanup, CleanupOutcome::Clean);

    let (process, state) = FakeProcessService::completed(&fixture("malformed.jsonl"));
    let (_, malformed, cleanup) = run_completed(process, state, "malformed");
    assert_status_code(
        &malformed,
        "swallowtail.qwen.headless.malformed_stream",
        false,
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
    let public = format!("{provider:?}{malformed:?}");
    assert!(!public.contains("fixture-provider-secret-never-diagnose"));
    assert!(!public.contains("fixture-private-workspace"));
}

#[test]
fn native_budget_exits_have_separate_provider_failure_codes() {
    for (exit, expected) in [
        (53, "swallowtail.qwen.headless.native_turn_limit"),
        (55, "swallowtail.qwen.headless.native_budget"),
    ] {
        let (process, state) =
            FakeProcessService::with_exit("", ProcessExit::new(false, Some(exit)));
        let (_, terminal, cleanup) = run_completed(process, state, expected);
        assert_status_code(&terminal, expected, true);
        assert_eq!(cleanup, CleanupOutcome::Clean);
    }
}
