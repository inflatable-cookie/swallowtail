use crate::prepared_support::{
    Fixture, FixtureServer, OwnedFixture, ProcessStop, PropertiesFixture, ScriptedOwnedServices,
    StreamFixture,
};
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZeroU64;
use swallowtail_adapter_llama_cpp::{
    LlamaCppAttachedPreparationInput, LlamaCppCatalogueProfileInput, LlamaCppContextSize,
    LlamaCppInferenceProfileInput, LlamaCppModelSelection, LlamaCppOwnedPreparationInput,
    LlamaCppOwnedServingSelection, LlamaCppPreparedCatalogue, LlamaCppPreparedInferenceAttempt,
    LlamaCppPreparedServingStart, LlamaCppReasoningSelection, llama_cpp_attached_access_profile,
    llama_cpp_owned_access_profile, prepare_llama_cpp_attached, prepare_llama_cpp_owned,
};
use swallowtail_core::{
    AccessProfile, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, Deadline, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, RequestId, ScopeId, ServingInstanceId,
};

use super::ledger::{CATALOGUE, INFERENCE, SERVING};
use super::naming::{RowIdentity, identities, source};

const STARTUP_SUCCESS: &str =
    include_str!("../fixtures/llama-cpp-b10069-owned/startup-success.stderr");

pub(super) fn catalogue() -> LlamaCppPreparedCatalogue {
    attached("1")
        .prepare_catalogue(LlamaCppCatalogueProfileInput::new(
            RequestId::new("llama-cpp.projection.catalogue").expect("request"),
        ))
        .expect("catalogue prepares")
}

pub(super) fn inference() -> LlamaCppPreparedInferenceAttempt {
    attached("1")
        .prepare_inference_attempt(inference_input("projection"))
        .expect("inference prepares")
}

pub(super) fn alternate_inference() -> LlamaCppPreparedInferenceAttempt {
    attached("2")
        .prepare_inference_attempt(inference_input("projection-alternate"))
        .expect("alternate inference prepares")
}

pub(super) fn serving() -> LlamaCppPreparedServingStart {
    serving_with(
        Some(LlamaCppContextSize::from_u64(4096).expect("admitted context size")),
        Some(LlamaCppReasoningSelection::Disabled),
    )
}

pub(super) fn serving_omitted() -> LlamaCppPreparedServingStart {
    serving_with(None, None)
}

pub(super) fn catalogue_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    catalogue()
        .consumer_route_projection_contribution(source(id))
        .expect("catalogue contributes")
}

pub(super) fn inference_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    inference()
        .consumer_route_projection_contribution(source(id))
        .expect("inference contributes")
}

pub(super) fn serving_contribution(id: &str) -> ConsumerRouteProjectionContribution {
    serving()
        .consumer_route_projection_contribution(source(id))
        .expect("serving contributes")
}

pub(super) fn observed_attached() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([
        (
            CATALOGUE,
            identities(
                &catalogue_contribution("llama-cpp.attached.catalogue"),
                super::ledger::ATTACHED_ROUTE,
            ),
        ),
        (
            INFERENCE,
            identities(
                &inference_contribution("llama-cpp.attached.inference"),
                super::ledger::ATTACHED_ROUTE,
            ),
        ),
    ])
}

pub(super) fn observed_owned() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([(
        SERVING,
        identities(
            &serving_contribution("llama-cpp.owned.serving"),
            super::ledger::OWNED_ROUTE,
        ),
    )])
}

pub(super) fn attached(
    revision: &str,
) -> swallowtail_adapter_llama_cpp::LlamaCppAttachedPreparedIntegration {
    attached_with("host.llama-cpp", revision, ready_status()).expect("attached prepares")
}

pub(super) fn attached_with(
    host: &str,
    revision: &str,
    status: AccessStatus,
) -> Result<
    swallowtail_adapter_llama_cpp::LlamaCppAttachedPreparedIntegration,
    swallowtail_runtime::PreparationFailure,
> {
    let fixture = Fixture::with_host(host);
    let access = llama_cpp_attached_access_profile();
    prepare_llama_cpp_attached(
        LlamaCppAttachedPreparationInput::new(
            ConfiguredInstanceId::new(format!("llama-cpp.attached.projection.{revision}"))
                .expect("instance"),
            InstanceRevision::new(revision).expect("revision"),
            ExecutionHostId::new(host).expect("host"),
            InstanceTargetRef::new("llama-cpp-fixture-endpoint").expect("target"),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        ),
        &fixture.services(),
    )
}

pub(super) fn ready_status() -> AccessStatus {
    status(
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

pub(super) fn status(
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint: EndpointAuthorization,
    readiness: RuntimeReadiness,
    authority: SupportAuthority,
) -> AccessStatus {
    AccessStatus::new(
        swallowtail_core::AccessProfileId::new(
            swallowtail_adapter_llama_cpp::LLAMA_CPP_ATTACHED_ACCESS_PROFILE_ID,
        )
        .expect("profile id"),
        credential,
        entitlement,
        endpoint,
        readiness,
        authority,
    )
}

pub(super) fn drifted_observations() -> [AccessStatus; 5] {
    [
        status(
            CredentialState::Expired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Exhausted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Denied,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Degraded,
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        ),
    ]
}

fn serving_with(
    context_size: Option<LlamaCppContextSize>,
    reasoning: Option<LlamaCppReasoningSelection>,
) -> LlamaCppPreparedServingStart {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let access = llama_cpp_owned_access_profile();
    let mut serving =
        LlamaCppOwnedServingSelection::new(fixture.artifact(), model_selection("llama-cpp-b10069"));
    if let Some(context_size) = context_size {
        serving = serving.with_context_size(context_size);
    }
    if let Some(reasoning) = reasoning {
        serving = serving.with_reasoning(reasoning);
    }
    let prepared = prepare_llama_cpp_owned(
        LlamaCppOwnedPreparationInput::new(
            ConfiguredInstanceId::new("llama-cpp.owned.projection").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            fixture.host_id(),
            InstanceTargetRef::new("llama-server.b10069").expect("target"),
            access.clone(),
            evidence(&access),
            serving,
        ),
        &fixture.services(),
    )
    .expect("owned prepares");
    prepared
        .prepare_serving_start(
            ScopeId::new("owned-projection-scope").expect("scope"),
            ServingInstanceId::new("owned-projection-instance").expect("serving id"),
            Deadline::at(MonotonicInstant::from_ticks(10_000)),
        )
        .expect("serving start prepares")
}

pub(super) fn inference_input(id: &str) -> LlamaCppInferenceProfileInput {
    LlamaCppInferenceProfileInput::new(
        RequestId::new(format!("llama-cpp.projection.{id}")).expect("request"),
        model_selection("llama-cpp-b9910"),
        OperationContent::new("private projection prompt").expect("prompt"),
        NonZeroU64::new(8).expect("nonzero"),
    )
}

fn model_selection(prefix: &str) -> LlamaCppModelSelection {
    LlamaCppModelSelection::new(
        ModelRouteId::new(format!("{prefix}/stories260k")).expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ModelId::new("swallowtail-fixture-stories260k").expect("model"),
    )
}

fn evidence(access: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    ))
}
