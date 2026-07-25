use super::*;

#[test]
fn every_prepared_profile_executes_on_a_remote_authoritative_host() {
    let host = ExecutionHostId::new("host.remote-authoritative").unwrap();
    let recording = RecordingHostServices::default();
    let prepared_app = prepared_on_host(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &recording,
        true,
        host.clone(),
    );

    let catalogue = prepared_app
        .prepare_catalogue(RequestId::new("remote-catalogue").unwrap(), None)
        .expect("remote catalogue prepares");
    assert_eq!(catalogue.plan().execution_host_id(), &host);
    let (process, _) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    block_on(catalogue.list_models(host_services_for(host.clone(), process)))
        .expect("remote catalogue executes");

    let read_only = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("remote-read-only").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("remote read-only session prepares");
    assert_eq!(read_only.plan().execution_host_id(), &host);
    let (process, _) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(read_only.open_session(host_services_for(host.clone(), process)))
        .expect("remote read-only session executes");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let bounded = prepared_app
        .prepare_bounded_workspace_session(CodexSessionProfileInput::new(
            RequestId::new("remote-bounded").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("remote bounded session prepares");
    assert_eq!(bounded.plan().execution_host_id(), &host);
    let (process, _) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(bounded.open_session(host_services_with_for(
        host.clone(),
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("remote bounded session executes");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let prepared_exec = prepared_on_host(
        CodexPreparedDriver::StructuredExec,
        "0.145.0",
        &recording,
        false,
        host.clone(),
    );
    let exec = prepared_exec
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new("remote-exec").unwrap(),
            OperationContent::new("consumer-owned prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .expect("remote exec prepares");
    assert_eq!(exec.plan().execution_host_id(), &host);
    let (process, _) = FakeProcessService::completed(COMPLETED_JSONL);
    let handle =
        block_on(exec.start_run(host_services_for(host, process))).expect("remote exec executes");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}
