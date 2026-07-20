use crate::access::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use crate::capability::Capability;
use crate::diagnostic::{ValueRequired, required_text};
use crate::event::ExtensionNamespace;
use crate::model::ReasoningMode;
use crate::runtime_identity::{
    AccessProfileId, DriverRole, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceOwnership, OperationShape,
};
use crate::session_access::{ResourceAccess, ResourceRepresentation, SessionAccessPolicy};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CancellationScope {
    StructuredRun,
    ActiveTurn,
    InteractiveSession,
    OwnedServingInstance,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NamedCapabilityConstraint {
    namespace: ExtensionNamespace,
    name: String,
}

impl NamedCapabilityConstraint {
    pub fn new(
        namespace: ExtensionNamespace,
        name: impl Into<String>,
    ) -> Result<Self, ValueRequired> {
        Ok(Self {
            namespace,
            name: required_text("capability constraint name", name)?,
        })
    }

    #[must_use]
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A requirement parameter. Unknown provider parameters remain named and explicit.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityConstraint {
    CancellationScope(CancellationScope),
    AttachmentMediaType(String),
    AttachmentMaximumBytes(u64),
    AttachmentMaximumCount(u32),
    SchemaDialect(String),
    ToolSchemaDialect(String),
    ToolMaximumSchemaBytes(u64),
    ToolMaximumCount(u32),
    ReasoningMode(ReasoningMode),
    ResourceAccess(ResourceAccess),
    ResourceRepresentation(ResourceRepresentation),
    ContextLimit(u64),
    MaximumConcurrency(u32),
    Named(NamedCapabilityConstraint),
}

impl CapabilityConstraint {
    pub fn attachment_media_type(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("attachment media type", value).map(Self::AttachmentMediaType)
    }

    pub fn schema_dialect(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("schema dialect", value).map(Self::SchemaDialect)
    }

    pub fn tool_schema_dialect(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("tool schema dialect", value).map(Self::ToolSchemaDialect)
    }

    #[must_use]
    pub const fn reasoning_mode(value: ReasoningMode) -> Self {
        Self::ReasoningMode(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRequirement {
    capability: Capability,
    constraints: BTreeSet<CapabilityConstraint>,
}

impl CapabilityRequirement {
    #[must_use]
    pub fn new(
        capability: Capability,
        constraints: impl IntoIterator<Item = CapabilityConstraint>,
    ) -> Self {
        Self {
            capability,
            constraints: constraints.into_iter().collect(),
        }
    }

    #[must_use]
    pub const fn capability(&self) -> Capability {
        self.capability
    }

    pub fn constraints(&self) -> impl ExactSizeIterator<Item = &CapabilityConstraint> {
        self.constraints.iter()
    }
}

/// Named capabilities plus the exact constraints observed for each one.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CapabilityProfile {
    supported: BTreeMap<Capability, BTreeSet<CapabilityConstraint>>,
}

impl CapabilityProfile {
    #[must_use]
    pub fn new(requirements: impl IntoIterator<Item = CapabilityRequirement>) -> Self {
        let mut supported = BTreeMap::new();
        for requirement in requirements {
            supported
                .entry(requirement.capability)
                .or_insert_with(BTreeSet::new)
                .extend(requirement.constraints);
        }
        Self { supported }
    }

    #[must_use]
    pub fn supports(&self, capability: Capability) -> bool {
        self.supported.contains_key(&capability)
    }

    #[must_use]
    pub fn supports_constraint(
        &self,
        capability: Capability,
        constraint: &CapabilityConstraint,
    ) -> bool {
        self.supported
            .get(&capability)
            .is_some_and(|constraints| constraints.contains(constraint))
    }

    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (Capability, &BTreeSet<CapabilityConstraint>)> {
        self.supported
            .iter()
            .map(|(capability, constraints)| (*capability, constraints))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessRequirement {
    profile_id: AccessProfileId,
    credential_states: BTreeSet<CredentialState>,
    entitlement_states: BTreeSet<EntitlementState>,
    endpoint_authorizations: BTreeSet<EndpointAuthorization>,
    runtime_readiness: BTreeSet<RuntimeReadiness>,
    support_authorities: BTreeSet<SupportAuthority>,
}

impl AccessRequirement {
    #[must_use]
    pub fn new(profile_id: AccessProfileId) -> Self {
        Self {
            profile_id,
            credential_states: BTreeSet::new(),
            entitlement_states: BTreeSet::new(),
            endpoint_authorizations: BTreeSet::new(),
            runtime_readiness: BTreeSet::new(),
            support_authorities: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn with_credential_states(
        mut self,
        states: impl IntoIterator<Item = CredentialState>,
    ) -> Self {
        self.credential_states = states.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_entitlement_states(
        mut self,
        states: impl IntoIterator<Item = EntitlementState>,
    ) -> Self {
        self.entitlement_states = states.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_endpoint_authorizations(
        mut self,
        states: impl IntoIterator<Item = EndpointAuthorization>,
    ) -> Self {
        self.endpoint_authorizations = states.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_runtime_readiness(
        mut self,
        states: impl IntoIterator<Item = RuntimeReadiness>,
    ) -> Self {
        self.runtime_readiness = states.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_support_authorities(
        mut self,
        authorities: impl IntoIterator<Item = SupportAuthority>,
    ) -> Self {
        self.support_authorities = authorities.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn profile_id(&self) -> &AccessProfileId {
        &self.profile_id
    }

    #[must_use]
    pub fn accepts_credential(&self, state: CredentialState) -> bool {
        self.credential_states.contains(&state)
    }

    #[must_use]
    pub fn accepts_entitlement(&self, state: EntitlementState) -> bool {
        self.entitlement_states.contains(&state)
    }

    #[must_use]
    pub fn accepts_endpoint_authorization(&self, state: EndpointAuthorization) -> bool {
        self.endpoint_authorizations.contains(&state)
    }

    #[must_use]
    pub fn accepts_runtime_readiness(&self, state: RuntimeReadiness) -> bool {
        self.runtime_readiness.contains(&state)
    }

    #[must_use]
    pub fn accepts_support_authority(&self, authority: SupportAuthority) -> bool {
        self.support_authorities.contains(&authority)
    }
}

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
    session_access_policy: Option<SessionAccessPolicy>,
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
            session_access_policy,
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
    pub fn with_session_access_policy(mut self, policy: SessionAccessPolicy) -> Self {
        self.session_access_policy = Some(policy);
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
    pub const fn session_access_policy(&self) -> Option<&SessionAccessPolicy> {
        self.session_access_policy.as_ref()
    }
}
