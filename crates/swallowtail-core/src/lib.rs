//! Provider-neutral records shared by Swallowtail adapters and consumers.
//!
//! This crate contains no execution traits, transport, process management, or
//! consumer product concepts.

#![forbid(unsafe_code)]

mod access;
mod capability;
mod diagnostic;
mod event;
mod identity;
mod instance;
mod model;
mod preflight;
mod provider_reference;
mod registration;
mod requirement;
mod runtime_identity;
mod session_access;

pub use access::{
    AccessProfile, AccessStatus, CredentialMechanism, CredentialState, EndpointAuthorization,
    EntitlementMetering, EntitlementState, RuntimeReadiness, SupportAuthority,
};
pub use capability::{Capability, CapabilityManifest, UnsupportedCapability};
pub use diagnostic::{Diagnostic, SafeDiagnostic, ValueRequired};
pub use event::{
    EventEnvelope, EventKind, ExtensionNamespace, ExtensionPolicy, ExtensionRejected,
    ProviderExtension,
};
pub use identity::{AdapterId, AdapterIdentity, AdapterVersion};
pub use instance::{ConfiguredInstance, ModelRoute};
pub use model::{ModelCatalogEntry, ModelId, ModelMetadata, ReasoningMetadata, ReasoningMode};
pub use preflight::{
    PreflightContext, PreflightDimension, PreflightFailure, PreflightPlan, StalePreflightPlan,
    preflight,
};
pub use provider_reference::{ProviderRequestRef, RunRef, SessionRef, TurnRef};
pub use registration::{
    DiscoveryAction, DiscoveryOutcome, DiscoveryStatus, DriverDescriptor, SignInAction,
};
pub use requirement::{
    AccessRequirement, CancellationScope, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, NamedCapabilityConstraint, OperationRequirements,
};
pub use runtime_identity::{
    AccessProfileId, ConfiguredInstanceId, DriverRole, EndpointAudience, ExecutionHostId,
    ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, IntegrationFamilyId, ModelRouteId, ModelRouteRevision, OperationShape,
    ProtocolFacadeId, TransportFamilyId,
};
pub use session_access::{
    ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary,
    IncompatibleSessionAccessPolicy, ProviderApprovalPolicy, ProviderRequestHandling,
    ProviderRequestPolicy, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
};
