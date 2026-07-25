use super::{AccessRequirement, CapabilityRequirement};
use crate::{
    AttachedRuntimeRequirements, DirectContinuationRequirements, DriverRole, ExecutionHostId,
    ExecutionLayer, ExtensionNamespace, HarnessConfigurationPosture, HarnessIsolation,
    HarnessRpcPolicy, HostServiceKind, InstanceOwnership, InterfaceVersionBinding, OperationShape,
    PlannedConnectionRolloverPolicy, RealtimeMediaRequirements, RemoteAcpRequirements,
    SessionAccessPolicy, SessionProviderStatePolicy,
};
use std::collections::BTreeSet;

mod options;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationRequirements {
    execution_layer: ExecutionLayer,
    operation_shape: OperationShape,
    driver_role: DriverRole,
    execution_host_id: ExecutionHostId,
    access: AccessRequirement,
    ownership_modes: BTreeSet<InstanceOwnership>,
    host_services: BTreeSet<HostServiceKind>,
    capabilities: Vec<CapabilityRequirement>,
    extension_namespaces: BTreeSet<ExtensionNamespace>,
    model_route_required: bool,
    harness_isolation: Option<HarnessIsolation>,
    session_access_policy: Option<SessionAccessPolicy>,
    session_provider_state_policy: Option<SessionProviderStatePolicy>,
    realtime_media: Option<RealtimeMediaRequirements>,
    planned_connection_rollover: PlannedConnectionRolloverPolicy,
    direct_continuation: Option<DirectContinuationRequirements>,
    attached_runtime: Option<AttachedRuntimeRequirements>,
    remote_acp: Option<RemoteAcpRequirements>,
    interface_versions: BTreeSet<InterfaceVersionBinding>,
    harness_rpc_policy: Option<HarnessRpcPolicy>,
    harness_configuration_posture: Option<HarnessConfigurationPosture>,
}

impl OperationRequirements {
    #[must_use]
    pub fn new(
        execution_layer: ExecutionLayer,
        operation_shape: OperationShape,
        driver_role: DriverRole,
        execution_host_id: ExecutionHostId,
        access: AccessRequirement,
    ) -> Self {
        Self {
            execution_layer,
            operation_shape,
            driver_role,
            execution_host_id,
            access,
            ownership_modes: BTreeSet::new(),
            host_services: BTreeSet::new(),
            capabilities: Vec::new(),
            extension_namespaces: BTreeSet::new(),
            model_route_required: false,
            harness_isolation: None,
            session_access_policy: None,
            session_provider_state_policy: None,
            realtime_media: None,
            planned_connection_rollover: PlannedConnectionRolloverPolicy::Disabled,
            direct_continuation: None,
            attached_runtime: None,
            remote_acp: None,
            interface_versions: BTreeSet::new(),
            harness_rpc_policy: None,
            harness_configuration_posture: None,
        }
    }

    #[must_use]
    pub fn with_ownership_modes(
        mut self,
        modes: impl IntoIterator<Item = InstanceOwnership>,
    ) -> Self {
        self.ownership_modes = modes.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_host_services(
        mut self,
        services: impl IntoIterator<Item = HostServiceKind>,
    ) -> Self {
        self.host_services = services.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    ) -> Self {
        self.capabilities = capabilities.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_extension_namespaces(
        mut self,
        namespaces: impl IntoIterator<Item = ExtensionNamespace>,
    ) -> Self {
        self.extension_namespaces = namespaces.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn require_model_route(mut self) -> Self {
        self.model_route_required = true;
        self
    }

    #[must_use]
    pub const fn execution_layer(&self) -> ExecutionLayer {
        self.execution_layer
    }

    #[must_use]
    pub const fn operation_shape(&self) -> OperationShape {
        self.operation_shape
    }

    #[must_use]
    pub const fn driver_role(&self) -> DriverRole {
        self.driver_role
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    pub const fn access(&self) -> &AccessRequirement {
        &self.access
    }

    #[must_use]
    pub fn accepts_ownership(&self, ownership: InstanceOwnership) -> bool {
        self.ownership_modes.contains(&ownership)
    }

    pub fn host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.host_services.iter().copied()
    }

    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = &CapabilityRequirement> {
        self.capabilities.iter()
    }

    pub fn extension_namespaces(&self) -> impl ExactSizeIterator<Item = &ExtensionNamespace> {
        self.extension_namespaces.iter()
    }

    #[must_use]
    pub const fn model_route_required(&self) -> bool {
        self.model_route_required
    }
}
