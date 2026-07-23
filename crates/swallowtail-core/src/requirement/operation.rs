use super::{AccessRequirement, CapabilityRequirement};
use crate::{
    AttachedRuntimeRequirements, DirectContinuationRequirements, DriverRole, ExecutionHostId,
    ExecutionLayer, ExtensionNamespace, HarnessIsolation, HarnessRpcPolicy, HostServiceKind,
    InstanceOwnership, InterfaceVersionBinding, OperationShape, PlannedConnectionRolloverPolicy,
    RealtimeMediaRequirements, SessionAccessPolicy, SessionProviderStatePolicy,
};
use std::collections::BTreeSet;

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
    interface_versions: BTreeSet<InterfaceVersionBinding>,
    harness_rpc_policy: Option<HarnessRpcPolicy>,
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
        let session_access_policy = (operation_shape == OperationShape::InteractiveSession)
            .then(SessionAccessPolicy::default);
        let session_provider_state_policy = (operation_shape == OperationShape::InteractiveSession)
            .then(SessionProviderStatePolicy::default);
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
            session_access_policy,
            session_provider_state_policy,
            realtime_media: None,
            planned_connection_rollover: PlannedConnectionRolloverPolicy::Disabled,
            direct_continuation: None,
            attached_runtime: None,
            interface_versions: BTreeSet::new(),
            harness_rpc_policy: None,
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
    pub const fn with_harness_isolation(mut self, isolation: HarnessIsolation) -> Self {
        self.harness_isolation = Some(isolation);
        self
    }

    #[must_use]
    pub fn with_session_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.session_access_policy = Some(policy);
        self
    }

    #[must_use]
    pub const fn with_session_provider_state_policy(
        mut self,
        policy: SessionProviderStatePolicy,
    ) -> Self {
        self.session_provider_state_policy = Some(policy);
        self
    }

    #[must_use]
    pub fn with_realtime_media(mut self, requirements: RealtimeMediaRequirements) -> Self {
        self.realtime_media = Some(requirements);
        self
    }

    #[must_use]
    pub const fn with_planned_connection_rollover(
        mut self,
        policy: PlannedConnectionRolloverPolicy,
    ) -> Self {
        self.planned_connection_rollover = policy;
        self
    }

    #[must_use]
    pub fn with_direct_continuation(
        mut self,
        requirements: DirectContinuationRequirements,
    ) -> Self {
        self.direct_continuation = Some(requirements);
        self
    }

    #[must_use]
    pub fn with_attached_runtime(mut self, requirements: AttachedRuntimeRequirements) -> Self {
        self.attached_runtime = Some(requirements);
        self
    }

    #[must_use]
    pub fn with_interface_versions(
        mut self,
        versions: impl IntoIterator<Item = InterfaceVersionBinding>,
    ) -> Self {
        self.interface_versions = versions.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_harness_rpc_policy(mut self, policy: HarnessRpcPolicy) -> Self {
        self.harness_rpc_policy = Some(policy);
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

    #[must_use]
    pub const fn harness_isolation(&self) -> Option<HarnessIsolation> {
        self.harness_isolation
    }

    #[must_use]
    pub const fn session_access_policy(&self) -> Option<&SessionAccessPolicy> {
        self.session_access_policy.as_ref()
    }

    #[must_use]
    pub const fn session_provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.session_provider_state_policy
    }

    #[must_use]
    pub const fn realtime_media(&self) -> Option<&RealtimeMediaRequirements> {
        self.realtime_media.as_ref()
    }

    #[must_use]
    pub const fn planned_connection_rollover(&self) -> PlannedConnectionRolloverPolicy {
        self.planned_connection_rollover
    }

    #[must_use]
    pub const fn direct_continuation(&self) -> Option<&DirectContinuationRequirements> {
        self.direct_continuation.as_ref()
    }

    #[must_use]
    pub const fn attached_runtime(&self) -> Option<&AttachedRuntimeRequirements> {
        self.attached_runtime.as_ref()
    }

    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.interface_versions.iter()
    }

    #[must_use]
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.harness_rpc_policy.as_ref()
    }
}
