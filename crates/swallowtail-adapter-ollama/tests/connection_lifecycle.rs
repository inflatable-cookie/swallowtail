//! Contract 057 lifecycle proof for local Ollama attach: admission of the
//! local-unauthenticated attached runtime and preparation reuse.
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
    OllamaPreparationProbe, ollama_attached_addable_route_descriptor, ollama_runtime_binding,
    prepare_ollama_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AdmittedInstanceRecord, AttachedModelTag,
    ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    EndpointAuthorization, EntitlementMetering, EntitlementState, InstanceRevision,
    IntegrationFamilyId, ModelId, ModelManifestDigest, ModelRouteId, ModelRouteRevision,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConnectionLifecycleStore, DiscoveryCancellation, HostServices,
    InstanceAdmissionRequest, PreparedAccessEvidence, ScopeId, admit_instance,
};

const INSTANCE: &str = "ollama.work";
const ACCESS: &str = "ollama.work.access";
const MODEL: &str = "fixture-model:8b";
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
            ConfigFieldRef::new("ollama.work.endpoint").expect("config ref is valid"),
        )]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    fixture: &Fixture,
    admitted: &AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> OllamaPreparationInput {
    OllamaPreparationInput::new(
        admitted.id().clone(),
        InstanceRevision::new("1").expect("revision is valid"),
        fixture.host_id().clone(),
        fixture.target().clone(),
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
        preparation_input(&fixture, &admitted, &profile),
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
