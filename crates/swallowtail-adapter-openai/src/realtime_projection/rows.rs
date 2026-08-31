use super::contribution::{Builder, ObservedReasoning, feature_for, single};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture, ConsumerRouteValueDomain,
    ConsumerRouteValueKind,
};

impl Builder<'_> {
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

    pub(super) fn with_selection_summary(mut self) -> Self {
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

    /// Publishes the session-start reasoning control from prepared evidence.
    ///
    /// The row stays prepared truth on the prepared source. It is pending
    /// until an acknowledgement settles it, and the settled provider-effective
    /// or rejected state is published separately as active-session truth.
    fn reasoning_control(
        &self,
        requested: &str,
        observed: ObservedReasoning<'_>,
    ) -> ConsumerRouteProjectionRow {
        let mut state = ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_prepared();
        if matches!(observed, ObservedReasoning::None) {
            state = state.with_pending();
        }
        self.prepared_row(
            ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ReasoningSelection),
            ConsumerRouteSourceClass::AdapterPreparedInput,
            ConsumerRouteEvidenceStrength::RouteValidation,
            ConsumerRouteLifecycle::SessionStartOnly,
        )
        .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
        .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
            self.prepared_source.id().clone(),
        ))
        .with_state_support(state)
        .with_control_value(ConsumerRouteControlValue::new(
            ConsumerRouteValueKind::AcknowledgedEnumeration,
            single(requested),
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ))
    }

    pub(super) fn with_session_start_controls(mut self, observed: ObservedReasoning<'_>) -> Self {
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
    ///
    /// This row is the only place a provider-effective or rejected claim may
    /// appear, and it always names the distinct active-observation source.
    pub(super) fn with_observed_reasoning(mut self, observed: ObservedReasoning<'_>) -> Self {
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
        let source = self.observation_source();
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
}
