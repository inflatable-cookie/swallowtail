//! Contract 057 lifecycle proof for hosted DeepSeek continuation: admission,
//! API-key collection, readiness refresh, subject observation, and the 047
//! snapshot plus model-presentation overlay.
//!
//! Deterministic harness only: no live provider calls, no browser ports, no
//! secret bytes in portable records.

use std::sync::Arc;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID, DEEPSEEK_ENDPOINT, DEEPSEEK_ENDPOINT_AUDIENCE,
    DeepSeekPreparationInput, deepseek_continuation_addable_route_descriptor,
    prepare_deepseek_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, ConnectionLifecycleStore, CredentialService, HostServices,
    InstanceAdmissionRequest, PreparedAccessEvidence, ScopeId, SignInAuthorityBinding,
    SignInMethod, SignInStatus, admit_instance, complete_sign_in, poll_sign_in, start_sign_in,
    submit_sign_in_credential_field,
};

const INSTANCE: &str = "deepseek.work";
const CREDENTIAL_REF: &str = "deepseek.work.api-key";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("deepseek.admission.host").expect("host id is valid")
}

fn services() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("deepseek").expect("family id is valid")
}

fn instance_id() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new(INSTANCE).expect("instance id is valid")
}

fn admitted_record(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
) -> swallowtail_core::AdmittedInstanceRecord {
    let descriptor = deepseek_continuation_addable_route_descriptor(services);
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(instance_id(), family(), descriptor_route_id(&catalog)),
    )
    .expect("admission succeeds")
}

fn descriptor_route_id(catalog: &AddableRouteCatalog) -> swallowtail_core::AddableRouteId {
    catalog
        .routes()
        .next()
        .expect("catalog has the deepseek route")
        .id()
        .clone()
}

fn access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("deepseek.work.access").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(DEEPSEEK_ENDPOINT_AUDIENCE).expect("audience is valid"),
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
    assert_eq!(record.family().as_str(), "deepseek");
    assert_eq!(record.route_id().as_str(), "deepseek.continuation");
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
    let descriptor = deepseek_continuation_addable_route_descriptor(&services);
    let fields: Vec<_> = descriptor.credential_fields().cloned().collect();

    let mut session = start_sign_in(
        &services,
        swallowtail_runtime::SignInStartRequest::new(
            ScopeId::new("deepseek.admission.sign-in:work").expect("scope is valid"),
            admitted.id().clone(),
            family(),
            admitted.route_id().clone(),
            SignInAuthorityBinding::new(
                CredentialMechanism::ApiKey,
                EndpointAudience::new(DEEPSEEK_ENDPOINT_AUDIENCE).expect("audience is valid"),
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
        swallowtail_core::CredentialFieldId::new(DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID)
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
    let admitted = admitted_record(&services, &store);
    let profile = access_profile(CredentialRef::new(CREDENTIAL_REF).expect("ref is valid"));
    let evidence = ready_evidence(&profile);

    let prepared = prepare_deepseek_direct(
        DeepSeekPreparationInput::new(
            admitted.id().clone(),
            InstanceRevision::new("1").expect("revision is valid"),
            host_id(),
            InstanceTargetRef::new(DEEPSEEK_ENDPOINT).expect("target is valid"),
            profile.clone(),
            evidence,
        ),
        &services,
    )
    .expect("admitted instance still prepares");

    assert_eq!(prepared.instance().id(), admitted.id());
    assert_eq!(prepared.access_profile(), &profile);
}
