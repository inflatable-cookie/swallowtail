use super::{ConnectionLifecycleStore, ConnectionLifecycleStoreFailure};
use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId,
    AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId, CredentialRef,
    CredentialState, EndpointAuthorization, EntitlementState, InstanceEnablement, InstanceLabel,
    IntegrationFamilyId, ModelId, OverlayMarker, ProviderId, RouteTopology, RuntimeReadiness,
    SupportAuthority,
};

type OverlayKey = (String, Option<String>, String);

struct MemoryStore {
    instances: Mutex<BTreeMap<String, AdmittedInstanceRecord>>,
    overlays: Mutex<BTreeMap<OverlayKey, OverlayMarker>>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            instances: Mutex::new(BTreeMap::new()),
            overlays: Mutex::new(BTreeMap::new()),
        }
    }
}

impl ConnectionLifecycleStore for MemoryStore {
    fn put_instance(
        &self,
        record: AdmittedInstanceRecord,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.instances
            .lock()
            .expect("store lock poisoned")
            .insert(record.id().as_str().to_owned(), record);
        Ok(())
    }

    fn get_instance(
        &self,
        id: &ConfiguredInstanceId,
    ) -> Result<Option<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .get(id.as_str())
            .cloned())
    }

    fn list_instances(
        &self,
    ) -> Result<Vec<AdmittedInstanceRecord>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .instances
            .lock()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect())
    }

    fn put_overlay_marker(
        &self,
        marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        self.overlays.lock().expect("store lock poisoned").insert(
            (
                marker.instance_id().as_str().to_owned(),
                marker
                    .provider_id()
                    .map(|provider| provider.as_str().to_owned()),
                marker.model_id().as_str().to_owned(),
            ),
            marker,
        );
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(self
            .overlays
            .lock()
            .expect("store lock poisoned")
            .values()
            .cloned()
            .collect())
    }
}

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

#[test]
fn put_get_list_round_trip_references_labels_enablement_and_overlay() {
    let store = MemoryStore::new();
    let record = instance("work")
        .with_credential_refs([(
            swallowtail_core::CredentialFieldId::new("api_key").expect("field id is valid"),
            CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
        )])
        .with_config_refs([(
            ConfigFieldId::new("endpoint").expect("config id is valid"),
            ConfigFieldRef::new("config-ref-work").expect("config ref is valid"),
        )])
        .with_enablement(InstanceEnablement::Disabled)
        .with_label(InstanceLabel::new("Work").expect("label is valid"));
    let marker = OverlayMarker::new(
        record.id().clone(),
        ProviderId::new("anthropic").expect("provider id is valid"),
        ModelId::new("claude-opus").expect("model id is valid"),
    )
    .with_favourite(true);

    store.put_instance(record.clone()).expect("put instance");
    store
        .put_overlay_marker(marker.clone())
        .expect("put overlay");

    let stored = store
        .get_instance(record.id())
        .expect("get instance")
        .expect("instance exists");
    assert_eq!(stored, record);
    assert_eq!(
        store.list_instances().expect("list instances"),
        vec![record]
    );
    assert_eq!(
        store.list_overlay_markers().expect("list overlays"),
        vec![marker]
    );
}

#[test]
fn several_instances_of_one_family_remain_distinct_ids() {
    let store = MemoryStore::new();
    store
        .put_instance(instance("work"))
        .expect("put work instance");
    store
        .put_instance(instance("personal"))
        .expect("put personal instance");

    let ids: Vec<_> = store
        .list_instances()
        .expect("list instances")
        .into_iter()
        .map(|record| record.id().as_str().to_owned())
        .collect();
    assert_eq!(ids, vec!["personal".to_owned(), "work".to_owned()]);
    assert_eq!(
        store
            .list_instances()
            .expect("list instances")
            .iter()
            .map(|record| record.family().as_str())
            .collect::<Vec<_>>(),
        vec!["anthropic", "anthropic"]
    );
}

#[test]
fn disabled_instance_can_store_ready_access_dimensions() {
    let store = MemoryStore::new();
    let record = instance("work")
        .with_enablement(InstanceEnablement::Disabled)
        .with_access_status(ready_access());

    store.put_instance(record.clone()).expect("put instance");
    let stored = store
        .get_instance(record.id())
        .expect("get instance")
        .expect("instance exists");

    assert_eq!(stored.enablement(), InstanceEnablement::Disabled);
    assert_eq!(
        stored.access_status().map(AccessStatus::credential),
        Some(CredentialState::Ready)
    );
    assert_eq!(
        stored.access_status().map(AccessStatus::runtime_readiness),
        Some(RuntimeReadiness::Ready)
    );
}

#[test]
fn enabled_instance_can_store_not_ready_access_dimensions() {
    let store = MemoryStore::new();
    let record = instance("work")
        .with_enablement(InstanceEnablement::Enabled)
        .with_access_status(not_ready_access());

    store.put_instance(record.clone()).expect("put instance");
    let stored = store
        .get_instance(record.id())
        .expect("get instance")
        .expect("instance exists");

    assert_eq!(stored.enablement(), InstanceEnablement::Enabled);
    assert_eq!(
        stored.access_status().map(AccessStatus::credential),
        Some(CredentialState::Required)
    );
    assert_eq!(
        stored.access_status().map(AccessStatus::runtime_readiness),
        Some(RuntimeReadiness::Unavailable)
    );
}

#[test]
fn store_trait_round_trips_credential_references_not_secret_bytes() {
    let store = MemoryStore::new();
    let secret = "sk-secret-bytes-xyz";
    let record = instance("work").with_credential_refs([(
        swallowtail_core::CredentialFieldId::new("api_key").expect("field id is valid"),
        CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
    )]);

    store.put_instance(record.clone()).expect("put instance");
    let stored = store
        .get_instance(record.id())
        .expect("get instance")
        .expect("instance exists");
    let debug = format!("{stored:?}");

    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(!debug.contains(secret));
    assert_eq!(
        stored
            .credential_refs()
            .next()
            .map(|(_, reference)| reference.as_host_value()),
        Some("cred-ref-work")
    );
}
