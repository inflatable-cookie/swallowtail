//! Contract 057 addable-route descriptor tests for installed Claude Agent ACP.

use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID, CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID,
    CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID, claude_agent_acp_addable_route_descriptor,
    claude_agent_acp_descriptor, claude_code_headless_descriptor,
    claude_code_response_only_descriptor,
};
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteMissingRequirement, ConfigFieldId, ConfigFieldKind,
    DiscoveryAction, ExecutionHostId, ExecutionLayer, RouteTopology,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{AddableRouteCatalog, HostServices, ProcessService};

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("claude-agent.addable.host").expect("host id is valid")
}

fn services_with_process() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_process(Arc::new(host) as Arc<dyn ProcessService>)
}

#[test]
fn descriptor_is_installed_and_matches_the_acp_driver() {
    let descriptor = claude_agent_acp_addable_route_descriptor(&services_with_process());

    assert_eq!(descriptor.id().as_str(), CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID);
    assert_eq!(descriptor.id().as_str(), "claude-agent.acp");
    assert_eq!(descriptor.topology(), RouteTopology::Installed);
    assert_eq!(
        descriptor.driver(),
        claude_agent_acp_descriptor().identity()
    );
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
    assert_eq!(descriptor.sign_in_actions().len(), 0);
    assert_ne!(
        descriptor.driver(),
        claude_code_headless_descriptor().identity()
    );
    assert_ne!(
        descriptor.driver(),
        claude_code_response_only_descriptor().identity()
    );
}

#[test]
fn missing_process_service_marks_the_route_unavailable() {
    let services = HostServices::new(host_id());
    let descriptor = claude_agent_acp_addable_route_descriptor(&services);

    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
}

#[test]
fn subscription_path_advertises_no_secret_credential_field() {
    let descriptor = claude_agent_acp_addable_route_descriptor(&services_with_process());

    assert_eq!(descriptor.credential_fields().len(), 0);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("sk-"));
    assert!(!debug.contains("token"));
    assert!(!debug.contains("keychain"));
}

#[test]
fn config_fields_are_opaque_references_not_paths_or_env_bodies() {
    let descriptor = claude_agent_acp_addable_route_descriptor(&services_with_process());
    let binary_path = descriptor
        .config_field(
            &ConfigFieldId::new(CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID).expect("config id is valid"),
        )
        .expect("binary path field is advertised");
    let environment = descriptor
        .config_field(
            &ConfigFieldId::new(CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID).expect("config id is valid"),
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
        claude_agent_acp_descriptor().supports_execution_layer(ExecutionLayer::HarnessInteraction)
    );
    assert!(
        !claude_agent_acp_descriptor()
            .supports_execution_layer(ExecutionLayer::DirectModelInference)
    );
}

#[test]
fn discovery_candidates_are_not_catalog_rows() {
    let descriptor = claude_agent_acp_addable_route_descriptor(&services_with_process());
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");

    assert_eq!(catalog.routes().len(), 1);
    assert!(
        claude_agent_acp_descriptor()
            .discovery_actions()
            .eq([DiscoveryAction::Probe])
    );
}
