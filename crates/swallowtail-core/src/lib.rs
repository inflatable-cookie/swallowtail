//! Provider-neutral records shared by Swallowtail adapters and consumers.
//!
//! This crate contains no execution traits, transport, process management, or
//! consumer product concepts.

#![forbid(unsafe_code)]

mod access;
mod capability;
mod diagnostic;
mod event;
mod harness_rpc;
mod identity;
mod instance;
mod interface_version;
mod model;
mod model_artifact;
mod model_catalog;
mod planned_connection_rollover;
mod preflight;
mod provider_agent;
mod provider_reference;
mod realtime_media;
mod registration;
mod remote_resource;
mod requirement;
mod runtime_identity;
mod session_access;
mod session_provider_state;

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
pub use harness_rpc::{
    HarnessBackgroundAction, HarnessConfigurationSource, HarnessMessageClass, HarnessRpcPolicy,
    HarnessSchedulingBounds,
};
pub use identity::{AdapterId, AdapterIdentity, AdapterVersion};
pub use instance::{ConfiguredInstance, ModelRoute};
pub use interface_version::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceCompatibilityMatch, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
    InvalidInterfaceCompatibilityClaim,
};
pub use model::{
    ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, ProviderId, ReasoningMetadata,
    ReasoningMode,
};
pub use model_artifact::{
    ModelArtifactBinding, ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat,
    ModelArtifactId, ModelArtifactRef, ModelArtifactRevision,
};
pub use model_catalog::{
    CatalogObservation, CatalogTimestamp, InvalidCatalogObservation, ModelCatalogObservations,
    ModelCustomizationType, ModelInferenceType, ModelLifecycleObservation, ModelLifecycleStatus,
    ModelLifecycleTransition, ModelModality, ProviderCatalogValue,
};
pub use planned_connection_rollover::PlannedConnectionRolloverPolicy;
pub use preflight::{
    PreflightContext, PreflightDimension, PreflightFailure, PreflightPlan, StalePreflightPlan,
    preflight,
};
pub use provider_agent::{ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion};
pub use provider_reference::{ProviderRequestRef, RunRef, SessionRef, TurnRef};
pub use realtime_media::{
    AudioEncoding, MediaDirection, MediaFormat, MediaKind, RealtimeMediaConfig,
    RealtimeMediaRequirements,
};
pub use registration::{
    DiscoveryAction, DiscoveryOutcome, DiscoveryStatus, DriverDescriptor, SignInAction,
};
pub use remote_resource::OwnedRemoteResourceKind;
pub use requirement::{
    AccessRequirement, CancellationScope, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, NamedCapabilityConstraint, OperationRequirements,
};
pub use runtime_identity::{
    AccessProfileId, ConfiguredInstanceId, CredentialRef, DriverRole, EndpointAudience,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, ModelRouteId, ModelRouteRevision,
    OperationShape, ProtocolFacadeId, TransportFamilyId,
};
pub use session_access::{
    ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary, HarnessIsolation,
    IncompatibleSessionAccessPolicy, ProviderApprovalPolicy, ProviderRequestHandling,
    ProviderRequestPolicy, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
};
pub use session_provider_state::SessionProviderStatePolicy;
