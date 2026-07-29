use crate::support::Fixture;
use futures_executor::block_on;
use std::num::NonZeroU64;
use std::time::Duration;
use swallowtail_adapter_ollama::{
    OllamaInferenceAttemptInput, OllamaInventoryProfileInput, OllamaModelSelection,
    OllamaPreparationInput, OllamaPreparationProbe, OllamaPreparedIntegration,
    OllamaSessionProfileInput, prepare_ollama_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AttachedModelTag, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, EndpointAuthorization, EntitlementMetering,
    EntitlementState, InstanceRevision, ModelId, ModelManifestDigest, ModelRouteId,
    ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    DiscoveryCancellation, OperationContent, PreparedAccessEvidence, RequestId, ScopeId,
};

const MODEL: &str = "fixture-model:8b";
const DIGEST: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

pub fn prepared(fixture: &Fixture) -> OllamaPreparedIntegration {
    block_on(prepare_ollama_attached(
        preparation_input(fixture),
        probe(fixture, DiscoveryCancellation::new()),
        fixture.services(),
    ))
    .expect("attached Ollama prepares")
}

pub fn preparation_input(fixture: &Fixture) -> OllamaPreparationInput {
    let access = access_profile(fixture);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    OllamaPreparationInput::new(
        ConfiguredInstanceId::new("ollama.prepared").unwrap(),
        InstanceRevision::new("2").unwrap(),
        fixture.host_id().clone(),
        fixture.target().clone(),
        access,
        PreparedAccessEvidence::caller_asserted(status),
        model_selection(),
        AttachedModelTag::new(MODEL).unwrap(),
        ModelManifestDigest::new(DIGEST).unwrap(),
    )
}

pub fn access_profile(fixture: &Fixture) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("ollama.prepared.access").unwrap(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::LocalCompute,
        fixture.audience().clone(),
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

pub fn probe(fixture: &Fixture, cancellation: DiscoveryCancellation) -> OllamaPreparationProbe {
    OllamaPreparationProbe::new(
        ScopeId::new("ollama-prepared-probe").unwrap(),
        fixture.thread.deadline_after(Duration::from_secs(1)),
        cancellation,
    )
}

pub fn model_selection() -> OllamaModelSelection {
    OllamaModelSelection::new(
        ModelRouteId::new("ollama.prepared.route").unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new("ollama.fixture.model").unwrap(),
    )
}

pub fn inventory_input(id: &str) -> OllamaInventoryProfileInput {
    OllamaInventoryProfileInput::new(RequestId::new(id).unwrap())
}

pub fn attempt_input(id: &str) -> OllamaInferenceAttemptInput {
    OllamaInferenceAttemptInput::new(
        RequestId::new(id).unwrap(),
        OperationContent::new("prepared Ollama prompt").unwrap(),
        NonZeroU64::new(8).unwrap(),
    )
}

pub fn session_input(id: &str) -> OllamaSessionProfileInput {
    OllamaSessionProfileInput::new(RequestId::new(id).unwrap())
}
