use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteAvailability, AddableRouteDescriptor,
    AddableRouteId, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind, ConfigFieldRef,
    ConfiguredInstanceId, CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility,
    CredentialRef, FieldLabel, InstanceLabel, IntegrationFamilyId, RouteTopology,
};
use swallowtail_host_local::{JsonFileConnectionLifecycleStore, MemoryConnectionLifecycleStore};
use swallowtail_runtime::{
    AddableRouteCatalog, ConnectionLifecycleStore, InstanceAdmissionRequest, admit_instance,
};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

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

fn installed_available_with_config() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-binary").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Available,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("binary").expect("config id is valid"),
            FieldLabel::new("Binary").expect("label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("endpoint").expect("config id is valid"),
            FieldLabel::new("Endpoint").expect("label is valid"),
            ConfigFieldKind::ApiEndpoint,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("environment").expect("config id is valid"),
            FieldLabel::new("Environment").expect("label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
}

fn catalog() -> AddableRouteCatalog {
    AddableRouteCatalog::from_descriptors([hosted_available(), installed_available_with_config()])
        .expect("catalog assembles")
}

fn family() -> IntegrationFamilyId {
    IntegrationFamilyId::new("fixture-family").expect("family is valid")
}

fn hosted_request(id: &str) -> InstanceAdmissionRequest {
    InstanceAdmissionRequest::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        family(),
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
    )
}

fn installed_request(id: &str) -> InstanceAdmissionRequest {
    InstanceAdmissionRequest::new(
        ConfiguredInstanceId::new(id).expect("instance id is valid"),
        family(),
        AddableRouteId::new("fixture-installed-binary").expect("route id is valid"),
    )
}

fn temporary_root() -> PathBuf {
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "swallowtail-lifecycle-admission-{}-{nanos}-{sequence}",
        std::process::id()
    ));
    std::fs::create_dir_all(&path).expect("fixture root is created");
    path
}

#[test]
fn memory_store_admits_two_instances_of_one_family() {
    let store = MemoryConnectionLifecycleStore::new();
    let catalog = catalog();
    admit_instance(&catalog, &store, hosted_request("work")).expect("admit work");
    admit_instance(&catalog, &store, hosted_request("personal")).expect("admit personal");

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
fn json_file_store_keeps_config_values_behind_opaque_refs() {
    let path = "/host/private/bin/provider";
    let url = "https://example.invalid/v1";
    let env_body = "PROVIDER_TOKEN=secret-bytes";
    let document_path = temporary_root().join("lifecycle.json");
    let store = JsonFileConnectionLifecycleStore::open(&document_path)
        .expect("json store opens on missing path");
    let catalog = catalog();
    let record = admit_instance(
        &catalog,
        &store,
        installed_request("work")
            .with_config_refs([
                (
                    ConfigFieldId::new("binary").expect("config id is valid"),
                    ConfigFieldRef::new("config-ref-binary").expect("config ref is valid"),
                ),
                (
                    ConfigFieldId::new("endpoint").expect("config id is valid"),
                    ConfigFieldRef::new("config-ref-endpoint").expect("config ref is valid"),
                ),
                (
                    ConfigFieldId::new("environment").expect("config id is valid"),
                    ConfigFieldRef::new("config-ref-environment").expect("config ref is valid"),
                ),
            ])
            .with_label(InstanceLabel::new("Work").expect("label is valid")),
    )
    .expect("admit installed instance");

    let document = std::fs::read_to_string(&document_path).expect("json document is readable");
    let debug = format!("{record:?}");
    assert!(document.contains("config-ref-binary"));
    assert!(document.contains("config-ref-endpoint"));
    assert!(document.contains("config-ref-environment"));
    assert!(!document.contains(path));
    assert!(!document.contains(url));
    assert!(!document.contains(env_body));
    assert!(!debug.contains(path));
    assert!(!debug.contains(url));
    assert!(!debug.contains(env_body));
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
}

#[test]
fn hosted_admission_keeps_credential_bytes_out_of_portable_records() {
    let secret = "sk-secret-bytes-xyz";
    let store = MemoryConnectionLifecycleStore::new();
    let catalog = catalog();
    let record = admit_instance(
        &catalog,
        &store,
        hosted_request("work")
            .with_credential_refs([(
                CredentialFieldId::new("api_key").expect("field id is valid"),
                CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
            )])
            .with_config_refs([(
                ConfigFieldId::new("endpoint").expect("config id is valid"),
                ConfigFieldRef::new("config-ref-work").expect("config ref is valid"),
            )]),
    )
    .expect("admit hosted instance");

    let debug = format!("{record:?}");
    assert!(!debug.contains(secret));
    assert!(!debug.contains("https://"));
    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
}
