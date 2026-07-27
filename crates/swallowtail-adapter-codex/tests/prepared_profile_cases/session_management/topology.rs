use super::support::lifecycle_binding;
use super::*;

#[test]
fn management_binding_drift_stops_before_effect_and_remote_host_executes() {
    let local = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let remote_host = ExecutionHostId::new("host.remote-authoritative").unwrap();
    let remote = prepared_on_host(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
        remote_host.clone(),
    );
    assert!(
        remote
            .prepare_archive_session(CodexSessionManagementInput::new(
                RequestId::new("drifted-binding").unwrap(),
                lifecycle_binding(&local, "0.145.0"),
            ))
            .is_err()
    );

    let operation = remote
        .prepare_archive_session(CodexSessionManagementInput::new(
            RequestId::new("remote-archive").unwrap(),
            lifecycle_binding(&remote, "0.145.0"),
        ))
        .expect("remote archive prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::LifecycleSuccess);
    let outcome =
        block_on(operation.execute(crate::support::host_services_for(remote_host, process)))
            .expect("remote-authoritative lifecycle executes");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(
        state.request().working_resource.as_deref(),
        Some("workspace.main")
    );
    assert!(state.waited());
}
