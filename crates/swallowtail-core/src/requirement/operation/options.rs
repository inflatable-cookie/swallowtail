use super::OperationRequirements;
use crate::{
    AttachedRuntimeRequirements, DirectContinuationRequirements, HarnessConfigurationPosture,
    HarnessIsolation, HarnessRpcPolicy, InterfaceVersionBinding, PlannedConnectionRolloverPolicy,
    RealtimeMediaRequirements, RemoteAcpRequirements, SessionAccessPolicy,
    SessionProviderStatePolicy,
};

impl OperationRequirements {
    #[must_use]
    /// Requires a specific installed-harness isolation posture.
    pub const fn with_harness_isolation(mut self, isolation: HarnessIsolation) -> Self {
        self.harness_isolation = Some(isolation);
        self
    }

    #[must_use]
    /// Adds the session resource, permission, and network access policy.
    pub fn with_session_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.session_access_policy = Some(policy);
        self
    }

    #[must_use]
    /// Adds the allowed provider-state retention policy.
    pub const fn with_session_provider_state_policy(
        mut self,
        policy: SessionProviderStatePolicy,
    ) -> Self {
        self.session_provider_state_policy = Some(policy);
        self
    }

    #[must_use]
    /// Adds exact realtime-media requirements.
    pub fn with_realtime_media(mut self, requirements: RealtimeMediaRequirements) -> Self {
        self.realtime_media = Some(requirements);
        self
    }

    #[must_use]
    /// Adds the planned realtime connection-rollover policy.
    pub const fn with_planned_connection_rollover(
        mut self,
        policy: PlannedConnectionRolloverPolicy,
    ) -> Self {
        self.planned_connection_rollover = policy;
        self
    }

    #[must_use]
    /// Adds bounded direct tool-continuation requirements.
    pub fn with_direct_continuation(
        mut self,
        requirements: DirectContinuationRequirements,
    ) -> Self {
        self.direct_continuation = Some(requirements);
        self
    }

    #[must_use]
    /// Adds exact requirements for an attached model runtime.
    pub fn with_attached_runtime(mut self, requirements: AttachedRuntimeRequirements) -> Self {
        self.attached_runtime = Some(requirements);
        self
    }

    #[must_use]
    /// Adds exact requirements for a remote ACP connection.
    pub fn with_remote_acp(mut self, requirements: RemoteAcpRequirements) -> Self {
        self.remote_acp = Some(requirements);
        self
    }

    #[must_use]
    /// Replaces exact interface-version bindings required by the operation.
    pub fn with_interface_versions(
        mut self,
        versions: impl IntoIterator<Item = InterfaceVersionBinding>,
    ) -> Self {
        self.interface_versions = versions.into_iter().collect();
        self
    }

    #[must_use]
    /// Adds bounded harness-RPC scheduling and message policy.
    pub fn with_harness_rpc_policy(mut self, policy: HarnessRpcPolicy) -> Self {
        self.harness_rpc_policy = Some(policy);
        self
    }

    #[must_use]
    /// Adds the required harness-configuration posture.
    pub const fn with_harness_configuration_posture(
        mut self,
        posture: HarnessConfigurationPosture,
    ) -> Self {
        self.harness_configuration_posture = Some(posture);
        self
    }

    #[must_use]
    /// Returns the required harness-isolation posture, when applicable.
    pub const fn harness_isolation(&self) -> Option<HarnessIsolation> {
        self.harness_isolation
    }

    #[must_use]
    /// Returns the session access policy, when the shape requires one.
    pub const fn session_access_policy(&self) -> Option<&SessionAccessPolicy> {
        self.session_access_policy.as_ref()
    }

    #[must_use]
    /// Returns the session provider-state policy, when applicable.
    pub const fn session_provider_state_policy(&self) -> Option<SessionProviderStatePolicy> {
        self.session_provider_state_policy
    }

    #[must_use]
    /// Returns exact realtime-media requirements, when applicable.
    pub const fn realtime_media(&self) -> Option<&RealtimeMediaRequirements> {
        self.realtime_media.as_ref()
    }

    #[must_use]
    /// Returns the planned connection-rollover policy.
    pub const fn planned_connection_rollover(&self) -> PlannedConnectionRolloverPolicy {
        self.planned_connection_rollover
    }

    #[must_use]
    /// Returns bounded direct-continuation requirements, when applicable.
    pub const fn direct_continuation(&self) -> Option<&DirectContinuationRequirements> {
        self.direct_continuation.as_ref()
    }

    #[must_use]
    /// Returns attached-runtime requirements, when applicable.
    pub const fn attached_runtime(&self) -> Option<&AttachedRuntimeRequirements> {
        self.attached_runtime.as_ref()
    }

    #[must_use]
    /// Returns remote ACP requirements, when applicable.
    pub const fn remote_acp(&self) -> Option<&RemoteAcpRequirements> {
        self.remote_acp.as_ref()
    }

    /// Iterates required interface versions in stable order.
    pub fn interface_versions(&self) -> impl ExactSizeIterator<Item = &InterfaceVersionBinding> {
        self.interface_versions.iter()
    }

    #[must_use]
    /// Returns the harness-RPC policy, when applicable.
    pub const fn harness_rpc_policy(&self) -> Option<&HarnessRpcPolicy> {
        self.harness_rpc_policy.as_ref()
    }

    #[must_use]
    /// Returns the required harness-configuration posture, when applicable.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture
    }
}
