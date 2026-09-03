use super::*;

#[test]
fn prepared_sessions_return_inactive_management_authority_after_close() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("managed-open").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("session prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = crate::support::host_services(process);
    let handle = block_on(profile.open_session(services.clone())).expect("session opens");
    let binding = handle
        .management_binding()
        .expect("prepared session returns management authority")
        .clone();
    assert_eq!(binding.origin(), ProviderSessionBindingOrigin::Created);
    assert!(binding.supports(Capability::ProviderSessionArchive));
    assert!(binding.supports(Capability::ProviderSessionRestore));
    assert!(binding.supports(Capability::ProviderSessionDelete));
    assert_eq!(
        block_on(crate::support::close_session(handle, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(
        binding.provider_session_ref().as_provider_value(),
        "thread-provider-new"
    );
    assert!(state.waited());
}
