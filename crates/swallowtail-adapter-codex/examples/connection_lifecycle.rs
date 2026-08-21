#![allow(dead_code)]

use swallowtail_adapter_codex::{
    CODEX_APP_SERVER_BINARY_PATH_FIELD_ID, CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID, CODEX_CLI_AXIS,
    CodexPreparationInput, CodexPreparationProbe, CodexPreparedIntegration,
    codex_app_server_addable_route_descriptor, codex_app_server_claim,
    codex_chatgpt_subscription_access_profile, prepare_codex,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef,
    ConfiguredInstanceId, ExecutionHostId, InstalledExecutableObservation, InstanceRevision,
    InstanceUpdateObservation, IntegrationFamilyId, InterfaceVersionAxis,
    InvalidInstanceUpdateObservation, ModelId, OverlayMarker,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceRecord, ConnectionLifecycleStore,
    ConnectionLifecycleStoreFailure, Deadline, DiscoveryCancellation, ExecutableRef, HostServices,
    InstalledExecutableTarget, InstanceAdmissionFailure, InstanceAdmissionRequest,
    ModelPresentationOverlay, ModelPresentationOverlayFailure, PreparationFailure,
    PreparedAccessEvidence, ReadinessRefreshRequest, RequestId, ScopeId, admit_instance,
    apply_stored_model_presentation_overlay, observe_instance_update, refresh_readiness,
};

fn admit_app_server(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = codex_app_server_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            instance_id,
            IntegrationFamilyId::new("codex").expect("family id is valid"),
            route_id,
        )
        .with_config_refs([
            (
                ConfigFieldId::new(CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex-app-server").expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex.work.login").expect("config ref is valid"),
            ),
        ]),
    )
}

fn refresh_app_server(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    access_status: AccessStatus,
) -> Result<AdmittedInstanceRecord, swallowtail_runtime::ReadinessRefreshFailure> {
    refresh_readiness(
        store,
        ReadinessRefreshRequest::new(instance_id, access_status),
    )
}

async fn prepare_after_admission(
    admitted: &AdmittedInstanceRecord,
    host: ExecutionHostId,
    access_status: AccessStatus,
    deadline: Deadline,
    services: HostServices,
) -> Result<CodexPreparedIntegration, PreparationFailure> {
    let profile = codex_chatgpt_subscription_access_profile(access_status.profile_id().clone());
    let input = CodexPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        profile,
        PreparedAccessEvidence::caller_asserted(access_status),
    )?;
    let probe = CodexPreparationProbe::new(
        RequestId::new("codex-lifecycle-prepare").expect("request id is valid"),
        ScopeId::new("codex-lifecycle-prepare").expect("scope is valid"),
        deadline,
        DiscoveryCancellation::new(),
    );
    prepare_codex(input, probe, services).await
}

fn observe_app_server_update(
    installed: Option<&InstalledExecutableObservation>,
) -> Result<InstanceUpdateObservation, InvalidInstanceUpdateObservation> {
    let claim = codex_app_server_claim();
    observe_instance_update(&claim, installed.cloned())
}

fn store_instance_model_overlay(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    model_id: ModelId,
) -> Result<(), ConnectionLifecycleStoreFailure> {
    store.put_overlay_marker(
        OverlayMarker::without_provider(instance_id, model_id).with_favourite(true),
    )
}

fn project_overlay(
    store: &MemoryConnectionLifecycleStore,
    record: &ConfiguredProviderInstanceRecord,
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    apply_stored_model_presentation_overlay(store, record)
}

fn app_server_target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new("codex-app-server").expect("executable ref is valid"),
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).expect("version axis is valid"),
    )
}

fn chatgpt_profile_id() -> AccessProfileId {
    AccessProfileId::new("codex.work.chatgpt").expect("access id is valid")
}

fn main() {}
