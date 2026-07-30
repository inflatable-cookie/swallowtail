use swallowtail_adapter_openai::{
    OPENAI_MODELS_ENDPOINT, OpenAiModelsPreparationInput, OpenAiModelsProfileInput,
    openai_background_access_profile, prepare_openai_models,
};
use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAuthorization, EntitlementState, InstanceRevision,
    InstanceTargetRef, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{CredentialRef, PreparedAccessEvidence, RequestId};
use swallowtail_testkit::{
    RecordingHostServices, assert_observable_activity_not_applicable,
    assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn prepared_models_catalogue_is_not_ordinary_agent_activity() {
    let host = RecordingHostServices::default();
    let access = openai_background_access_profile(
        CredentialRef::new("openai.models.fixture").expect("credential reference is valid"),
    );
    let prepared = prepare_openai_models(
        OpenAiModelsPreparationInput::new(
            InstanceRevision::new("fixture-1").expect("revision is valid"),
            host.services().execution_host_id().clone(),
            InstanceTargetRef::new(OPENAI_MODELS_ENDPOINT).expect("target is valid"),
            access.clone(),
            evidence(&access),
        ),
        host.services(),
    )
    .expect("OpenAI Models integration prepares");
    let catalogue = prepared
        .prepare_catalogue(OpenAiModelsProfileInput::new(
            RequestId::new("openai-models-activity").expect("request id is valid"),
        ))
        .expect("catalogue prepares");

    assert_prepared_operation_evidence_matches_plan(catalogue.evidence(), catalogue.plan());
    assert_observable_activity_not_applicable(catalogue.evidence());
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
