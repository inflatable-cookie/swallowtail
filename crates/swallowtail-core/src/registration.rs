#![deny(missing_docs)]

use crate::event::ExtensionNamespace;
use crate::identity::AdapterIdentity;
use crate::interface_version::{
    InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim, InterfaceCompatibilityMatch,
    InterfaceVersionAxis, InterfaceVersionBinding,
};
use crate::runtime_identity::{
    DriverRole, ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape,
    TransportFamilyId,
};
use std::collections::{BTreeMap, BTreeSet};

mod discovery;

pub use discovery::{DiscoveryOutcome, DiscoveryStatus};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Discovery operation supported by a driver.
pub enum DiscoveryAction {
    /// Inspect current availability without assuming prior state.
    Probe,
    /// Refresh previously observed discovery state.
    Refresh,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Authentication setup action exposed by a driver.
pub enum SignInAction {
    /// Launch an interactive sign-in flow.
    Interactive,
    /// Start a device-authorization flow.
    DeviceAuthorization,
    /// Ask the installed harness to own sign-in.
    DelegateToHarness,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Static identity, role, topology, and compatibility claims for one driver.
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
    /// Creates a descriptor with identity and transport but no advertised roles.
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
    /// Replaces runtime roles implemented by the driver.
    pub fn with_roles(mut self, roles: impl IntoIterator<Item = DriverRole>) -> Self {
        self.roles = roles.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces execution layers implemented by the driver.
    pub fn with_execution_layers(
        mut self,
        layers: impl IntoIterator<Item = ExecutionLayer>,
    ) -> Self {
        self.execution_layers = layers.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces operation shapes implemented by the driver.
    pub fn with_operation_shapes(
        mut self,
        shapes: impl IntoIterator<Item = OperationShape>,
    ) -> Self {
        self.operation_shapes = shapes.into_iter().collect();
        self
    }

    #[must_use]
    /// Sets host services required when acting in one role.
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
    /// Replaces discovery actions implemented by the driver.
    pub fn with_discovery_actions(
        mut self,
        actions: impl IntoIterator<Item = DiscoveryAction>,
    ) -> Self {
        self.discovery_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces sign-in actions implemented by the driver.
    pub fn with_sign_in_actions(mut self, actions: impl IntoIterator<Item = SignInAction>) -> Self {
        self.sign_in_actions = actions.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces provider-extension namespaces understood by the driver.
    pub fn with_extension_namespaces(
        mut self,
        namespaces: impl IntoIterator<Item = ExtensionNamespace>,
    ) -> Self {
        self.extension_namespaces = namespaces.into_iter().collect();
        self
    }

    #[must_use]
    /// Adds or replaces a compatibility claim for one interface axis.
    pub fn with_interface_compatibility(mut self, claim: InterfaceCompatibilityClaim) -> Self {
        self.interface_compatibility
            .insert(claim.axis().clone(), claim);
        self
    }

    #[must_use]
    /// Returns the stable adapter identity and implementation version.
    pub const fn identity(&self) -> &AdapterIdentity {
        &self.identity
    }

    #[must_use]
    /// Returns the provider or harness integration family.
    pub const fn integration_family(&self) -> &IntegrationFamilyId {
        &self.integration_family
    }

    #[must_use]
    /// Returns the transport family used by the driver.
    pub const fn transport_family(&self) -> &TransportFamilyId {
        &self.transport_family
    }

    #[must_use]
    /// Reports whether the driver implements a runtime role.
    pub fn supports_role(&self, role: DriverRole) -> bool {
        self.roles.contains(&role)
    }

    #[must_use]
    /// Reports whether the driver implements an execution layer.
    pub fn supports_execution_layer(&self, layer: ExecutionLayer) -> bool {
        self.execution_layers.contains(&layer)
    }

    #[must_use]
    /// Reports whether the driver implements an operation shape.
    pub fn supports_operation_shape(&self, shape: OperationShape) -> bool {
        self.operation_shapes.contains(&shape)
    }

    /// Iterates host services required for one role.
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
    /// Reports whether the driver understands an extension namespace.
    pub fn supports_extension(&self, namespace: &ExtensionNamespace) -> bool {
        self.extension_namespaces.contains(namespace)
    }

    /// Iterates supported discovery actions in stable order.
    pub fn discovery_actions(&self) -> impl ExactSizeIterator<Item = DiscoveryAction> + '_ {
        self.discovery_actions.iter().copied()
    }

    /// Iterates supported sign-in actions in stable order.
    pub fn sign_in_actions(&self) -> impl ExactSizeIterator<Item = SignInAction> + '_ {
        self.sign_in_actions.iter().copied()
    }

    #[must_use]
    /// Returns the compatibility claim for one interface axis.
    pub fn interface_compatibility(
        &self,
        axis: &InterfaceVersionAxis,
    ) -> Option<&InterfaceCompatibilityClaim> {
        self.interface_compatibility.get(axis)
    }

    #[must_use]
    /// Reports whether a version belongs to a qualified compatibility segment.
    pub fn supports_interface_version(&self, binding: &InterfaceVersionBinding) -> bool {
        self.interface_compatibility
            .get(binding.axis())
            .is_some_and(|claim| claim.supports(binding.version()))
    }

    #[must_use]
    /// Returns qualified behavior evidence for an exact interface version.
    pub fn classify_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> Option<InterfaceCompatibilityMatch> {
        self.interface_compatibility
            .get(binding.axis())
            .and_then(|claim| claim.classify(binding.version()))
    }

    #[must_use]
    /// Assesses an interface version including permitted unverified-newer state.
    pub fn assess_interface_version(
        &self,
        binding: &InterfaceVersionBinding,
    ) -> InterfaceCompatibilityAssessment {
        self.interface_compatibility
            .get(binding.axis())
            .map_or(InterfaceCompatibilityAssessment::Incompatible, |claim| {
                claim.assess(binding.version())
            })
    }

    #[must_use]
    /// Reports whether the driver permits use of an interface version.
    pub fn permits_interface_version(&self, binding: &InterfaceVersionBinding) -> bool {
        self.assess_interface_version(binding).is_permitted()
    }
}
