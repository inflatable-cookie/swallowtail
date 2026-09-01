//! Contract 061 contributions for the prepared Cline adapter routes.

#[path = "consumer_route_projection/builder.rs"]
mod builder;
#[path = "consumer_route_projection/contribution.rs"]
mod contribution;
#[path = "consumer_route_projection/open.rs"]
mod open;

pub use open::{ClineProjectionOpenFailure, ClineProjectionOpenFuture, ClineProjectionOpenOutcome};

use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteAvailability, ConsumerRouteControlId,
    ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteNamespacedExtension, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteSupportPosture,
    ConsumerRouteValueDomain,
};

#[derive(Clone, Copy)]
enum ProjectionRoute {
    Acp,
    Headless,
}

impl ProjectionRoute {
    const fn id(self) -> &'static str {
        match self {
            Self::Acp => "cline.acp",
            Self::Headless => "cline.headless",
        }
    }
}

struct ProjectionBuilder<'a> {
    plan: &'a PreflightPlan,
    route: ProjectionRoute,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    active_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> ProjectionBuilder<'a> {
    fn prepared(
        plan: &'a PreflightPlan,
        route: ProjectionRoute,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(plan, route, source_id, None)
    }

    fn observed(
        plan: &'a PreflightPlan,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(
            plan,
            ProjectionRoute::Acp,
            prepared_source_id,
            Some(active_source_id),
        )
    }

    fn new(
        plan: &'a PreflightPlan,
        route: ProjectionRoute,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_source_id: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            route,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: ConsumerRouteProjectionSourceIdentity::new(
                prepared_source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            active_source: active_source_id.map(|id| {
                ConsumerRouteProjectionSourceIdentity::new(
                    id,
                    ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
                )
            }),
            availability: availability(plan.access_status()),
            rejected: None,
            selection: Vec::new(),
            session_start: Vec::new(),
            active_session: Vec::new(),
        }
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source: &ConsumerRouteProjectionSourceIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            source.clone(),
            source_class,
            evidence,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    fn prepared_row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        self.row(
            identity,
            &self.prepared_source,
            source_class,
            evidence,
            lifecycle,
        )
    }

    fn extension(
        &self,
        semantic_id: &str,
    ) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
        ConsumerRouteNamespacedExtension::new(
            self.route.id(),
            self.plan.protocol_facade_id().as_str(),
            semantic_id,
        )
    }

    fn local_control(
        &self,
        semantic_id: &str,
    ) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
        self.extension(semantic_id)
            .map(ConsumerRouteControlId::Namespaced)
    }

    fn local_feature(
        &self,
        semantic_id: &str,
    ) -> Result<ConsumerRouteFeatureId, ConsumerRouteProjectionFailure> {
        self.extension(semantic_id)
            .map(ConsumerRouteFeatureId::Namespaced)
    }

    fn prepared_authority(&self) -> ConsumerRouteMutationAuthority {
        ConsumerRouteMutationAuthority::PreparedSessionStart(self.prepared_source.id().clone())
    }

    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(rejection) = self.rejected {
            return Err(rejection);
        }
        let active_source = self.active_source.filter(|source| {
            self.active_session
                .iter()
                .any(|row| row.source().id() == source.id())
        });
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            std::iter::once(self.prepared_source).chain(active_source),
            self.selection,
            self.session_start,
            self.active_session,
        )
    }
}

fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

fn bounded(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    ConsumerRouteEnumerableValue::new(value).map(ConsumerRouteValueDomain::Unenumerated)
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
