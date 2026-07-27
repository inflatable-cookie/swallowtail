use super::{
    FixtureServer, LocalHost, import_input, local_input, prepare_local, probe, source_authority,
    value,
};
use futures_executor::block_on;
use swallowtail_adapter_kimi::{
    KimiLocalServerBindingImportInput, prepare_kimi_local_server_attached,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{Deadline, DiscoveryCancellation, MonotonicInstant, RequestId, ScopeId};

#[test]
fn source_and_target_identity_mismatch_stops_before_lookup() {
    let server = FixtureServer::start();
    let host = LocalHost::new(&server);
    let host_id = value(ExecutionHostId::new, "fixture.host.identity");
    let (prepared, services) =
        prepare_local(&server, &host, host_id.clone(), "fixture.kimi.state-root");
    let before = server.requests().len();

    for (source, code) in [
        (
            source_authority(
                value(ExecutionHostId::new, "fixture.host.other"),
                "0.29.0",
                Some("fixture.kimi.state-root"),
                "session-1",
            )
            .unwrap(),
            "swallowtail.kimi.local_server.import.host_mismatch",
        ),
        (
            source_authority(
                host_id.clone(),
                "0.28.1",
                Some("fixture.kimi.state-root"),
                "session-1",
            )
            .unwrap(),
            "swallowtail.kimi.local_server.import.version_mismatch",
        ),
        (
            source_authority(
                host_id.clone(),
                "0.29.0",
                Some("fixture.kimi.other-root"),
                "session-1",
            )
            .unwrap(),
            "swallowtail.kimi.local_server.import.state_root_mismatch",
        ),
    ] {
        let error = prepared
            .prepare_binding_import(import_input(&prepared, source, "fixture-mismatch"))
            .expect_err("source mismatch rejects");
        assert_eq!(error.diagnostic().safe().code(), code);
    }

    for (suffix, instance, endpoint, credential) in [
        (
            "instance",
            "fixture.kimi.other-instance",
            "fixture.endpoint",
            "fixture.bearer",
        ),
        (
            "endpoint",
            "fixture.kimi.local",
            "fixture.other-endpoint",
            "fixture.bearer",
        ),
        (
            "credential",
            "fixture.kimi.local",
            "fixture.endpoint",
            "fixture.other-bearer",
        ),
    ] {
        let other = block_on(prepare_kimi_local_server_attached(
            local_input(
                &server,
                host_id.clone(),
                "fixture.kimi.state-root",
                instance,
                endpoint,
                credential,
            ),
            probe(&format!("fixture-{suffix}-prepare")),
            services.clone(),
        ))
        .expect("drifted target independently prepares");
        let source = source_authority(
            host_id.clone(),
            "0.29.0",
            Some("fixture.kimi.state-root"),
            "session-1",
        )
        .unwrap();
        let input = KimiLocalServerBindingImportInput::new(
            value(RequestId::new, &format!("fixture-{suffix}-mismatch")),
            value(ScopeId::new, &format!("fixture-{suffix}-mismatch")),
            source,
            other.binding_import_target(),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        );
        assert_eq!(
            prepared
                .prepare_binding_import(input)
                .expect_err("target identity drift rejects")
                .diagnostic()
                .safe()
                .code(),
            "swallowtail.kimi.local_server.import.target_mismatch"
        );
    }
    assert_eq!(
        server.requests()[before..]
            .iter()
            .filter(|request| request.path.starts_with("/api/v1/sessions/"))
            .count(),
        0
    );
}

#[test]
fn raw_identity_without_bound_acp_state_root_cannot_mint_import_authority() {
    let error = source_authority(
        value(ExecutionHostId::new, "fixture.host.raw-id"),
        "0.29.0",
        None,
        "private-session-id",
    )
    .expect_err("a raw provider id and ACP family match are insufficient");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.import.state_root_missing"
    );
    assert!(!format!("{error:?}").contains("private-session-id"));

    let authority = source_authority(
        value(ExecutionHostId::new, "fixture.host.redaction"),
        "0.29.0",
        Some("private-kimi-state-root"),
        "private-session-id",
    )
    .expect("fully bound ACP evidence can mint authority");
    let debug = format!("{authority:?}");
    assert!(!debug.contains("private-session-id"));
    assert!(!debug.contains("private-kimi-state-root"));
}
