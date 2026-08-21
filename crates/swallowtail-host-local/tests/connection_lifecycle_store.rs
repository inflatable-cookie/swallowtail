use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId,
    AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialFieldId,
    CredentialRef, CredentialState, EndpointAuthorization, EntitlementState, InstanceEnablement,
    InstanceLabel, IntegrationFamilyId, ModelId, OverlayMarker, ProviderId, RouteTopology,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{JsonFileConnectionLifecycleStore, MemoryConnectionLifecycleStore};
use swallowtail_runtime::ConnectionLifecycleStore;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn driver() -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new("swallowtail-adapter-anthropic").expect("adapter id is valid"),
        AdapterVersion::new("0.3.3").expect("adapter version is valid"),
    )
}

fn instance(id: &str) -> AdmittedInstanceRecord {
    AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        IntegrationFamilyId::new("anthropic").expect("family is valid"),
        AddableRouteId::new("anthropic-messages").expect("route id is valid"),
        driver(),
        RouteTopology::Hosted,
    )
}

fn populated(id: &str) -> AdmittedInstanceRecord {
    instance(id)
        .with_credential_refs([(
            CredentialFieldId::new("api_key").expect("field id is valid"),
            CredentialRef::new(format!("cred-ref-{id}")).expect("credential ref is valid"),
        )])
        .with_config_refs([(
            ConfigFieldId::new("endpoint").expect("config id is valid"),
            ConfigFieldRef::new(format!("config-ref-{id}")).expect("config ref is valid"),
        )])
        .with_enablement(InstanceEnablement::Disabled)
        .with_label(InstanceLabel::new(id).expect("label is valid"))
        .with_access_status(AccessStatus::new(
            AccessProfileId::new("access").expect("access id is valid"),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ))
}

fn marker(instance_id: &str) -> OverlayMarker {
    OverlayMarker::new(
        ConfiguredInstanceId::new(instance_id).expect("instance id is valid"),
        ProviderId::new("anthropic").expect("provider id is valid"),
        ModelId::new("claude-opus").expect("model id is valid"),
    )
    .with_favourite(true)
}

fn unmarked_marker(instance_id: &str) -> OverlayMarker {
    OverlayMarker::without_provider(
        ConfiguredInstanceId::new(instance_id).expect("instance id is valid"),
        ModelId::new("gpt-fixture").expect("model id is valid"),
    )
    .with_favourite(true)
}

fn temporary_root() -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-lifecycle-store-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture root is created");
    path
}

fn round_trip_family_instances(store: &impl ConnectionLifecycleStore) {
    store
        .put_instance(populated("work"))
        .expect("put work instance");
    store
        .put_instance(populated("personal"))
        .expect("put personal instance");
    store
        .put_overlay_marker(marker("work"))
        .expect("put overlay");

    let ids: Vec<_> = store
        .list_instances()
        .expect("list instances")
        .into_iter()
        .map(|record| record.id().as_str().to_owned())
        .collect();
    assert_eq!(ids, vec!["personal".to_owned(), "work".to_owned()]);
    assert_eq!(
        store
            .get_instance(&ConfiguredInstanceId::new("work").expect("instance id is valid"))
            .expect("get work")
            .expect("work exists"),
        populated("work")
    );
    assert_eq!(
        store.list_overlay_markers().expect("list overlays").len(),
        1
    );
}

#[test]
fn memory_adapter_round_trips_distinct_family_instances() {
    round_trip_family_instances(&MemoryConnectionLifecycleStore::new());
}

#[test]
fn json_file_adapter_round_trips_distinct_family_instances_without_secret_bytes() {
    let secret = "sk-secret-bytes-xyz";
    let path = temporary_root().join("lifecycle.json");
    {
        let store = JsonFileConnectionLifecycleStore::open(&path)
            .expect("json store opens on missing path");
        round_trip_family_instances(&store);
    }

    let reopened =
        JsonFileConnectionLifecycleStore::open(&path).expect("json store reopens existing path");
    let persisted_ids: Vec<_> = reopened
        .list_instances()
        .expect("list persisted instances")
        .into_iter()
        .map(|record| record.id().as_str().to_owned())
        .collect();
    assert_eq!(
        persisted_ids,
        vec!["personal".to_owned(), "work".to_owned()]
    );
    assert_eq!(
        reopened
            .get_instance(&ConfiguredInstanceId::new("work").expect("instance id is valid"))
            .expect("get persisted work")
            .expect("work exists"),
        populated("work")
    );
    let document = std::fs::read_to_string(&path).expect("json document is readable");
    assert!(document.contains("cred-ref-work"));
    assert!(document.contains("config-ref-work"));
    assert!(document.contains("\"enablement\": \"disabled\""));
    assert!(!document.contains(secret));
    assert!(!document.contains("secret_bytes"));
    assert!(!document.contains("expose_secret"));
}

#[test]
fn json_file_adapter_round_trips_unmarked_overlay_markers() {
    let path = temporary_root().join("unmarked-overlay.json");
    {
        let store = JsonFileConnectionLifecycleStore::open(&path)
            .expect("json store opens on missing path");
        store
            .put_overlay_marker(unmarked_marker("work"))
            .expect("put unmarked overlay");
    }

    let reopened =
        JsonFileConnectionLifecycleStore::open(&path).expect("json store reopens existing path");
    let markers = reopened.list_overlay_markers().expect("list overlays");
    assert_eq!(markers.len(), 1);
    assert_eq!(markers[0].provider_id(), None);
    assert_eq!(markers[0].model_id().as_str(), "gpt-fixture");
    assert!(markers[0].favourite());
    let document = std::fs::read_to_string(&path).expect("json document is readable");
    assert!(!document.contains("provider_id"));
}
