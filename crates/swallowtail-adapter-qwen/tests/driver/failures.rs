use super::support::{plan_request, plan_with_decoy_plan_axis};
use super::*;

#[test]
fn unknown_events_are_observed_without_exposing_provider_payloads() {
    let (process, state) = FakeProcessService::completed(&fixture("unknown-event.jsonl"));
    let (events, terminal, cleanup) = run_completed(process, state, "unknown");

    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if matches!(activity.kind(), swallowtail_runtime::ActivityKind::Unknown(_))
    )));
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
fn malformed_stream_emits_correlated_debug_observation_when_observer_registered() {
    let (process, _state) = FakeProcessService::completed(&fixture("malformed.jsonl"));
    let observer = Arc::new(CapturingDebugObserver::default());
    let services = host_services(process, Arc::new(PendingTimeService))
        .with_diagnostic_observer(observer.clone());
    let mut handle = block_on(driver().start_run(plan(), request("malformed-debug"), services))
        .expect("run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_status_code(
        &terminal,
        "swallowtail.qwen.headless.malformed_stream",
        false,
    );
    let observations = observer.observations();
    assert!(
        observations.iter().any(|observation| {
            observation.kind() == DebugObservationKind::ProtocolParse
                && observation.correlated_code()
                    == Some("swallowtail.qwen.headless.malformed_stream")
                && observation.route() == Some("qwen.headless")
                && observation.stage() == Some("headless.pump.decode")
        }),
        "expected protocol-parse debug observation, got {observations:?}"
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[derive(Default)]
struct CapturingDebugObserver {
    observations: Mutex<Vec<DebugObservation>>,
}

impl CapturingDebugObserver {
    fn observations(&self) -> Vec<DebugObservation> {
        self.observations.lock().expect("lock").clone()
    }
}

impl DiagnosticObserver for CapturingDebugObserver {
    fn observe(&self, _diagnostic: &Diagnostic) {}

    fn observe_debug(&self, observation: &DebugObservation) {
        self.observations
            .lock()
            .expect("lock")
            .push(observation.clone());
    }
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

#[test]
fn decoy_axis_plan_version_cannot_admit_unqualified_qwen_plan() {
    let (process, state) = FakeProcessService::completed("");
    let error = block_on(driver().start_run(
        plan_with_decoy_plan_axis(),
        plan_request("plan-decoy-axis"),
        host_services(process, Arc::new(PendingTimeService)),
    ))
    .err()
    .expect("decoy-axis Plan must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.harness_mode_mismatch"
    );
    assert!(!state.started());
}
