use super::{FixtureServer, LocalHost, import_input, prepare_local, source_authority, value};
use futures_executor::block_on;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{CancellationControl, Deadline, MonotonicInstant};

#[test]
fn missing_archived_and_disconnected_targets_never_issue_authority() {
    for (session, code) in [
        (
            "missing-session",
            "swallowtail.kimi.local_server.import.target_missing",
        ),
        (
            "archived-session",
            "swallowtail.kimi.local_server.import.target_ineligible",
        ),
        (
            "disconnect-session",
            "swallowtail.kimi.local_server.transport_failed",
        ),
    ] {
        let server = FixtureServer::start();
        let host = LocalHost::new(&server);
        let host_id = value(ExecutionHostId::new, "fixture.host.lookup");
        let (prepared, services) =
            prepare_local(&server, &host, host_id.clone(), "fixture.kimi.state-root");
        let source =
            source_authority(host_id, "0.29.0", Some("fixture.kimi.state-root"), session).unwrap();
        let operation = prepared
            .prepare_binding_import(import_input(&prepared, source, "fixture-lookup-failure"))
            .expect("lookup failure still preflights");
        let error = block_on(operation.execute(services)).expect_err("lookup cannot import");
        assert_eq!(error.diagnostic().safe().code(), code);
        assert_eq!(host.credential_releases(), 2);
        assert!(
            server
                .requests()
                .iter()
                .all(|request| request.method != "POST")
        );
    }
}

#[test]
fn cancellation_and_deadline_join_read_only_lookup_without_effects() {
    let server = FixtureServer::start();
    let host = LocalHost::new(&server);
    let host_id = value(ExecutionHostId::new, "fixture.host.control");
    let (prepared, services) =
        prepare_local(&server, &host, host_id.clone(), "fixture.kimi.state-root");

    let source = source_authority(
        host_id.clone(),
        "0.29.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .unwrap();
    let cancellation = swallowtail_runtime::DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("preflight cancellation requests");
    let input = swallowtail_adapter_kimi::KimiLocalServerBindingImportInput::new(
        value(swallowtail_runtime::RequestId::new, "fixture-cancel-before"),
        value(swallowtail_runtime::ScopeId::new, "fixture-cancel-before"),
        source,
        prepared.binding_import_target(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation,
    );
    let operation = prepared
        .prepare_binding_import(input)
        .expect("cancelled import still has side-effect-free preflight");
    assert_eq!(
        block_on(operation.execute(services.clone()))
            .expect_err("cancelled import fails")
            .diagnostic()
            .safe()
            .code(),
        "swallowtail.kimi.local_server.preparation.cancelled"
    );

    server.delay_lifecycle_response(250);
    let source = source_authority(
        host_id.clone(),
        "0.29.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .unwrap();
    let cancellation = swallowtail_runtime::DiscoveryCancellation::new();
    let input = swallowtail_adapter_kimi::KimiLocalServerBindingImportInput::new(
        value(swallowtail_runtime::RequestId::new, "fixture-cancel-after"),
        value(swallowtail_runtime::ScopeId::new, "fixture-cancel-after"),
        source,
        prepared.binding_import_target(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation.clone(),
    );
    let operation = prepared.prepare_binding_import(input).unwrap();
    let error = std::thread::scope(|scope| {
        let running = scope.spawn(|| block_on(operation.execute(services.clone())));
        server.wait_until_seen("/api/v1/sessions/session-1");
        block_on(cancellation.request()).expect("lookup cancellation requests");
        running.join().expect("lookup thread joins")
    })
    .expect_err("cancelled lookup does not import");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.cancelled"
    );

    let source = source_authority(
        host_id,
        "0.29.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .unwrap();
    let input = import_input(&prepared, source, "fixture-deadline-after");
    let operation = prepared.prepare_binding_import(input).unwrap();
    let error = std::thread::scope(|scope| {
        let running = scope.spawn(|| block_on(operation.execute(services)));
        server.wait_until_seen_count("/api/v1/sessions/session-1", 2);
        host.set_now(100);
        running.join().expect("deadline lookup thread joins")
    })
    .expect_err("deadline lookup does not import");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.timed_out"
    );
    assert!(
        server
            .requests()
            .iter()
            .all(|request| request.method != "POST")
    );
    assert!(host.credential_releases() >= 3);
}
