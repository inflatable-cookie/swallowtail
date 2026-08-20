//! Contract 057 lifecycle proof for hosted Anthropic Messages: admission,
//! API-key collection, readiness refresh, subject observation, and the 047
//! snapshot plus model-presentation overlay.
//!
//! Deterministic harness only: no live provider calls, no browser ports, no
//! secret bytes in portable records.

mod support;

use std::sync::Arc;
use support::ThreadServices;
use swallowtail_adapter_anthropic::{
    ANTHROPIC_MESSAGES_API_KEY_FIELD_ID, ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID,
    AnthropicPreparationInput, anthropic_direct_descriptor,
    anthropic_messages_addable_route_descriptor, prepare_anthropic_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus,
    AuthenticatedSubjectObservation, Capability, CapabilityRequirement, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, InstanceEnablement, InstanceRevision, InstanceTargetRef,
    IntegrationFamilyId, ModelCatalogEntry, ModelId, ModelMetadata, OperationRequirements,
    OperationShape, OverlayMarker, PreflightContext, ProviderId, RuntimeReadiness,
    SubjectDisclosure, SupportAuthority, preflight,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord,
    ConfiguredProviderInstanceSelectionReadiness, ConfiguredProviderModelCatalogueInput,
    ConnectionLifecycleStore, CredentialService, HostServices, InstanceAdmissionRequest,
    PreparedAccessEvidence, PreparedOperationEvidence, ReadinessRefreshRequest, ScopeId,
    SignInAuthorityBinding, SignInMethod, SignInStartRequest, SignInStatus, admit_instance,
    apply_stored_model_presentation_overlay, complete_sign_in, observe_authenticated_subject,
    poll_sign_in, refresh_readiness, start_sign_in, submit_sign_in_credential_field,
};
use swallowtail_runtime::{BlockingWorkService, ScopedTaskService, TimeService};

const INSTANCE: &str = "anthropic.work";
const CREDENTIAL_REF: &str = "anthropic.work.api-key";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("anthropic.admission.host").expect("host id is valid")
}

fn services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("anthropic").expect("family id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = anthropic_messages_addable_route_descriptor(services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), descriptor_route_id(&catalog))
            .with_config_refs([(
                ConfigFieldId::new(ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("anthropic.work.endpoint").expect("config ref is valid"),
            )]),
    )
    .expect("admission succeeds")
}

fn descriptor_route_id(catalog: &AddableRouteCatalog) -> swallowtail_core::AddableRouteId {
    catalog
        .routes()
        .next()
        .expect("catalog has the anthropic route")
        .id()
        .clone()
}

fn access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("anthropic.work.access").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("api.anthropic.com").expect("audience is valid"),
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
    assert_eq!(record.family().as_str(), "anthropic");
    assert_eq!(record.route_id().as_str(), "anthropic.messages");
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
    let descriptor = anthropic_messages_addable_route_descriptor(&services);
    let fields: Vec<_> = descriptor.credential_fields().cloned().collect();

    let mut session = start_sign_in(
        &services,
        SignInStartRequest::new(
            ScopeId::new("anthropic.admission.sign-in:work").expect("scope is valid"),
            admitted.id().clone(),
            family(),
            admitted.route_id().clone(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                EndpointAudience::new("api.anthropic.com").expect("audience is valid"),
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
        swallowtail_core::CredentialFieldId::new(ANTHROPIC_MESSAGES_API_KEY_FIELD_ID)
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
        swallowtail_core::CredentialFieldId::new(ANTHROPIC_MESSAGES_API_KEY_FIELD_ID)
            .expect("credential id is valid"),
        CredentialRef::new(CREDENTIAL_REF).expect("credential ref is valid"),
    )]);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let prepared = prepare_anthropic_direct(
        AnthropicPreparationInput::from_admitted(
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

fn prepared_route_evidence(
    services: &HostServices,
    driver: &swallowtail_core::DriverDescriptor,
    instance: &swallowtail_core::ConfiguredInstance,
    profile: &AccessProfile,
    evidence: &PreparedAccessEvidence,
) -> PreparedOperationEvidence {
    let status = evidence.status();
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
    .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])]);
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
    .with_provider_id(ProviderId::new("anthropic").expect("provider id is valid"))
}

fn prepared_services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    let thread = Arc::new(ThreadServices::new());
    HostServices::new(host_id())
        .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
        .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
        .with_time(thread as Arc<dyn TimeService>)
        .with_network(Arc::new(host.clone()) as Arc<dyn swallowtail_runtime::NetworkPolicyService>)
        .with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn snapshot_record(services: &HostServices) -> ConfiguredProviderInstanceRecord {
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);
    let prepared = prepare_anthropic_direct(
        AnthropicPreparationInput::new(
            instance_id(),
            InstanceRevision::new("1").expect("revision is valid"),
            host_id(),
            InstanceTargetRef::new("anthropic.work.endpoint").expect("target is valid"),
            profile.clone(),
            evidence.clone(),
        ),
        services,
    )
    .expect("instance prepares");
    let driver = anthropic_direct_descriptor();
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
                catalogue_entry("claude-fixture-primary", true),
                catalogue_entry("claude-fixture-secondary", false),
            ],
        )),
    )
    .expect("047 snapshot assembles")
}

#[test]
fn refresh_writes_host_supplied_access_status_without_touching_enablement() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    let descriptor = anthropic_messages_addable_route_descriptor(&services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    let admitted = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(
            instance_id(),
            family(),
            swallowtail_core::AddableRouteId::new("anthropic.messages").expect("route id is valid"),
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
fn subject_stays_absent_for_anthropic_messages() {
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
fn overlay_marks_anthropic_catalogue_rows_without_changing_readiness() {
    let services = services();
    let store = MemoryConnectionLifecycleStore::new();
    admitted_record(&services, &store);
    let record = snapshot_record(&prepared_services());
    assert_eq!(
        record.selection_readiness(),
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    let snapshot_debug = format!("{record:?}");
    assert!(!snapshot_debug.contains('@'));
    assert!(!snapshot_debug.contains("sk-"));

    store
        .put_overlay_marker(
            OverlayMarker::new(
                instance_id(),
                ProviderId::new("anthropic").expect("provider id is valid"),
                ModelId::new("claude-fixture-secondary").expect("model id is valid"),
            )
            .with_favourite(true)
            .with_ordinal(Some(0)),
        )
        .expect("overlay marker stores");

    let overlay = apply_stored_model_presentation_overlay(&store, &record)
        .expect("overlay projects onto the anthropic catalogue");

    assert_eq!(overlay.selection_readiness(), record.selection_readiness());
    assert_eq!(overlay.instance_id(), &instance_id());
    let entries: Vec<_> = overlay.entries().collect();
    assert_eq!(entries.len(), 2);
    let secondary = entries
        .iter()
        .find(|entry| entry.model_id().as_str() == "claude-fixture-secondary")
        .expect("secondary row is present");
    assert!(secondary.favourite());
    assert_eq!(secondary.ordinal(), Some(0));
    assert!(!secondary.provider_default());
    let primary = entries
        .iter()
        .find(|entry| entry.model_id().as_str() == "claude-fixture-primary")
        .expect("primary row is present");
    assert!(primary.provider_default());
    assert!(!primary.consumer_default());
    assert!(!primary.hidden());
}
