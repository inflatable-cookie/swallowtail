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

#[path = "builder/capability.rs"]
mod capability;
#[path = "builder/control.rs"]
mod control;

#[derive(Clone, Copy)]
pub(crate) enum ProjectionRoute {
    Agent,
    CodeHeadless,
    CodeResponseOnly,
}

impl ProjectionRoute {
    pub(crate) const fn id(self) -> &'static str {
        match self {
            Self::Agent => "claude-agent.acp",
            Self::CodeHeadless => "claude-code.headless",
            Self::CodeResponseOnly => "claude-code.response-only",
        }
    }
}

enum Feature {
    Portable(ConsumerRouteFeatureId),
    Local(&'static str),
}

pub(crate) struct ProjectionBuilder<'a> {
    plan: &'a PreflightPlan,
    route: ProjectionRoute,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    observation_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> ProjectionBuilder<'a> {
    pub(crate) fn prepared(
        plan: &'a PreflightPlan,
        route: ProjectionRoute,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(plan, route, source_id, None)
    }

    pub(crate) fn observed(
        plan: &'a PreflightPlan,
        route: ProjectionRoute,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        observation_source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(plan, route, prepared_source_id, Some(observation_source_id))
    }

    fn new(
        plan: &'a PreflightPlan,
        route: ProjectionRoute,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        observation_source_id: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            route,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: ConsumerRouteProjectionSourceIdentity::new(
                prepared_source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            observation_source: observation_source_id.map(|id| {
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

    pub(crate) fn local_control(
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

    fn prepared_authority(&self) -> ConsumerRouteMutationAuthority {
        ConsumerRouteMutationAuthority::PreparedSessionStart(self.prepared_source.id().clone())
    }

    pub(crate) fn build(
        self,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(rejection) = self.rejected {
            return Err(rejection);
        }
        let observation = self.observation_source.filter(|source| {
            self.active_session
                .iter()
                .any(|row| row.source().id() == source.id())
        });
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            std::iter::once(self.prepared_source).chain(observation),
            self.selection,
            self.session_start,
            self.active_session,
        )
    }
}

fn feature_for(route: ProjectionRoute, capability: Capability) -> Option<Feature> {
    Some(match capability {
        Capability::StructuredRun => Feature::Portable(ConsumerRouteFeatureId::StructuredRun),
        Capability::InteractiveSession => {
            Feature::Portable(ConsumerRouteFeatureId::InteractiveSession)
        }
        Capability::StreamingEvents => Feature::Portable(ConsumerRouteFeatureId::StreamingEvents),
        Capability::UsageReporting => Feature::Portable(ConsumerRouteFeatureId::UsageEvidence),
        Capability::ReasoningSelection => {
            Feature::Portable(ConsumerRouteFeatureId::ReasoningSelection)
        }
        Capability::Interruption => {
            Feature::Portable(ConsumerRouteFeatureId::CancellationOrInterruption)
        }
        Capability::WorkingResource => Feature::Portable(ConsumerRouteFeatureId::WorkingResource),
        Capability::ObservableActivity => {
            Feature::Portable(ConsumerRouteFeatureId::ActivityObservation)
        }
        Capability::LoadSession if matches!(route, ProjectionRoute::Agent) => {
            Feature::Portable(ConsumerRouteFeatureId::LoadSession)
        }
        Capability::Resume if matches!(route, ProjectionRoute::Agent) => {
            Feature::Portable(ConsumerRouteFeatureId::ResumeSession)
        }
        Capability::ProviderSessionDelete if matches!(route, ProjectionRoute::Agent) => {
            Feature::Portable(ConsumerRouteFeatureId::ProviderSessionDelete)
        }
        Capability::ProviderDurableRetention | Capability::ProviderTemporaryRetention
            if matches!(route, ProjectionRoute::Agent) =>
        {
            Feature::Portable(ConsumerRouteFeatureId::PersistentSessionPosture)
        }
        Capability::ProviderNativeSessionClose if matches!(route, ProjectionRoute::Agent) => {
            Feature::Local("feature.native-session-close")
        }
        Capability::OwnedRemoteResourceDeletion if matches!(route, ProjectionRoute::Agent) => {
            Feature::Local("feature.owned-remote-resource-cleanup")
        }
        _ => return None,
    })
}

pub(crate) fn exact(
    value: &str,
) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

pub(crate) fn bounded(
    value: &str,
) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
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
