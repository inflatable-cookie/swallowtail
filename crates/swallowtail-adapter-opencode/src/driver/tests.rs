#[cfg(test)]
mod tests {
    use super::{DRIVER_ID, opencode_http_descriptor};
    use crate::selection::{
        OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, opencode_server_binding,
    };
    use swallowtail_core::{
        DriverRole, ExecutionLayer, HostServiceKind, InterfaceCompatibilityAssessment,
        OperationShape,
    };

    #[test]
    fn descriptor_claims_only_attached_harness_roles() {
        let descriptor = opencode_http_descriptor();
        assert_eq!(descriptor.identity().id().as_str(), DRIVER_ID);
        assert!(descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionManagement));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionReconciliation));
        assert!(descriptor.supports_role(DriverRole::ProviderSessionHistory));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(!descriptor.supports_execution_layer(ExecutionLayer::DirectModelInference));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionManagement)
        );
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionReconciliation)
        );
        assert!(
            descriptor.supports_operation_shape(OperationShape::ProviderSessionHistory)
        );
        assert!(
            descriptor
                .required_host_services(DriverRole::InteractiveSession)
                .any(|service| service == HostServiceKind::BlockingWork)
        );
        assert!(
            descriptor
                .required_host_services(DriverRole::StructuredRun)
                .any(|service| service == HostServiceKind::WorkingResource)
        );
        for version in [OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION] {
            assert!(
                descriptor.supports_interface_version(
                    &opencode_server_binding(version).expect("version is safe")
                )
            );
        }
        assert!(!descriptor.supports_interface_version(
            &opencode_server_binding("1.15.8").expect("gap version is safe")
        ));
        let gap = opencode_server_binding("1.18.36").expect("gap version is safe");
        assert!(!descriptor.supports_interface_version(&gap));
        assert!(!descriptor.permits_interface_version(&gap));
        let newer = opencode_server_binding("1.19.5").expect("newer version is safe");
        assert!(!descriptor.supports_interface_version(&newer));
        assert!(descriptor.permits_interface_version(&newer));
        assert!(matches!(
            descriptor.assess_interface_version(&newer),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

#[test]
fn session_history_control_failures_use_the_history_namespace() {
    let cancelled = HISTORY_CONTROL_CODES.failure(ControlFailureKind::Cancelled);
    let timed_out = HISTORY_CONTROL_CODES.failure(ControlFailureKind::TimedOut);
    assert_eq!(
        cancelled.diagnostic().code(),
        "swallowtail.opencode.session_history.cancelled"
    );
    assert_eq!(
        timed_out.diagnostic().code(),
        "swallowtail.opencode.session_history.timed_out"
    );

    let reconciliation = RECONCILIATION_CONTROL_CODES.failure(ControlFailureKind::Cancelled);
    assert_eq!(
        reconciliation.diagnostic().code(),
        "swallowtail.opencode.session_reconciliation.cancelled"
    );
}

#[test]
fn a_refused_cancellation_at_close_is_reported_not_discarded() {
    let refused = cancellation_cleanup(Err(RuntimeFailure::new(
        swallowtail_core::SafeDiagnostic::new("fixture.abort_refused", "abort refused"),
    )));
    assert_eq!(
        refused.diagnostic().map(swallowtail_core::SafeDiagnostic::code),
        Some("fixture.abort_refused")
    );
    assert_eq!(
        cancellation_cleanup(Ok(swallowtail_runtime::CancellationAcknowledgement::Requested)),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(
        CleanupOutcome::merge(refused, CleanupOutcome::Clean)
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some("fixture.abort_refused"),
        "a refused cancellation is not hidden by a clean join"
    );
}

#[test]
fn project_opencode_page_advances_cursor_when_full_page_is_filtered_by_directory() {
    use std::collections::BTreeMap;
    use std::num::NonZeroU32;
    use swallowtail_core::{
        AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CapabilityProfile,
        CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialState,
        EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
        ExecutionHostId, ExtensionNamespace, InstancePolicyId, InstanceRevision,
        InstanceTargetRef, OperationRequirements, PreflightContext, ProtocolFacadeId,
        ProviderSessionCatalogueBounds, RuntimeReadiness, SupportAuthority, preflight,
    };
    use swallowtail_runtime::{
        ProviderSessionCatalogueAgreement, ProviderSessionCatalogueId,
        ProviderSessionCatalogueScope, WorkingResourceRef,
    };

    let host = ExecutionHostId::new("opencode.page-cursor.test").expect("host id");
    let descriptor = opencode_http_descriptor();
    let access_id = AccessProfileId::new("opencode.delegated").expect("access id");
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new("opencode/delegated-auth").expect("namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new("opencode.local").expect("audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let version = crate::selection::opencode_server_binding(
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("version binding");
    let capabilities = [
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        crate::prepared::working_resource_capability(ResourceAccess::Read),
    ];
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("opencode.page-cursor.instance").expect("instance id"),
        InstanceRevision::new("1").expect("revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new("opencode.local").expect("target"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("opencode-http-sse-v1").expect("facade"),
        InstancePolicyId::new("read-only-deny-first").expect("policy"),
        CapabilityProfile::new(capabilities.clone()),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionCatalogue,
        DriverRole::ProviderSessionCatalogue,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(descriptor.required_host_services(DriverRole::ProviderSessionCatalogue))
    .with_capabilities(capabilities)
    .with_interface_versions([version])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let context = PreflightContext::new(
        &descriptor,
        &instance,
        &access,
        &access_status,
        descriptor.required_host_services(DriverRole::ProviderSessionCatalogue),
    );
    let preflight_plan = preflight(&context, &requirements).expect("preflight plan builds");
    let bounds = ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(2).expect("non-zero"),
        NonZeroU32::new(4).expect("non-zero"),
        NonZeroU32::new(64).expect("non-zero"),
        NonZeroU32::new(128).expect("non-zero"),
        NonZeroU32::new(128).expect("non-zero"),
    )
    .expect("bounds build");
    let plan = ProviderSessionCataloguePlan::new(
        preflight_plan,
        ProviderSessionCatalogueAgreement::new(
            ProviderSessionCatalogueId::new("opencode-catalogue-test").expect("catalogue id"),
            ProviderSessionCatalogueScope::working_resource(
                WorkingResourceRef::new("workspace.main").expect("resource"),
            ),
            bounds,
            None,
        ),
    )
    .expect("catalogue plan builds");

    let observations = vec![
        OpenCodeSessionObservation {
            id: "ses-other-1".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 1".to_owned(),
            updated_at: 1_700_000_000_000,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        },
        OpenCodeSessionObservation {
            id: "ses-other-2".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 2".to_owned(),
            updated_at: 1_700_000_000_100,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        },
    ];
    let (candidates, cursor) = project_opencode_page(
        &plan,
        observations,
        &BTreeMap::new(),
        0,
        2,
        "/target/workspace",
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("full filtered page projects cleanly");
    assert!(candidates.is_empty());
    assert_eq!(
        cursor.as_deref(),
        Some("2"),
        "full page filtered by directory must still advance the cursor"
    );

    // A partial page (< limit) ends pagination.
    let (_, partial_cursor) = project_opencode_page(
        &plan,
        vec![OpenCodeSessionObservation {
            id: "ses-other-3".to_owned(),
            directory: "/other/workspace".to_owned(),
            title: "other 3".to_owned(),
            updated_at: 1_700_000_000_200,
            version: crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION.to_owned(),
            parent: false,
            archived: false,
        }],
        &BTreeMap::new(),
        2,
        2,
        "/target/workspace",
        crate::selection::OPENCODE_LATEST_QUALIFIED_VERSION,
    )
    .expect("partial page projects cleanly");
    assert_eq!(partial_cursor, None);
}

#[test]
fn finalize_turn_events_preserves_authentic_terminal_status_when_event_stream_closed() {
    let cases = [
        TerminalStatus::Completed,
        TerminalStatus::Cancelled,
        TerminalStatus::TimedOut,
        TerminalStatus::ProviderFailed(SafeDiagnostic::new(
            "swallowtail.opencode.provider_failed",
            "OpenCode reported a provider failure",
        )),
    ];
    for expected in cases {
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY).expect("channel constructs");
        events.close();
        drop(stream);
        let mut activity = crate::activity::OpenCodeActivityProjection::new(
            RuntimeTurnId::new("turn-closed-events").expect("turn id"),
        );
        let _ = activity
            .project(&Event::ReasoningSnapshot {
                message_id: "msg-1".to_owned(),
                part_id: "part-1".to_owned(),
                text: "thinking".to_owned(),
            })
            .expect("open reasoning activity projects");
        let usage = swallowtail_runtime::TokenUsage::new(Some(10), Some(5));
        let preserved =
            finalize_turn_events(&mut activity, &events, 2, expected.clone(), Some(usage));
        assert_eq!(
            preserved, expected,
            "closed event stream must not overwrite authentic terminal status"
        );
    }
}
