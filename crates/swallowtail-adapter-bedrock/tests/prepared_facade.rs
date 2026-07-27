use aws_credential_types::Credentials;
use std::num::NonZeroU64;
use swallowtail_adapter_bedrock::{
    BedrockCataloguePreparationInput, BedrockCatalogueProfileInput, BedrockCatalogueRouteInput,
    BedrockCloudClientConfig, BedrockCredentialProvider, BedrockFacadePreparationInput,
    BedrockModelSelection, BedrockRegion, BedrockRuntimePreparationInput,
    BedrockRuntimeProfileInput, BedrockRuntimeRouteInput, CATALOGUE_SDK_CRATE,
    CATALOGUE_SDK_VERSION, CATALOGUE_SERVICE_API, SDK_CRATE, SDK_VERSION, SERVICE_API,
    bedrock_catalogue_access_profile, bedrock_catalogue_descriptor, bedrock_direct_descriptor,
    bedrock_runtime_access_profile, prepare_bedrock, prepare_bedrock_catalogue,
    prepare_bedrock_runtime,
};
use swallowtail_core::{
    AccessStatus, CredentialState, DriverRole, EndpointAuthorization, EntitlementState,
    InstanceRevision, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{CredentialRef, OperationContent, PreparedAccessEvidence, RequestId};
use swallowtail_testkit::{
    ExecutionTopologyFixture, RecordedHostCall, RecordingHostServices, RecordingOutcome,
    assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn composite_facade_prepares_separate_sdk_operations_on_both_hosts() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let host = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let region = BedrockRegion::new("eu-west-2").expect("region is valid");
        let bedrock = prepare_bedrock(
            BedrockFacadePreparationInput::new(
                topology.execution_host_id().clone(),
                cloud_client(region.clone()),
            ),
            host.services(),
        )
        .expect("Bedrock facade prepares");
        assert_eq!(bedrock.execution_host_id(), topology.execution_host_id());
        assert_eq!(bedrock.region(), &region);

        let runtime_access = bedrock_runtime_access_profile(credential("bedrock.runtime.identity"));
        let attempt = bedrock
            .runtime(
                BedrockRuntimeRouteInput::new(
                    topology.configured_instance_id().clone(),
                    InstanceRevision::new("runtime-1").expect("revision is valid"),
                    topology.instance_target().clone(),
                    runtime_access.clone(),
                    evidence(&runtime_access),
                ),
                host.services(),
            )
            .expect("runtime integration prepares")
            .prepare_inference_attempt(BedrockRuntimeProfileInput::new(
                RequestId::new("bedrock-runtime-prepared").expect("request id is valid"),
                BedrockModelSelection::new(
                    ModelRouteId::new("bedrock.anthropic.fixture").expect("route id is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ModelId::new("anthropic.claude-fixture-v1:0").expect("model id is valid"),
                    ProviderId::new("anthropic").expect("provider id is valid"),
                ),
                OperationContent::new("prepared fixture prompt").expect("content is valid"),
                NonZeroU64::new(64).expect("output bound is nonzero"),
            ))
            .expect("runtime attempt prepares");

        assert_eq!(
            attempt.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_eq!(attempt.evidence().region(), &region);
        assert_eq!(attempt.evidence().sdk_crate(), SDK_CRATE);
        assert_eq!(attempt.evidence().sdk_version(), SDK_VERSION);
        assert_eq!(attempt.evidence().service_api(), SERVICE_API);
        assert_eq!(
            attempt
                .evidence()
                .operation()
                .interface_compatibility()
                .len(),
            2
        );
        assert_prepared_operation_evidence_matches_plan(
            attempt.evidence().operation(),
            attempt.plan(),
        );

        let catalogue_access =
            bedrock_catalogue_access_profile(credential("bedrock.catalogue.identity"));
        let catalogue = bedrock
            .catalogue(
                BedrockCatalogueRouteInput::new(
                    topology.configured_instance_id().clone(),
                    InstanceRevision::new("catalogue-1").expect("revision is valid"),
                    topology.instance_target().clone(),
                    catalogue_access.clone(),
                    evidence(&catalogue_access),
                ),
                host.services(),
            )
            .expect("catalogue integration prepares")
            .prepare_catalogue(BedrockCatalogueProfileInput::new(
                RequestId::new("bedrock-catalogue-prepared").expect("request id is valid"),
            ))
            .expect("catalogue operation prepares");

        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert!(catalogue.plan().model_route_id().is_none());
        assert!(catalogue.plan().model_id().is_none());
        assert_eq!(catalogue.evidence().region(), &region);
        assert_eq!(catalogue.evidence().sdk_crate(), CATALOGUE_SDK_CRATE);
        assert_eq!(catalogue.evidence().sdk_version(), CATALOGUE_SDK_VERSION);
        assert_eq!(catalogue.evidence().service_api(), CATALOGUE_SERVICE_API);
        assert_eq!(
            catalogue
                .evidence()
                .operation()
                .interface_compatibility()
                .len(),
            2
        );
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );

        assert_ne!(
            attempt.plan().driver_identity(),
            catalogue.plan().driver_identity()
        );
        assert_eq!(host.count(RecordedHostCall::NetworkAuthorize), 0);
        assert_eq!(host.count(RecordedHostCall::CredentialAcquire), 0);
    }
}

#[test]
fn composite_facade_rejects_a_different_execution_host() {
    let topology = ExecutionTopologyFixture::local();
    let other = ExecutionTopologyFixture::remote_authoritative();
    let host = RecordingHostServices::for_host(
        other.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let error = prepare_bedrock(
        BedrockFacadePreparationInput::new(
            topology.execution_host_id().clone(),
            cloud_client(BedrockRegion::new("eu-west-2").expect("region is valid")),
        ),
        host.services(),
    )
    .expect_err("facade host drift must fail");

    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.bedrock.facade.preparation.host_mismatch"
    );
    assert_eq!(host.count(RecordedHostCall::NetworkAuthorize), 0);
    assert_eq!(host.count(RecordedHostCall::CredentialAcquire), 0);
}

#[test]
fn access_profiles_and_descriptors_cannot_cross_runtime_and_catalogue() {
    let topology = ExecutionTopologyFixture::local();
    let host = RecordingHostServices::for_host(
        topology.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let runtime_access = bedrock_runtime_access_profile(credential("bedrock.runtime.identity"));
    let catalogue_access =
        bedrock_catalogue_access_profile(credential("bedrock.catalogue.identity"));

    let runtime_with_catalogue_access = BedrockRuntimePreparationInput::new(
        topology.configured_instance_id().clone(),
        InstanceRevision::new("runtime-1").expect("revision is valid"),
        topology.execution_host_id().clone(),
        topology.instance_target().clone(),
        catalogue_access.clone(),
        evidence(&catalogue_access),
        cloud_client(BedrockRegion::new("eu-west-2").expect("region is valid")),
    );
    assert!(prepare_bedrock_runtime(runtime_with_catalogue_access, host.services()).is_err());

    let catalogue_with_runtime_access = BedrockCataloguePreparationInput::new(
        topology.configured_instance_id().clone(),
        InstanceRevision::new("catalogue-1").expect("revision is valid"),
        topology.execution_host_id().clone(),
        topology.instance_target().clone(),
        runtime_access.clone(),
        evidence(&runtime_access),
        cloud_client(BedrockRegion::new("eu-west-2").expect("region is valid")),
    );
    assert!(prepare_bedrock_catalogue(catalogue_with_runtime_access, host.services()).is_err());

    assert_ne!(
        bedrock_direct_descriptor().identity(),
        bedrock_catalogue_descriptor().identity()
    );
    assert_eq!(host.count(RecordedHostCall::NetworkAuthorize), 0);
    assert_eq!(host.count(RecordedHostCall::CredentialAcquire), 0);
}

fn credential(value: &str) -> CredentialRef {
    CredentialRef::new(value).expect("credential reference is valid")
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

fn provider() -> BedrockCredentialProvider {
    BedrockCredentialProvider::new(Credentials::new(
        "fixture-access-key",
        "fixture-secret-key",
        None,
        None,
        "fixture",
    ))
}

fn cloud_client(region: BedrockRegion) -> BedrockCloudClientConfig {
    BedrockCloudClientConfig::new(region, provider())
}
