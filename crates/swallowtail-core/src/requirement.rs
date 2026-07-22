use crate::access::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use crate::capability::Capability;
use crate::diagnostic::{ValueRequired, required_text};
use crate::event::ExtensionNamespace;
use crate::model::ReasoningMode;
use crate::realtime_media::{MediaDirection, MediaFormat};
use crate::remote_resource::OwnedRemoteResourceKind;
use crate::runtime_identity::AccessProfileId;
use crate::session_access::{ResourceAccess, ResourceRepresentation};
use std::collections::{BTreeMap, BTreeSet};

mod operation;

pub use operation::OperationRequirements;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CancellationScope {
    StructuredRun,
    ActiveTurn,
    InteractiveSession,
    OwnedServingInstance,
    ActiveResponse,
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
    WorkingResourceMaximumBytes(u64),
    ReplayMaximumItems(u32),
    ReplayMaximumBytes(u64),
    ReattachmentMaximumCount(u32),
    OwnedRemoteResource(OwnedRemoteResourceKind),
    ContextLimit(u64),
    MaximumConcurrency(u32),
    MaximumTurns(u32),
    RealtimeMediaFormat(MediaDirection, MediaFormat),
    RealtimeMediaMaximumChunkBytes(u64),
    PlannedConnectionRolloverMaximumCount(u32),
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
