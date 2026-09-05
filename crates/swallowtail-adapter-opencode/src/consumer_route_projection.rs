//! Contract 061 contributions for prepared OpenCode operations.
//!
//! Rows are admitted from the exact prepared plan and bound request. The
//! feature matrix is not a source of projection truth; unsupported or
//! matrix-only rows stay absent at construction.

use crate::{
    OpenCodePreparedCatalogue, OpenCodePreparedDelete, OpenCodePreparedRun,
    OpenCodePreparedSession, OpenCodePreparedSessionCatalogue, OpenCodePreparedSessionHistory,
    OpenCodePreparedSessionImport, OpenCodePreparedSessionReconciliation,
};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    ExtensionNamespace, OperationShape, PreflightPlan, ProviderApprovalPolicy, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

const ROUTE_ID: &str = "opencode.http";
const PERMISSION_NAMESPACE: &str = "opencode/permission";
const QUESTION_NAMESPACE: &str = "opencode/question";

type Contribution = Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;

struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

mod builder;
impl OpenCodePreparedCatalogue {
    /// Emits the model-catalogue and prepared-facade rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl OpenCodePreparedRun {
    /// Emits the structured-run rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_question_and_permission()
            .with_owned_remote_cleanup()
            .with_run_controls(self)
            .build()
    }
}

impl OpenCodePreparedSession {
    /// Emits the interactive-session rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_question_and_permission()
            .with_session_controls()
            .build()
    }
}

impl OpenCodePreparedSessionCatalogue {
    /// Emits the retained-session catalogue rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .with_provider_session_query()
            .build()
    }
}

impl OpenCodePreparedSessionImport {
    /// Emits the retained-session import rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl OpenCodePreparedDelete {
    /// Emits the provider-session deletion rows proved by this operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan().preflight(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl OpenCodePreparedSessionHistory {
    /// Withholds history rows because they are outside candidate L's census tranche.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan().preflight(), source_id).build()
    }
}

impl OpenCodePreparedSessionReconciliation {
    /// Withholds matrix-only provider-turn-reference rows at construction.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan().preflight(), source_id).build()
    }
}

fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::StructuredOutput => ConsumerRouteFeatureId::StructuredOutput,
        Capability::Attachments => ConsumerRouteFeatureId::Attachments,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
        Capability::Resume => ConsumerRouteFeatureId::ResumeSession,
        Capability::ProviderSessionCatalogue => ConsumerRouteFeatureId::ProviderSessionCatalogue,
        Capability::ProviderSessionImport => ConsumerRouteFeatureId::ProviderSessionImport,
        Capability::ProviderSessionDelete => ConsumerRouteFeatureId::ProviderSessionDelete,
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::OwnedRemoteResourceDeletion => return None,
        _ => return None,
    })
}

fn has_capability(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}

fn namespaced_feature(
    namespace: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(ROUTE_ID, namespace, semantic_id)
}

fn namespaced_control(plan: &PreflightPlan, semantic_id: &str) -> ConsumerRouteControlId {
    ConsumerRouteControlId::Namespaced(
        ConsumerRouteNamespacedExtension::new(
            ROUTE_ID,
            plan.protocol_facade_id().as_str(),
            semantic_id,
        )
        .expect("static OpenCode projection identity is bounded"),
    )
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)
            .expect("static OpenCode projection bound is admissible"),
    )
}

fn prepared_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::PreparedSessionStart(source.id().clone())
}

fn per_turn_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(source.id().clone())
}

fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(status.credential(), CredentialState::Ready)
        && matches!(status.entitlement(), EntitlementState::Available)
        && matches!(
            status.endpoint_authorization(),
            EndpointAuthorization::Allowed
        )
        && matches!(status.runtime_readiness(), RuntimeReadiness::Ready)
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}

#[cfg(test)]
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
