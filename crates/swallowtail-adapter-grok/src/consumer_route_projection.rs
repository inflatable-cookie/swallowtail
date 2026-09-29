//! Contract 061 contributions for prepared Grok Build ACP operations.
//!
//! Prepared sessions and runs publish only their own plan-backed rows. The
//! negotiated model-options row is produced only by the additive successful-open seam.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/contribution.rs"]
mod contribution;
#[path = "consumer_route_projection/open.rs"]
mod open;

pub use open::{GrokProjectionOpenFailure, GrokProjectionOpenFuture, GrokProjectionOpenOutcome};

use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteAvailability, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteSupportPosture,
    ConsumerRouteValueDomain,
};

struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    active_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active: Vec<ConsumerRouteProjectionRow>,
    rejected: Option<ConsumerRouteProjectionFailure>,
}
impl<'a> Projection<'a> {
    fn new(plan: &'a PreflightPlan, source: ConsumerRouteProjectionSourceId) -> Self {
        Self::sources(plan, source, None)
    }
    fn observed(
        plan: &'a PreflightPlan,
        prepared: ConsumerRouteProjectionSourceId,
        active: ConsumerRouteProjectionSourceId,
        options: bool,
    ) -> Self {
        let mut projection = Self::sources(plan, prepared, Some(active));
        if options {
            projection.model_observation();
        }
        projection
    }
    fn sources(
        plan: &'a PreflightPlan,
        prepared: ConsumerRouteProjectionSourceId,
        active: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: ConsumerRouteProjectionSourceIdentity::new(
                prepared,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            active_source: active.map(|id| {
                ConsumerRouteProjectionSourceIdentity::new(
                    id,
                    ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
                )
            }),
            availability: availability(plan.access_status()),
            selection: Vec::new(),
            session_start: Vec::new(),
            active: Vec::new(),
            rejected: None,
        }
    }
    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source: &ConsumerRouteProjectionSourceIdentity,
        class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            source.clone(),
            class,
            evidence,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }
    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(error) = self.rejected {
            return Err(error);
        };
        let sources =
            std::iter::once(self.prepared_source).chain(self.active_source.filter(|source| {
                self.active
                    .iter()
                    .any(|row| row.source().id() == source.id())
            }));
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            sources,
            self.selection,
            self.session_start,
            self.active,
        )
    }
}
const fn feature_for(capability: Capability, session: bool) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::InteractiveSession if session => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StructuredRun if !session => ConsumerRouteFeatureId::StructuredRun,
        Capability::ModelCatalog if !session => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => return None,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}
fn exact(
    value: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteValueDomain> {
    match ConsumerRouteEnumerableValue::new(value)
        .and_then(|value| ConsumerRouteEnumeratedValues::new([value]))
    {
        Ok(values) => Some(ConsumerRouteValueDomain::Enumerated(values)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
}
fn bounded(
    value: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteValueDomain> {
    match ConsumerRouteEnumerableValue::new(value) {
        Ok(value) => Some(ConsumerRouteValueDomain::Unenumerated(value)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
}
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
