use super::{
    ArchiveProviderSessionRequest, DeleteProviderSessionRequest,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementAgreement,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RestoreProviderSessionRequest,
    validate_provider_session_management_request,
};
use crate::{
    AccessEvidenceSourceId, Deadline, HostServices, MonotonicInstant, PreparedAccessEvidence,
    ProviderSessionManagementBinding, RateLimitKind, RateLimitObservation, RequestId,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverDescriptor, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, InterfaceBehaviorRevision,
    InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    InterfaceVersionScheme, InterfaceVersionSegment, OperationRequirements, OperationShape,
    PreflightContext, ProtocolFacadeId, ProviderRequestRef, ProviderSessionActivityEvidence,
    ProviderSessionAffectedScope, ProviderSessionBindingOrigin, ProviderSessionCancellationPosture,
    ProviderSessionDeletionStrength, ProviderSessionEffectTruth,
    ProviderSessionInitialStateRequirement, ProviderSessionManagementAction,
    ProviderSessionManagementEffect, RuntimeReadiness, SessionRef, SupportAuthority,
    TransportFamilyId, preflight,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

fn fixture() -> Fixture {
    let axis = InterfaceVersionAxis::new("fixture.rpc").expect("axis is valid");
    let version = InterfaceVersion::new("1.2.0").expect("version is valid");
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            AdapterVersion::new("1.0.0").expect("driver version is valid"),
        ),
        IntegrationFamilyId::new("fixture").expect("family is valid"),
        TransportFamilyId::new("fixture-rpc").expect("transport is valid"),
    )
    .with_roles([DriverRole::ProviderSessionManagement])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::ProviderSessionManagement])
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [HostServiceKind::Task, HostServiceKind::Time],
    )
    .with_interface_compatibility(
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.rpc.support").expect("claim id is valid"),
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
    let instance = ConfiguredInstance::new(
        ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
        InstanceRevision::new("revision-1").expect("revision is valid"),
        AdapterId::new("fixture.driver").expect("driver id is valid"),
        ExecutionHostId::new("fixture.host").expect("host id is valid"),
        InstanceTargetRef::new("private/service/target").expect("target is valid"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
        InstancePolicyId::new("fixture.policy").expect("policy is valid"),
        CapabilityProfile::new([
            CapabilityRequirement::new(Capability::ProviderSessionArchive, []),
            CapabilityRequirement::new(Capability::ProviderSessionRestore, []),
            CapabilityRequirement::new(Capability::ProviderSessionDelete, []),
        ]),
    )
    .with_interface_versions([InterfaceVersionBinding::new(axis, version)]);
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
        access_profile,
        access_status,
    }
}

fn plan_for(action: ProviderSessionManagementAction) -> ProviderSessionManagementPlan {
    let fixture = fixture();
    let capability = action.required_capability();
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionManagement,
        DriverRole::ProviderSessionManagement,
        fixture.instance.execution_host_id().clone(),
        AccessRequirement::new(fixture.access_profile.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([HostServiceKind::Task, HostServiceKind::Time])
    .with_capabilities([CapabilityRequirement::new(capability, [])])
    .with_interface_versions(fixture.instance.interface_versions().cloned());
    let preflight = preflight(
        &PreflightContext::new(
            &fixture.driver,
            &fixture.instance,
            &fixture.access_profile,
            &fixture.access_status,
            [HostServiceKind::Task, HostServiceKind::Time],
        ),
        &requirements,
    )
    .expect("preflight is valid");
    let binding = ProviderSessionManagementBinding::from_bound_session(
        SessionRef::new("provider/private/session").expect("session ref is valid"),
        &fixture.driver,
        &fixture.instance,
        PreparedAccessEvidence::observed(
            fixture.access_status,
            AccessEvidenceSourceId::new("private/access/source").expect("source is valid"),
        ),
        None,
        ProviderSessionBindingOrigin::Created,
    )
    .expect("binding is valid");
    let initial_state = match action {
        ProviderSessionManagementAction::Archive => {
            ProviderSessionInitialStateRequirement::Unarchived
        }
        ProviderSessionManagementAction::Restore => {
            ProviderSessionInitialStateRequirement::Archived
        }
        ProviderSessionManagementAction::Delete(_) => {
            ProviderSessionInitialStateRequirement::UnarchivedOrArchived
        }
    };
    ProviderSessionManagementPlan::new(
        preflight,
        ProviderSessionManagementAgreement::new(
            binding,
            action,
            initial_state,
            ProviderSessionAffectedScope::TargetOnly,
            ProviderSessionActivityEvidence::CallerAssertedInactive,
            ProviderSessionCancellationPosture::BeforeDispatchOnly,
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
    .expect("management plan is valid")
}

#[test]
fn typed_requests_preserve_exact_action_and_prepared_evidence() {
    let archive = plan_for(ProviderSessionManagementAction::Archive);
    let restore = plan_for(ProviderSessionManagementAction::Restore);
    let delete = plan_for(ProviderSessionManagementAction::Delete(
        ProviderSessionDeletionStrength::ProviderDataDeleted,
    ));

    ArchiveProviderSessionRequest::from_plan(
        RequestId::new("archive").expect("request id is valid"),
        &archive,
    )
    .expect("archive request is valid");
    RestoreProviderSessionRequest::from_plan(
        RequestId::new("restore").expect("request id is valid"),
        &restore,
    )
    .expect("restore request is valid");
    DeleteProviderSessionRequest::from_plan(
        RequestId::new("delete").expect("request id is valid"),
        &delete,
    )
    .expect("delete request is valid");
    let prepared = PreparedProviderSessionManagementEvidence::from_plan(delete)
        .expect("prepared evidence is valid");

    assert_eq!(
        prepared.operation().binding().driver_role(),
        DriverRole::ProviderSessionManagement
    );
}

#[test]
fn request_drift_and_missing_services_fail_before_effects() {
    let archive = plan_for(ProviderSessionManagementAction::Archive);
    let restore = plan_for(ProviderSessionManagementAction::Restore);
    let request = ArchiveProviderSessionRequest::from_plan(
        RequestId::new("archive").expect("request id is valid"),
        &archive,
    )
    .expect("archive request is valid");
    let wrong_host =
        HostServices::new(ExecutionHostId::new("wrong.host").expect("host id is valid"));
    let matching_host = HostServices::new(archive.preflight().execution_host_id().clone());

    let drift =
        validate_provider_session_management_request(&restore, request.agreement(), &matching_host)
            .expect_err("drift must fail");
    let missing =
        validate_provider_session_management_request(&archive, request.agreement(), &matching_host)
            .expect_err("missing services must fail");
    let host =
        validate_provider_session_management_request(&archive, request.agreement(), &wrong_host)
            .expect_err("wrong host must fail");

    assert_eq!(
        drift.diagnostic().code(),
        "swallowtail.provider_session_management.plan_mismatch"
    );
    assert_eq!(
        missing.diagnostic().code(),
        "swallowtail.provider_session_management.service_unavailable"
    );
    assert_eq!(
        host.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
}

#[test]
fn after_dispatch_uncertainty_cannot_become_confirmation() {
    let plan = plan_for(ProviderSessionManagementAction::Delete(
        ProviderSessionDeletionStrength::ProviderHardDeleted,
    ));
    let effect =
        ProviderSessionManagementEffect::unconfirmed_after_effect(plan.agreement().action());
    let outcome = ProviderSessionManagementOutcome::new(plan.agreement().binding().clone(), effect)
        .with_provider_request_ref(
            ProviderRequestRef::new("provider/private/request").expect("request ref is valid"),
        )
        .with_rate_limits([RateLimitObservation::new(
            RateLimitKind::Requests,
            Some(100),
            Some(99),
            None,
        )]);

    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
    assert_eq!(outcome.rate_limits().len(), 1);
    let debug = format!("{outcome:?}");
    assert!(!debug.contains("provider/private/session"));
    assert!(!debug.contains("provider/private/request"));
}
