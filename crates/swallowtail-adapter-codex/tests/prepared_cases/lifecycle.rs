use super::*;

#[test]
fn cancellation_and_deadline_remain_distinct_safe_preparation_failures() {
    let cancellation = DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("cancellation is accepted");
    let cancelled_fixture = fixture_with_cancellation(
        CodexPreparedDriver::StructuredExec,
        "host.local",
        "codex",
        cancellation,
    );
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let failure = block_on(prepare_codex(
        cancelled_fixture.input,
        cancelled_fixture.probe,
        services(cancelled_fixture.host, process),
    ))
    .expect_err("cancelled preparation fails");
    assert_eq!(failure.stage(), PreparationStage::BoundedOutput);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.discovery_cancelled"
    );
    assert!(!state.started());

    let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let recording = RecordingHostServices::default();
    let services = services(fixture.host, process).with_time(
        recording
            .services()
            .time()
            .expect("recording time service is present")
            .clone(),
    );
    let failure = block_on(prepare_codex(fixture.input, fixture.probe, services))
        .expect_err("expired preparation fails");
    assert_eq!(failure.stage(), PreparationStage::BoundedOutput);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.discovery_timed_out"
    );
    assert!(state.force_stopped());
    assert!(state.waited());
}
