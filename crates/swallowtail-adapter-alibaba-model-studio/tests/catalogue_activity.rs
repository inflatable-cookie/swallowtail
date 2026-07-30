use swallowtail_adapter_alibaba_model_studio::{
    ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID, ALIBABA_DEPLOYABLE_MODELS_ENDPOINT,
    ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE, AlibabaDeployableModelsPreparationInput,
    AlibabaDeployableModelsProfileInput, prepare_alibaba_deployable_models,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, CredentialMechanism, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    InstanceRevision, InstanceTargetRef, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{CredentialRef, PreparedAccessEvidence, RequestId};
use swallowtail_testkit::{
    RecordingHostServices, assert_observable_activity_not_applicable,
    assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn prepared_deployable_models_catalogue_is_not_ordinary_agent_activity() {
    let host = RecordingHostServices::default();
    let access = access_profile();
    let prepared = prepare_alibaba_deployable_models(
        AlibabaDeployableModelsPreparationInput::new(
            InstanceRevision::new("fixture-1").expect("revision is valid"),
            host.services().execution_host_id().clone(),
            InstanceTargetRef::new(ALIBABA_DEPLOYABLE_MODELS_ENDPOINT).expect("target is valid"),
            access.clone(),
            evidence(&access),
        ),
        host.services(),
    )
    .expect("Alibaba deployable-models integration prepares");
    let catalogue = prepared
        .prepare_catalogue(AlibabaDeployableModelsProfileInput::new(
            RequestId::new("alibaba-models-activity").expect("request id is valid"),
        ))
        .expect("catalogue prepares");

    assert_prepared_operation_evidence_matches_plan(catalogue.evidence(), catalogue.plan());
    assert_observable_activity_not_applicable(catalogue.evidence());
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new(ALIBABA_DEPLOYABLE_MODELS_ACCESS_PROFILE_ID)
            .expect("access profile id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE)
            .expect("endpoint audience is valid"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(
        CredentialRef::new("alibaba.deployable-models.fixture")
            .expect("credential reference is valid"),
    )
}

fn evidence(access: &swallowtail_core::AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}
