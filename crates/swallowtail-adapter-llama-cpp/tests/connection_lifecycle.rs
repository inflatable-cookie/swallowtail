//! Contract 057 lifecycle proof for local llama.cpp attach: admission of the
//! local-unauthenticated attached runtime, preparation reuse, readiness
//! refresh, subject observation, update observation, and overlay.
//!
//! Deterministic harness only: no live start, stop, or `/health`, and no
//! secret bytes or endpoint URLs in portable records.

#[allow(dead_code)]
#[path = "support/services.rs"]
mod services;

use services::ThreadServices;
use std::sync::Arc;
use swallowtail_adapter_llama_cpp::{
    LLAMA_CPP_ATTACHED_ENDPOINT_FIELD_ID, LlamaCppAttachedPreparationInput,
    LlamaCppAttachedPreparedIntegration, llama_cpp_attached_access_profile,
    llama_cpp_attached_addable_route_descriptor, llama_cpp_attached_descriptor,
    llama_cpp_attached_runtime_binding, llama_cpp_attached_runtime_claim,
    llama_cpp_owned_access_profile, prepare_llama_cpp_attached,
};
use swallowtail_core::{
    AccessProfile, AccessRequirement, AccessStatus, AdmittedInstanceRecord,
    AuthenticatedSubjectObservation, Capability, CapabilityRequirement, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole,
    EndpointAuthorization, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceEnablement, InstanceRevision, IntegrationFamilyId, ModelCatalogEntry, ModelId,
    ModelMetadata, OperationRequirements, OperationShape, OverlayMarker, PreflightContext,
    ProviderId, RuntimeReadiness, SubjectDisclosure, SupportAuthority, preflight,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, BlockingWorkService, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogueInput, ConnectionLifecycleStore, HostServices,
    InstanceAdmissionRequest, ModelPresentationOverlayFailureKind, NetworkPolicyService,
    PreparedAccessEvidence, PreparedOperationEvidence, ReadinessRefreshRequest, ScopedTaskService,
    TimeService, admit_instance, apply_stored_model_presentation_overlay,
    observe_authenticated_subject, observe_instance_update, refresh_readiness,
};

const INSTANCE: &str = "llama-cpp.work";
const MODEL: &str = "swallowtail-fixture-stories260k";
const ENDPOINT_REF: &str = "llama-cpp-fixture-endpoint";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("llama-cpp.lifecycle.host").expect("host id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("llama.cpp").expect("family id is valid")
}

fn access_profile() -> AccessProfile {
    llama_cpp_attached_access_profile()
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

fn services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let thread = Arc::new(ThreadServices::new());
    HostServices::new(host_id())
        .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
        .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
        .with_time(thread as Arc<dyn TimeService>)
        .with_network(Arc::new(host) as Arc<dyn NetworkPolicyService>)
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> AdmittedInstanceRecord {
    let descriptor = llama_cpp_attached_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id).with_config_refs([(
            ConfigFieldId::new(LLAMA_CPP_ATTACHED_ENDPOINT_FIELD_ID).expect("config id is valid"),
            ConfigFieldRef::new(ENDPOINT_REF).expect("config ref is valid"),
        )]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    admitted: &AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> Result<LlamaCppAttachedPreparationInput, swallowtail_runtime::PreparationFailure> {
    LlamaCppAttachedPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        ready_evidence(profile),
    )
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "llama.cpp");
    assert_eq!(record.route_id().as_str(), "llama-cpp.attached");
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
    assert!(!debug.contains("llama-cpp.work.endpoint"));
    assert!(!debug.contains("127.0.0.1"));
    assert!(!debug.contains("://"));
    assert!(!debug.contains("token"));
}

#[test]
fn missing_sign_in_ports_do_not_fail_the_local_unauthenticated_path() {
    let services = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    let profile = access_profile();
    assert_eq!(
        profile.credential_mechanism(),
        &CredentialMechanism::LocalUnauthenticated
    );
    assert!(profile.credential_reference().is_none());
    assert_eq!(profile.endpoint_audience().as_str(), "llama.cpp.attached");
    assert_ne!(&profile, &llama_cpp_owned_access_profile());
    assert_eq!(record.credential_refs().len(), 0);
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();

    let prepared = prepare_llama_cpp_attached(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        &services,
    )
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
    assert_eq!(prepared.expected_build(), "9910");
    assert_eq!(prepared.expected_commit(), "f5525f7e7");
    assert_ne!(prepared.access_profile(), &llama_cpp_owned_access_profile());
    let debug = format!("{:?}", prepared.instance());
    assert!(!debug.contains("llama-cpp.work.endpoint"));
    assert!(!debug.contains("127.0.0.1"));
    assert!(!debug.contains("token"));
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = llama_cpp_attached_addable_route_descriptor(&services);
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

    let profile = access_profile();
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
    assert_eq!(
        status.support_authority(),
        SupportAuthority::IntegrationMaintainerSupported
    );
    assert_eq!(refreshed.enablement(), InstanceEnablement::Disabled);
}

#[test]
fn subject_stays_absent_for_llama_cpp_attach() {
    let services = services();
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
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();
    let prepared = prepare_llama_cpp_attached(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        &services,
    )
    .expect("admitted instance prepares");
    let claim = llama_cpp_attached_runtime_claim();

    // Attached runtime identity is the exact opaque b9910/f5525f7e7 binding,
    // not a Contract 032 installed-executable observation, so 032 stays
    // unobserved while the 029 claim still classifies the prepared runtime.
    assert_eq!(prepared.expected_build(), "9910");
    assert_eq!(prepared.expected_commit(), "f5525f7e7");
    assert!(
        claim
            .assess(llama_cpp_attached_runtime_binding().version())
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
    prepared: &LlamaCppAttachedPreparedIntegration,
    profile: &AccessProfile,
    evidence: &PreparedAccessEvidence,
) -> PreparedOperationEvidence {
    let instance = prepared.instance();
    let status = evidence.status();
    let version = llama_cpp_attached_runtime_binding();
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
    let descriptor = llama_cpp_attached_descriptor();
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
    services: &HostServices,
    admitted: &AdmittedInstanceRecord,
) -> ConfiguredProviderInstanceRecord {
    let profile = access_profile();
    let evidence = ready_evidence(&profile);
    let prepared = prepare_llama_cpp_attached(
        preparation_input(admitted, &profile).expect("admitted fields produce preparation input"),
        services,
    )
    .expect("admitted instance prepares");
    let route = prepared_route_evidence(services, &prepared, &profile, &evidence);
    ConfiguredProviderInstanceRecord::admit(
        ConfiguredProviderInstanceAdmission::new(
            llama_cpp_attached_descriptor(),
            prepared.instance().clone(),
            profile,
            evidence,
        )
        .with_prepared_routes([route.clone()])
        .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
            route,
            [
                catalogue_entry(MODEL, true),
                catalogue_entry("swallowtail-fixture-other", false),
            ],
        )),
    )
    .expect("047 snapshot assembles")
}

#[test]
fn overlay_keys_llama_cpp_rows_by_instance_and_model() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let record = snapshot_record(&services, &admitted);
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("llama-cpp.work.endpoint"));
    assert!(!snapshot_debug.contains("127.0.0.1"));

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the llama.cpp catalogue");

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
            OverlayMarker::without_provider(
                instance_id(),
                ModelId::new("swallowtail-fixture-other").expect("model id is valid"),
            )
            .with_favourite(true),
        )
        .expect("unmarked overlay marker stores");
    let marked = apply_stored_model_presentation_overlay(&store, &record)
        .expect("instance-plus-model marker applies");
    assert_eq!(marked.selection_readiness(), record.selection_readiness());
    let secondary = marked
        .entries()
        .find(|entry| entry.model_id().as_str() == "swallowtail-fixture-other")
        .expect("secondary row is present");
    assert_eq!(secondary.provider_id(), None);
    assert!(secondary.favourite());

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new("llama-cpp").expect("provider id is valid"),
                ModelId::new("swallowtail-fixture-other").expect("model id is valid"),
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
