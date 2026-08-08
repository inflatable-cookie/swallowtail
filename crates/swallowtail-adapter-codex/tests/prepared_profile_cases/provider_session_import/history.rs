fn history_binding(
    prepared_app: &swallowtail_adapter_codex::CodexPreparedIntegration,
) -> SessionResumeBinding {
    let session = prepared_app
        .prepare_read_only_session(session_input("history-session"))
        .expect("read-only session prepares");
    let plan = session.plan();
    SessionResumeBinding::new(
        swallowtail_core::SessionRef::new("thread-provider-import").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        working_resource(),
        session.request().access_policy().clone(),
    )
}

fn history_bounds(page_items: u32, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).unwrap(),
        NonZeroU64::new(4096).unwrap(),
        NonZeroU32::new(64).unwrap(),
        NonZeroU32::new(snapshot_items).unwrap(),
    )
}

fn history_input(
    suffix: &str,
    binding: SessionResumeBinding,
    page_items: u32,
    snapshot_items: u32,
) -> CodexSessionHistoryInput {
    CodexSessionHistoryInput::new(
        RequestId::new(format!("history-{suffix}")).unwrap(),
        ProviderSessionHistoryId::new(format!("codex-history-{suffix}")).unwrap(),
        model(),
        binding,
        history_bounds(page_items, snapshot_items),
    )
}

fn forbidden_control_methods(methods: &[String]) -> bool {
    methods.iter().any(|method| {
        matches!(
            method.as_str(),
            "turn/start"
                | "turn/interrupt"
                | "thread/resume"
                | "thread/archive"
                | "thread/unarchive"
                | "thread/delete"
        )
    })
}

#[test]
fn newest_first_history_pages_over_thread_read_without_control_dispatch() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let binding = history_binding(&prepared_app);
    let history = prepared_app
        .prepare_session_history(history_input("first", binding, 1, 8))
        .expect("history prepares");

    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let first = block_on(history.page_history(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("first history page succeeds");

    assert_eq!(first.fetched_count(), 1);
    assert!(first.has_older());
    assert_eq!(
        first.total(),
        swallowtail_runtime::ProviderSessionHistoryTotal::Exact(2)
    );
    assert_eq!(
        first
            .items()
            .filter_map(|item| item.content().map(|content| content.as_str().to_owned()))
            .collect::<Vec<_>>(),
        ["Earlier answer."]
    );
    let older_cursor = first.older_cursor().expect("older cursor").clone();
    let older_request = history
        .older_page_request(RequestId::new("history-older").unwrap(), older_cursor)
        .expect("older request prepares");

    let (process, state_older) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let older = block_on(history.page(
        older_request,
        host_services_with(process, &recording, [HostServiceKind::WorkingResource]),
    ))
    .expect("older history page succeeds");

    assert_eq!(older.fetched_count(), 1);
    assert!(!older.has_older());
    assert!(older.older_cursor().is_none());
    assert_eq!(
        older
            .items()
            .filter_map(|item| item.content().map(|content| content.as_str().to_owned()))
            .collect::<Vec<_>>(),
        ["Earlier question."]
    );

    let methods = state.methods();
    assert!(methods.iter().any(|method| method == "thread/read"));
    assert!(!forbidden_control_methods(&methods));
    assert!(!forbidden_control_methods(&state_older.methods()));
    let read = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/read")
        .expect("thread/read is captured");
    assert_eq!(read["params"]["threadId"], "thread-provider-import");
    assert_eq!(read["params"]["includeTurns"], true);
}

#[test]
fn empty_history_page_is_exact_zero_without_cursor() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let binding = history_binding(&prepared_app);
    let history = prepared_app
        .prepare_session_history(history_input("empty", binding, 2, 8))
        .expect("history prepares");
    let (process, _) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::EmptyHistory,
    ));
    let page = block_on(history.page_history(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("empty history page succeeds");

    assert_eq!(page.fetched_count(), 0);
    assert!(!page.has_older());
    assert!(page.older_cursor().is_none());
    assert_eq!(
        page.total(),
        swallowtail_runtime::ProviderSessionHistoryTotal::Exact(0)
    );
}

#[test]
fn history_snapshot_overflow_fails_closed() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.146.0", &recording, true);
    let binding = history_binding(&prepared_app);
    let history = prepared_app
        .prepare_session_history(history_input("overflow", binding, 1, 1))
        .expect("history prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    let failure = block_on(history.page_history(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect_err("snapshot overflow fails closed");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
    assert!(!forbidden_control_methods(&state.methods()));
}

#[test]
fn history_capability_tracks_catalogue_version_gate() {
    let recording = RecordingHostServices::default();
    for (version, expected) in [("0.104.0", false), ("0.105.0", true)] {
        let prepared_app = prepared(CodexPreparedDriver::AppServer, version, &recording, true);
        assert_eq!(
            prepared_app
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionHistory),
            expected,
            "history advertisement for {version}"
        );
        if expected {
            let binding = history_binding(&prepared_app);
            prepared_app
                .prepare_session_history(history_input(version, binding, 2, 8))
                .expect("history prepares on supported version");
        }
    }
}
