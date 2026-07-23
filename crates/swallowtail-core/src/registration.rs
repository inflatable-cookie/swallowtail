use crate::InstalledExecutableObservation;
use crate::diagnostic::SafeDiagnostic;
use crate::event::ExtensionNamespace;
use crate::identity::AdapterIdentity;
use crate::interface_version::{
    InterfaceCompatibilityClaim, InterfaceCompatibilityMatch, InterfaceVersionAxis,
    InterfaceVersionBinding,
};
use crate::runtime_identity::{
    DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape,
    TransportFamilyId,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscoveryAction {
    Probe,
    Refresh,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SignInAction {
    Interactive,
    DeviceAuthorization,
    DelegateToHarness,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverDescriptor {
    identity: AdapterIdentity,
    integration_family: IntegrationFamilyId,
    transport_family: TransportFamilyId,
    roles: BTreeSet<DriverRole>,
    execution_layers: BTreeSet<ExecutionLayer>,
    operation_shapes: BTreeSet<OperationShape>,
    required_host_services: BTreeMap<DriverRole, BTreeSet<HostServiceKind>>,
    discovery_actions: BTreeSet<DiscoveryAction>,
    sign_in_actions: BTreeSet<SignInAction>,
    extension_namespaces: BTreeSet<ExtensionNamespace>,
    interface_compatibility: BTreeMap<InterfaceVersionAxis, InterfaceCompatibilityClaim>,
}

impl DriverDescriptor {
    #[must_use]
    pub fn new(
        identity: AdapterIdentity,
        integration_family: IntegrationFamilyId,
        transport_family: TransportFamilyId,
    ) -> Self {
        Self {
            identity,
            integration_family,
            transport_family,
            roles: BTreeSet::new(),
            execution_layers: BTreeSet::new(),
            operation_shapes: BTreeSet::new(),
            required_host_services: BTreeMap::new(),
            discovery_actions: BTreeSet::new(),
            sign_in_actions: BTreeSet::new(),
            extension_namespaces: BTreeSet::new(),
            interface_compatibility: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_roles(mut self, roles: impl IntoIterator<Item = DriverRole>) -> Self {
        self.roles = roles.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_execution_layers(
        mut self,
        layers: impl IntoIterator<Item = ExecutionLayer>,
    ) -> Self {
        self.execution_layers = layers.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_operation_shapes(
        mut self,
        shapes: impl IntoIterator<Item = OperationShape>,
    ) -> Self {
        self.operation_shapes = shapes.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_required_host_services(
        mut self,
        role: DriverRole,
        services: impl IntoIterator<Item = HostServiceKind>,
    ) -> Self {
        self.required_host_services
            .insert(role, services.into_iter().collect());
        self
    }

    #[must_use]
    pub fn with_discovery_actions(
        mut self,
        actions: impl IntoIterator<Item = DiscoveryAction>,
    ) -> Self {
        self.discovery_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_sign_in_actions(mut self, actions: impl IntoIterator<Item = SignInAction>) -> Self {
        self.sign_in_actions = actions.into_iter().collect();
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
    pub fn with_interface_compatibility(mut self, claim: InterfaceCompatibilityClaim) -> Self {
        self.interface_compatibility
            .insert(claim.axis().clone(), claim);
        self
    }

    #[must_use]
    pub const fn identity(&self) -> &AdapterIdentity {
        &self.identity
    }

    #[must_use]
    pub const fn integration_family(&self) -> &IntegrationFamilyId {
        &self.integration_family
    }

    #[must_use]
    pub const fn transport_family(&self) -> &TransportFamilyId {
        &self.transport_family
    }

    #[must_use]
    pub fn supports_role(&self, role: DriverRole) -> bool {
        self.roles.contains(&role)
    }

    #[must_use]
    pub fn supports_execution_layer(&self, layer: ExecutionLayer) -> bool {
        self.execution_layers.contains(&layer)
    }

    #[must_use]
    pub fn supports_operation_shape(&self, shape: OperationShape) -> bool {
        self.operation_shapes.contains(&shape)
    }

    pub fn required_host_services(
        &self,
        role: DriverRole,
    ) -> impl Iterator<Item = HostServiceKind> + '_ {
        self.required_host_services
            .get(&role)
            .into_iter()
            .flatten()
            .copied()
    }

    #[must_use]
    pub fn supports_extension(&self, namespace: &ExtensionNamespace) -> bool {
        self.extension_namespaces.contains(namespace)
    }

    pub fn discovery_actions(&self) -> impl ExactSizeIterator<Item = DiscoveryAction> + '_ {
        self.discovery_actions.iter().copied()
    }

    pub fn sign_in_actions(&self) -> impl ExactSizeIterator<Item = SignInAction> + '_ {
        self.sign_in_actions.iter().copied()
    }

    #[must_use]
    pub fn interface_compatibility(
        &self,
        axis: &InterfaceVersionAxis,
    ) -> Option<&InterfaceCompatibilityClaim> {
        self.interface_compatibility.get(axis)
    }

    #[must_use]
    pub fn supports_interface_version(&self, binding: &InterfaceVersionBinding) -> bool {
        self.interface_compatibility
            .get(binding.axis())
            .is_some_and(|claim| claim.supports(binding.version()))
    }

    #[must_use]
    pub fn classify_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> Option<InterfaceCompatibilityMatch> {
        self.interface_compatibility
            .get(binding.axis())
            .and_then(|claim| claim.classify(binding.version()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryStatus {
    Absent,
    Discovered,
    Incompatible,
    Malformed,
    TimedOut,
    Cancelled,
    Failed,
    CleanupFailed,
}

/// Safe discovery result. It never promotes a candidate into configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryOutcome {
    status: DiscoveryStatus,
    installed_executable: Option<InstalledExecutableObservation>,
    diagnostic: Option<SafeDiagnostic>,
}

impl DiscoveryOutcome {
    #[must_use]
    pub const fn new(status: DiscoveryStatus, diagnostic: Option<SafeDiagnostic>) -> Self {
        Self {
            status,
            installed_executable: None,
            diagnostic,
        }
    }

    #[must_use]
    pub fn installed_executable(observation: InstalledExecutableObservation) -> Self {
        let status = if observation.is_compatible() {
            DiscoveryStatus::Discovered
        } else {
            DiscoveryStatus::Incompatible
        };
        Self {
            status,
            installed_executable: Some(observation),
            diagnostic: None,
        }
    }

    #[must_use]
    pub const fn status(&self) -> DiscoveryStatus {
        self.status
    }

    #[must_use]
    pub const fn installed_executable_observation(
        &self,
    ) -> Option<&InstalledExecutableObservation> {
        self.installed_executable.as_ref()
    }

    #[must_use]
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}
