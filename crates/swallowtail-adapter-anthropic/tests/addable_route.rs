//! Contract 057 addable-route descriptor tests for hosted Anthropic Messages.

use std::sync::Arc;
use swallowtail_adapter_anthropic::{
    ANTHROPIC_MESSAGES_ADDABLE_ROUTE_ID, ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME,
    ANTHROPIC_MESSAGES_API_KEY_FIELD_ID, ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID,
    anthropic_direct_descriptor, anthropic_messages_addable_route_descriptor,
};
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteMissingRequirement, ConfigFieldKind,
    CredentialFieldVisibility, ExecutionHostId, ExecutionLayer, RouteTopology,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{CredentialService, HostServices};

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("anthropic.addable.host").expect("host id is valid")
}

fn services_with_credential() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_credential(Arc::new(host) as Arc<dyn CredentialService>)
}

#[test]
fn descriptor_is_hosted_and_matches_the_direct_driver() {
    let descriptor = anthropic_messages_addable_route_descriptor(&services_with_credential());

    assert_eq!(
        descriptor.id().as_str(),
        ANTHROPIC_MESSAGES_ADDABLE_ROUTE_ID
    );
    assert_eq!(descriptor.id().as_str(), "anthropic.messages");
    assert_eq!(descriptor.topology(), RouteTopology::Hosted);
    assert_eq!(
        descriptor.driver(),
        anthropic_direct_descriptor().identity()
    );
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
    assert_eq!(descriptor.sign_in_actions().len(), 0);
}

#[test]
fn missing_credential_service_marks_the_route_unavailable() {
    let services = HostServices::new(host_id());
    let descriptor = anthropic_messages_addable_route_descriptor(&services);

    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
}

#[test]
fn credential_field_is_secret_and_carries_no_secret_bytes() {
    let descriptor = anthropic_messages_addable_route_descriptor(&services_with_credential());
    let field = descriptor
        .credential_field(
            &swallowtail_core::CredentialFieldId::new(ANTHROPIC_MESSAGES_API_KEY_FIELD_ID)
                .expect("field id is valid"),
        )
        .expect("api key field is advertised");

    assert_eq!(field.visibility(), CredentialFieldVisibility::Secret);
    assert_eq!(
        field.environment_name().map(|name| name.as_str()),
        Some(ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME)
    );
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("sk-"));
    assert!(debug.contains(ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME));
}

#[test]
fn endpoint_config_is_an_opaque_field_not_a_url() {
    let descriptor = anthropic_messages_addable_route_descriptor(&services_with_credential());
    let field = descriptor
        .config_field(
            &swallowtail_core::ConfigFieldId::new(ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID)
                .expect("config id is valid"),
        )
        .expect("endpoint field is advertised");

    assert_eq!(field.kind(), ConfigFieldKind::ApiEndpoint);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("https://"));
    assert!(!debug.contains("api.anthropic.com"));
}

#[test]
fn direct_execution_layer_is_unchanged() {
    assert!(
        anthropic_direct_descriptor()
            .supports_execution_layer(ExecutionLayer::DirectModelInference)
    );
    assert!(
        !anthropic_direct_descriptor().supports_execution_layer(ExecutionLayer::HarnessInteraction)
    );
}
