use futures_executor::block_on;
use swallowtail_adapter_kimi::{
    KimiLocalServerAttachedInput, KimiLocalServerPermissionMode, KimiLocalServerPreparationProbe,
    KimiLocalServerPreparedIntegration, KimiLocalServerSessionConfiguration,
    KimiLocalServerSessionInput, KimiModelSelection, kimi_code_binding,
    prepare_kimi_local_server_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, Deadline, DiscoveryCancellation, HostServices, MonotonicInstant,
    OperationContent, PreparedAccessEvidence, RequestId, RuntimeTurnId, ScopeId, TurnRequest,
    WorkingResourceRef,
};

pub(super) fn prepare(
    execution_host: ExecutionHostId,
    services: HostServices,
    version: &str,
) -> KimiLocalServerPreparedIntegration {
    block_on(prepare_kimi_local_server_attached(
        attached_input(execution_host, version),
        probe(),
        services,
    ))
    .expect("local server prepares")
}

pub(super) fn session_profile(
    prepared: &KimiLocalServerPreparedIntegration,
    permission: KimiLocalServerPermissionMode,
    request: &str,
) -> swallowtail_adapter_kimi::KimiLocalServerPreparedSession {
    prepared
        .prepare_session(session_input(
            request,
            KimiLocalServerSessionConfiguration::new(permission),
        ))
        .expect("session prepares")
}

pub(super) fn session_input(
    request: &str,
    configuration: KimiLocalServerSessionConfiguration,
) -> KimiLocalServerSessionInput {
    KimiLocalServerSessionInput::new(
        id(RequestId::new, request),
        KimiModelSelection::new(
            id(ModelRouteId::new, "fixture.kimi.route"),
            id(ModelRouteRevision::new, "1"),
            id(ModelId::new, "kimi-k2.5"),
        ),
        id(WorkingResourceRef::new, "fixture.kimi.workspace"),
        configuration,
    )
}

pub(super) fn turn(id_value: &str) -> TurnRequest {
    TurnRequest::new(
        id(RuntimeTurnId::new, id_value),
        OperationContent::new("fixture prompt").expect("content"),
    )
}

pub(super) fn probe() -> KimiLocalServerPreparationProbe {
    KimiLocalServerPreparationProbe::new(
        id(ScopeId::new, "fixture.kimi.interactive.preparation"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

pub(super) fn id<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T {
    constructor(value.to_owned())
        .ok()
        .expect("fixture id is valid")
}

pub(super) fn attached_input(
    execution_host: ExecutionHostId,
    version: &str,
) -> KimiLocalServerAttachedInput {
    let access = access_profile();
    KimiLocalServerAttachedInput::new(
        id(ConfiguredInstanceId::new, "fixture.kimi.local.interactive"),
        id(InstanceRevision::new, "1"),
        execution_host,
        id(InstanceTargetRef::new, "fixture.kimi.local.endpoint"),
        access.clone(),
        PreparedAccessEvidence::observed(
            AccessStatus::new(
                access.id().clone(),
                CredentialState::Ready,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                RuntimeReadiness::Ready,
                SupportAuthority::IntegrationMaintainerSupported,
            ),
            id(
                AccessEvidenceSourceId::new,
                "fixture.kimi.interactive.access",
            ),
        ),
        id(WorkingResourceRef::new, "fixture.kimi.state-root"),
        kimi_code_binding(version).expect("Kimi version binds"),
    )
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        id(AccessProfileId::new, "kimi-local-server-bearer"),
        CredentialMechanism::ProviderSpecific(id(
            ExtensionNamespace::new,
            "kimi-code/local-server-bearer",
        )),
        EntitlementMetering::LocalCompute,
        id(EndpointAudience::new, "kimi-local-server-loopback"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(id(CredentialRef::new, "fixture.kimi.local-bearer"))
}
