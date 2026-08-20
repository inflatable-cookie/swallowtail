//! Contract 057 lifecycle proof for local Ollama attach: admission of the
//! local-unauthenticated attached runtime, preparation reuse, readiness
//! refresh, subject observation, update observation, and overlay.
//!
//! Deterministic harness only: no live install, start, pull, or sign-in, and
//! no secret bytes or endpoint URLs in portable records.

#[allow(unused_imports)]
mod support;

use crate::support::Fixture;
use futures_executor::block_on;
use std::time::Duration;
use swallowtail_adapter_ollama::{
    OLLAMA_ATTACHED_ENDPOINT_FIELD_ID, OllamaModelSelection, OllamaPreparationInput,
    OllamaPreparationProbe, OllamaPreparedIntegration, ollama_attached_addable_route_descriptor,
    ollama_native_descriptor, ollama_runtime_binding, ollama_runtime_claim,
    prepare_ollama_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdmittedInstanceRecord,
    AttachedModelTag, AuthenticatedSubjectObservation, Capability, CapabilityRequirement,
    ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverRole, EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionLayer,
    HostServiceKind, InstanceEnablement, InstanceRevision, IntegrationFamilyId, ModelCatalogEntry,
    ModelId, ModelManifestDigest, ModelMetadata, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, OverlayMarker, PreflightContext, ProviderId,
    RuntimeReadiness, SubjectDisclosure, SupportAuthority, preflight,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceSelectionReadiness, ConfiguredProviderModelCatalogueInput,
    ConnectionLifecycleStore, DiscoveryCancellation, HostServices, InstanceAdmissionRequest,
    ModelPresentationOverlayFailureKind, PreparedAccessEvidence, PreparedOperationEvidence,
    ReadinessRefreshRequest, ScopeId, admit_instance, apply_stored_model_presentation_overlay,
    observe_authenticated_subject, observe_instance_update, refresh_readiness,
};

const INSTANCE: &str = "ollama.work";
const ACCESS: &str = "ollama.work.access";
const MODEL: &str = "fixture-model:8b";
const ENDPOINT_REF: &str = "ollama-fixture-endpoint";
const DIGEST: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("ollama").expect("family id is valid")
}

fn access_profile(fixture: &Fixture) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new(ACCESS).expect("access id is valid"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::LocalCompute,
        fixture.audience().clone(),
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn ready_access_status(profile: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        profile.id().clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(ready_access_status(profile))
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> AdmittedInstanceRecord {
    let descriptor = ollama_attached_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id).with_config_refs([(
            ConfigFieldId::new(OLLAMA_ATTACHED_ENDPOINT_FIELD_ID).expect("config id is valid"),
            ConfigFieldRef::new(ENDPOINT_REF).expect("config ref is valid"),
        )]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    fixture: &Fixture,
    admitted: &AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> Result<OllamaPreparationInput, swallowtail_runtime::PreparationFailure> {
    OllamaPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        fixture.host_id().clone(),
        profile.clone(),
        ready_evidence(profile),
        OllamaModelSelection::new(
            ModelRouteId::new("ollama.work.route").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            ModelId::new("ollama.work.model").expect("model id is valid"),
        ),
        AttachedModelTag::new(MODEL).expect("tag is valid"),
        ModelManifestDigest::new(DIGEST).expect("digest is valid"),
    )
}

fn probe(fixture: &Fixture) -> OllamaPreparationProbe {
    OllamaPreparationProbe::new(
        ScopeId::new("ollama.lifecycle.prepare").expect("scope id is valid"),
        fixture.thread.deadline_after(Duration::from_secs(1)),
        DiscoveryCancellation::new(),
    )
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "ollama");
    assert_eq!(record.route_id().as_str(), "ollama.attached");
    assert_eq!(
        record.topology(),
        swallowtail_core::RouteTopology::LocalRuntime
    );
    assert_eq!(record.credential_refs().len(), 0);
    assert_eq!(record.config_refs().len(), 1);
    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let debug = format!("{stored:?}");
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
    assert!(!debug.contains("ollama.work.endpoint"));
    assert!(!debug.contains("127.0.0.1"));
    assert!(!debug.contains("token"));
}

#[test]
fn missing_sign_in_ports_do_not_fail_the_local_unauthenticated_path() {
    let fixture = Fixture::new();
    let services = fixture.services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    let profile = access_profile(&fixture);
    assert_eq!(
        profile.credential_mechanism(),
        &CredentialMechanism::LocalUnauthenticated
    );
    assert!(profile.credential_reference().is_none());
    assert_eq!(record.credential_refs().len(), 0);
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile(&fixture);

    let prepared = block_on(prepare_ollama_attached(
        preparation_input(&fixture, &admitted, &profile)
            .expect("admitted fields produce preparation input"),
        probe(&fixture),
        services,
    ))
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
    assert_eq!(
        prepared.runtime().runtime_version(),
        &ollama_runtime_binding("0.30.0").expect("fixture Ollama version is valid")
    );
    let debug = format!("{prepared:?}");
    assert!(!debug.contains("ollama.work.endpoint"));
    assert!(!debug.contains("127.0.0.1"));
    assert!(!debug.contains("token"));
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = ollama_attached_addable_route_descriptor(&services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    let admitted = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id)
            .with_enablement(InstanceEnablement::Disabled),
    )
    .expect("admission succeeds");
    assert_eq!(admitted.enablement(), InstanceEnablement::Disabled);
    assert!(admitted.access_status().is_none());

    let profile = access_profile(&fixture);
    let refreshed = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(instance_id(), ready_access_status(&profile)),
    )
    .expect("refresh succeeds");

    let status = refreshed.access_status().expect("access status is stored");
    assert_eq!(status.profile_id(), profile.id());
    assert_eq!(status.credential(), CredentialState::NotRequired);
    assert_eq!(status.entitlement(), EntitlementState::Available);
    assert_eq!(
        status.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(status.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(refreshed.enablement(), InstanceEnablement::Disabled);
}

#[test]
fn subject_stays_absent_for_ollama_attach() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    admitted_record(&services, &store);

    let observed = observe_authenticated_subject(
        &store,
        &instance_id(),
        AuthenticatedSubjectObservation::undisclosed(),
    )
    .expect("subject observation succeeds");

    assert_eq!(observed.email(), &SubjectDisclosure::Absent);
    assert_eq!(observed.login(), &SubjectDisclosure::Absent);
    assert_eq!(observed.plan(), &SubjectDisclosure::Absent);
}

#[test]
fn update_observation_reuses_the_runtime_claim_with_032_unobserved() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile(&fixture);
    let prepared = block_on(prepare_ollama_attached(
        preparation_input(&fixture, &admitted, &profile)
            .expect("admitted fields produce preparation input"),
        probe(&fixture),
        services,
    ))
    .expect("admitted instance prepares");
    let claim = ollama_runtime_claim();

    // The runtime version comes from preparation's /api/version observation,
    // not a Contract 032 installed-executable observation, so 032 stays
    // unobserved while the 029 claim still classifies the prepared runtime.
    assert!(
        claim
            .assess(prepared.runtime().runtime_version().version())
            .is_permitted()
    );
    let unobserved = observe_instance_update(&claim, None).expect("claim alone is valid");
    assert_eq!(unobserved.claim_id(), claim.id());
    assert!(unobserved.is_unobserved());
    assert_eq!(unobserved.compatibility(), None);
}

fn catalogue_entry(model_id: &str, provider_default: bool) -> ModelCatalogEntry {
    ModelCatalogEntry::new(
        ModelId::new(model_id).expect("model id is valid"),
        ModelMetadata::default().with_default(provider_default),
    )
}

fn prepared_route_evidence(
    services: &HostServices,
    prepared: &OllamaPreparedIntegration,
    profile: &AccessProfile,
    evidence: &PreparedAccessEvidence,
) -> PreparedOperationEvidence {
    let instance = prepared.instance();
    let status = evidence.status();
    let version = prepared.runtime().runtime_version().clone();
    let requirements = OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        DriverRole::ModelCatalog,
        instance.execution_host_id().clone(),
        AccessRequirement::new(profile.id().clone())
            .with_credential_states([status.credential()])
            .with_entitlement_states([status.entitlement()])
            .with_endpoint_authorizations([status.endpoint_authorization()])
            .with_runtime_readiness([status.runtime_readiness()])
            .with_support_authorities([status.support_authority()]),
    )
    .with_ownership_modes([instance.ownership()])
    .with_host_services([
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
    ])
    .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])])
    .with_interface_versions([version]);
    let descriptor = ollama_native_descriptor();
    let plan = preflight(
        &PreflightContext::new(
            &descriptor,
            instance,
            profile,
            status,
            services.available_kinds(),
        ),
        &requirements,
    )
    .expect("preflight succeeds for the prepared route");
    PreparedOperationEvidence::from_plan(plan, evidence.clone())
        .expect("prepared evidence is accepted")
}

fn snapshot_record(
    fixture: &Fixture,
    services: &HostServices,
    admitted: &AdmittedInstanceRecord,
) -> ConfiguredProviderInstanceRecord {
    let profile = access_profile(fixture);
    let evidence = ready_evidence(&profile);
    let prepared = block_on(prepare_ollama_attached(
        preparation_input(fixture, admitted, &profile)
            .expect("admitted fields produce preparation input"),
        probe(fixture),
        services.clone(),
    ))
    .expect("admitted instance prepares");
    let route = prepared_route_evidence(services, &prepared, &profile, &evidence);
    ConfiguredProviderInstanceRecord::admit(
        ConfiguredProviderInstanceAdmission::new(
            ollama_native_descriptor(),
            prepared.instance().clone(),
            profile,
            evidence,
        )
        .with_prepared_routes([route.clone()])
        .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
            route,
            [
                catalogue_entry(MODEL, true),
                catalogue_entry("fixture-model:70b", false),
            ],
        )),
    )
    .expect("047 snapshot assembles")
}

#[test]
fn overlay_does_not_invent_an_ollama_catalogue_provider_id() {
    let fixture = Fixture::new();
    let services = fixture.services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let record = snapshot_record(&fixture, &services, &admitted);
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("ollama.work.endpoint"));
    assert!(!snapshot_debug.contains("127.0.0.1"));

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the ollama catalogue");

    assert_eq!(overlay.selection_readiness(), record.selection_readiness());
    assert_eq!(overlay.instance_id(), &instance_id());
    let entries: Vec<_> = overlay.entries().collect();
    assert_eq!(entries.len(), 2);
    for entry in &entries {
        assert_eq!(entry.provider_id(), None);
        assert!(!entry.hidden());
        assert_eq!(entry.ordinal(), None);
        assert!(!entry.consumer_default());
        assert!(!entry.favourite());
    }
    let primary = entries
        .iter()
        .find(|entry| entry.model_id().as_str() == MODEL)
        .expect("primary row is present");
    assert!(primary.provider_default());

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new("ollama").expect("provider id is valid"),
                ModelId::new("fixture-model:70b").expect("model id is valid"),
            )
            .with_favourite(true),
        )
        .expect("overlay marker stores");
    let failure = apply_stored_model_presentation_overlay(&store, &record)
        .expect_err("an invented catalogue provider id fails closed");
    assert_eq!(
        failure.kind(),
        ModelPresentationOverlayFailureKind::UnknownModel
    );
}
