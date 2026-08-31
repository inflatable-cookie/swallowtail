use crate::OpenAiPreparedRealtimeSession;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteAvailability, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteFeatureId, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteValueDomain, OpenRealtimeMediaSessionRequest,
};

/// Exact acknowledgement state the prepared-open result observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ObservedReasoning<'a> {
    /// No post-open observation is claimed.
    None,
    /// The provider acknowledged exactly the selected effort.
    Effective(&'a str),
    /// The provider acknowledged an exact, well-formed different effort.
    Rejected(&'a str),
}

pub(super) const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::RealtimeMedia => ConsumerRouteFeatureId::RealtimeMediaSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        _ => return None,
    })
}

const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
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

pub(super) fn value(text: &str) -> ConsumerRouteEnumerableValue {
    ConsumerRouteEnumerableValue::new(text)
        .unwrap_or_else(|_| unreachable!("route-bounded projection text is admissible"))
}

pub(super) fn single(text: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([value(text)])
            .unwrap_or_else(|_| unreachable!("one admitted value is within the maximum")),
    )
}

/// Collects contributed rows from one exact prepared Realtime plan.
///
/// Prepared selection and session-start rows keep the prepared source. Only
/// post-open observation, provider-effective, and rejected truth may name the
/// separate active-session observation source.
pub(super) struct Builder<'a> {
    pub(super) plan: &'a PreflightPlan,
    pub(super) request: &'a OpenRealtimeMediaSessionRequest,
    pub(super) applicability: ConsumerRouteApplicability,
    pub(super) prepared_source: ConsumerRouteProjectionSourceIdentity,
    pub(super) observation_source: Option<ConsumerRouteProjectionSourceIdentity>,
    pub(super) availability: ConsumerRouteAvailability,
    pub(super) selection: Vec<ConsumerRouteProjectionRow>,
    pub(super) session_start: Vec<ConsumerRouteProjectionRow>,
    pub(super) active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Builder<'a> {
    /// Binds one contribution to the exact source identities the caller supplied.
    ///
    /// The prepared source always carries adapter-contribution evidence. An
    /// opened contribution additionally names one distinct active-session
    /// observation source, which never becomes the source of prepared truth.
    fn new(
        session: &'a OpenAiPreparedRealtimeSession,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        observation_source_id: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        let plan = session.plan();
        Self {
            plan,
            request: session.request(),
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
            selection: Vec::new(),
            session_start: Vec::new(),
            active_session: Vec::new(),
        }
    }

    /// Returns the distinct active-session observation source.
    pub(super) fn observation_source(&self) -> ConsumerRouteProjectionSourceIdentity {
        self.observation_source
            .clone()
            .expect("an observed acknowledgement always names its observation source")
    }

    /// Names only the sources that actually proved an admitted row.
    ///
    /// An open that observed no reasoning acknowledgement publishes no
    /// active-session row, so it never names an observation source that proved
    /// nothing.
    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let observation = self.observation_source.filter(|source| {
            self.selection
                .iter()
                .chain(&self.session_start)
                .chain(&self.active_session)
                .any(|row| row.source().id() == source.id())
        });
        let sources = std::iter::once(self.prepared_source).chain(observation);
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            sources,
            self.selection,
            self.session_start,
            self.active_session,
        )
    }
}

pub(crate) fn prepared_contribution(
    session: &OpenAiPreparedRealtimeSession,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Builder::new(session, source_id, None)
        .with_selection_summary()
        .with_session_start_controls(ObservedReasoning::None)
        .build()
}

pub(crate) fn observed_contribution(
    session: &OpenAiPreparedRealtimeSession,
    prepared_source_id: ConsumerRouteProjectionSourceId,
    observation_source_id: ConsumerRouteProjectionSourceId,
    observed: ObservedReasoning<'_>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Builder::new(session, prepared_source_id, Some(observation_source_id))
        .with_selection_summary()
        .with_session_start_controls(observed)
        .with_observed_reasoning(observed)
        .build()
}

impl OpenAiPreparedRealtimeSession {
    /// Emits only the prepared Realtime selection and session-start truth.
    ///
    /// Provider-effective or rejected reasoning state is never inferred here.
    /// It comes only from the prepared-open result that parsed the exact
    /// `session.updated` acknowledgement.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        prepared_contribution(self, source_id)
    }
}
