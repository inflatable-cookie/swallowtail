use crate::OpenAiPreparedRealtimeSession;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    OpenRealtimeMediaSessionRequest,
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

const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
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

fn value(text: &str) -> ConsumerRouteEnumerableValue {
    ConsumerRouteEnumerableValue::new(text)
        .unwrap_or_else(|_| unreachable!("route-bounded projection text is admissible"))
}

fn single(text: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([value(text)])
            .unwrap_or_else(|_| unreachable!("one admitted value is within the maximum")),
    )
}

struct Builder<'a> {
    plan: &'a PreflightPlan,
    request: &'a OpenRealtimeMediaSessionRequest,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    observation_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Builder<'a> {
    /// Binds one contribution to the exact source identity the caller supplied.
    ///
    /// A prepared-only contribution names its adapter source. An opened
    /// contribution names the single active-session observation the accepted
    /// signature supplies, and each row keeps its own evidence class.
    fn new(
        session: &'a OpenAiPreparedRealtimeSession,
        source_id: ConsumerRouteProjectionSourceId,
        observed: bool,
    ) -> Self {
        let plan = session.plan();
        let identity = ConsumerRouteProjectionSourceIdentity::new(
            source_id,
            if observed {
                ConsumerRouteProjectionSourceKind::ActiveSessionObservation
            } else {
                ConsumerRouteProjectionSourceKind::AdapterContribution
            },
        );
        Self {
            plan,
            request: session.request(),
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: identity.clone(),
            observation_source: observed.then_some(identity),
            availability: availability(plan.access_status()),
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
        evidence_strength: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            source.clone(),
            source_class,
            evidence_strength,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    fn prepared_row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence_strength: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        self.row(
            identity,
            &self.prepared_source,
            source_class,
            evidence_strength,
            lifecycle,
        )
    }

    fn with_selection_summary(mut self) -> Self {
        self.selection.push(
            self.prepared_row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability()) else {
                continue;
            };
            self.selection.push(
                self.prepared_row(
                    ConsumerRouteRowIdentity::Feature(feature),
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            );
        }
        self
    }

    fn session_start_control(
        &self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) -> ConsumerRouteProjectionRow {
        self.prepared_row(
            ConsumerRouteRowIdentity::Control(control),
            ConsumerRouteSourceClass::AdapterPreparedInput,
            ConsumerRouteEvidenceStrength::RouteValidation,
            ConsumerRouteLifecycle::SessionStartOnly,
        )
        .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
        .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
            self.prepared_source.id().clone(),
        ))
        .with_state_support(
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_prepared(),
        )
        .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission))
    }

    /// Publishes the session-start reasoning control at its exact settled state.
    ///
    /// Before the acknowledgement arrives the row is pending. Only the exact
    /// parsed `session.updated` event moves it to effective or rejected.
    fn reasoning_control(
        &self,
        requested: &str,
        observed: ObservedReasoning<'_>,
    ) -> ConsumerRouteProjectionRow {
        let base = ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_prepared();
        let (source, class, strength, state, authority) = match observed {
            ObservedReasoning::None => (
                self.prepared_source.clone(),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                base.with_pending(),
                ConsumerRouteMutationAuthority::PreparedSessionStart(
                    self.prepared_source.id().clone(),
                ),
            ),
            ObservedReasoning::Effective(_) | ObservedReasoning::Rejected(_) => {
                let source = self
                    .observation_source
                    .clone()
                    .expect("an observed acknowledgement always names its observation source");
                let state = if matches!(observed, ObservedReasoning::Effective(_)) {
                    base.with_provider_effective()
                } else {
                    base.with_rejected()
                };
                let authority = ConsumerRouteMutationAuthority::Acknowledged(source.id().clone());
                (
                    source,
                    ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
                    ConsumerRouteEvidenceStrength::WireAcknowledgement,
                    state,
                    authority,
                )
            }
        };
        self.row(
            ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection),
            &source,
            class,
            strength,
            ConsumerRouteLifecycle::SessionStartOnly,
        )
        .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
        .with_mutation_authority(authority)
        .with_state_support(state)
        .with_control_value(ConsumerRouteControlValue::new(
            ConsumerRouteValueKind::AcknowledgedEnumeration,
            single(requested),
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ))
    }

    fn with_session_start_controls(mut self, observed: ObservedReasoning<'_>) -> Self {
        self.session_start.push(self.session_start_control(
            ConsumerRouteControlId::RealtimeMediaConfig,
            ConsumerRouteValueKind::FixedStructuredConfig,
            single("manual PCM 24 kHz input and output"),
            ConsumerRouteOmissionSemantics::Required,
        ));
        self.session_start.push(self.session_start_control(
            ConsumerRouteControlId::PlannedConnectionRollover,
            ConsumerRouteValueKind::BoundedPolicy,
            single("Disabled"),
            ConsumerRouteOmissionSemantics::Required,
        ));
        if let Some(maximum) = self.request.maximum_output_tokens() {
            self.session_start.push(self.session_start_control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                single(&maximum.get().to_string()),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            ));
        }
        if let Some(mode) = self.request.reasoning_mode() {
            let requested = mode.as_str().to_owned();
            self.session_start
                .push(self.reasoning_control(&requested, observed));
        }
        self
    }

    /// Publishes the exact acknowledged reasoning state as post-open truth.
    fn with_observed_reasoning(mut self, observed: ObservedReasoning<'_>) -> Self {
        let (effort, state) = match observed {
            ObservedReasoning::None => return self,
            ObservedReasoning::Effective(effort) => (
                effort,
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_provider_effective(),
            ),
            ObservedReasoning::Rejected(effort) => (
                effort,
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_rejected(),
            ),
        };
        let source = self
            .observation_source
            .clone()
            .expect("an observed acknowledgement always names its observation source");
        let authority = ConsumerRouteMutationAuthority::Acknowledged(source.id().clone());
        self.active_session.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                ),
                &source,
                ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
                ConsumerRouteEvidenceStrength::WireAcknowledgement,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
            .with_state_support(state)
            .with_mutation_authority(authority)
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::AcknowledgementState,
                single(effort),
                ConsumerRouteOmissionSemantics::NotSelectable,
            )),
        );
        self
    }

    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let observation = self
            .observation_source
            .filter(|source| source.id() != self.prepared_source.id());
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
    Builder::new(session, source_id, false)
        .with_selection_summary()
        .with_session_start_controls(ObservedReasoning::None)
        .build()
}

pub(crate) fn observed_contribution(
    session: &OpenAiPreparedRealtimeSession,
    observation_id: ConsumerRouteProjectionSourceId,
    observed: ObservedReasoning<'_>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Builder::new(session, observation_id, true)
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
