use super::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, AdmittedInstanceRecord, AuthenticatedSubjectObservation,
    ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind, CredentialFieldDescriptor,
    CredentialFieldId, CredentialFieldVisibility, EnvironmentVariableName, FieldLabel,
    InstanceEnablement, InstanceLabel, OverlayMarker, RouteTopology, SubjectDisclosure,
};
use crate::{
    AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, ConfigFieldRef,
    ConfiguredInstanceId, CredentialRef, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionLayer, IntegrationFamilyId, ModelId, PlannedConnectionRolloverPolicy, ProviderId,
    RuntimeReadiness, SignInAction, SupportAuthority,
};

fn driver() -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new("swallowtail-adapter-anthropic").expect("adapter id is valid"),
        AdapterVersion::new("0.3.3").expect("adapter version is valid"),
    )
}

#[test]
fn topology_is_not_an_execution_layer_alias() {
    let topology = RouteTopology::Hosted;
    let layer = ExecutionLayer::HarnessInteraction;

    assert_ne!(format!("{topology:?}"), format!("{layer:?}"));
    assert_eq!(RouteTopology::Installed, RouteTopology::Installed);
    assert_eq!(RouteTopology::LocalRuntime, RouteTopology::LocalRuntime);
    let _hosted = RouteTopology::Hosted;
    let _harness = ExecutionLayer::HarnessInteraction;
    let _direct = ExecutionLayer::DirectModelInference;
}

#[test]
fn field_descriptors_carry_no_secret_bytes_or_paths() {
    let secret = "sk-secret-bytes-xyz";
    let field = CredentialFieldDescriptor::new(
        CredentialFieldId::new("api_key").expect("field id is valid"),
        FieldLabel::new("API key").expect("label is valid"),
        CredentialFieldVisibility::Secret,
    )
    .with_environment_name(
        EnvironmentVariableName::new("ANTHROPIC_API_KEY").expect("environment name is valid"),
    );
    let config = ConfigFieldDescriptor::new(
        ConfigFieldId::new("binary").expect("config id is valid"),
        FieldLabel::new("Binary").expect("label is valid"),
        ConfigFieldKind::BinaryPath,
    );

    assert_eq!(field.visibility(), CredentialFieldVisibility::Secret);
    assert_eq!(
        field
            .environment_name()
            .map(EnvironmentVariableName::as_str),
        Some("ANTHROPIC_API_KEY")
    );
    assert_eq!(config.kind(), ConfigFieldKind::BinaryPath);
    assert!(!format!("{field:?}").contains(secret));
    assert!(!format!("{config:?}").contains("/usr/bin/provider"));
}

#[test]
fn overlay_markers_cannot_be_constructed_with_an_empty_model_id() {
    let error = ModelId::new("").expect_err("empty model id must fail");
    assert_eq!(error.field(), "model id");

    let marker = OverlayMarker::new(
        ConfiguredInstanceId::new("work").expect("instance id is valid"),
        ProviderId::new("anthropic").expect("provider id is valid"),
        ModelId::new("claude-opus").expect("model id is valid"),
    )
    .with_hidden(true)
    .with_ordinal(Some(2))
    .with_consumer_default(true)
    .with_favourite(true);

    assert_eq!(marker.model_id().as_str(), "claude-opus");
    assert_eq!(
        marker.provider_id().map(ProviderId::as_str),
        Some("anthropic")
    );
    assert!(marker.hidden());
    assert_eq!(marker.ordinal(), Some(2));
    assert!(marker.consumer_default());
    assert!(marker.favourite());

    let unmarked = OverlayMarker::without_provider(
        ConfiguredInstanceId::new("work").expect("instance id is valid"),
        ModelId::new("gpt-fixture").expect("model id is valid"),
    );
    assert_eq!(unmarked.model_id().as_str(), "gpt-fixture");
    assert_eq!(unmarked.provider_id(), None);
}

#[test]
fn subject_records_default_to_redacted() {
    let observation = AuthenticatedSubjectObservation::default();
    let debug = format!("{observation:?}");

    assert!(observation.is_redacted());
    assert_eq!(observation.email(), &SubjectDisclosure::Redacted);
    assert_eq!(observation.login(), &SubjectDisclosure::Redacted);
    assert_eq!(observation.plan(), &SubjectDisclosure::Redacted);
    assert!(!debug.contains("user@example.com"));

    let revealed = observation
        .reveal_email("user@example.com")
        .expect("email is valid");
    assert_eq!(revealed.email().revealed_text(), Some("user@example.com"));
    assert!(!format!("{revealed:?}").contains("user@example.com"));
}

#[test]
fn subject_fields_can_be_absent_redacted_or_revealed() {
    let observation = AuthenticatedSubjectObservation::undisclosed()
        .with_email_disclosed()
        .with_login_absent()
        .reveal_plan("pro")
        .expect("plan is valid");

    assert_eq!(observation.email(), &SubjectDisclosure::Redacted);
    assert_eq!(observation.login(), &SubjectDisclosure::Absent);
    assert_eq!(observation.plan().revealed_text(), Some("pro"));
    assert!(!observation.is_redacted());
    assert!(!format!("{observation:?}").contains("pro"));

    let concealed = observation.without_revealed_text();
    assert_eq!(concealed.email(), &SubjectDisclosure::Redacted);
    assert_eq!(concealed.login(), &SubjectDisclosure::Absent);
    assert_eq!(concealed.plan(), &SubjectDisclosure::Redacted);
    assert!(concealed.is_redacted());
}

#[test]
fn addable_route_descriptor_keeps_topology_and_sign_in_requirements() {
    let descriptor = AddableRouteDescriptor::new(
        AddableRouteId::new("anthropic-messages").expect("route id is valid"),
        driver(),
        RouteTopology::Hosted,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService),
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
    .with_sign_in_actions([SignInAction::Interactive]);

    assert_eq!(descriptor.topology(), RouteTopology::Hosted);
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
    assert_eq!(descriptor.credential_fields().len(), 1);
    assert_eq!(
        descriptor
            .credential_field(&CredentialFieldId::new("api_key").expect("field id is valid"))
            .map(CredentialFieldDescriptor::visibility),
        Some(CredentialFieldVisibility::Secret)
    );
    assert_eq!(descriptor.config_fields().len(), 1);
    assert_eq!(
        descriptor
            .config_field(&ConfigFieldId::new("endpoint").expect("config id is valid"))
            .map(ConfigFieldDescriptor::kind),
        Some(ConfigFieldKind::ApiEndpoint)
    );
    assert_eq!(
        descriptor.sign_in_actions().collect::<Vec<_>>(),
        vec![SignInAction::Interactive]
    );
}

#[test]
fn admitted_instance_stores_opaque_refs_and_independent_enablement() {
    let secret = "sk-secret-bytes-xyz";
    let path = "/host/private/bin/provider";
    let record = AdmittedInstanceRecord::new(
        ConfiguredInstanceId::new("work").expect("instance id is valid"),
        IntegrationFamilyId::new("anthropic").expect("family is valid"),
        AddableRouteId::new("anthropic-messages").expect("route id is valid"),
        driver(),
        RouteTopology::Hosted,
    )
    .with_credential_refs([(
        CredentialFieldId::new("api_key").expect("field id is valid"),
        CredentialRef::new("cred-ref-work").expect("credential ref is valid"),
    )])
    .with_config_refs([(
        ConfigFieldId::new("binary").expect("config id is valid"),
        ConfigFieldRef::new("config-ref-work").expect("config ref is valid"),
    )])
    .with_enablement(InstanceEnablement::Disabled)
    .with_label(InstanceLabel::new("Work").expect("label is valid"))
    .with_access_status(AccessStatus::new(
        AccessProfileId::new("access").expect("access id is valid"),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ));

    assert_eq!(record.enablement(), InstanceEnablement::Disabled);
    assert_eq!(
        record.access_status().map(AccessStatus::credential),
        Some(CredentialState::Ready)
    );
    assert_eq!(record.label().map(InstanceLabel::as_str), Some("Work"));
    let debug = format!("{record:?}");
    assert!(!debug.contains(secret));
    assert!(!debug.contains(path));
    assert!(debug.contains("CredentialRef(\"<opaque>\")"));
    assert!(debug.contains("ConfigFieldRef(\"<opaque>\")"));
}

#[test]
fn planned_connection_rollover_policy_remains_a_distinct_realtime_record() {
    assert_eq!(
        PlannedConnectionRolloverPolicy::default(),
        PlannedConnectionRolloverPolicy::Disabled
    );
}
