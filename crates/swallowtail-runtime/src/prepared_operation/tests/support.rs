use crate::PreparedAccessEvidence;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, ActivityContentStream,
    ActivityDisclosure, ActivityInterfaceBasis, ActivityKindClass, ActivityKindProfile,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, InterfaceBehaviorRevision,
    InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    InterfaceVersionScheme, InterfaceVersionSegment, ObservableActivityProfile,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    RuntimeReadiness, SupportAuthority, TransportFamilyId, preflight,
};

pub(super) const ACTIVITY_REVISION: &str = "activity-schema-v1";

pub(super) fn activity_requirement(
    lifecycle: ActivityLifecycleFidelity,
) -> Option<CapabilityRequirement> {
    Some(CapabilityRequirement::new(
        Capability::ObservableActivity,
        [
            CapabilityConstraint::ObservableActivityKind(ActivityKindClass::AssistantMessage),
            CapabilityConstraint::ObservableActivityLifecycle(
                ActivityKindClass::AssistantMessage,
                lifecycle,
            ),
        ],
    ))
}

pub(super) fn full_activity_profile(revision: &str) -> ObservableActivityProfile {
    ObservableActivityProfile::available(
        [activity_basis(revision)],
        [ActivityKindProfile::new(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompleteLifecycle,
            [
                ActivityContentStream::IntermediateAssistantText,
                ActivityContentStream::FinalAnswerText,
            ],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )
        .expect("full profile is valid")],
        ActivityUnknownEventPosture::FailClosed,
    )
    .expect("full route profile is valid")
}

pub(super) fn activity_basis(revision: &str) -> ActivityInterfaceBasis {
    ActivityInterfaceBasis::new(
        InterfaceVersionAxis::new("fixture-executable").expect("axis is valid"),
        InterfaceBehaviorRevision::new(revision).expect("revision is valid"),
    )
}

pub(super) struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    pub(super) provider_effect_count: usize,
}

impl Fixture {
    pub(super) fn new(activity: Option<CapabilityRequirement>) -> Self {
        let adapter_id = AdapterId::new("fixture.activity").expect("adapter id is valid");
        let host_id = ExecutionHostId::new("fixture.host").expect("host id is valid");
        let access_id = AccessProfileId::new("fixture.access").expect("access id is valid");
        let observed = observed_interface();
        let advertised_activity = full_activity_profile(ACTIVITY_REVISION)
            .capability_requirement()
            .expect("available profile advertises capability");
        let capabilities = CapabilityProfile::new([
            CapabilityRequirement::new(Capability::StructuredRun, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            advertised_activity,
        ]);
        let driver = driver(adapter_id.clone(), interface_claim(observed.axis().clone()));
        let instance = instance(
            adapter_id,
            host_id.clone(),
            access_id.clone(),
            capabilities,
            observed.clone(),
        );
        let access_profile = access_profile(access_id.clone());
        let access_status = access_status(access_id.clone());
        let requirements = requirements(host_id, access_id, observed, activity);

        Self {
            driver,
            instance,
            access_profile,
            access_status,
            requirements,
            provider_effect_count: 0,
        }
    }

    pub(super) fn plan(&self) -> PreflightPlan {
        preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                &self.access_status,
                [],
            ),
            &self.requirements,
        )
        .expect("fixture preflight succeeds")
    }

    pub(super) fn access_evidence(&self) -> PreparedAccessEvidence {
        PreparedAccessEvidence::caller_asserted(self.access_status.clone())
    }
}

fn observed_interface() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("fixture-executable").expect("axis is valid"),
        InterfaceVersion::new("1.6.0").expect("version is valid"),
    )
}

fn interface_claim(axis: InterfaceVersionAxis) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("fixture.activity-range").expect("claim id is valid"),
        axis,
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new("1.0.0").expect("minimum is valid"),
            InterfaceVersion::new("1.5.0").expect("maximum is valid"),
            InterfaceBehaviorRevision::new(ACTIVITY_REVISION).expect("behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("compatibility claim is valid")
}

fn driver(adapter_id: AdapterId, claim: InterfaceCompatibilityClaim) -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            adapter_id,
            AdapterVersion::new("0.1.0").expect("adapter version is valid"),
        ),
        IntegrationFamilyId::new("fixture").expect("integration family is valid"),
        TransportFamilyId::new("fixture-jsonl").expect("transport family is valid"),
    )
    .with_roles([DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_interface_compatibility(claim)
}

fn instance(
    adapter_id: AdapterId,
    host_id: ExecutionHostId,
    access_id: AccessProfileId,
    capabilities: CapabilityProfile,
    observed: InterfaceVersionBinding,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid"),
        InstanceRevision::new("revision-1").expect("instance revision is valid"),
        adapter_id,
        host_id,
        InstanceTargetRef::new("private-target").expect("target ref is valid"),
        InstanceOwnership::ExternalAttached,
        access_id,
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("fixture.protocol").expect("protocol facade is valid"),
        InstancePolicyId::new("fixture.policy").expect("policy id is valid"),
        capabilities,
    )
    .with_interface_versions([observed])
}

fn access_profile(access_id: AccessProfileId) -> AccessProfile {
    AccessProfile::new(
        access_id,
        CredentialMechanism::Unauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("fixture").expect("endpoint audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn access_status(access_id: AccessProfileId) -> AccessStatus {
    AccessStatus::new(
        access_id,
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn requirements(
    host_id: ExecutionHostId,
    access_id: AccessProfileId,
    observed: InterfaceVersionBinding,
    activity: Option<CapabilityRequirement>,
) -> OperationRequirements {
    let access = AccessRequirement::new(access_id)
        .with_credential_states([CredentialState::NotRequired])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]);
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
    ];
    capabilities.extend(activity);
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host_id,
        access,
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_capabilities(capabilities)
    .with_interface_versions([observed])
}
