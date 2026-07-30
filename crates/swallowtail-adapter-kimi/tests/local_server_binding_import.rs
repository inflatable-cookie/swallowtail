#[path = "local_server_binding_import/authority.rs"]
mod authority;
#[path = "local_server_binding_import/control.rs"]
mod control;
#[path = "local_server_binding_import/identity.rs"]
mod identity;

use crate::lifecycle_support as local_support;
use crate::{discovery_support, fixtures, support};

use discovery_support::FakeProcessService;
use futures_executor::block_on;
use local_support::{FixtureHost as LocalHost, FixtureServer};
use swallowtail_adapter_kimi::{
    KimiAcpSessionImportAuthority, KimiLocalServerAttachedInput, KimiLocalServerBindingImportInput,
    KimiLocalServerOwnedInput, KimiLocalServerPreparationProbe, KimiLocalServerPreparedIntegration,
    KimiPreparationInput, KimiSessionProfileInput, kimi_code_binding, prepare_kimi,
    prepare_kimi_local_server_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, InstanceRevision, InstanceTargetRef,
    SessionRef, SupportAuthority,
};
use swallowtail_runtime::{
    AccessEvidenceSourceId, Deadline, DiscoveryCancellation, HostServices, MonotonicInstant,
    PreparedAccessEvidence, RequestId, ScopeId, SessionResumeBinding, WorkingResourceRef,
};

fn source_authority(
    host_id: ExecutionHostId,
    version: &str,
    state_root: Option<&str>,
    provider_session_id: &str,
) -> Result<KimiAcpSessionImportAuthority, swallowtail_runtime::PreparationFailure> {
    let operation_host = support::FixtureHost::new(support::Scenario::Complete);
    let mut input = KimiPreparationInput::new(
        value(ConfiguredInstanceId::new, "fixture.kimi.acp"),
        value(InstanceRevision::new, "fixture-acp-revision"),
        host_id.clone(),
        fixtures::target(),
        value(
            swallowtail_runtime::EnvironmentRef::new,
            "fixture.kimi.environment",
        ),
        fixtures::access_profile(),
        PreparedAccessEvidence::caller_asserted(fixtures::access_status()),
    );
    if let Some(state_root) = state_root {
        input = input.with_state_root(value(WorkingResourceRef::new, state_root));
    }
    let (process, _) = FakeProcessService::completed(&format!("{version}\n"));
    let prepared = block_on(prepare_kimi(
        input,
        fixtures::probe(),
        fixtures::preparation_services(&operation_host, host_id, process),
    ))
    .expect("ACP source prepares");
    let profile = prepared
        .prepare_session(profile_input())
        .expect("ACP session prepares");
    let binding = SessionResumeBinding::new(
        value(SessionRef::new, provider_session_id),
        profile.plan().instance_id().clone(),
        profile.plan().execution_host_id().clone(),
        profile
            .plan()
            .model_route_id()
            .expect("model route")
            .clone(),
        profile.plan().model_id().expect("model").clone(),
        profile
            .request()
            .working_resource()
            .expect("working resource")
            .clone(),
        profile.request().access_policy().clone(),
    );
    profile.authorize_local_server_import(binding)
}

fn profile_input() -> KimiSessionProfileInput {
    fixtures::profile_input(
        "fixture-import-source",
        swallowtail_runtime::SessionOptions::default(),
    )
}

fn prepare_local(
    server: &FixtureServer,
    host: &LocalHost,
    host_id: ExecutionHostId,
    state_root: &str,
) -> (KimiLocalServerPreparedIntegration, HostServices) {
    let services = host.services(host_id.clone(), false);
    let prepared = block_on(prepare_kimi_local_server_attached(
        local_input(
            server,
            host_id,
            state_root,
            "fixture.kimi.local",
            "fixture.endpoint",
            "fixture.bearer",
        ),
        probe("fixture-local-prepare"),
        services.clone(),
    ))
    .expect("local server prepares");
    (prepared, services)
}

fn local_input(
    _server: &FixtureServer,
    host_id: ExecutionHostId,
    state_root: &str,
    instance_id: &str,
    endpoint_id: &str,
    credential_id: &str,
) -> KimiLocalServerAttachedInput {
    local_input_with_version(
        _server,
        host_id,
        state_root,
        instance_id,
        endpoint_id,
        credential_id,
        "0.29.0",
    )
}

#[allow(clippy::too_many_arguments)]
fn local_input_with_version(
    _server: &FixtureServer,
    host_id: ExecutionHostId,
    state_root: &str,
    instance_id: &str,
    endpoint_id: &str,
    credential_id: &str,
    version: &str,
) -> KimiLocalServerAttachedInput {
    let profile = local_access_profile(credential_id);
    KimiLocalServerAttachedInput::new(
        value(ConfiguredInstanceId::new, instance_id),
        value(InstanceRevision::new, "fixture-local-revision"),
        host_id,
        value(InstanceTargetRef::new, endpoint_id),
        profile.clone(),
        PreparedAccessEvidence::observed(
            AccessStatus::new(
                profile.id().clone(),
                CredentialState::Ready,
                EntitlementState::Available,
                EndpointAuthorization::Allowed,
                swallowtail_core::RuntimeReadiness::Ready,
                SupportAuthority::IntegrationMaintainerSupported,
            ),
            value(
                AccessEvidenceSourceId::new,
                "fixture.local.access-observation",
            ),
        ),
        value(WorkingResourceRef::new, state_root),
        kimi_code_binding(version).expect("version binds"),
    )
}

fn local_access_profile(credential_id: &str) -> AccessProfile {
    AccessProfile::new(
        value(AccessProfileId::new, "fixture.kimi.local-access"),
        CredentialMechanism::ProviderSpecific(value(
            ExtensionNamespace::new,
            "kimi-code/local-server-bearer",
        )),
        EntitlementMetering::LocalCompute,
        value(EndpointAudience::new, "kimi-local-server-loopback"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(value(CredentialRef::new, credential_id))
}

fn import_input(
    prepared: &KimiLocalServerPreparedIntegration,
    source: KimiAcpSessionImportAuthority,
    id: &str,
) -> KimiLocalServerBindingImportInput {
    KimiLocalServerBindingImportInput::new(
        value(RequestId::new, id),
        value(ScopeId::new, id),
        source,
        prepared.binding_import_target(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn probe(id: &str) -> KimiLocalServerPreparationProbe {
    KimiLocalServerPreparationProbe::new(
        value(ScopeId::new, id),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn owned_input(server: &FixtureServer, host_id: ExecutionHostId) -> KimiLocalServerOwnedInput {
    KimiLocalServerOwnedInput::new(
        local_input(
            server,
            host_id,
            "fixture.kimi.state-root",
            "fixture.kimi.owned",
            "fixture.endpoint",
            "fixture.bearer",
        ),
        value(InstanceTargetRef::new, "fixture.kimi.executable"),
    )
}

fn value<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("fixture value is valid")
}
