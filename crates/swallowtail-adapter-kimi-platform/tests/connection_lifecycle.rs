//! Contract 057 lifecycle proof for hosted Kimi Platform Chat: admission,
//! API-key collection, readiness refresh, subject observation, and the 047
//! snapshot plus model-presentation overlay.
//!
//! Deterministic harness only: no live provider calls, no browser ports, no
//! secret bytes in portable records.

#[allow(dead_code)]
mod support;

use std::num::NonZeroU64;
use std::sync::Arc;
use support::ThreadServices;
use swallowtail_adapter_kimi_platform::{
    KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID, KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID,
    KIMI_PLATFORM_ENDPOINT_AUDIENCE, KIMI_PLATFORM_FACADE_REVISION, KIMI_PLATFORM_MODEL_ID,
    KIMI_PLATFORM_PROVIDER_ID, KimiPlatformCatalogueProfileInput,
    KimiPlatformInferenceAttemptInput, KimiPlatformModelSelection, KimiPlatformPreparationInput,
    KimiPlatformPreparedIntegration, kimi_platform_chat_addable_route_descriptor,
    kimi_platform_direct_descriptor, prepare_kimi_platform_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, AuthenticatedSubjectObservation, ConfigFieldId,
    ConfigFieldRef, ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState,
    DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, InstanceEnablement, InstanceRevision, IntegrationFamilyId, ModelCatalogEntry,
    ModelId, ModelMetadata, ModelRouteId, ModelRouteRevision, OverlayMarker, ProviderId,
    ReasoningMode, RuntimeReadiness, SubjectDisclosure, SupportAuthority,
};
use swallowtail_host_local::{
    LocalProcessHost, LocalProcessLimits, MemoryConnectionLifecycleStore,
};
use swallowtail_runtime::{
    AddableRouteCatalog, BlockingWorkService, ConfiguredProviderInstanceAdmission,
    ConfiguredProviderInstanceRecord, ConfiguredProviderInstanceSelectionReadiness,
    ConfiguredProviderModelCatalogueInput, ConnectionLifecycleStore, CredentialService,
    HostServices, InstanceAdmissionRequest, NetworkPolicyService, OperationContent,
    PreparedAccessEvidence, ReadinessRefreshRequest, RequestId, ScopeId, ScopedTaskService,
    SignInAuthorityBinding, SignInMethod, SignInStartRequest, SignInStatus, TimeService,
    admit_instance, apply_stored_model_presentation_overlay, complete_sign_in,
    observe_authenticated_subject, poll_sign_in, refresh_readiness, start_sign_in,
    submit_sign_in_credential_field,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const INSTANCE: &str = "kimi-platform.work";
const CREDENTIAL_REF: &str = "kimi-platform.work.api-key";

include!("connection_lifecycle/admission.rs");
include!("connection_lifecycle/preparation.rs");
include!("connection_lifecycle/failures.rs");
include!("connection_lifecycle/refresh.rs");
include!("connection_lifecycle/catalogue.rs");
include!("connection_lifecycle/contract_047.rs");
include!("connection_lifecycle/support.rs");
