use super::{
    ConnectionLifecycleStore, ConnectionLifecycleStoreFailure, ReadinessRefreshFailureKind,
    ReadinessRefreshRequest, refresh_readiness,
};
use crate::ConfiguredProviderInstanceSelectionReadiness;
use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, AddableRouteId,
    AdmittedInstanceRecord, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, InstanceEnablement, IntegrationFamilyId, OverlayMarker, RouteTopology,
    RuntimeReadiness, SupportAuthority,
};

struct MemoryStore {
    instances: Mutex<BTreeMap<String, AdmittedInstanceRecord>>,
}

impl MemoryStore {
    fn new() -> Self {
        Self {
            instances: Mutex::new(BTreeMap::new()),
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
        _marker: OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        Ok(())
    }

    fn list_overlay_markers(&self) -> Result<Vec<OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(Vec::new())
    }
}

fn driver() -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new("swallowtail-adapter-fixture-hosted").expect("adapter id is valid"),
        AdapterVersion::new("0.0.0").expect("adapter version is valid"),
    )
}

fn instance(id: &str) -> AdmittedInstanceRecord {
    AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        IntegrationFamilyId::new("fixture-family").expect("family is valid"),
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
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
fn disabled_instance_can_refresh_to_ready_access_dimensions() {
    let store = MemoryStore::new();
    let record = instance("work").with_enablement(InstanceEnablement::Disabled);
    store.put_instance(record.clone()).expect("put instance");
    let snapshot = ConfiguredProviderInstanceSelectionReadiness::NotReady;

    let refreshed = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(record.id().clone(), ready_access()),
    )
    .expect("refresh succeeds");

    assert_eq!(refreshed.enablement(), InstanceEnablement::Disabled);
    assert_eq!(
        refreshed.access_status().map(AccessStatus::credential),
        Some(CredentialState::Ready)
    );
    assert_eq!(
        refreshed.access_status().map(AccessStatus::entitlement),
        Some(EntitlementState::Available)
    );
    assert_eq!(
        refreshed
            .access_status()
            .map(AccessStatus::endpoint_authorization),
        Some(EndpointAuthorization::Allowed)
    );
    assert_eq!(
        refreshed
            .access_status()
            .map(AccessStatus::runtime_readiness),
        Some(RuntimeReadiness::Ready)
    );
    assert_eq!(
        refreshed
            .access_status()
            .map(AccessStatus::support_authority),
        Some(SupportAuthority::ProviderSupported)
    );
    assert_eq!(
        snapshot,
        ConfiguredProviderInstanceSelectionReadiness::NotReady
    );
    assert_eq!(
        store
            .get_instance(record.id())
            .expect("get instance")
            .expect("instance exists"),
        refreshed
    );
}

#[test]
fn enabled_instance_can_refresh_to_not_ready_access_dimensions() {
    let store = MemoryStore::new();
    let record = instance("work")
        .with_enablement(InstanceEnablement::Enabled)
        .with_access_status(ready_access());
    store.put_instance(record.clone()).expect("put instance");
    let snapshot = ConfiguredProviderInstanceSelectionReadiness::Ready;

    let refreshed = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(record.id().clone(), not_ready_access()),
    )
    .expect("refresh succeeds");

    assert_eq!(refreshed.enablement(), InstanceEnablement::Enabled);
    assert_eq!(
        refreshed.access_status().map(AccessStatus::credential),
        Some(CredentialState::Required)
    );
    assert_eq!(
        refreshed
            .access_status()
            .map(AccessStatus::runtime_readiness),
        Some(RuntimeReadiness::Unavailable)
    );
    assert_eq!(
        snapshot,
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
}

#[test]
fn refresh_does_not_probe_unrelated_instances() {
    let store = MemoryStore::new();
    store
        .put_instance(instance("work").with_enablement(InstanceEnablement::Disabled))
        .expect("put work");
    let personal = instance("personal").with_access_status(not_ready_access());
    store.put_instance(personal.clone()).expect("put personal");

    refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(
            ConfiguredInstanceId::new("work").expect("instance id is valid"),
            ready_access(),
        ),
    )
    .expect("refresh work");

    assert_eq!(
        store
            .get_instance(personal.id())
            .expect("get personal")
            .expect("personal exists"),
        personal
    );
}

#[test]
fn absent_instance_cannot_refresh() {
    let store = MemoryStore::new();
    let error = refresh_readiness(
        &store,
        ReadinessRefreshRequest::new(
            ConfiguredInstanceId::new("missing").expect("instance id is valid"),
            ready_access(),
        ),
    )
    .expect_err("absent instance must fail");

    assert_eq!(error.kind(), ReadinessRefreshFailureKind::InstanceAbsent);
    assert!(store.list_instances().expect("list instances").is_empty());
}
