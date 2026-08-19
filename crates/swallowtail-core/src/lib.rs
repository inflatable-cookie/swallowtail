//! Provider-neutral records shared by Swallowtail adapters and consumers.
//!
//! This crate contains no execution traits, transport, process management, or
//! consumer product concepts.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod attached_runtime;
mod capability;
mod connection_lifecycle;
mod diagnostic;
mod direct_continuation;
mod event;
mod failure;
mod harness_configuration;
mod harness_mode;
mod harness_rpc;
mod identity;
mod installed_executable;
mod instance;
mod interface_version;
mod model;
mod model_artifact;
mod model_catalog;
mod observable_activity;
mod planned_connection_rollover;
mod preflight;
mod provider_agent;
mod provider_recovered_resource_cleanup;
mod provider_reference;
mod provider_session_catalogue;
mod provider_session_management;
mod realtime_media;
mod registration;
mod remote_acp;
mod remote_resource;
mod requirement;
mod runtime_identity;
mod session_access;
mod session_provider_state;

pub use access::{
    AccessProfile, AccessStatus, CredentialMechanism, CredentialState, EndpointAuthorization,
    EntitlementMetering, EntitlementState, RuntimeReadiness, SupportAuthority,
};
pub use attached_runtime::{
    AttachedModelObservation, AttachedModelObservationScope, AttachedModelTag,
    AttachedRuntimeRequirements, AttachedRuntimeResidency, InvalidAttachedRuntimeRecord,
    ModelManifestDigest,
};
pub use capability::{Capability, CapabilityManifest, UnsupportedCapability};
pub use connection_lifecycle::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, AdmittedInstanceRecord, AuthenticatedSubjectObservation,
    ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind, CredentialFieldDescriptor,
    CredentialFieldId, CredentialFieldVisibility, EnvironmentVariableName, FieldLabel,
    InstanceEnablement, InstanceLabel, OverlayMarker, RouteTopology, SubjectDisclosure,
};
pub use diagnostic::{Diagnostic, SafeDiagnostic, ValueRequired};
pub use direct_continuation::{
    DirectAttemptTransport, DirectContinuationConfig, DirectContinuationRequirements,
    DirectToolSelection, ProviderInferenceCachePolicy,
};
pub use event::{
    EventEnvelope, EventKind, ExtensionNamespace, ExtensionPolicy, ExtensionRejected,
    ProviderExtension,
};
pub use failure::{FailureClassification, FailureKind, FailureOrigin, FailureRecovery};
pub use harness_configuration::HarnessConfigurationPosture;
pub use harness_mode::HarnessMode;
pub use harness_rpc::{
    HarnessBackgroundAction, HarnessConfigurationSource, HarnessMessageClass, HarnessRpcPolicy,
    HarnessSchedulingBounds,
};
pub use identity::{AdapterId, AdapterIdentity, AdapterVersion};
pub use installed_executable::{
    InstalledExecutableCompatibility, InstalledExecutableObservation,
    InvalidInstalledExecutableObservation,
};
pub use instance::{ConfiguredInstance, ModelRoute};
pub use interface_version::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceCompatibilityMatch, InterfaceNewerVersionPosture,
    InterfaceSupportStatus, InterfaceUnverifiedNewer, InterfaceVersion, InterfaceVersionAxis,
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
pub use observable_activity::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityInterfaceBasis,
    ActivityKindClass, ActivityKindProfile, ActivityLifecycleFidelity, ActivityUnknownEventPosture,
    InvalidObservableActivityProfile, ObservableActivityAvailability, ObservableActivityProfile,
    SubagentControlActionKind, SubagentObservationFidelity,
};
pub use planned_connection_rollover::PlannedConnectionRolloverPolicy;
pub use preflight::{
    PreflightContext, PreflightDimension, PreflightFailure, PreflightPlan, StalePreflightPlan,
    preflight,
};
pub use provider_agent::{ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion};
pub use provider_recovered_resource_cleanup::ProviderRecoveredResourceCleanupEffect;
pub use provider_reference::{
    InvalidProviderActivityRef, ProviderActivityRef, ProviderRequestRef,
    ProviderRequestRepresentation, RunRef, SessionRef, TurnRef,
};
pub use provider_session_catalogue::{
    InvalidProviderSessionCatalogueRecord, InvalidProviderSessionCatalogueRecordKind,
    ProviderSessionActivityState, ProviderSessionCatalogueBounds, ProviderSessionDiscoveryScope,
    ProviderSessionDisplayContent, ProviderSessionImportAvailability,
    ProviderSessionImportUnavailableReason,
};
pub use provider_session_management::{
    ProviderSessionActivityEvidence, ProviderSessionAffectedScope, ProviderSessionBindingOrigin,
    ProviderSessionCancellationPosture, ProviderSessionDeletionStrength,
    ProviderSessionEffectTruth, ProviderSessionInitialStateRequirement,
    ProviderSessionInterfaceCompatibility, ProviderSessionLifecycleState,
    ProviderSessionManagementAction, ProviderSessionManagementEffect,
};
pub use realtime_media::{
    AudioEncoding, MediaDirection, MediaFormat, MediaKind, RealtimeMediaConfig,
    RealtimeMediaRequirements,
};
pub use registration::{
    DiscoveryAction, DiscoveryOutcome, DiscoveryStatus, DriverDescriptor, SignInAction,
};
pub use remote_acp::{
    InvalidRemoteAcpRequirements, REMOTE_ACP_WIRE_VERSION, RemoteAcpAffinityPolicy,
    RemoteAcpConnectionBounds, RemoteAcpCoreSdkVersion, RemoteAcpRequirements,
    RemoteAcpRfdRevision, RemoteAcpRfdStatus, RemoteAcpTransport, RemoteAcpTransportSdkVersion,
    RemoteAcpVersionEvidence,
};
pub use remote_resource::OwnedRemoteResourceKind;
pub use requirement::{
    AccessRequirement, CancellationScope, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, NamedCapabilityConstraint, OperationDetachmentScope,
    OperationRequirements, StructuredOutputEnforcement,
};
pub use runtime_identity::{
    AccessProfileId, ConfigFieldRef, ConfiguredInstanceId, CredentialRef, DriverRole,
    EndpointAudience, ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId, ModelRouteId,
    ModelRouteRevision, OperationShape, ProtocolFacadeId, TransportFamilyId,
};
pub use session_access::{
    ExternalNetworkPolicy, ExternalSearchPolicy, FilesystemBoundary, HarnessIsolation,
    IncompatibleSessionAccessPolicy, ProviderApprovalPolicy, ProviderRequestHandling,
    ProviderRequestPolicy, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
};
pub use session_provider_state::SessionProviderStatePolicy;
