use crate::discovery_support::{self, FakeProcessService};
use crate::support::{CleanupEvent, FixtureHost};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KimiModelSelection, KimiPreparationInput, KimiPreparationProbe,
    KimiPreparedIntegration, KimiSessionProfileInput, prepare_kimi,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, HostServices,
    InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, RequestId, ScopeId,
    SessionOptions, WorkingResourceRef,
};

pub fn prepared(
    operation_host: &FixtureHost,
    host_id: ExecutionHostId,
    version: &str,
) -> KimiPreparedIntegration {
    prepared_with_state_root(
        operation_host,
        host_id,
        version,
        Some("fixture.kimi.state-root"),
    )
}

pub fn prepared_with_state_root(
    operation_host: &FixtureHost,
    host_id: ExecutionHostId,
    version: &str,
    state_root: Option<&str>,
) -> KimiPreparedIntegration {
    let mut input = KimiPreparationInput::new(
        ConfiguredInstanceId::new("kimi.prepared").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host_id.clone(),
        target(),
        EnvironmentRef::new("kimi.prepared.state").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(access_status()),
    );
    if let Some(state_root) = state_root {
        input = input.with_state_root(WorkingResourceRef::new(state_root).unwrap());
    }
    let (process, _) = FakeProcessService::completed(&format!("{version}\n"));
    block_on(prepare_kimi(
        input,
        probe(),
        preparation_services(operation_host, host_id, process),
    ))
    .expect("installed Kimi prepares")
}

pub fn preparation_services(
    operation_host: &FixtureHost,
    host_id: ExecutionHostId,
    process: Arc<dyn swallowtail_runtime::ProcessService>,
) -> HostServices {
    let operation = operation_host.services(host_id.clone());
    discovery_support::services(host_id, process)
        .with_credential(operation.credential().unwrap().clone())
        .with_working_resource(operation.working_resource().unwrap().clone())
        .with_working_resource_io(operation.working_resource_io().unwrap().clone())
}

pub fn profile_input(id: &str, options: SessionOptions) -> KimiSessionProfileInput {
    KimiSessionProfileInput::new(
        RequestId::new(id).unwrap(),
        KimiModelSelection::new(
            ModelRouteId::new("kimi.prepared.route").unwrap(),
            ModelRouteRevision::new("1").unwrap(),
            ModelId::new("kimi-coder").unwrap(),
        ),
        swallowtail_runtime::WorkingResourceRef::new("kimi.prepared.workspace").unwrap(),
        options,
    )
}

pub fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new("kimi.prepared.executable").unwrap(),
        InterfaceVersionAxis::new(KIMI_CODE_AXIS).unwrap(),
    )
}

pub fn probe() -> KimiPreparationProbe {
    KimiPreparationProbe::new(
        RequestId::new("kimi-prepared-probe").unwrap(),
        ScopeId::new("kimi-prepared-probe").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

pub fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("kimi.prepared.access").unwrap(),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("kimi-code-membership").unwrap(),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(CredentialRef::new("kimi.prepared.credential").unwrap())
}

pub fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("kimi.prepared.access").unwrap(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

pub fn joined_cleanup() -> [CleanupEvent; 3] {
    [
        CleanupEvent::ProcessWait,
        CleanupEvent::ResourceRelease,
        CleanupEvent::CredentialRelease,
    ]
}
