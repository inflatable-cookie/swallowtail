use super::{
    FixtureServer, LocalHost, import_input, local_input_with_version, prepare_local, probe,
    source_authority, value,
};
use futures_executor::block_on;
use swallowtail_adapter_kimi::{
    KimiLocalServerSessionManagementInput, prepare_kimi_local_server_attached,
    start_kimi_local_server_owned,
};
use swallowtail_core::{
    Capability, ExecutionHostId, ProviderSessionBindingOrigin, ProviderSessionEffectTruth,
};
use swallowtail_runtime::RequestId;

#[test]
fn attached_import_is_lookup_only_and_issues_a_new_route_bound_binding() {
    let server = FixtureServer::start();
    let host = LocalHost::new(&server);
    let host_id = value(ExecutionHostId::new, "fixture.host.shared");
    let (prepared, services) =
        prepare_local(&server, &host, host_id.clone(), "fixture.kimi.state-root");
    let source = source_authority(
        host_id,
        "0.29.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .expect("ACP source authorizes import");
    let before = server.requests().len();
    let import = prepared
        .prepare_binding_import(import_input(&prepared, source, "fixture-import"))
        .expect("side-effect-free import preflight succeeds");
    assert_eq!(server.requests().len(), before);

    let binding =
        block_on(import.execute(services.clone())).expect("target lookup imports binding");
    assert_eq!(
        binding.origin(),
        ProviderSessionBindingOrigin::ExplicitlyImported
    );
    assert!(binding.supports(Capability::ProviderSessionArchive));
    assert!(binding.supports(Capability::ProviderSessionRestore));
    assert!(!binding.supports(Capability::ProviderSessionDelete));
    let import_requests = &server.requests()[before..];
    assert_eq!(import_requests.len(), 1);
    assert_eq!(import_requests[0].method, "GET");
    assert_eq!(import_requests[0].path, "/api/v1/sessions/session-1");
    assert!(import_requests[0].authenticated);

    let archive = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "imported-archive"),
            binding.clone(),
        ))
        .expect("imported binding prepares archive");
    assert_eq!(
        block_on(archive.execute(services.clone()))
            .expect("archive executes")
            .effect()
            .truth(),
        ProviderSessionEffectTruth::Applied
    );
    let restore = prepared
        .prepare_restore_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "imported-restore"),
            binding,
        ))
        .expect("imported binding prepares restore");
    assert_eq!(
        block_on(restore.execute(services))
            .expect("restore executes")
            .effect()
            .truth(),
        ProviderSessionEffectTruth::Applied
    );
}

#[test]
fn owned_import_uses_the_same_binding_contract_and_joins_its_child() {
    let server = FixtureServer::start();
    let host = LocalHost::new(&server);
    let host_id = value(ExecutionHostId::new, "fixture.host.owned");
    let services = host.services(host_id.clone(), true);
    let owned = block_on(start_kimi_local_server_owned(
        super::owned_input(&server, host_id.clone()),
        probe("fixture-owned-prepare"),
        services.clone(),
    ))
    .expect("owned server prepares");
    let source = source_authority(
        host_id,
        "0.29.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .expect("ACP source authorizes import");
    let import = owned
        .prepared()
        .prepare_binding_import(import_input(
            owned.prepared(),
            source,
            "fixture-owned-import",
        ))
        .expect("owned import preflights");
    let binding = block_on(import.execute(services)).expect("owned target imports");
    assert!(binding.supports(Capability::ProviderSessionArchive));
    assert!(binding.supports(Capability::ProviderSessionRestore));
    assert_eq!(
        block_on(owned.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert!(host.process_stopped_and_joined());
}

#[test]
fn unverified_newer_import_requires_explicit_acceptance() {
    let server = FixtureServer::start_with_version("0.37.0");
    let host = LocalHost::new(&server);
    let host_id = value(ExecutionHostId::new, "fixture.host.newer");
    let services = host.services(host_id.clone(), false);
    let input = local_input_with_version(
        &server,
        host_id.clone(),
        "fixture.kimi.state-root",
        "fixture.kimi.newer",
        "fixture.endpoint",
        "fixture.bearer",
        "0.37.0",
    );
    let prepared = block_on(prepare_kimi_local_server_attached(
        input,
        probe("fixture-newer-prepare"),
        services.clone(),
    ))
    .expect("newer local server remains visible");
    let source = source_authority(
        host_id,
        "0.37.0",
        Some("fixture.kimi.state-root"),
        "session-1",
    )
    .unwrap();
    let rejected = prepared
        .prepare_binding_import(import_input(
            &prepared,
            source.clone(),
            "fixture-newer-rejected",
        ))
        .expect_err("newer import needs acceptance");
    assert_eq!(
        rejected.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.import.unverified_newer"
    );
    let accepted = prepared
        .prepare_binding_import(
            import_input(&prepared, source, "fixture-newer-accepted").allow_unverified_newer(),
        )
        .expect("explicitly accepted newer import preflights");
    block_on(accepted.execute(services)).expect("accepted newer target imports");
}

#[test]
fn provider_neutral_lifecycle_contract_remains_green() {
    swallowtail_testkit::assert_provider_session_management_contract();
}
