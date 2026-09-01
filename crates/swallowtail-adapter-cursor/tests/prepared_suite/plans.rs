#[test]
fn explicit_routes_prepare_only_their_typed_operations() {
    let host_id = host_id();
    let catalogue =
        prepare(CursorPreparedDriver::Catalogue, host_id.clone()).expect("catalogue prepares");
    let CursorPreparedIntegration::Catalogue(catalogue) = catalogue else {
        panic!("catalogue route remains explicit");
    };
    let prepared = catalogue
        .prepare_catalogue(CursorCatalogueProfileInput::new(request_id("catalogue")))
        .expect("catalogue operation prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.catalogue"
    );
    let operation_host = support::FixtureHost::completed([stdout(CATALOGUE)]);
    let models = block_on(prepared.list_models(operation_host.services(host_id.clone())))
        .expect("prepared catalogue executes");
    assert_eq!(models.len(), 2);

    let acp = prepare(CursorPreparedDriver::Acp, host_id.clone()).expect("ACP prepares");
    let CursorPreparedIntegration::Acp(acp) = acp else {
        panic!("ACP route remains explicit");
    };
    let prepared = acp
        .prepare_session(CursorAcpSessionProfileInput::new(
            request_id("acp"),
            working_resource(),
        ))
        .expect("ACP session prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.acp"
    );
    assert_eq!(
        prepared.request().access_policy(),
        &swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
    );
    let binding = SessionResumeBinding::without_model(
        SessionRef::new("cursor-prepared-session").expect("session"),
        prepared.plan().instance_id().clone(),
        prepared.plan().execution_host_id().clone(),
        working_resource(),
        prepared.request().access_policy().clone(),
    );
    assert_eq!(
        prepared
            .prepare_working_state_restoration(
                request_id("cursor-recovery"),
                binding,
                RuntimeTurnId::new("lost-cursor-turn").expect("turn"),
            )
            .expect("attachment recovery prepares")
            .method(),
        WorkingStateRestorationMethod::ProviderSessionAttachmentRecovery
    );

    let headless =
        prepare(CursorPreparedDriver::Headless, host_id.clone()).expect("headless prepares");
    let CursorPreparedIntegration::Headless(headless) = headless else {
        panic!("headless route remains explicit");
    };
    let prepared = headless
        .prepare_run(headless_input(ResourceAccess::Read))
        .expect("headless run prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.headless"
    );
    assert_eq!(
        prepared.plan().model_id().map(ModelId::as_str),
        Some("fixture-model")
    );
    assert_eq!(
        prepared.evidence().observable_activity().availability(),
        swallowtail_core::ObservableActivityAvailability::Available
    );
    let operation_host = support::FixtureHost::completed([stdout(HEADLESS)]);
    let mut handle = block_on(prepared.start_run(operation_host.services(host_id)))
        .expect("prepared headless run starts");
    let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn headless_preparation_keeps_read_and_write_authority_distinct() {
    for access in [ResourceAccess::Read, ResourceAccess::ReadWrite] {
        let prepared = prepare(CursorPreparedDriver::Headless, host_id()).expect("prepares");
        let CursorPreparedIntegration::Headless(prepared) = prepared else {
            panic!("headless route");
        };
        let run = prepared
            .prepare_run(headless_input(access))
            .expect("run prepares");
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == swallowtail_core::Capability::WorkingResource
                && requirement.constraints().any(|constraint| {
                    constraint == &swallowtail_core::CapabilityConstraint::ResourceAccess(access)
                })
        }));
    }
}

#[test]
fn prepared_read_mode_is_immutable_and_agrees_with_the_driver_and_argv() {
    for release in QUALIFIED_RELEASES {
        let host_id = host_id();
        let CursorPreparedIntegration::Headless(headless) = prepare_release(
            CursorPreparedDriver::Headless,
            host_id.clone(),
            &format!("{release}\n"),
        )
        .unwrap_or_else(|_| panic!("headless prepares on {release}")) else {
            panic!("headless route remains explicit");
        };

        let default_read = headless
            .prepare_run(headless_input(ResourceAccess::Read))
            .unwrap_or_else(|_| panic!("default read run prepares on {release}"));
        assert_eq!(
            default_read.read_mode(),
            Some(CursorHeadlessReadMode::Plan),
            "{release}"
        );
        assert_eq!(
            default_read.low_level_driver().read_mode(),
            Some(CursorHeadlessReadMode::Plan),
            "{release}"
        );

        let write = headless
            .prepare_run(headless_input(ResourceAccess::ReadWrite))
            .unwrap_or_else(|_| panic!("write run prepares on {release}"));
        assert_eq!(write.read_mode(), None, "{release}");
        assert_eq!(write.low_level_driver().read_mode(), None, "{release}");

        let ask = headless
            .prepare_run(
                headless_input(ResourceAccess::Read)
                    .with_read_mode(CursorHeadlessReadMode::Ask)
                    .unwrap_or_else(|_| panic!("ask selection is admitted on {release}")),
            )
            .unwrap_or_else(|_| panic!("ask run prepares on {release}"));
        assert_eq!(
            ask.read_mode(),
            Some(CursorHeadlessReadMode::Ask),
            "{release}"
        );
        assert_eq!(
            ask.low_level_driver().read_mode(),
            Some(CursorHeadlessReadMode::Ask),
            "{release}"
        );
        assert_eq!(
            ask.plan().model_id().map(ModelId::as_str),
            Some("fixture-model"),
            "{release}"
        );
        assert_eq!(
            ask.evidence().binding().driver_identity().id().as_str(),
            "swallowtail.cursor-agent.headless",
            "{release}"
        );

        let operation_host = support::FixtureHost::completed([stdout(HEADLESS)]);
        let mut handle = block_on(ask.start_run(operation_host.services(host_id)))
            .unwrap_or_else(|_| panic!("prepared ask run starts on {release}"));
        let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
        let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed, "{release}");
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean, "{release}");

        let arguments = operation_host.observed().arguments;
        assert!(
            arguments.ends_with(&["--mode".to_owned(), "ask".to_owned()]),
            "{release}"
        );
        assert_eq!(
            arguments.iter().filter(|value| *value == "--mode").count(),
            1,
            "{release}"
        );
    }
}
