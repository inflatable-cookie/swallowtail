use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion,
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId, ConfiguredInstanceId,
    CredentialState, EndpointAuthorization, EntitlementState, InstanceEnablement,
    IntegrationFamilyId, RouteTopology, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{JsonFileConnectionLifecycleStore, MemoryConnectionLifecycleStore};
use swallowtail_runtime::{
    AddableRouteCatalog, ConnectionLifecycleStore, InstanceAdmissionRequest,
    ReadinessRefreshRequest, admit_instance, refresh_readiness,
};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn fixture_driver() -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new("swallowtail-adapter-fixture-hosted").expect("fixture adapter id is valid"),
        AdapterVersion::new("0.0.0").expect("fixture adapter version is valid"),
    )
}

fn catalog() -> AddableRouteCatalog {
    AddableRouteCatalog::from_descriptors([AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
        fixture_driver(),
        RouteTopology::Hosted,
        AddableRouteAvailability::Available,
    )])
    .expect("catalog assembles")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("fixture-family").expect("family is valid")
}

fn request(id: &str) -> InstanceAdmissionRequest {
    InstanceAdmissionRequest::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        family(),
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
    )
}

fn ready_access() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("access").expect("access id is valid"),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn not_ready_access() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("access").expect("access id is valid"),
        CredentialState::Required,
        EntitlementState::Unavailable,
        EndpointAuthorization::Denied,
        RuntimeReadiness::Unavailable,
        SupportAuthority::ExperimentalObserved,
    )
}

fn temporary_root() -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-lifecycle-refresh-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture root is created");
    path
}

fn refresh_keeps_enablement(store: &impl ConnectionLifecycleStore) {
    let catalog = catalog();
    let disabled = admit_instance(
        &catalog,
        store,
        request("work").with_enablement(InstanceEnablement::Disabled),
    )
    .expect("admit disabled instance");
    let enabled = admit_instance(
        &catalog,
        store,
        request("personal").with_enablement(InstanceEnablement::Enabled),
    )
    .expect("admit enabled instance");

    let ready = refresh_readiness(
        store,
        ReadinessRefreshRequest::new(disabled.id().clone(), ready_access()),
    )
    .expect("refresh disabled instance");
    let not_ready = refresh_readiness(
        store,
        ReadinessRefreshRequest::new(enabled.id().clone(), not_ready_access()),
    )
    .expect("refresh enabled instance");

    assert_eq!(ready.enablement(), InstanceEnablement::Disabled);
    assert_eq!(
        ready.access_status().map(AccessStatus::credential),
        Some(CredentialState::Ready)
    );
    assert_eq!(not_ready.enablement(), InstanceEnablement::Enabled);
    assert_eq!(
        not_ready
            .access_status()
            .map(AccessStatus::runtime_readiness),
        Some(RuntimeReadiness::Unavailable)
    );
}

#[test]
fn memory_store_refresh_leaves_enablement_unchanged() {
    refresh_keeps_enablement(&MemoryConnectionLifecycleStore::new());
}

#[test]
fn json_file_store_persists_refreshed_access_status_without_047() {
    let document_path = temporary_root().join("lifecycle.json");
    {
        let store = JsonFileConnectionLifecycleStore::open(&document_path)
            .expect("json store opens on missing path");
        refresh_keeps_enablement(&store);
    }

    let reopened =
        JsonFileConnectionLifecycleStore::open(&document_path).expect("json store reopens");
    let work = reopened
        .get_instance(&ConfiguredInstanceId::new("work").expect("instance id is valid"))
        .expect("get work")
        .expect("work exists");
    assert_eq!(work.enablement(), InstanceEnablement::Disabled);
    assert_eq!(
        work.access_status().map(AccessStatus::credential),
        Some(CredentialState::Ready)
    );
    let document = std::fs::read_to_string(&document_path).expect("json document is readable");
    assert!(document.contains("\"credential\": \"ready\""));
    assert!(document.contains("\"enablement\": \"disabled\""));
    assert!(!document.contains("Ready"));
    assert!(!document.contains("NotReady"));
    assert!(!document.contains("email"));
}
