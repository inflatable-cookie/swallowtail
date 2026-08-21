//! Contract 057 lifecycle proof for hosted Kimi Platform Chat: admission,
//! API-key collection, readiness refresh, subject observation, and the 047
//! snapshot plus model-presentation overlay.
//!
//! Deterministic harness only: no live provider calls, no browser ports, no
//! secret bytes in portable records.

#[allow(dead_code)]
mod support;

use std::num::NonZeroU64;
use std::sync::Arc;
use support::ThreadServices;
use swallowtail_adapter_kimi_platform::{
    KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID, KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID,
    KIMI_PLATFORM_ENDPOINT_AUDIENCE, KIMI_PLATFORM_FACADE_REVISION, KIMI_PLATFORM_MODEL_ID,
    KIMI_PLATFORM_PROVIDER_ID, KimiPlatformCatalogueProfileInput,
    KimiPlatformInferenceAttemptInput, KimiPlatformModelSelection, KimiPlatformPreparationInput,
    KimiPlatformPreparedIntegration, kimi_platform_chat_addable_route_descriptor,
    kimi_platform_direct_descriptor, prepare_kimi_platform_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AuthenticatedSubjectObservation, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, InstanceEnablement, InstanceRevision, IntegrationFamilyId, ModelCatalogEntry,
    ModelId, ModelMetadata, ModelRouteId, ModelRouteRevision, OverlayMarker, ProviderId,
    ReasoningMode, RuntimeReadiness, SubjectDisclosure, SupportAuthority,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, BlockingWorkService, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogueInput, ConnectionLifecycleStore, CredentialService,
    HostServices, InstanceAdmissionRequest, NetworkPolicyService, OperationContent,
    PreparedAccessEvidence, ReadinessRefreshRequest, RequestId, ScopeId, ScopedTaskService,
    SignInAuthorityBinding, SignInMethod, SignInStartRequest, SignInStatus, TimeService,
    admit_instance, apply_stored_model_presentation_overlay, complete_sign_in,
    observe_authenticated_subject, poll_sign_in, refresh_readiness, start_sign_in,
    submit_sign_in_credential_field,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const INSTANCE: &str = "kimi-platform.work";
const CREDENTIAL_REF: &str = "kimi-platform.work.api-key";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("kimi-platform.admission.host").expect("host id is valid")
}

fn services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("kimi-platform").expect("family id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = kimi_platform_chat_addable_route_descriptor(services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), descriptor_route_id(&catalog))
            .with_config_refs([(
                ConfigFieldId::new(KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("kimi-platform.work.endpoint").expect("config ref is valid"),
            )]),
    )
    .expect("admission succeeds")
}

fn descriptor_route_id(catalog: &AddableRouteCatalog) -> swallowtail_core::AddableRouteId {
    catalog
        .routes()
        .next()
        .expect("catalog has the kimi-platform route")
        .id()
        .clone()
}

fn access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("kimi-platform.work.access").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(KIMI_PLATFORM_ENDPOINT_AUDIENCE).expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services(), &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "kimi-platform");
    assert_eq!(record.route_id().as_str(), "kimi-platform.chat");
    assert_eq!(record.topology(), swallowtail_core::RouteTopology::Hosted);
    assert_eq!(record.credential_refs().len(), 0);
    assert!(
        store
            .get_instance(&instance_id())
            .expect("store read succeeds")
            .is_some()
    );
}

#[test]
fn api_key_collection_completes_without_browser_ports() {
    let services = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let descriptor = kimi_platform_chat_addable_route_descriptor(&services);
    let fields: Vec<_> = descriptor.credential_fields().cloned().collect();

    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            ScopeId::new("kimi-platform.admission.sign-in:work").expect("scope is valid"),
            admitted.id().clone(),
            family(),
            admitted.route_id().clone(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                EndpointAudience::new(KIMI_PLATFORM_ENDPOINT_AUDIENCE).expect("audience is valid"),
                EntitlementMetering::PayAsYouGo,
            ),
            SignInMethod::ApiKeyCollection { fields },
        ),
    )
    .expect("API-key collection starts without browser ports");
    assert_eq!(
        poll_sign_in(&mut session, &services).expect("poll succeeds"),
        SignInStatus::InProgress
    );

    let status = submit_sign_in_credential_field(
        &mut session,
        swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("field id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )
    .expect("field submit succeeds");
    assert_eq!(status, SignInStatus::ReadyToComplete);

    let outcome =
        complete_sign_in(session, &services, Some(&store)).expect("sign-in loop completes");
    assert_eq!(outcome.instance_id(), &instance_id());
    let refs: Vec<_> = outcome.credential_refs().collect();
    assert_eq!(refs.len(), 1);
    assert_eq!(refs[0].1.as_host_value(), CREDENTIAL_REF);

    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let stored_refs: Vec<_> = stored.credential_refs().collect();
    assert_eq!(stored_refs.len(), 1);
    assert_eq!(stored_refs[0].1.as_host_value(), CREDENTIAL_REF);
    let debug = format!("{stored:?}");
    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(!debug.contains("sk-"));
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store).with_credential_refs([(
        swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("credential id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )]);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let prepared = prepare_kimi_platform_direct(
        KimiPlatformPreparationInput::from_admitted(
            &admitted,
            InstanceRevision::new("1").expect("revision is valid"),
            host_id(),
            profile.clone(),
            evidence,
        )
        .expect("admitted fields produce preparation input"),
        &services,
    )
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
}

#[test]
fn preparation_fails_closed_on_admission_drift() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let wrong_route = swallowtail_core::AdmittedInstanceRecord::new(
        instance_id(),
        family(),
        swallowtail_core::AddableRouteId::new("kimi-platform.other").expect("route id is valid"),
        swallowtail_adapter_kimi_platform::kimi_platform_direct_descriptor()
            .identity()
            .clone(),
        swallowtail_core::RouteTopology::Hosted,
    );
    let route_mismatch = KimiPlatformPreparationInput::from_admitted(
        &wrong_route,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("another route fails closed");
    assert_eq!(
        route_mismatch.stage(),
        swallowtail_runtime::PreparationStage::TargetSelection
    );

    let no_refs = swallowtail_core::AdmittedInstanceRecord::new(
        instance_id(),
        family(),
        swallowtail_core::AddableRouteId::new("kimi-platform.chat").expect("route id is valid"),
        swallowtail_adapter_kimi_platform::kimi_platform_direct_descriptor()
            .identity()
            .clone(),
        swallowtail_core::RouteTopology::Hosted,
    );
    let endpoint_missing = KimiPlatformPreparationInput::from_admitted(
        &no_refs,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("missing endpoint ref fails closed");
    assert_eq!(
        endpoint_missing.stage(),
        swallowtail_runtime::PreparationStage::TargetSelection
    );

    let missing_refs = KimiPlatformPreparationInput::from_admitted(
        &admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        evidence.clone(),
    )
    .expect_err("missing credential ref fails closed");
    assert_eq!(
        missing_refs.stage(),
        swallowtail_runtime::PreparationStage::AccessEvidence
    );

    let mismatched_credential = KimiPlatformPreparationInput::from_admitted(
        &admitted.with_credential_refs([(
            swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
                .expect("credential id is valid"),
            CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
        )]),
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        access_profile(CredentialRef::new("kimi-platform.work.other-key").expect("ref is valid")),
        evidence,
    )
    .expect_err("mismatched credential ref fails closed");
    assert_eq!(
        mismatched_credential.stage(),
        swallowtail_runtime::PreparationStage::AccessEvidence
    );
}

fn prepared_services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let thread = Arc::new(ThreadServices::new());
    HostServices::new(host_id())
        .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
        .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
        .with_time(thread as Arc<dyn TimeService>)
        .with_network(Arc::new(host.clone()) as Arc<dyn NetworkPolicyService>)
        .with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn ready_access_status(profile: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        profile.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn k3_model_selection() -> KimiPlatformModelSelection {
    KimiPlatformModelSelection::new(
        ModelRouteId::new("kimi-platform.lifecycle.k3").expect("route id is valid"),
        ModelRouteRevision::new("2026-07-21").expect("route revision is valid"),
        ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
    )
}

fn prepared_after_admission(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> (
    KimiPlatformPreparedIntegration,
    AccessProfile,
    PreparedAccessEvidence,
) {
    let admitted = admitted_record(services, store).with_credential_refs([(
        swallowtail_core::CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("credential id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )]);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);
    let prepared = prepare_kimi_platform_direct(
        KimiPlatformPreparationInput::from_admitted(
            &admitted,
            InstanceRevision::new("1").expect("revision is valid"),
            host_id(),
            profile.clone(),
            evidence.clone(),
        )
        .expect("admitted fields produce preparation input"),
        services,
    )
    .expect("admitted instance prepares");
    (prepared, profile, evidence)
}

fn kimi_catalogue_entry() -> ModelCatalogEntry {
    ModelCatalogEntry::new(
        ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
        ModelMetadata::default().with_default(true),
    )
    .with_provider_id(ProviderId::new(KIMI_PLATFORM_PROVIDER_ID).expect("provider id is valid"))
}

fn snapshot_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> ConfiguredProviderInstanceRecord {
    let (prepared, profile, evidence) = prepared_after_admission(services, store);
    let catalogue = prepared
        .prepare_catalogue(KimiPlatformCatalogueProfileInput::new(
            RequestId::new("lifecycle-catalogue").expect("request id is valid"),
        ))
        .expect("catalogue prepares");
    let attempt = prepared
        .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
            RequestId::new("lifecycle-attempt").expect("request id is valid"),
            k3_model_selection(),
            OperationContent::new("lifecycle fixture prompt").expect("content is valid"),
            ReasoningMode::new("high").expect("reasoning is valid"),
            NonZeroU64::new(128).expect("output bound is valid"),
        ))
        .expect("attempt prepares");
    ConfiguredProviderInstanceRecord::admit(
        ConfiguredProviderInstanceAdmission::new(
            kimi_platform_direct_descriptor(),
            prepared.instance().clone(),
            profile,
            evidence,
        )
        .with_prepared_routes([
            catalogue.evidence().operation().clone(),
            attempt.evidence().operation().clone(),
        ])
        .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
            catalogue.evidence().operation().clone(),
            [kimi_catalogue_entry()],
        )),
    )
    .expect("047 snapshot assembles")
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = kimi_platform_chat_addable_route_descriptor(&services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    let admitted = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(
            instance_id(),
            family(),
            swallowtail_core::AddableRouteId::new("kimi-platform.chat").expect("route id is valid"),
        )
        .with_enablement(InstanceEnablement::Disabled),
    )
    .expect("admission succeeds");
    assert_eq!(admitted.enablement(), InstanceEnablement::Disabled);
    assert!(admitted.access_status().is_none());

    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let refreshed = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(instance_id(), ready_access_status(&profile)),
    )
    .expect("refresh succeeds");

    let status = refreshed.access_status().expect("access status is stored");
    assert_eq!(status.credential(), CredentialState::Ready);
    assert_eq!(status.entitlement(), EntitlementState::Available);
    assert_eq!(
        status.endpoint_authorization(),
        EndpointAuthorization::Allowed
    );
    assert_eq!(status.runtime_readiness(), RuntimeReadiness::Ready);
    assert_eq!(refreshed.enablement(), InstanceEnablement::Disabled);
}

#[test]
fn subject_stays_absent_for_kimi_platform_chat() {
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
fn catalogue_and_one_k3_attempt_prepare_after_admission() {
    let services = prepared_services();
    let store = MemoryConnectionLifecycleStore::new();
    let (prepared, _, _) = prepared_after_admission(&services, &store);

    let catalogue = prepared
        .prepare_catalogue(KimiPlatformCatalogueProfileInput::new(
            RequestId::new("lifecycle-catalogue").expect("request id is valid"),
        ))
        .expect("catalogue prepares");
    assert_eq!(
        catalogue.plan().requirements().driver_role(),
        DriverRole::ModelCatalog
    );
    assert!(catalogue.plan().model_route_id().is_none());
    assert_prepared_operation_evidence_matches_plan(
        catalogue.evidence().operation(),
        catalogue.plan(),
    );

    let attempt = prepared
        .prepare_inference_attempt(KimiPlatformInferenceAttemptInput::new(
            RequestId::new("lifecycle-attempt").expect("request id is valid"),
            k3_model_selection(),
            OperationContent::new("lifecycle fixture prompt").expect("content is valid"),
            ReasoningMode::new("high").expect("reasoning is valid"),
            NonZeroU64::new(128).expect("output bound is valid"),
        ))
        .expect("attempt prepares");
    assert_eq!(
        attempt.plan().requirements().driver_role(),
        DriverRole::StructuredRun
    );
    assert_eq!(
        attempt.plan().model_id().expect("model").as_str(),
        KIMI_PLATFORM_MODEL_ID
    );
    assert_eq!(attempt.request().tools().len(), 0);
    assert_prepared_operation_evidence_matches_plan(attempt.evidence().operation(), attempt.plan());
}

#[test]
fn overlay_marks_kimi_catalogue_rows_without_changing_readiness() {
    let services = prepared_services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = snapshot_record(&services, &store);

    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(record.instance_id(), &instance_id());
    assert_eq!(
        record.driver_identity(),
        kimi_platform_direct_descriptor().identity()
    );
    assert_eq!(
        record.protocol_facade_id().as_str(),
        KIMI_PLATFORM_FACADE_REVISION
    );
    assert_eq!(
        record.instance_policy_id().as_str(),
        "public-platform-api-key"
    );
    assert_eq!(record.routes().len(), 2);
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("sk-"));
    assert!(!snapshot_debug.contains("fixture-secret"));

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new(KIMI_PLATFORM_PROVIDER_ID).expect("provider id is valid"),
                ModelId::new(KIMI_PLATFORM_MODEL_ID).expect("model id is valid"),
            )
            .with_favourite(true)
            .with_ordinal(Some(0)),
        )
        .expect("overlay marker stores");

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the kimi catalogue");

    assert_eq!(overlay.selection_readiness(), record.selection_readiness());
    assert_eq!(overlay.instance_id(), &instance_id());
    let entries: Vec<_> = overlay.entries().collect();
    assert_eq!(entries.len(), 1);
    let entry = entries[0];
    assert_eq!(entry.model_id().as_str(), KIMI_PLATFORM_MODEL_ID);
    assert!(entry.favourite());
    assert_eq!(entry.ordinal(), Some(0));
    assert!(entry.provider_default());
    assert!(!entry.consumer_default());
    assert!(!entry.hidden());
}
