//! Contract 061 contribution emitted by the prepared Qoder headless run.
//!
//! Every row is proved by the exact prepared run plan and its capability
//! requirements. The route publishes no enumerable route-specific operation
//! option, so the census no-control audit stays negative coverage: the
//! session-start view is empty by construction rather than by a later filter.
//! The model-catalogue row is a matrix posture the prepared run cannot prove,
//! so it is withheld at construction.

use crate::QoderHeadlessPreparedRun;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture,
};

impl QoderHeadlessPreparedRun {
    /// Emits only the structured-run truth this prepared run proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// No catalogue, persistence, control, acknowledgement, or observation
    /// claim is constructed, because the prepared headless run proves none of
    /// them. Activity stays a post-open descriptor-only observation row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        RunProjection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

/// Collects contributed rows from one exact prepared Qoder run plan.
struct RunProjection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> RunProjection<'a> {
    fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            source: ConsumerRouteProjectionSourceIdentity::new(
                source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            availability: availability(plan.access_status()),
            selection: Vec::new(),
            active_session: Vec::new(),
        }
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
            source_class,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    /// Emits one selection-summary feature row per exact prepared capability.
    ///
    /// Activity observation keeps its post-open lifecycle and observation-only
    /// posture. Prepared capability evidence proves no observation, so its
    /// state support stays descriptor-only and never becomes observed,
    /// provider-effective, pending, or acknowledged truth.
    fn with_prepared_capabilities(mut self) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability()) else {
                continue;
            };
            if matches!(feature, ConsumerRouteFeatureId::ActivityObservation) {
                self.active_session.push(
                    self.row(
                        ConsumerRouteRowIdentity::Feature(feature),
                        ConsumerRouteSourceClass::PreparedOperationRecord,
                        ConsumerRouteLifecycle::PostOpenObservationOnly,
                    )
                    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                    .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
                );
                continue;
            }
            self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(feature),
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            );
        }
        self
    }

    /// Publishes the contribution with an intentionally empty control view.
    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            [self.source],
            self.selection,
            [],
            self.active_session,
        )
    }
}

/// Maps one exact prepared capability to portable feature identity.
///
/// `Capability::ModelCatalog` is deliberately absent: the prepared headless run
/// carries no catalogue observation, so a model-catalogue row could never be
/// constructed even if a capability profile advertised one. A capability
/// without a `qoder.headless` census row stays withheld the same way.
const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

/// Reports current availability without flattening the access dimensions.
///
/// The route is host-owned and unauthenticated, so `NotRequired` is the exact
/// satisfied credential state rather than a missing credential.
const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(
        status.credential(),
        CredentialState::Ready | CredentialState::NotRequired
    ) && matches!(status.entitlement(), EntitlementState::Available)
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
