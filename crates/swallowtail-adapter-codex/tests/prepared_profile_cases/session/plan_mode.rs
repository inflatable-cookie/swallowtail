#[test]
fn plan_mode_is_gated_by_the_exact_codex_release() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.84.0",
        &RecordingHostServices::default(),
        false,
    );
    let failure = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("plan-mode-before-support").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default().with_harness_mode(HarnessMode::Plan),
        ))
        .expect_err("pre-plan-mode Codex release is rejected");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.harness_mode_unsupported"
    );
}

