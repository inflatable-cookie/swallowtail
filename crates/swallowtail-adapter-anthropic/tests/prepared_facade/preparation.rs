use super::fixtures::PreparedFixture;
use swallowtail_adapter_anthropic::{
    AnthropicCatalogueProfileInput, AnthropicPreparationInput, prepare_anthropic_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InstanceTargetRef, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{HostServices, PreparationStage, PreparedAccessEvidence, RequestId};

#[test]
fn access_and_host_mismatch_fail_before_network_or_credential_work() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.rejected").unwrap());
    let wrong_host = HostServices::new(ExecutionHostId::new("other.host").unwrap());
    let failure = prepare_anthropic_direct(input("api.anthropic.com"), &wrong_host)
        .expect_err("host mismatch rejects preparation");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    assert!(fixture.server.requests().is_empty());

    let failure = prepare_anthropic_direct(input("proxy.example"), &services())
        .expect_err("alternate audience rejects preparation");
    assert_eq!(failure.stage(), PreparationStage::AccessEvidence);
    assert!(fixture.server.requests().is_empty());
}

#[test]
fn operation_preflight_rejects_missing_services_and_target_drift() {
    let services = services();
    let prepared = prepare_anthropic_direct(input("api.anthropic.com"), &services)
        .expect("base integration prepares without provider effects");
    let failure = prepared
        .prepare_catalogue(AnthropicCatalogueProfileInput::new(
            RequestId::new("missing-services").unwrap(),
        ))
        .expect_err("catalogue service requirements remain explicit");
    assert_eq!(failure.stage(), PreparationStage::Preflight);

    let failure = prepared
        .validate_execution_binding(
            services.execution_host_id(),
            &InstanceTargetRef::new("another.endpoint").unwrap(),
        )
        .expect_err("endpoint drift is rejected");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
}

fn input(audience: &str) -> AnthropicPreparationInput {
    let profile = AccessProfile::new(
        AccessProfileId::new("anthropic.preparation.access").unwrap(),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(audience).unwrap(),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(CredentialRef::new("anthropic.preparation.key").unwrap());
    let status = AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    AnthropicPreparationInput::new(
        ConfiguredInstanceId::new("anthropic.preparation").unwrap(),
        InstanceRevision::new("1").unwrap(),
        ExecutionHostId::new("anthropic.preparation.host").unwrap(),
        InstanceTargetRef::new("anthropic.preparation.endpoint").unwrap(),
        profile,
        PreparedAccessEvidence::caller_asserted(status),
    )
}

fn services() -> HostServices {
    HostServices::new(ExecutionHostId::new("anthropic.preparation.host").unwrap())
}
