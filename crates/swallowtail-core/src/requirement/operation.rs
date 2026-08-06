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
/// Complete provider-neutral admission requirements for one operation.
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
    /// Starts requirements with exact route identity and no optional claims.
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
    /// Replaces the admitted configured-instance ownership modes.
    pub fn with_ownership_modes(
        mut self,
        modes: impl IntoIterator<Item = InstanceOwnership>,
    ) -> Self {
        self.ownership_modes = modes.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces the host services required before dispatch.
    pub fn with_host_services(
        mut self,
        services: impl IntoIterator<Item = HostServiceKind>,
    ) -> Self {
        self.host_services = services.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces required capabilities and their constraints.
    pub fn with_capabilities(
        mut self,
        capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    ) -> Self {
        self.capabilities = capabilities.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces provider-extension namespaces admitted by the operation.
    pub fn with_extension_namespaces(
        mut self,
        namespaces: impl IntoIterator<Item = ExtensionNamespace>,
    ) -> Self {
        self.extension_namespaces = namespaces.into_iter().collect();
        self
    }

    #[must_use]
    /// Requires preflight to bind an exact model route.
    pub const fn require_model_route(mut self) -> Self {
        self.model_route_required = true;
        self
    }

    #[must_use]
    /// Returns the required execution layer.
    pub const fn execution_layer(&self) -> ExecutionLayer {
        self.execution_layer
    }

    #[must_use]
    /// Returns the required operation shape.
    pub const fn operation_shape(&self) -> OperationShape {
        self.operation_shape
    }

    #[must_use]
    /// Returns the required runtime driver role.
    pub const fn driver_role(&self) -> DriverRole {
        self.driver_role
    }

    #[must_use]
    /// Returns the execution host that must own preparation.
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    #[must_use]
    /// Returns the exact access requirement.
    pub const fn access(&self) -> &AccessRequirement {
        &self.access
    }

    #[must_use]
    /// Reports whether an instance ownership mode is admitted.
    pub fn accepts_ownership(&self, ownership: InstanceOwnership) -> bool {
        self.ownership_modes.contains(&ownership)
    }

    /// Iterates required host services in stable order.
    pub fn host_services(&self) -> impl ExactSizeIterator<Item = HostServiceKind> + '_ {
        self.host_services.iter().copied()
    }

    /// Iterates required capabilities.
    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = &CapabilityRequirement> {
        self.capabilities.iter()
    }

    /// Iterates admitted provider-extension namespaces in stable order.
    pub fn extension_namespaces(&self) -> impl ExactSizeIterator<Item = &ExtensionNamespace> {
        self.extension_namespaces.iter()
    }

    #[must_use]
    /// Reports whether a model route must be bound by preflight.
    pub const fn model_route_required(&self) -> bool {
        self.model_route_required
    }
}
