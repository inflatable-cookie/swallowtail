#[test]
fn exact_versions_advertise_and_prepare_thread_catalogue_only_inside_the_corpus() {
    for (version, expected) in [
        ("0.104.0", false),
        ("0.105.0", true),
        ("0.107.0", true),
        ("0.110.0", true),
        ("0.146.0", true),
        ("0.147.0", true),
        ("0.148.0", false),
    ] {
        let recording = RecordingHostServices::default();
        let prepared_app = prepared(CodexPreparedDriver::AppServer, version, &recording, true);
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionCatalogue),
            expected,
            "catalogue advertisement for {version}"
        );
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionImport),
            expected,
            "import advertisement for {version}"
        );
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionReconciliation),
            expected,
            "reconciliation advertisement for {version}"
        );
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionHistory),
            expected,
            "history advertisement for {version}"
        );
        let result = prepared_app.prepare_session_catalogue(catalogue_input(version));
        assert_eq!(
            result.is_ok(),
            expected,
            "catalogue preparation for {version}"
        );
    }
}

#[test]
fn prepared_catalogue_is_resource_scoped_bounded_paginated_and_redacted() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("resource"))
        .expect("thread catalogue prepares");

    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let first = block_on(catalogue.list_sessions(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("first page projects");
    let candidates = first.candidates().collect::<Vec<_>>();
    assert_eq!(candidates.len(), 2);
    assert_eq!(
        candidates[0].candidate_id().as_str(),
        "codex-thread-candidate-0"
    );
    assert_eq!(candidates[0].display().title(), Some(PRIVATE_TITLE));
    assert_eq!(candidates[0].display().preview(), Some(PRIVATE_PREVIEW));
    assert_eq!(
        candidates[0].updated_at_unix_milliseconds(),
        Some(1_775_000_000_000)
    );
    assert_eq!(
        candidates[0].activity(),
        ProviderSessionActivityState::Inactive
    );
    assert_eq!(
        candidates[0].import_availability(),
        ProviderSessionImportAvailability::Available
    );
    assert_eq!(
        candidates[1].activity(),
        ProviderSessionActivityState::Active
    );
    assert_eq!(
        candidates[1].import_availability(),
        ProviderSessionImportAvailability::Unavailable(
            ProviderSessionImportUnavailableReason::Active
        )
    );
    let cursor = first
        .next_cursor()
        .expect("first page has a cursor")
        .clone();
    assert_eq!(cursor.observed_candidates(), 2);
    let list = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/list")
        .expect("thread/list request is captured");
    assert_eq!(list["params"]["limit"], 2);
    assert_eq!(list["params"]["archived"], false);
    assert_eq!(
        list["params"]["sourceKinds"],
        serde_json::json!(["cli", "vscode", "appServer"])
    );
    assert_eq!(list["params"]["cwd"], "/private/recording/workspace");
    assert!(state.waited());

    let next = catalogue
        .next_page_request(RequestId::new("catalogue-page-2").unwrap(), cursor)
        .expect("second page request prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let second = block_on(catalogue.list_page(
        next,
        host_services_with(process, &recording, [HostServiceKind::WorkingResource]),
    ))
    .expect("second page projects");
    let second_candidates = second.candidates().collect::<Vec<_>>();
    assert_eq!(second_candidates.len(), 1);
    assert_eq!(
        second_candidates[0].candidate_id().as_str(),
        "codex-thread-candidate-2"
    );
    assert!(second.next_cursor().is_none());
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 2);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 2);

    let debug = format!("{first:?}{second:?}");
    assert!(!debug.contains(PRIVATE_TITLE));
    assert!(!debug.contains(PRIVATE_PREVIEW));
    assert!(!debug.contains("private-thread-page-2"));
    assert!(!debug.contains("thread-provider-import"));
}

#[test]
fn resource_mismatch_fails_closed_without_projecting_candidates() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("wrong-resource"))
        .expect("thread catalogue prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::WrongResource,
    ));
    let failure = block_on(catalogue.list_sessions(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect_err("another cwd is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.thread_catalogue.resource_mismatch"
    );
    assert!(!format!("{failure:?}").contains("/private/another/workspace"));
    assert!(state.waited());
}

