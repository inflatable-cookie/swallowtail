//! Contract 061 contributions for the prepared Gemini routes.
//!
//! Rows are constructed only from the exact prepared plan and bound request.
//! Negotiated model options stay withheld until the additive open path observes
//! them on a successfully opened ACP session.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/contribution.rs"]
mod contribution;
#[path = "consumer_route_projection/open.rs"]
mod open;

pub use open::{
    GeminiProjectionOpenFailure, GeminiProjectionOpenFuture, GeminiProjectionOpenOutcome,
};

use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteAvailability, ConsumerRouteControlId,
    ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteSupportPosture,
    ConsumerRouteValueDomain,
};

#[derive(Clone, Copy)]
enum Route {
    Acp,
    Headless,
    Live,
}
impl Route {
    const fn id(self) -> &'static str {
        match self {
            Self::Acp => "gemini-cli.acp",
            Self::Headless => "gemini-cli.headless",
            Self::Live => "gemini.live",
        }
    }
}

struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: Route,
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
    fn new(plan: &'a PreflightPlan, route: Route, source: ConsumerRouteProjectionSourceId) -> Self {
        Self::sources(plan, route, source, None)
    }
    fn observed(
        plan: &'a PreflightPlan,
        route: Route,
        prepared: ConsumerRouteProjectionSourceId,
        active: ConsumerRouteProjectionSourceId,
        has_options: bool,
    ) -> Self {
        let mut projection = Self::sources(plan, route, prepared, Some(active));
        if has_options {
            projection.model_observation();
        }
        projection
    }
    fn sources(
        plan: &'a PreflightPlan,
        route: Route,
        prepared: ConsumerRouteProjectionSourceId,
        active: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            route,
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
fn feature_for(route: Route, capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::InteractiveSession if !matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::InteractiveSession
        }
        Capability::RealtimeMedia if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::RealtimeMediaSession
        }
        Capability::StructuredRun if matches!(route, Route::Headless) => {
            ConsumerRouteFeatureId::StructuredRun
        }
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource if !matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::WorkingResource
        }
        Capability::OutputTokenLimit if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::OutputTokenLimit
        }
        Capability::ReasoningSelection if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::ReasoningSelection
        }
        Capability::PlannedConnectionRollover if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::Namespaced(
                ConsumerRouteNamespacedExtension::new(
                    route.id(),
                    "gemini.live",
                    "feature.planned-connection-rollover",
                )
                .ok()?,
            )
        }
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}
fn namespaced(
    route: Route,
    plan: &PreflightPlan,
    semantic: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteControlId> {
    match ConsumerRouteNamespacedExtension::new(
        route.id(),
        plan.protocol_facade_id().as_str(),
        semantic,
    ) {
        Ok(extension) => Some(ConsumerRouteControlId::Namespaced(extension)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
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
