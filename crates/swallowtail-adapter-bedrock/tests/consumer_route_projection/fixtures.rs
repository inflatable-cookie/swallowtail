use aws_credential_types::Credentials;
use std::num::NonZeroU64;
use swallowtail_adapter_bedrock::{
    BedrockCatalogueProfileInput, BedrockCatalogueRouteInput, BedrockCloudClientConfig,
    BedrockCredentialProvider, BedrockFacadePreparationInput, BedrockModelSelection,
    BedrockPreparedCatalogue, BedrockPreparedInferenceAttempt, BedrockRegion,
    BedrockRuntimeProfileInput, BedrockRuntimeRouteInput, bedrock_catalogue_access_profile,
    bedrock_runtime_access_profile, prepare_bedrock,
};
use swallowtail_core::{
    AccessStatus, CredentialState, EndpointAuthorization, EntitlementState, InstanceRevision,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{CredentialRef, OperationContent, PreparedAccessEvidence, RequestId};
use swallowtail_testkit::{ExecutionTopologyFixture, RecordingHostServices, RecordingOutcome};

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

pub fn catalogue() -> BedrockPreparedCatalogue {
    let topology = ExecutionTopologyFixture::local();
    let host = RecordingHostServices::for_host(
        topology.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let region = BedrockRegion::new("us-east-1").expect("region is valid");
    let bedrock = prepare_bedrock(
        BedrockFacadePreparationInput::new(
            topology.execution_host_id().clone(),
            cloud_client(region),
        ),
        host.services(),
    )
    .expect("Bedrock facade prepares");

    let access = bedrock_catalogue_access_profile(credential("bedrock.catalogue.identity"));
    bedrock
        .catalogue(
            BedrockCatalogueRouteInput::new(
                topology.configured_instance_id().clone(),
                InstanceRevision::new("catalogue-1").expect("revision is valid"),
                topology.instance_target().clone(),
                access.clone(),
                evidence(&access),
            ),
            host.services(),
        )
        .expect("catalogue integration prepares")
        .prepare_catalogue(BedrockCatalogueProfileInput::new(
            RequestId::new("cat").expect("request id is valid"),
        ))
        .expect("catalogue operation prepares")
}

pub fn runtime_attempt() -> BedrockPreparedInferenceAttempt {
    let topology = ExecutionTopologyFixture::local();
    let host = RecordingHostServices::for_host(
        topology.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let region = BedrockRegion::new("us-east-1").expect("region is valid");
    let bedrock = prepare_bedrock(
        BedrockFacadePreparationInput::new(
            topology.execution_host_id().clone(),
            cloud_client(region),
        ),
        host.services(),
    )
    .expect("Bedrock facade prepares");

    let access = bedrock_runtime_access_profile(credential("bedrock.runtime.identity"));
    let model = BedrockModelSelection::new(
        ModelRouteId::new("bedrock.anthropic.fixture").expect("route id is valid"),
        ModelRouteRevision::new("1").expect("route revision is valid"),
        ModelId::new("anthropic.claude-fixture-v1:0").expect("model id is valid"),
        ProviderId::new("anthropic").expect("provider id is valid"),
    );

    bedrock
        .runtime(
            BedrockRuntimeRouteInput::new(
                topology.configured_instance_id().clone(),
                InstanceRevision::new("runtime-1").expect("revision is valid"),
                topology.instance_target().clone(),
                access.clone(),
                evidence(&access),
            ),
            host.services(),
        )
        .expect("runtime integration prepares")
        .prepare_inference_attempt(BedrockRuntimeProfileInput::new(
            RequestId::new("attempt").expect("request id is valid"),
            model,
            OperationContent::new("prompt").expect("content is valid"),
            NonZeroU64::new(1024).expect("maximum output tokens is nonzero"),
        ))
        .expect("runtime attempt prepares")
}
