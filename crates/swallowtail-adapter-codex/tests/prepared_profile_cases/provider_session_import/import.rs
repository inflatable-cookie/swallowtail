#[test]
fn import_revalidates_exact_thread_and_binding_load_resume_stay_unchanged() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let catalogue = prepared_app
        .prepare_session_catalogue(catalogue_input("import"))
        .expect("thread catalogue prepares");
    let candidate = catalogue_candidate(&catalogue, &recording);
    let import = prepared_app
        .prepare_read_only_session_import(&catalogue, candidate, session_input("import-request"))
        .expect("thread import prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let outcome = block_on(import.import_session(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("thread import succeeds");
    assert_eq!(
        outcome.binding().origin(),
        ProviderSessionBindingOrigin::ExplicitlyImported
    );
    assert_eq!(
        outcome.binding().provider_session_ref().as_provider_value(),
        "thread-provider-import"
    );
    let read = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/read")
        .expect("thread/read request is captured");
    assert_eq!(read["params"]["threadId"], "thread-provider-import");
    assert_eq!(read["params"]["includeTurns"], true);
    assert!(state.waited());

    let session = prepared_app
        .prepare_read_only_session(session_input("continuation-profile"))
        .expect("existing read-only continuation profile prepares");
    let (process, load_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let load_services = support::host_services(process);
    let loaded = block_on(
        session
            .load_session(
                RequestId::new("imported-load").unwrap(),
                outcome.binding().clone(),
                load_services.clone(),
            )
            .expect("imported load request prepares"),
    )
    .expect("imported session loads through the existing path");
    assert_eq!(loaded.replay().count(), 2);
    let (_, handle) = loaded.into_parts();
    assert_eq!(
        block_on(support::close_session(handle, load_services)),
        CleanupOutcome::Clean
    );
    assert!(load_state.methods().contains(&"thread/resume".to_owned()));
    assert!(load_state.waited());

    let (process, resume_state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let resume_services = support::host_services(process);
    let handle = block_on(
        session
            .resume_session(
                RequestId::new("imported-resume").unwrap(),
                outcome.binding().clone(),
                resume_services.clone(),
            )
            .expect("imported resume request prepares"),
    )
    .expect("imported session resumes through the existing path");
    assert_eq!(
        block_on(support::close_session(handle, resume_services)),
        CleanupOutcome::Clean
    );
    assert!(resume_state.methods().contains(&"thread/resume".to_owned()));
    assert!(resume_state.waited());
}

#[test]
fn stale_missing_active_and_mismatched_revalidation_issue_no_binding() {
    for (mode, expected_code) in [
        (
            ThreadCatalogueMode::Changed,
            "swallowtail.codex.thread_import.candidate_changed",
        ),
        (
            ThreadCatalogueMode::Missing,
            "swallowtail.codex.app_server.request_failed",
        ),
        (
            ThreadCatalogueMode::Active,
            "swallowtail.provider_session_import.revalidation_mismatch",
        ),
        (
            ThreadCatalogueMode::Mismatched,
            "swallowtail.codex.thread_import.candidate_changed",
        ),
        (
            ThreadCatalogueMode::WrongResource,
            "swallowtail.codex.thread_catalogue.resource_mismatch",
        ),
    ] {
        let recording = RecordingHostServices::default();
        let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
        let catalogue = prepared_app
            .prepare_session_catalogue(catalogue_input("revalidate"))
            .expect("thread catalogue prepares");
        let candidate = catalogue_candidate(&catalogue, &recording);
        let import = prepared_app
            .prepare_read_only_session_import(
                &catalogue,
                candidate,
                session_input("revalidation-request"),
            )
            .expect("thread import prepares");
        let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(mode));
        let failure = block_on(import.import_session(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect_err("drifted thread cannot issue a binding");
        assert_eq!(failure.diagnostic().code(), expected_code);
        assert!(state.waited());
    }
}
