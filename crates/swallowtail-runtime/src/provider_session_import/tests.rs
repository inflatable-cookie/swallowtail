use super::{
    ProviderSessionCandidate, ProviderSessionCatalogueAgreement, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope, ProviderSessionCursor,
    ProviderSessionImportAgreement, ProviderSessionImportPlan, ProviderSessionImportRequest,
    validate_provider_session_catalogue_request, validate_provider_session_import_request,
};
use crate::{
    Deadline, MonotonicInstant, ProviderSessionCandidateId, ProviderSessionCatalogueId, RequestId,
    SessionPlanAgreement, WorkingResourceRef,
};
use std::collections::BTreeSet;
use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, HarnessIsolation, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    ProtocolFacadeId, ProviderSessionActivityState, ProviderSessionCatalogueBounds,
    ProviderSessionDisplayContent, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SessionRef,
    SupportAuthority, TransportFamilyId, preflight,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

fn fixture() -> Fixture {
    let axis = InterfaceVersionAxis::new("fixture.session").expect("axis is valid");
    let version = InterfaceVersion::new("1.0.0").expect("version is valid");
    let capabilities = capability_profile();
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            AdapterVersion::new("1.0.0").expect("driver version is valid"),
        ),
        IntegrationFamilyId::new("fixture").expect("family is valid"),
        TransportFamilyId::new("fixture-rpc").expect("transport is valid"),
    )
    .with_roles([
        DriverRole::ProviderSessionCatalogue,
        DriverRole::ProviderSessionImport,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::ProviderSessionCatalogue,
        OperationShape::ProviderSessionImport,
    ])
    .with_interface_compatibility(
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.session.support")
                .expect("claim id is valid"),
            axis.clone(),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [InterfaceVersionSegment::exact(
                version.clone(),
                InterfaceBehaviorRevision::new("fixture-v1").expect("revision is valid"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("claim is valid"),
    );
    let access_id = AccessProfileId::new("fixture.access").expect("access id is valid");
    let instance_id = ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("revision-1").expect("revision is valid"),
        AdapterId::new("fixture.driver").expect("driver id is valid"),
        ExecutionHostId::new("fixture.host").expect("host id is valid"),
        InstanceTargetRef::new("private/service/target").expect("target is valid"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
        InstancePolicyId::new("fixture.policy").expect("policy is valid"),
        capabilities.clone(),
    )
    .with_interface_versions([InterfaceVersionBinding::new(axis, version)]);
    let route = ModelRoute::new(
        ModelRouteId::new("fixture.route").expect("route id is valid"),
        ModelRouteRevision::new("route-1").expect("route revision is valid"),
        instance_id,
        ModelId::new("fixture-model").expect("model id is valid"),
        capabilities,
    );
    let access_profile = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::Unauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("fixture").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_status = AccessStatus::new(
        access_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    Fixture {
        driver,
        instance,
        route,
        access_profile,
        access_status,
    }
}

fn capability_profile() -> CapabilityProfile {
    CapabilityProfile::new([
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        CapabilityRequirement::new(Capability::ProviderSessionImport, []),
        CapabilityRequirement::new(Capability::LoadSession, []),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        working_resource_requirement(),
    ])
}

fn working_resource_requirement() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}

fn access_requirement(fixture: &Fixture) -> AccessRequirement {
    AccessRequirement::new(fixture.access_profile.id().clone())
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported])
}

fn bounds(content_bytes: u32, reference_bytes: u32) -> ProviderSessionCatalogueBounds {
    ProviderSessionCatalogueBounds::new(
        NonZeroU32::new(20).expect("nonzero"),
        NonZeroU32::new(100).expect("nonzero"),
        NonZeroU32::new(64).expect("nonzero"),
        NonZeroU32::new(content_bytes).expect("nonzero"),
        NonZeroU32::new(reference_bytes).expect("nonzero"),
    )
    .expect("bounds are valid")
}

fn catalogue_plan(
    fixture: &Fixture,
    id: &str,
    resource: &str,
    bounds: ProviderSessionCatalogueBounds,
) -> ProviderSessionCataloguePlan {
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionCatalogue,
        DriverRole::ProviderSessionCatalogue,
        fixture.instance.execution_host_id().clone(),
        access_requirement(fixture),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::WorkingResource,
    ])
    .with_capabilities([
        CapabilityRequirement::new(Capability::ProviderSessionCatalogue, []),
        working_resource_requirement(),
    ])
    .with_interface_versions(fixture.instance.interface_versions().cloned());
    let plan = preflight(
        &PreflightContext::new(
            &fixture.driver,
            &fixture.instance,
            &fixture.access_profile,
            &fixture.access_status,
            [
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::WorkingResource,
            ],
        ),
        &requirements,
    )
    .expect("catalogue preflight is valid");
    ProviderSessionCataloguePlan::new(
        plan,
        ProviderSessionCatalogueAgreement::new(
            ProviderSessionCatalogueId::new(id).expect("catalogue id is valid"),
            ProviderSessionCatalogueScope::working_resource(
                WorkingResourceRef::new(resource).expect("resource is valid"),
            ),
            bounds,
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
    .expect("catalogue plan is valid")
}

fn candidate(
    plan: &ProviderSessionCataloguePlan,
    id: &str,
    availability: ProviderSessionImportAvailability,
) -> ProviderSessionCandidate {
    ProviderSessionCandidate::new(
        plan,
        ProviderSessionCandidateId::new(id).expect("candidate id is valid"),
        SessionRef::new("provider/private/session").expect("session ref is valid"),
        ProviderSessionDisplayContent::new(
            Some("private title".to_owned()),
            Some("private preview".to_owned()),
        )
        .expect("display content is valid"),
        Some(1_775_000_000_000),
        ProviderSessionActivityState::Inactive,
        availability,
    )
    .expect("candidate is valid")
}

fn import_preflight(fixture: &Fixture, include_resume: bool) -> swallowtail_core::PreflightPlan {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::ProviderSessionImport, []),
        CapabilityRequirement::new(Capability::LoadSession, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        working_resource_requirement(),
    ];
    if include_resume {
        capabilities.push(CapabilityRequirement::new(Capability::Resume, []));
    }
    let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionImport,
        DriverRole::ProviderSessionImport,
        fixture.instance.execution_host_id().clone(),
        access_requirement(fixture),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::WorkingResource,
    ])
    .with_capabilities(capabilities)
    .with_interface_versions(fixture.instance.interface_versions().cloned())
    .require_model_route()
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_session_access_policy(policy)
    .with_session_provider_state_policy(
        SessionProviderStatePolicy::DurableProviderSessionPreserved,
    );
    preflight(
        &PreflightContext::new(
            &fixture.driver,
            &fixture.instance,
            &fixture.access_profile,
            &fixture.access_status,
            [
                HostServiceKind::Task,
                HostServiceKind::Time,
                HostServiceKind::WorkingResource,
            ],
        )
        .with_model_route(&fixture.route),
        &requirements,
    )
    .expect("import preflight is valid")
}

fn import_plan(
    fixture: &Fixture,
    source: ProviderSessionCataloguePlan,
    candidate: ProviderSessionCandidate,
    resource: &str,
    include_resume: bool,
) -> Result<ProviderSessionImportPlan, crate::RuntimeFailure> {
    let preflight = import_preflight(fixture, include_resume);
    let session = SessionPlanAgreement::from_plan(&preflight).expect("session agreement is valid");
    ProviderSessionImportPlan::new(
        preflight,
        source,
        ProviderSessionImportAgreement::new(
            candidate,
            WorkingResourceRef::new(resource).expect("resource is valid"),
            session,
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
}

#[test]
fn catalogue_and_import_plans_preserve_separate_shapes_and_requests() {
    let fixture = fixture();
    let catalogue = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(128, 128));
    let selected = candidate(
        &catalogue,
        "candidate-a",
        ProviderSessionImportAvailability::Available,
    );
    let import = import_plan(&fixture, catalogue.clone(), selected, "resource-a", true)
        .expect("import plan is valid");
    let catalogue_request = ProviderSessionCatalogueRequest::from_plan(
        RequestId::new("list-a").expect("request id is valid"),
        &catalogue,
        None,
    )
    .expect("catalogue request is valid");
    let import_request = ProviderSessionImportRequest::from_plan(
        RequestId::new("import-a").expect("request id is valid"),
        &import,
    )
    .expect("import request is valid");

    assert_eq!(
        catalogue.preflight().requirements().operation_shape(),
        OperationShape::ProviderSessionCatalogue
    );
    assert_eq!(
        import.preflight().requirements().operation_shape(),
        OperationShape::ProviderSessionImport
    );
    validate_provider_session_catalogue_request(&catalogue, &catalogue_request)
        .expect("catalogue request matches");
    validate_provider_session_import_request(&import, &import_request)
        .expect("import request matches");
    assert_eq!(
        import_request.provider_session_ref().as_provider_value(),
        "provider/private/session"
    );
}

#[test]
fn cursor_candidate_and_content_are_bounded_and_redacted() {
    let fixture = fixture();
    let catalogue = catalogue_plan(&fixture, "catalogue-a", "resource-a", bounds(8, 8));
    let cursor = ProviderSessionCursor::new(
        &catalogue,
        "private-cursor",
        BTreeSet::new(),
        BTreeSet::new(),
    )
    .expect("cursor fits its planned bound");
    let content = ProviderSessionDisplayContent::new(Some("private title".to_owned()), None)
        .expect("display content is structurally valid");
    let content_error = ProviderSessionCandidate::new(
        &catalogue,
        ProviderSessionCandidateId::new("candidate-a").expect("candidate id is valid"),
        SessionRef::new("short").expect("session ref is valid"),
        content,
        None,
        ProviderSessionActivityState::Unknown,
        ProviderSessionImportAvailability::Available,
    )
    .expect_err("planned content bound must be enforced");
    let reference_error = ProviderSessionCandidate::new(
        &catalogue,
        ProviderSessionCandidateId::new("candidate-b").expect("candidate id is valid"),
        SessionRef::new("private/provider/reference").expect("session ref is valid"),
        ProviderSessionDisplayContent::empty(),
        None,
        ProviderSessionActivityState::Unknown,
        ProviderSessionImportAvailability::Available,
    )
    .expect_err("planned reference bound must be enforced");

    assert!(!format!("{cursor:?}").contains("private-cursor"));
    assert_eq!(
        content_error.diagnostic().code(),
        "swallowtail.provider_session_catalogue.content_limit_exceeded"
    );
    assert_eq!(
        reference_error.diagnostic().code(),
        "swallowtail.provider_session_catalogue.reference_limit_exceeded"
    );
    assert!(!content_error.to_string().contains("private title"));
    assert!(!reference_error.to_string().contains("private/provider"));

    let roomy_catalogue = catalogue_plan(&fixture, "catalogue-b", "resource-a", bounds(128, 128));
    let visible_candidate = candidate(
        &roomy_catalogue,
        "candidate-visible",
        ProviderSessionImportAvailability::Available,
    );
    let debug = format!("{visible_candidate:?}");
    assert!(!debug.contains("private title"));
    assert!(!debug.contains("private preview"));
    assert!(!debug.contains("provider/private/session"));
    assert!(!debug.contains("resource-a"));
}

mod runtime_tests;
mod validation_tests;
