use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion,
    CapabilityProfile, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverDescriptor, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationShape, ProtocolFacadeId, RuntimeReadiness,
    SupportAuthority, TransportFamilyId,
};
use swallowtail_runtime::PreparedAccessEvidence;

use super::plan::{capabilities, role};

#[derive(Clone, Copy)]
pub(crate) enum AccessCase {
    Ready,
    DegradedRuntime,
    ExhaustedEntitlement,
}

pub(crate) struct ProviderOperationFixture {
    pub(crate) driver: DriverDescriptor,
    pub(crate) instance: ConfiguredInstance,
    pub(crate) route: ModelRoute,
    pub(crate) access_profile: AccessProfile,
    pub(crate) access_evidence: PreparedAccessEvidence,
    pub(crate) shape: OperationShape,
}

impl ProviderOperationFixture {
    pub(crate) fn new(shape: OperationShape, access: AccessCase) -> Self {
        let axis = InterfaceVersionAxis::new("fixture.provider-operation").expect("axis is valid");
        let version = InterfaceVersion::new("1.0.0").expect("version is valid");
        let adapter_id = AdapterId::new("fixture.provider-operation").expect("adapter id is valid");
        let access_id =
            AccessProfileId::new("fixture.provider-operation-access").expect("access id is valid");
        let capabilities = CapabilityProfile::new(capabilities(shape));
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                AdapterVersion::new("1.0.0").expect("adapter version is valid"),
            ),
            IntegrationFamilyId::new("fixture-provider-operation").expect("family is valid"),
            TransportFamilyId::new("fixture-provider-operation-rpc").expect("transport is valid"),
        )
        .with_roles([role(shape)])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([shape])
        .with_interface_compatibility(
            InterfaceCompatibilityClaim::new(
                InterfaceCompatibilityClaimId::new("fixture-provider-operation-support")
                    .expect("claim id is valid"),
                axis.clone(),
                InterfaceVersionScheme::Semantic,
                InterfaceNewerVersionPosture::QualifiedOnly,
                [InterfaceVersionSegment::exact(
                    version.clone(),
                    InterfaceBehaviorRevision::new("fixture-provider-operation-v1")
                        .expect("revision is valid"),
                    InterfaceSupportStatus::Maintained,
                )],
                [],
            )
            .expect("claim is valid"),
        );
        let instance_id = ConfiguredInstanceId::new("fixture.provider-operation-instance")
            .expect("instance id is valid");
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            InstanceRevision::new("revision-1").expect("revision is valid"),
            adapter_id,
            ExecutionHostId::new("fixture.provider-operation-host").expect("host id is valid"),
            InstanceTargetRef::new("private/provider-operation-target").expect("target is valid"),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("fixture.provider-operation-facade").expect("facade is valid"),
            InstancePolicyId::new("fixture.provider-operation-policy").expect("policy is valid"),
            capabilities.clone(),
        )
        .with_interface_versions([InterfaceVersionBinding::new(axis, version)]);
        let route = ModelRoute::new(
            ModelRouteId::new("fixture.provider-operation-route").expect("route id is valid"),
            ModelRouteRevision::new("route-1").expect("route revision is valid"),
            instance_id,
            ModelId::new("fixture-provider-operation-model").expect("model id is valid"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::Unauthenticated,
            EntitlementMetering::Unknown,
            EndpointAudience::new("fixture-provider-operation").expect("audience is valid"),
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let (entitlement, runtime) = match access {
            AccessCase::Ready => (EntitlementState::Available, RuntimeReadiness::Ready),
            AccessCase::DegradedRuntime => {
                (EntitlementState::Available, RuntimeReadiness::Degraded)
            }
            AccessCase::ExhaustedEntitlement => {
                (EntitlementState::Exhausted, RuntimeReadiness::Ready)
            }
        };
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            CredentialState::Ready,
            entitlement,
            EndpointAuthorization::Allowed,
            runtime,
            SupportAuthority::IntegrationMaintainerSupported,
        ));
        Self {
            driver,
            instance,
            route,
            access_profile,
            access_evidence,
            shape,
        }
    }
}
