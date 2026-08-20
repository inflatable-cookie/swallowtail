use super::{
    AddableRouteCatalog, ConnectionLifecycleStore, ConnectionLifecycleStoreFailure,
    InstanceAdmissionFailureKind, InstanceAdmissionRequest, admit_instance,
};
use crate::ConfiguredProviderInstanceSelectionReadiness;
use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteAvailability, AddableRouteDescriptor,
    AddableRouteId, AddableRouteMissingRequirement, AdmittedInstanceRecord, ConfigFieldDescriptor,
    ConfigFieldId, ConfigFieldKind, ConfigFieldRef, ConfiguredInstanceId,
    CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility, CredentialRef,
    DiscoveryOutcome, DiscoveryStatus, FieldLabel, InstanceEnablement, InstanceLabel,
    IntegrationFamilyId, RouteTopology,
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
        _marker: swallowtail_core::OverlayMarker,
    ) -> Result<(), ConnectionLifecycleStoreFailure> {
        Ok(())
    }

    fn list_overlay_markers(
        &self,
    ) -> Result<Vec<swallowtail_core::OverlayMarker>, ConnectionLifecycleStoreFailure> {
        Ok(Vec::new())
    }
}

fn fixture_driver(adapter: &str) -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new(adapter).expect("fixture adapter id is valid"),
        AdapterVersion::new("0.0.0").expect("fixture adapter version is valid"),
    )
}

fn hosted_available() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-hosted"),
        RouteTopology::Hosted,
        AddableRouteAvailability::Available,
    )
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new("api_key").expect("field id is valid"),
        FieldLabel::new("API key").expect("label is valid"),
        CredentialFieldVisibility::Secret,
    )])
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new("endpoint").expect("config id is valid"),
        FieldLabel::new("Endpoint").expect("label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}

fn installed_missing_install() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-harness").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::Install),
    )
}

fn installed_unsupported() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-unsupported").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unsupported,
    )
}

fn catalog() -> AddableRouteCatalog {
    AddableRouteCatalog::from_descriptors([
        hosted_available(),
        installed_missing_install(),
        installed_unsupported(),
    ])
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

#[test]
fn admission_writes_a_configured_instance_through_the_store() {
    let store = MemoryStore::new();
    let catalog = catalog();
    let record = admit_instance(&catalog, &store, request("work")).expect("admission succeeds");

    assert_eq!(record.id().as_str(), "work");
    assert_eq!(record.family().as_str(), "fixture-family");
    assert_eq!(record.route_id().as_str(), "fixture-hosted-messages");
    assert_eq!(record.enablement(), InstanceEnablement::Enabled);
    assert_eq!(record.access_status(), None);
    assert_eq!(
        store
            .get_instance(record.id())
            .expect("get instance")
            .expect("instance exists"),
        record
    );
}

#[test]
fn two_instances_of_one_family_remain_distinct_ids() {
    let store = MemoryStore::new();
    let catalog = catalog();
    admit_instance(&catalog, &store, request("work")).expect("admit work");
    admit_instance(&catalog, &store, request("personal")).expect("admit personal");

    let ids: Vec<_> = store
        .list_instances()
        .expect("list instances")
        .into_iter()
        .map(|record| record.id().as_str().to_owned())
        .collect();
    assert_eq!(ids, vec!["personal".to_owned(), "work".to_owned()]);
    assert!(
        store
            .list_instances()
            .expect("list instances")
            .iter()
            .all(|record| record.family().as_str() == "fixture-family")
    );
}

#[test]
fn discovered_candidate_cannot_be_admitted() {
    let discovery = DiscoveryOutcome::new(DiscoveryStatus::Discovered, None);
    let store = MemoryStore::new();
    let catalog = catalog();
    let error = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(
            ConfiguredInstanceId::new("work").expect("instance id is valid"),
            family(),
            AddableRouteId::new("discovered-candidate").expect("route id is valid"),
        ),
    )
    .expect_err("discovery ids are not addable routes");

    assert_eq!(discovery.status(), DiscoveryStatus::Discovered);
    assert_eq!(error.kind(), InstanceAdmissionFailureKind::RouteAbsent);
    assert!(store.list_instances().expect("list instances").is_empty());
}

#[test]
fn unavailable_and_unsupported_routes_cannot_be_admitted() {
    let store = MemoryStore::new();
    let catalog = catalog();
    let unavailable = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(
            ConfiguredInstanceId::new("work").expect("instance id is valid"),
            family(),
            AddableRouteId::new("fixture-installed-harness").expect("route id is valid"),
        ),
    )
    .expect_err("unavailable route cannot admit");
    let unsupported = admit_instance(
        &catalog,
        &store,
        InstanceAdmissionRequest::new(
            ConfiguredInstanceId::new("work").expect("instance id is valid"),
            family(),
            AddableRouteId::new("fixture-installed-unsupported").expect("route id is valid"),
        ),
    )
    .expect_err("unsupported route cannot admit");

    assert_eq!(
        unavailable.kind(),
        InstanceAdmissionFailureKind::RouteUnavailable
    );
    assert_eq!(
        unsupported.kind(),
        InstanceAdmissionFailureKind::RouteUnsupported
    );
    assert!(store.list_instances().expect("list instances").is_empty());
}

#[test]
fn admission_does_not_change_047_selection_readiness() {
    let store = MemoryStore::new();
    let catalog = catalog();
    let readiness = ConfiguredProviderInstanceSelectionReadiness::Ready;

    let record = admit_instance(&catalog, &store, request("work")).expect("admission succeeds");

    assert_eq!(
        readiness,
        ConfiguredProviderInstanceSelectionReadiness::Ready
    );
    assert_eq!(record.access_status(), None);
}

#[test]
fn admission_attaches_host_private_config_and_credential_refs() {
    let store = MemoryStore::new();
    let catalog = catalog();
    let path = "/host/private/bin/provider";
    let url = "https://example.invalid/v1";
    let env_body = "PROVIDER_TOKEN=secret-bytes";
    let record = admit_instance(
        &catalog,
        &store,
        request("work")
            .with_credential_refs([(
                CredentialFieldId::new("api_key").expect("field id is valid"),
                CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
            )])
            .with_config_refs([(
                ConfigFieldId::new("endpoint").expect("config id is valid"),
                ConfigFieldRef::new("config-ref-work").expect("config ref is valid"),
            )])
            .with_label(InstanceLabel::new("Work").expect("label is valid")),
    )
    .expect("admission succeeds");

    let debug = format!("{record:?}");
    assert!(!debug.contains(path));
    assert!(!debug.contains(url));
    assert!(!debug.contains(env_body));
    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
    assert_eq!(
        record
            .config_refs()
            .next()
            .map(|(id, reference)| (id.as_str(), reference.as_host_value())),
        Some(("endpoint", "config-ref-work"))
    );
}

#[test]
fn unknown_config_and_credential_fields_fail_closed() {
    let store = MemoryStore::new();
    let catalog = catalog();
    let unknown_credential = admit_instance(
        &catalog,
        &store,
        request("work").with_credential_refs([(
            CredentialFieldId::new("other_secret").expect("field id is valid"),
            CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
        )]),
    )
    .expect_err("unknown credential field must fail");
    let unknown_config = admit_instance(
        &catalog,
        &store,
        request("work").with_config_refs([(
            ConfigFieldId::new("binary").expect("config id is valid"),
            ConfigFieldRef::new("config-ref-work").expect("config ref is valid"),
        )]),
    )
    .expect_err("unknown config field must fail");

    assert_eq!(
        unknown_credential.kind(),
        InstanceAdmissionFailureKind::UnknownCredentialField
    );
    assert_eq!(
        unknown_config.kind(),
        InstanceAdmissionFailureKind::UnknownConfigField
    );
    assert!(store.list_instances().expect("list instances").is_empty());
}
