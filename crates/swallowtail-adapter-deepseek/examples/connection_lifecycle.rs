#![allow(dead_code)]

use swallowtail_adapter_deepseek::{
    DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID, DEEPSEEK_ENDPOINT_AUDIENCE, DeepSeekPreparationInput,
    DeepSeekPreparedIntegration, deepseek_continuation_addable_route_descriptor,
    prepare_deepseek_direct,
};
use swallowtail_core::{
    AccessStatus, AdmittedInstanceRecord, ConfiguredInstanceId, CredentialMechanism, CredentialRef,
    EndpointAudience, EntitlementMetering, ExecutionHostId, InstanceRevision, InstanceTargetRef,
    IntegrationFamilyId, ModelId, OverlayMarker, ProviderId,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceRecord, ConnectionLifecycleStore,
    ConnectionLifecycleStoreFailure, HostServices, InstanceAdmissionFailure,
    InstanceAdmissionRequest, ModelPresentationOverlay, ModelPresentationOverlayFailure,
    PreparationFailure, PreparedAccessEvidence, ReadinessRefreshRequest, ScopeId,
    SignInAuthorityBinding, SignInFailure, SignInMethod, SignInOutcome, admit_instance,
    apply_stored_model_presentation_overlay, complete_sign_in, refresh_readiness, start_sign_in,
    submit_sign_in_credential_field,
};

fn admit_hosted_continuation(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = deepseek_continuation_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            instance_id,
            IntegrationFamilyId::new("deepseek").expect("family id is valid"),
            route_id,
        ),
    )
}

fn collect_api_key(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    admitted: &AdmittedInstanceRecord,
    credential: CredentialRef,
) -> Result<SignInOutcome, SignInFailure> {
    let descriptor = deepseek_continuation_addable_route_descriptor(services);
    let fields: Vec<_> = descriptor.credential_fields().cloned().collect();
    let mut session = start_sign_in(
        services,
        swallowtail_runtime::SignInStartRequest::new(
            ScopeId::new("deepseek.lifecycle.sign-in").expect("scope is valid"),
            admitted.id().clone(),
            admitted.family().clone(),
            admitted.route_id().clone(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                EndpointAudience::new(DEEPSEEK_ENDPOINT_AUDIENCE).expect("audience is valid"),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::ApiKeyCollection { fields },
        ),
    )?;
    submit_sign_in_credential_field(
        &mut session,
        swallowtail_core::CredentialFieldId::new(DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID)
            .expect("field id is valid"),
        credential,
    )?;
    complete_sign_in(session, services, Some(store))
}

fn refresh_hosted_continuation(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    access_status: AccessStatus,
) -> Result<AdmittedInstanceRecord, swallowtail_runtime::ReadinessRefreshFailure> {
    refresh_readiness(
        store,
        ReadinessRefreshRequest::new(instance_id, access_status),
    )
}

fn prepare_after_admission(
    admitted: &AdmittedInstanceRecord,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    profile: swallowtail_core::AccessProfile,
    evidence: PreparedAccessEvidence,
    services: &HostServices,
) -> Result<DeepSeekPreparedIntegration, PreparationFailure> {
    prepare_deepseek_direct(
        DeepSeekPreparationInput::new(
            admitted.id().clone(),
            InstanceRevision::new("1").expect("revision is valid"),
            host,
            target,
            profile,
            evidence,
        ),
        services,
    )
}

fn mark_deepseek_favourite(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    model_id: ModelId,
) -> Result<(), ConnectionLifecycleStoreFailure> {
    store.put_overlay_marker(
        OverlayMarker::new(
            instance_id,
            ProviderId::new("deepseek").expect("provider id is valid"),
            model_id,
        )
        .with_favourite(true),
    )
}

fn project_overlay(
    store: &MemoryConnectionLifecycleStore,
    record: &ConfiguredProviderInstanceRecord,
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    apply_stored_model_presentation_overlay(store, record)
}

fn main() {}
