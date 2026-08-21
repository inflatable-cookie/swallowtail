//! Contract 057 lifecycle proof for installed Codex app-server: admission of
//! the cached ChatGPT subscription profile, preparation reuse, readiness
//! refresh, subject observation, and update observation.
//!
//! Deterministic harness only: no live login or install probes, no browser
//! ports, no ChatGPT token extraction, no secret bytes in portable records.

use crate::support::{FakeProcessService, host_services_for};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_codex::{
    CODEX_APP_SERVER_BINARY_PATH_FIELD_ID, CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID,
    CodexPreparationInput, CodexPreparationProbe, codex_app_server_addable_route_descriptor,
    codex_app_server_claim, codex_app_server_descriptor, codex_chatgpt_subscription_access_profile,
    prepare_codex,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus,
    AuthenticatedSubjectObservation, Capability, CapabilityRequirement, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture,
    HostServiceKind, InstalledExecutableCompatibility, InstanceEnablement, InstanceRevision,
    IntegrationFamilyId, ModelCatalogEntry, ModelId, ModelMetadata, OperationRequirements,
    OperationShape, OverlayMarker, PreflightContext, ProviderId, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SubjectDisclosure, SupportAuthority,
    preflight,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, BoxFuture, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogueInput, ConnectionLifecycleStore, Deadline, DeadlineObservation,
    DiscoveryCancellation, HostServices, InstanceAdmissionRequest,
    ModelPresentationOverlayFailureKind, MonotonicInstant, PreparedAccessEvidence,
    PreparedOperationEvidence, ReadinessRefreshRequest, RequestId, ScopeId, TimeService,
    admit_instance, apply_stored_model_presentation_overlay, observe_authenticated_subject,
    observe_instance_update, refresh_readiness,
};

const INSTANCE: &str = "codex.work";
const HOST: &str = "codex.lifecycle.host";
const ACCESS: &str = "codex.work.chatgpt";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new(HOST).expect("host id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("codex").expect("family id is valid")
}

fn access_profile() -> AccessProfile {
    codex_chatgpt_subscription_access_profile(
        AccessProfileId::new(ACCESS).expect("access id is valid"),
    )
}

fn ready_evidence(profile: &AccessProfile) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(ready_access_status(profile))
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

fn services() -> (HostServices, Arc<dyn swallowtail_runtime::ProcessService>) {
    let (process, _) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let services = host_services_for(host_id(), process.clone()).with_time(Arc::new(PendingTime));
    (services, process)
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = codex_app_server_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), route_id).with_config_refs([
            (
                ConfigFieldId::new(CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex-app-server").expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("codex.work.login").expect("config ref is valid"),
            ),
        ]),
    )
    .expect("admission succeeds")
}

fn preparation_input(
    admitted: &swallowtail_core::AdmittedInstanceRecord,
    profile: &AccessProfile,
) -> Result<CodexPreparationInput, swallowtail_runtime::PreparationFailure> {
    CodexPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host_id(),
        profile.clone(),
        ready_evidence(profile),
    )
}

fn probe() -> CodexPreparationProbe {
    CodexPreparationProbe::new(
        RequestId::new("codex.lifecycle.prepare").expect("request id is valid"),
        ScopeId::new("codex.lifecycle.prepare").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

#[test]
fn admission_writes_a_record_without_secret_bytes() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    assert_eq!(record.id(), &instance_id());
    assert_eq!(record.family().as_str(), "codex");
    assert_eq!(record.route_id().as_str(), "codex.app-server");
    assert_eq!(
        record.topology(),
        swallowtail_core::RouteTopology::Installed
    );
    assert_eq!(record.credential_refs().len(), 0);
    assert_eq!(record.config_refs().len(), 2);
    let stored = store
        .get_instance(&instance_id())
        .expect("store read succeeds")
        .expect("instance is stored");
    let debug = format!("{stored:?}");
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
    assert!(!debug.contains("codex.work.binary-path"));
    assert!(!debug.contains("codex.work.login"));
    assert!(!debug.contains("token"));
}

#[test]
fn missing_browser_ports_do_not_fail_the_chatgpt_path() {
    let (services, _) = services();
    assert!(services.url_open().is_none());
    assert!(services.loopback_callback().is_none());
    assert!(services.device_code_display().is_none());

    let store = MemoryConnectionLifecycleStore::new();
    let record = admitted_record(&services, &store);

    let profile = access_profile();
    assert_eq!(
        profile.credential_mechanism(),
        &swallowtail_core::CredentialMechanism::InteractiveOauth
    );
    assert!(profile.credential_reference().is_none());
    assert_eq!(record.credential_refs().len(), 0);
}

#[test]
fn prepare_still_accepts_the_admitted_identity_and_access_profile() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();

    let prepared = block_on(prepare_codex(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services,
    ))
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
    assert!(prepared.observation().is_permitted());
    let debug = format!("{prepared:?}");
    assert!(!debug.contains("codex.work.login"));
    assert!(!debug.contains("token"));
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

fn prepared_route_evidence(
    services: &HostServices,
    driver: &swallowtail_core::DriverDescriptor,
    instance: &swallowtail_core::ConfiguredInstance,
    profile: &AccessProfile,
    evidence: &PreparedAccessEvidence,
) -> PreparedOperationEvidence {
    let status = evidence.status();
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
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
    .with_host_services([HostServiceKind::Task, HostServiceKind::Process])
    .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])])
    .with_interface_versions(instance.interface_versions().cloned().collect::<Vec<_>>())
    .with_session_access_policy(SessionAccessPolicy::read_only())
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let plan = preflight(
        &PreflightContext::new(
            driver,
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

fn catalogue_entry(model_id: &str, provider_default: bool) -> ModelCatalogEntry {
    ModelCatalogEntry::new(
        ModelId::new(model_id).expect("model id is valid"),
        ModelMetadata::default().with_default(provider_default),
    )
}

fn snapshot_record(
    services: &HostServices,
    admitted: &swallowtail_core::AdmittedInstanceRecord,
) -> ConfiguredProviderInstanceRecord {
    let profile = access_profile();
    let evidence = ready_evidence(&profile);
    let prepared = block_on(prepare_codex(
        preparation_input(admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services.clone(),
    ))
    .expect("admitted instance prepares");
    let driver = codex_app_server_descriptor();
    let route =
        prepared_route_evidence(services, &driver, prepared.instance(), &profile, &evidence);
    ConfiguredProviderInstanceRecord::admit(
        ConfiguredProviderInstanceAdmission::new(
            driver,
            prepared.instance().clone(),
            profile,
            evidence,
        )
        .with_prepared_routes([route.clone()])
        .with_model_catalogue(ConfiguredProviderModelCatalogueInput::available(
            route,
            [
                catalogue_entry("gpt-fixture-primary", true),
                catalogue_entry("gpt-fixture-secondary", false),
            ],
        )),
    )
    .expect("047 snapshot assembles")
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = codex_app_server_addable_route_descriptor(&services);
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
fn subject_stays_absent_for_codex_app_server() {
    let (services, _) = services();
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
fn update_observation_reuses_the_app_server_claim_and_032_evidence() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let profile = access_profile();
    let prepared = block_on(prepare_codex(
        preparation_input(&admitted, &profile).expect("admitted fields produce preparation input"),
        probe(),
        services,
    ))
    .expect("admitted instance prepares");
    let claim = codex_app_server_claim();

    let unobserved = observe_instance_update(&claim, None).expect("claim alone is valid");
    assert_eq!(unobserved.claim_id(), claim.id());
    assert!(unobserved.is_unobserved());
    assert_eq!(unobserved.compatibility(), None);

    let observed = observe_instance_update(&claim, Some(prepared.observation().clone()))
        .expect("prepared 032 observation matches the claim");
    assert_eq!(observed.claim_id(), claim.id());
    assert_eq!(observed.installed(), Some(prepared.observation()));
    assert!(matches!(
        observed.compatibility(),
        Some(InstalledExecutableCompatibility::Qualified(_))
    ));
}

#[test]
fn overlay_keys_codex_rows_by_instance_and_model() {
    let (services, _) = services();
    let store = MemoryConnectionLifecycleStore::new();
    let admitted = admitted_record(&services, &store);
    let record = snapshot_record(&services, &admitted);
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("codex.work.login"));
    assert!(!snapshot_debug.contains("codex.work.binary-path"));

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the codex catalogue");

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
        .find(|entry| entry.model_id().as_str() == "gpt-fixture-primary")
        .expect("primary row is present");
    assert!(primary.provider_default());

    store
        .put_overlay_marker(
            OverlayMarker::without_provider(
                instance_id(),
                ModelId::new("gpt-fixture-secondary").expect("model id is valid"),
            )
            .with_favourite(true),
        )
        .expect("unmarked overlay marker stores");
    let marked = apply_stored_model_presentation_overlay(&store, &record)
        .expect("instance-plus-model marker applies");
    assert_eq!(marked.selection_readiness(), record.selection_readiness());
    let secondary = marked
        .entries()
        .find(|entry| entry.model_id().as_str() == "gpt-fixture-secondary")
        .expect("secondary row is present");
    assert_eq!(secondary.provider_id(), None);
    assert!(secondary.favourite());

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new("codex").expect("provider id is valid"),
                ModelId::new("gpt-fixture-secondary").expect("model id is valid"),
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
