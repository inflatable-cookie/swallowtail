#[test]
fn headless_model_parameters_reject_raw_grammar_and_unqualified_tuples() {
    let prepared = prepare(CursorPreparedDriver::Headless, host_id()).expect("prepares");
    let CursorPreparedIntegration::Headless(prepared) = prepared else {
        panic!("headless route");
    };

    for model in [
        "claude-opus-4-8[context=1m]",
        "composer-2.5[fast=true]",
        "claude-opus-5[context=1m]",
    ] {
        let err = prepared
            .prepare_run(parameterized_input(
                parameterized_selection(model).expect("selection"),
                ResourceAccess::Read,
            ))
            .expect_err(model);
        assert!(
            err.diagnostic().safe().code().contains("model_parameter"),
            "{model}"
        );
    }

    let err = parameterized_selection("composer-2.5")
        .expect("selection")
        .with_context(CursorHeadlessContext::ThreeHundredK)
        .expect_err("composer context");
    assert_eq!(
        err.diagnostic().safe().code(),
        "swallowtail.cursor.headless.model_parameter_rejected"
    );
}

#[test]
fn preparation_rejects_access_and_axis_drift_before_discovery() {
    let host_id = host_id();
    let access_id = AccessProfileId::new("cursor.fixture.access").expect("access id");
    let access = cursor_subscription_access_profile(access_id.clone());
    let evidence = evidence(access_id);
    let host = support::FixtureHost::completed([stdout(VERSION)]);
    let input = CursorPreparationInput::new(
        CursorPreparedDriver::Catalogue,
        swallowtail_core::ConfiguredInstanceId::new("cursor.fixture").expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("cursor.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new("wrong.axis").expect("axis"),
        ),
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        access,
        evidence,
    );
    let error = block_on(prepare_cursor(input, probe(), host.services(host_id)))
        .expect_err("axis drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cursor.preparation.target_axis_mismatch"
    );
    assert!(!host.started());
}

#[test]
fn ask_selection_rejects_read_write_authority_at_input_construction() {
    let failure = headless_input(ResourceAccess::ReadWrite)
        .with_read_mode(CursorHeadlessReadMode::Ask)
        .expect_err("ask rejects read-write authority");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.cursor.headless.read_mode_access_rejected"
    );
}

#[test]
fn ask_selection_rejects_unqualified_releases_at_preparation() {
    let CursorPreparedIntegration::Headless(headless) = prepare_release(
        CursorPreparedDriver::Headless,
        host_id(),
        "2026.08.12-abcdef1\n",
    )
    .expect("newer unverified release still prepares the route") else {
        panic!("headless route remains explicit");
    };

    headless
        .prepare_run(headless_input(ResourceAccess::Read))
        .expect("default read run still prepares on a newer release");

    let failure = headless
        .prepare_run(
            headless_input(ResourceAccess::Read)
                .with_read_mode(CursorHeadlessReadMode::Ask)
                .expect("ask selection is admitted for read authority"),
        )
        .expect_err("ask rejects a newer unverified release");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.cursor.headless.ask_mode_unqualified"
    );
}
