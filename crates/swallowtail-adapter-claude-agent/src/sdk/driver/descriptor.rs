use super::SDK_DRIVER_ID;
use crate::sdk::{
    CLAUDE_AGENT_SDK_WIRE, claude_agent_sdk_native_claim, claude_agent_sdk_node_claim,
    claude_agent_sdk_package_claim, claude_agent_sdk_sidecar_claim, claude_agent_sdk_wire_claim,
};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

/// Describes the Claude Agent SDK sidecar fresh interactive-session role.
///
/// The transport family is the private sidecar wire, not ACP and not the
/// Claude Code stream-JSON interface, so no existing Claude claim transfers.
#[must_use]
pub fn claude_agent_sdk_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(SDK_DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("claude-agent").expect("static family id is valid"),
        TransportFamilyId::new(CLAUDE_AGENT_SDK_WIRE).expect("static transport id is valid"),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::Time,
        ],
    )
    .with_interface_compatibility(claude_agent_sdk_package_claim())
    .with_interface_compatibility(claude_agent_sdk_native_claim())
    .with_interface_compatibility(claude_agent_sdk_node_claim())
    .with_interface_compatibility(claude_agent_sdk_wire_claim())
    .with_interface_compatibility(claude_agent_sdk_sidecar_claim())
}
