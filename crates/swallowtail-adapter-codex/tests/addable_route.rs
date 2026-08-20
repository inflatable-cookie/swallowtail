//! Contract 057 addable-route descriptor tests for installed Codex app-server.

use crate::support::{FakeProcessService, host_services_for};
use swallowtail_adapter_codex::{
    CODEX_APP_SERVER_ADDABLE_ROUTE_ID, CODEX_APP_SERVER_BINARY_PATH_FIELD_ID,
    CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID, codex_app_server_addable_route_descriptor,
    codex_app_server_descriptor,
};
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteMissingRequirement, ConfigFieldId, ConfigFieldKind,
    DiscoveryAction, ExecutionHostId, ExecutionLayer, RouteTopology,
};
use swallowtail_runtime::{AddableRouteCatalog, HostServices};

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("codex.addable.host").expect("host id is valid")
}

fn services_with_process() -> HostServices {
    let (process, _) = FakeProcessService::held_open();
    host_services_for(host_id(), process)
}

#[test]
fn descriptor_is_installed_and_matches_the_app_server_driver() {
    let descriptor = codex_app_server_addable_route_descriptor(&services_with_process());

    assert_eq!(descriptor.id().as_str(), CODEX_APP_SERVER_ADDABLE_ROUTE_ID);
    assert_eq!(descriptor.id().as_str(), "codex.app-server");
    assert_eq!(descriptor.topology(), RouteTopology::Installed);
    assert_eq!(
        descriptor.driver(),
        codex_app_server_descriptor().identity()
    );
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
    assert_eq!(descriptor.sign_in_actions().len(), 0);
}

#[test]
fn missing_process_service_marks_the_route_unavailable() {
    let services = HostServices::new(host_id());
    let descriptor = codex_app_server_addable_route_descriptor(&services);

    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
}

#[test]
fn chatgpt_subscription_path_advertises_no_secret_credential_field() {
    let descriptor = codex_app_server_addable_route_descriptor(&services_with_process());

    assert_eq!(descriptor.credential_fields().len(), 0);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("sk-"));
    assert!(!debug.contains("token"));
}

#[test]
fn config_fields_are_opaque_references_not_paths_or_env_bodies() {
    let descriptor = codex_app_server_addable_route_descriptor(&services_with_process());
    let binary_path = descriptor
        .config_field(
            &ConfigFieldId::new(CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
                .expect("config id is valid"),
        )
        .expect("binary path field is advertised");
    let environment = descriptor
        .config_field(
            &ConfigFieldId::new(CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
                .expect("config id is valid"),
        )
        .expect("environment field is advertised");

    assert_eq!(binary_path.kind(), ConfigFieldKind::BinaryPath);
    assert_eq!(environment.kind(), ConfigFieldKind::Environment);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains('/'));
    assert!(!debug.contains('='));
}

#[test]
fn harness_execution_layer_is_unchanged() {
    assert!(
        codex_app_server_descriptor().supports_execution_layer(ExecutionLayer::HarnessInteraction)
    );
    assert!(
        !codex_app_server_descriptor()
            .supports_execution_layer(ExecutionLayer::DirectModelInference)
    );
}

#[test]
fn discovery_candidates_are_not_catalog_rows() {
    let descriptor = codex_app_server_addable_route_descriptor(&services_with_process());
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");

    assert_eq!(catalog.routes().len(), 1);
    assert!(
        codex_app_server_descriptor()
            .discovery_actions()
            .eq([DiscoveryAction::Probe])
    );
}
