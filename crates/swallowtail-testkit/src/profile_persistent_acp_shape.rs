use crate::profile_shape::ProfileShape;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, CredentialMechanism, CredentialState,
    DriverRole, EntitlementMetering, ExecutionLayer, HostServiceKind, InstanceOwnership,
    OperationShape, ResourceAccess, ResourceRepresentation,
};

pub(crate) fn shape() -> ProfileShape {
    ProfileShape {
        adapter_id: "fixture.driver.acp-persistent",
        integration_family: "fixture-persistent-acp-agent",
        transport_family: "acp-v1-stdio",
        instance_id: "fixture.instance.acp-persistent",
        route_id: "fixture.route.acp-persistent",
        model_id: "fixture-model-acp-persistent",
        access_profile_id: "fixture.access.acp-delegated",
        audience: "fixture-acp-membership",
        role: DriverRole::InteractiveSession,
        layer: ExecutionLayer::HarnessInteraction,
        operation_shape: OperationShape::InteractiveSession,
        ownership: InstanceOwnership::HostOwnedEphemeral,
        credential: CredentialMechanism::InteractiveOauth,
        credential_state: CredentialState::Ready,
        metering: EntitlementMetering::SubscriptionAllowance,
        required_services: vec![
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    }
}

pub(crate) fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(
            Capability::LoadSession,
            [
                CapabilityConstraint::ReplayMaximumItems(512),
                CapabilityConstraint::ReplayMaximumBytes(4 * 1024 * 1024),
            ],
        ),
        CapabilityRequirement::new(Capability::Resume, []),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResourceTextWrite,
            [CapabilityConstraint::WorkingResourceMaximumBytes(
                1024 * 1024,
            )],
        ),
    ]
}
