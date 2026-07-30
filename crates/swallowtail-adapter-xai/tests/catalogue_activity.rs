use swallowtail_adapter_xai::{
    XAI_MODELS_ENDPOINT, XaiModelsPreparationInput, XaiModelsProfileInput, prepare_xai_models,
    xai_responses_access_profile,
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
    let access = xai_responses_access_profile(
        CredentialRef::new("xai.models.fixture").expect("credential reference is valid"),
    );
    let prepared = prepare_xai_models(
        XaiModelsPreparationInput::new(
            InstanceRevision::new("fixture-1").expect("revision is valid"),
            host.services().execution_host_id().clone(),
            InstanceTargetRef::new(XAI_MODELS_ENDPOINT).expect("target is valid"),
            access.clone(),
            evidence(&access),
        ),
        host.services(),
    )
    .expect("xAI Models integration prepares");
    let catalogue = prepared
        .prepare_catalogue(XaiModelsProfileInput::new(
            RequestId::new("xai-models-activity").expect("request id is valid"),
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
