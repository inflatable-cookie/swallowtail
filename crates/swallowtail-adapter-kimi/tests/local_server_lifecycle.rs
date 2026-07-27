#[path = "local_server_lifecycle/cancellation.rs"]
mod cancellation;
mod local_server_lifecycle_support;

use futures_executor::block_on;
use local_server_lifecycle_support::{FixtureHost, FixtureServer};
use swallowtail_adapter_kimi::{
    KimiLocalServerAttachedInput, KimiLocalServerCatalogueInput, KimiLocalServerOwnedInput,
    KimiLocalServerPreparationProbe, KimiLocalServerPreparedIntegration,
    KimiLocalServerSessionManagementInput, kimi_code_binding, kimi_local_server_descriptor,
    prepare_kimi_local_server_attached, start_kimi_local_server_owned,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExtensionNamespace, InstanceRevision,
    InstanceTargetRef, ProviderSessionBindingOrigin, ProviderSessionEffectTruth, RuntimeReadiness,
    SessionRef, SupportAuthority,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, CleanupOutcome, Deadline, DiscoveryCancellation, HostServices,
    MonotonicInstant, PreparedAccessEvidence, ProviderSessionManagementBinding, RequestId, ScopeId,
    WorkingResourceRef,
};

#[test]
fn attached_archive_and_restore_work_on_local_and_remote_authoritative_hosts() {
    for host_name in ["host.local", "host.remote-authoritative"] {
        let server = FixtureServer::start();
        let host = FixtureHost::new(&server);
        let execution_host = value(ExecutionHostId::new, host_name);
        let services = host.services(execution_host.clone(), false);
        let prepared = prepare_attached(execution_host, services.clone());
        let binding = binding(&prepared);

        let archive = prepared
            .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
                value(RequestId::new, "archive-request"),
                binding.clone(),
            ))
            .expect("archive prepares");
        let archived = block_on(archive.execute(services.clone())).expect("archive executes");
        assert_eq!(
            archived.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );

        let restore = prepared
            .prepare_restore_session(KimiLocalServerSessionManagementInput::new(
                value(RequestId::new, "restore-request"),
                binding,
            ))
            .expect("restore prepares");
        let restored = block_on(restore.execute(services)).expect("restore executes");
        assert_eq!(
            restored.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert!(
            !prepared
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionDelete)
        );

        let rejected = prepared
            .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
                value(RequestId::new, "archive-missing"),
                binding_for(&prepared, "missing-session"),
            ))
            .expect("missing-session archive prepares");
        let rejected = block_on(
            rejected.execute(host.services(value(ExecutionHostId::new, host_name), false)),
        )
        .expect("provider rejection returns effect truth");
        assert_eq!(
            rejected.effect().truth(),
            ProviderSessionEffectTruth::FailedBeforeEffect
        );

        let requests = server.requests();
        assert_eq!(
            requests
                .iter()
                .filter(|request| request.path == "/api/v1/healthz")
                .count(),
            1
        );
        assert!(
            requests
                .iter()
                .any(|request| { request.path == "/api/v1/meta" && request.authenticated })
        );
        assert!(requests.iter().any(|request| {
            request.path == "/api/v1/sessions/session-1:archive"
                && request.method == "POST"
                && request.authenticated
        }));
        assert!(requests.iter().any(|request| {
            request.path == "/api/v1/sessions/session-1:restore"
                && request.method == "POST"
                && request.authenticated
        }));
        assert_eq!(host.credential_releases(), 4);
        assert!(host.process_arguments().is_none());
    }
}

#[test]
fn attached_catalogue_lists_configured_aliases_without_session_or_refresh() {
    for version in ["0.28.1", "0.29.0"] {
        let server = FixtureServer::start_with_version(version);
        let host = FixtureHost::new(&server);
        let execution_host = value(ExecutionHostId::new, "host.local");
        let services = host.services(execution_host.clone(), false);
        let prepared = block_on(prepare_kimi_local_server_attached(
            attached_input_for_version(execution_host, version),
            probe(),
            services.clone(),
        ))
        .expect("attached Kimi prepares");
        let catalogue = prepared
            .prepare_catalogue(KimiLocalServerCatalogueInput::new(value(
                RequestId::new,
                "catalogue-request",
            )))
            .expect("catalogue prepares");
        let models = block_on(catalogue.list_models(services)).expect("catalogue executes");
        assert_eq!(
            models
                .iter()
                .map(|model| model.id().as_str())
                .collect::<Vec<_>>(),
            ["k2", "gpt4o"]
        );
        assert_eq!(
            models[0]
                .metadata()
                .catalog_observations()
                .and_then(|observations| observations.reasoning_supported()),
            Some(true)
        );
        assert!(server.requests().iter().any(|request| {
            request.path == "/api/v1/models" && request.method == "GET" && request.authenticated
        }));
        assert_eq!(host.credential_releases(), 2);
    }
}

#[test]
fn owned_topology_uses_exact_safe_command_and_joins_only_its_child() {
    let server = FixtureServer::start();
    let host = FixtureHost::new(&server);
    let execution_host = value(ExecutionHostId::new, "host.remote-authoritative");
    let services = host.services(execution_host.clone(), true);
    let input = KimiLocalServerOwnedInput::new(
        attached_input(execution_host),
        value(InstanceTargetRef::new, "fixture.kimi.executable"),
    );
    let owned = block_on(start_kimi_local_server_owned(
        input,
        probe(),
        services.clone(),
    ))
    .expect("owned Kimi starts");
    assert_eq!(
        host.process_arguments()
            .expect("process request is retained"),
        [
            "web",
            "--no-open",
            "--host",
            "127.0.0.1",
            "--port",
            server
                .endpoint()
                .rsplit_once(':')
                .expect("fixture endpoint has a port")
                .1,
            "--log-level",
            "info",
        ]
        .map(str::to_owned)
    );
    assert_eq!(
        owned.prepared().instance().ownership(),
        swallowtail_core::InstanceOwnership::HostOwnedEphemeral
    );
    assert_eq!(host.credential_releases(), 1);
    assert_eq!(block_on(owned.close()), CleanupOutcome::Clean);
    assert!(host.process_stopped_and_joined());
}

#[test]
fn owned_readiness_mismatch_stops_and_joins_the_spawned_child() {
    let server = FixtureServer::start();
    let host = FixtureHost::new(&server);
    host.set_ready_endpoint("http://127.0.0.1:1");
    let execution_host = value(ExecutionHostId::new, "host.local");
    let services = host.services(execution_host.clone(), true);
    let error = match block_on(start_kimi_local_server_owned(
        KimiLocalServerOwnedInput::new(
            attached_input(execution_host),
            value(InstanceTargetRef::new, "fixture.kimi.executable"),
        ),
        probe(),
        services,
    )) {
        Ok(_) => panic!("mismatched readiness endpoint must fail"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.kimi.local_server.preparation.ready_endpoint_mismatch"
    );
    assert!(host.process_stopped_and_joined());
    assert_eq!(host.credential_releases(), 0);
}

fn prepare_attached(
    execution_host: ExecutionHostId,
    services: HostServices,
) -> KimiLocalServerPreparedIntegration {
    block_on(prepare_kimi_local_server_attached(
        attached_input(execution_host),
        probe(),
        services,
    ))
    .expect("attached Kimi prepares")
}

fn attached_input(execution_host: ExecutionHostId) -> KimiLocalServerAttachedInput {
    attached_input_for_version(execution_host, "0.29.0")
}

fn attached_input_for_version(
    execution_host: ExecutionHostId,
    version: &str,
) -> KimiLocalServerAttachedInput {
    let access_profile = access_profile();
    let access_evidence = PreparedAccessEvidence::observed(
        AccessStatus::new(
            access_profile.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        value(
            AccessEvidenceSourceId::new,
            "fixture.kimi.access-observation",
        ),
    );
    KimiLocalServerAttachedInput::new(
        value(ConfiguredInstanceId::new, "fixture.kimi.local-server"),
        value(InstanceRevision::new, "fixture-revision-1"),
        execution_host,
        value(InstanceTargetRef::new, "fixture.kimi.endpoint"),
        access_profile,
        access_evidence,
        value(WorkingResourceRef::new, "fixture.kimi.state-root"),
        kimi_code_binding(version).expect("qualified Kimi version binds"),
    )
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        value(AccessProfileId::new, "kimi-local-server-bearer"),
        CredentialMechanism::ProviderSpecific(value(
            ExtensionNamespace::new,
            "kimi-code/local-server-bearer",
        )),
        EntitlementMetering::LocalCompute,
        value(EndpointAudience::new, "kimi-local-server-loopback"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(value(CredentialRef::new, "fixture.kimi.local-bearer"))
}

fn probe() -> KimiLocalServerPreparationProbe {
    KimiLocalServerPreparationProbe::new(
        value(ScopeId::new, "fixture.kimi.preparation"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn binding(prepared: &KimiLocalServerPreparedIntegration) -> ProviderSessionManagementBinding {
    binding_for(prepared, "session-1")
}

fn binding_for(
    prepared: &KimiLocalServerPreparedIntegration,
    session: &str,
) -> ProviderSessionManagementBinding {
    ProviderSessionManagementBinding::from_bound_session(
        value(SessionRef::new, session),
        &kimi_local_server_descriptor(),
        prepared.instance(),
        prepared.access_evidence().clone(),
        Some(prepared.state_root().clone()),
        ProviderSessionBindingOrigin::ExplicitlyImported,
    )
    .expect("fixture management binding is valid")
}

fn value<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("fixture text is valid")
}
