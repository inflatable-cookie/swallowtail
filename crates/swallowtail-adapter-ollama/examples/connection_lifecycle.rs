#![allow(dead_code)]

use swallowtail_adapter_ollama::{
    OLLAMA_ATTACHED_ENDPOINT_FIELD_ID, OllamaModelSelection, OllamaPreparationInput,
    OllamaPreparationProbe, OllamaPreparedIntegration, ollama_attached_addable_route_descriptor,
    ollama_runtime_claim, prepare_ollama_attached,
};
use swallowtail_core::{
    AccessProfile, AccessStatus, AdmittedInstanceRecord, AttachedModelTag, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, ExecutionHostId, InstalledExecutableObservation,
    InstanceRevision, InstanceUpdateObservation, IntegrationFamilyId,
    InvalidInstanceUpdateObservation, ModelManifestDigest,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceRecord, HostServices, InstanceAdmissionFailure,
    InstanceAdmissionRequest, ModelPresentationOverlay, ModelPresentationOverlayFailure,
    PreparationFailure, PreparedAccessEvidence, ReadinessRefreshRequest, admit_instance,
    apply_stored_model_presentation_overlay, observe_instance_update, refresh_readiness,
};

fn admit_attached_runtime(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = ollama_attached_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            instance_id,
            IntegrationFamilyId::new("ollama").expect("family id is valid"),
            route_id,
        )
        .with_config_refs([(
            ConfigFieldId::new(OLLAMA_ATTACHED_ENDPOINT_FIELD_ID).expect("config id is valid"),
            ConfigFieldRef::new("ollama.work.endpoint").expect("config ref is valid"),
        )]),
    )
}

fn refresh_attached_runtime(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    access_status: AccessStatus,
) -> Result<AdmittedInstanceRecord, swallowtail_runtime::ReadinessRefreshFailure> {
    refresh_readiness(
        store,
        ReadinessRefreshRequest::new(instance_id, access_status),
    )
}

#[allow(clippy::too_many_arguments)]
async fn prepare_after_admission(
    admitted: &AdmittedInstanceRecord,
    host: ExecutionHostId,
    profile: AccessProfile,
    evidence: PreparedAccessEvidence,
    model: OllamaModelSelection,
    selected_model_tag: AttachedModelTag,
    selected_manifest_digest: ModelManifestDigest,
    probe: OllamaPreparationProbe,
    services: HostServices,
) -> Result<OllamaPreparedIntegration, PreparationFailure> {
    let input = OllamaPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        profile,
        evidence,
        model,
        selected_model_tag,
        selected_manifest_digest,
    )?;
    prepare_ollama_attached(input, probe, services).await
}

fn observe_runtime_update(
    installed: Option<&InstalledExecutableObservation>,
) -> Result<InstanceUpdateObservation, InvalidInstanceUpdateObservation> {
    observe_instance_update(&ollama_runtime_claim(), installed.cloned())
}

fn project_unmarked_overlay(
    store: &MemoryConnectionLifecycleStore,
    record: &ConfiguredProviderInstanceRecord,
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    apply_stored_model_presentation_overlay(store, record)
}

fn main() {}
