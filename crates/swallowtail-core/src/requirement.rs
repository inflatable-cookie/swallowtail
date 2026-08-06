#![deny(missing_docs)]

use crate::access::{
    CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness, SupportAuthority,
};
use crate::capability::Capability;
use crate::diagnostic::{ValueRequired, required_text};
use crate::event::ExtensionNamespace;
use crate::harness_mode::HarnessMode;
use crate::model::ReasoningMode;
use crate::observable_activity::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture, SubagentControlActionKind,
    SubagentObservationFidelity,
};
use crate::realtime_media::{MediaDirection, MediaFormat};
use crate::remote_resource::OwnedRemoteResourceKind;
use crate::runtime_identity::AccessProfileId;
use crate::session_access::{ResourceAccess, ResourceRepresentation};
use std::collections::{BTreeMap, BTreeSet};

mod operation;

pub use operation::OperationRequirements;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Operation boundary to which cancellation authority applies.
pub enum CancellationScope {
    /// Cancel an active discovery probe.
    DiscoveryProbe,
    /// Cancel a bounded structured run.
    StructuredRun,
    /// Cancel only the current interactive turn.
    ActiveTurn,
    /// Close an entire interactive session.
    InteractiveSession,
    /// Stop an owned serving instance.
    OwnedServingInstance,
    /// Cancel the active provider response.
    ActiveResponse,
    /// Cancel a provider-session management operation.
    ProviderSessionManagement,
    /// Cancel provider-session catalogue enumeration.
    ProviderSessionCatalogue,
    /// Cancel provider-session import.
    ProviderSessionImport,
    /// Cancel provider-session reconciliation.
    ProviderSessionReconciliation,
    /// Cancel provider-run reconciliation.
    ProviderRunReconciliation,
    /// Cancel recovered-resource cleanup.
    ProviderRecoveredResourceCleanup,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Active operation boundary that can outlive its local observer.
pub enum OperationDetachmentScope {
    /// Detach observation from a provider-owned structured run.
    StructuredRun,
    /// Detach observation from the active interactive turn.
    ActiveTurn,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Authority responsible for enforcing structured output.
pub enum StructuredOutputEnforcement {
    /// Provider API or model service enforces the schema.
    ProviderNative,
    /// Installed harness validates output before returning it.
    HarnessValidated,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Provider-namespaced capability parameter outside the common vocabulary.
pub struct NamedCapabilityConstraint {
    namespace: ExtensionNamespace,
    name: String,
}

impl NamedCapabilityConstraint {
    /// Creates a named constraint after validating its local name.
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
    /// Returns the namespace that owns the constraint meaning.
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    #[must_use]
    /// Returns the validated namespace-local constraint name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A requirement parameter. Unknown provider parameters remain named and explicit.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapabilityConstraint {
    /// Bounds the operation surface affected by cancellation.
    CancellationScope(CancellationScope),
    /// Bounds the operation surface allowed to detach.
    OperationDetachmentScope(OperationDetachmentScope),
    /// Accepted attachment media type.
    AttachmentMediaType(String),
    /// Maximum bytes in one attachment.
    AttachmentMaximumBytes(u64),
    /// Maximum attachments in one request.
    AttachmentMaximumCount(u32),
    /// Supported structured-output schema dialect.
    SchemaDialect(String),
    /// Authority enforcing structured output.
    StructuredOutputEnforcement(StructuredOutputEnforcement),
    /// Supported tool-schema dialect.
    ToolSchemaDialect(String),
    /// Maximum encoded bytes in one tool schema.
    ToolMaximumSchemaBytes(u64),
    /// Maximum tools admitted for one operation.
    ToolMaximumCount(u32),
    /// Accepted reasoning mode.
    ReasoningMode(ReasoningMode),
    /// Accepted provider harness mode.
    HarnessMode(HarnessMode),
    /// Permitted access to the working resource.
    ResourceAccess(ResourceAccess),
    /// Representation through which the working resource is exposed.
    ResourceRepresentation(ResourceRepresentation),
    /// Maximum bytes available through the working-resource boundary.
    WorkingResourceMaximumBytes(u64),
    /// Maximum retained history items replayed to a consumer.
    ReplayMaximumItems(u32),
    /// Maximum encoded bytes replayed to a consumer.
    ReplayMaximumBytes(u64),
    /// Maximum recovered output bytes projected after reconciliation.
    RecoveredOutputMaximumBytes(u64),
    /// Maximum times an operation stream may be reattached.
    ReattachmentMaximumCount(u32),
    /// Kind of provider-owned remote resource governed by the capability.
    OwnedRemoteResource(OwnedRemoteResourceKind),
    /// Maximum model context tokens accepted by the route.
    ContextLimit(u64),
    /// Maximum concurrent work admitted by the route.
    MaximumConcurrency(u32),
    /// Maximum turns admitted by an operation.
    MaximumTurns(u32),
    /// Media format accepted in one direction.
    RealtimeMediaFormat(MediaDirection, MediaFormat),
    /// Maximum bytes in one realtime-media chunk.
    RealtimeMediaMaximumChunkBytes(u64),
    /// Maximum planned realtime connection rollovers.
    PlannedConnectionRolloverMaximumCount(u32),
    /// Maximum direct inference attempts across continuation.
    MaximumInferenceAttempts(u32),
    /// Maximum direct tool calls across continuation.
    MaximumToolCalls(u32),
    /// Maximum encoded bytes in tool arguments.
    ToolArgumentMaximumBytes(u64),
    /// Maximum encoded bytes in a tool result.
    ToolResultMaximumBytes(u64),
    /// Maximum bytes in private continuation state.
    PrivateContinuationMaximumBytes(u64),
    /// Maximum bytes in private retained history.
    PrivateHistoryMaximumBytes(u64),
    /// Maximum records retained from an operation stream.
    StreamRecordMaximumCount(u32),
    /// Maximum output tokens requested from the provider.
    OutputTokenMaximum(u64),
    /// Observable activity kind exposed by the route.
    ObservableActivityKind(ActivityKindClass),
    /// Lifecycle fidelity for one activity kind.
    ObservableActivityLifecycle(ActivityKindClass, ActivityLifecycleFidelity),
    /// Content stream exposed for one activity kind.
    ObservableActivityContentStream(ActivityKindClass, ActivityContentStream),
    /// Disclosure level exposed for one activity kind.
    ObservableActivityDisclosure(ActivityKindClass, ActivityDisclosure),
    /// Correlation evidence exposed for one activity kind.
    ObservableActivityCorrelation(ActivityKindClass, ActivityCorrelationKind),
    /// Indicates complete task-list snapshots for one activity kind.
    ObservableActivityTaskListSnapshots(ActivityKindClass),
    /// Fidelity available for observing provider subagents.
    ObservableSubagentObservation(SubagentObservationFidelity),
    /// Subagent control action exposed by the route.
    ObservableSubagentControlAction(SubagentControlActionKind),
    /// Posture for provider activity events outside the known vocabulary.
    ObservableActivityUnknownEventPosture(ActivityUnknownEventPosture),
    /// Provider-namespaced constraint outside the common vocabulary.
    Named(NamedCapabilityConstraint),
}

impl CapabilityConstraint {
    /// Creates a validated attachment media-type constraint.
    pub fn attachment_media_type(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("attachment media type", value).map(Self::AttachmentMediaType)
    }

    /// Creates a validated structured-output schema-dialect constraint.
    pub fn schema_dialect(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("schema dialect", value).map(Self::SchemaDialect)
    }

    /// Creates a validated tool-schema dialect constraint.
    pub fn tool_schema_dialect(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("tool schema dialect", value).map(Self::ToolSchemaDialect)
    }

    #[must_use]
    /// Creates a reasoning-mode constraint.
    pub const fn reasoning_mode(value: ReasoningMode) -> Self {
        Self::ReasoningMode(value)
    }

    #[must_use]
    /// Creates a harness-mode constraint.
    pub const fn harness_mode(value: HarnessMode) -> Self {
        Self::HarnessMode(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One required capability and all of its exact parameters.
pub struct CapabilityRequirement {
    capability: Capability,
    constraints: BTreeSet<CapabilityConstraint>,
}

impl CapabilityRequirement {
    /// Creates a deduplicated capability requirement.
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
    /// Returns the required capability.
    pub const fn capability(&self) -> Capability {
        self.capability
    }

    /// Iterates exact constraints in stable order.
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
    /// Creates a profile, merging repeated requirements for each capability.
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
    /// Reports whether the profile advertises `capability`.
    pub fn supports(&self, capability: Capability) -> bool {
        self.supported.contains_key(&capability)
    }

    #[must_use]
    /// Reports whether a capability advertises one exact constraint.
    pub fn supports_constraint(
        &self,
        capability: Capability,
        constraint: &CapabilityConstraint,
    ) -> bool {
        self.supported
            .get(&capability)
            .is_some_and(|constraints| constraints.contains(constraint))
    }

    /// Iterates capabilities and their observed constraints in stable order.
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (Capability, &BTreeSet<CapabilityConstraint>)> {
        self.supported
            .iter()
            .map(|(capability, constraints)| (*capability, constraints))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact access states an operation will admit for one access profile.
pub struct AccessRequirement {
    profile_id: AccessProfileId,
    credential_states: BTreeSet<CredentialState>,
    entitlement_states: BTreeSet<EntitlementState>,
    endpoint_authorizations: BTreeSet<EndpointAuthorization>,
    runtime_readiness: BTreeSet<RuntimeReadiness>,
    support_authorities: BTreeSet<SupportAuthority>,
}

impl AccessRequirement {
    /// Starts an access requirement with no admitted states.
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
    /// Replaces the admitted credential states.
    pub fn with_credential_states(
        mut self,
        states: impl IntoIterator<Item = CredentialState>,
    ) -> Self {
        self.credential_states = states.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces the admitted entitlement states.
    pub fn with_entitlement_states(
        mut self,
        states: impl IntoIterator<Item = EntitlementState>,
    ) -> Self {
        self.entitlement_states = states.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces the admitted endpoint-authorization states.
    pub fn with_endpoint_authorizations(
        mut self,
        states: impl IntoIterator<Item = EndpointAuthorization>,
    ) -> Self {
        self.endpoint_authorizations = states.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces the admitted runtime-readiness states.
    pub fn with_runtime_readiness(
        mut self,
        states: impl IntoIterator<Item = RuntimeReadiness>,
    ) -> Self {
        self.runtime_readiness = states.into_iter().collect();
        self
    }

    #[must_use]
    /// Replaces the admitted support authorities.
    pub fn with_support_authorities(
        mut self,
        authorities: impl IntoIterator<Item = SupportAuthority>,
    ) -> Self {
        self.support_authorities = authorities.into_iter().collect();
        self
    }

    #[must_use]
    /// Returns the access profile the operation requires.
    pub const fn profile_id(&self) -> &AccessProfileId {
        &self.profile_id
    }

    #[must_use]
    /// Reports whether a credential state is admitted.
    pub fn accepts_credential(&self, state: CredentialState) -> bool {
        self.credential_states.contains(&state)
    }

    #[must_use]
    /// Reports whether an entitlement state is admitted.
    pub fn accepts_entitlement(&self, state: EntitlementState) -> bool {
        self.entitlement_states.contains(&state)
    }

    #[must_use]
    /// Reports whether an endpoint-authorization state is admitted.
    pub fn accepts_endpoint_authorization(&self, state: EndpointAuthorization) -> bool {
        self.endpoint_authorizations.contains(&state)
    }

    #[must_use]
    /// Reports whether a runtime-readiness state is admitted.
    pub fn accepts_runtime_readiness(&self, state: RuntimeReadiness) -> bool {
        self.runtime_readiness.contains(&state)
    }

    #[must_use]
    /// Reports whether a support authority is admitted.
    pub fn accepts_support_authority(&self, authority: SupportAuthority) -> bool {
        self.support_authorities.contains(&authority)
    }
}
