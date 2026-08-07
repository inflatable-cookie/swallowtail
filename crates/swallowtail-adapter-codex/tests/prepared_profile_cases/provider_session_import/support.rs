pub(super) fn catalogue_candidate(
    catalogue: &swallowtail_adapter_codex::CodexPreparedSessionCatalogue,
    recording: &RecordingHostServices,
) -> swallowtail_runtime::ProviderSessionCandidate {
    let (process, _) = ScriptedAppServer::new(AppServerMode::ThreadCatalogue(
        ThreadCatalogueMode::Available,
    ));
    block_on(catalogue.list_sessions(host_services_with(
        process,
        recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("thread catalogue executes")
    .candidates()
    .next()
    .expect("available candidate exists")
    .clone()
}

pub(super) fn catalogue_input(suffix: &str) -> CodexSessionCatalogueInput {
    CodexSessionCatalogueInput::new(
        RequestId::new(format!("catalogue-{suffix}")).unwrap(),
        ProviderSessionCatalogueId::new(format!("codex-catalogue-{suffix}")).unwrap(),
        working_resource(),
        ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(4).unwrap(),
            NonZeroU32::new(64).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(128).unwrap(),
        )
        .unwrap(),
    )
}

pub(super) fn session_input(suffix: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(suffix).unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default(),
    )
}
