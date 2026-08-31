use super::controls::prepared_authority;
use super::{availability, feature_for};
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

/// Collects contributed rows from one exact prepared Codex plan.
pub(crate) struct CodexProjectionBuilder<'a> {
    pub(super) plan: &'a PreflightPlan,
    pub(super) applicability: ConsumerRouteApplicability,
    pub(super) source: ConsumerRouteProjectionSourceIdentity,
    pub(super) availability: ConsumerRouteAvailability,
    pub(super) rejected: Option<ConsumerRouteProjectionFailure>,
    pub(super) selection: Vec<ConsumerRouteProjectionRow>,
    pub(super) session_start: Vec<ConsumerRouteProjectionRow>,
    pub(super) active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> CodexProjectionBuilder<'a> {
    pub(crate) fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            source: ConsumerRouteProjectionSourceIdentity::new(
                source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            availability: availability(plan.access_status()),
            rejected: None,
            selection: Vec::new(),
            session_start: Vec::new(),
            active_session: Vec::new(),
        }
    }

    pub(super) fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence_strength: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
            source_class,
            evidence_strength,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    /// Emits one selection-summary feature row per exact prepared capability.
    pub(crate) fn with_prepared_capabilities(mut self) -> Self {
        self.selection.push(
            self.row(
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
            if matches!(feature, ConsumerRouteFeatureId::ActivityObservation) {
                let row = self.row(
                    ConsumerRouteRowIdentity::Feature(feature),
                    ConsumerRouteSourceClass::PreparedOperationRecord,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                );
                self.active_session.push(
                    row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                        .with_state_support(
                            ConsumerRouteStateSupport::descriptor_only().with_observed(),
                        ),
                );
                continue;
            }
            let row = self.row(
                ConsumerRouteRowIdentity::Feature(feature),
                ConsumerRouteSourceClass::CapabilityProfile,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            );
            self.selection
                .push(row.with_actor_posture(ConsumerRouteActorPosture::Informational));
        }
        self
    }

    /// Emits the consumer-mediated question-exchange feature when admitted.
    pub(crate) fn with_question_exchange(mut self) -> Self {
        if self
            .plan
            .requirements()
            .session_access_policy()
            .is_some_and(|policy| policy.provider_requests().exchanged_extensions().len() > 0)
        {
            self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange),
                    ConsumerRouteSourceClass::AdapterPreparedInput,
                    ConsumerRouteEvidenceStrength::RouteValidation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            );
        }
        self
    }

    /// Emits the exact selected model route as a selection-time control.
    pub(crate) fn with_model_selection(mut self) -> Self {
        if let Some(model) = self.applicability.model() {
            let admitted = match ConsumerRouteEnumerableValue::new(model.model_id().as_str()) {
                Ok(value) => value,
                Err(rejection) => {
                    self.rejected.get_or_insert(rejection);
                    return self;
                }
            };
            let domain = ConsumerRouteValueDomain::Enumerated(
                ConsumerRouteEnumeratedValues::new([admitted])
                    .unwrap_or_else(|_| unreachable!("one admitted value is within the maximum")),
            );
            let row = self
                .row(
                    ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection),
                    ConsumerRouteSourceClass::PreparedOperationRecord,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
                .with_mutation_authority(prepared_authority(&self.source))
                .with_state_support(
                    ConsumerRouteStateSupport::descriptor_only()
                        .with_requested()
                        .with_prepared(),
                )
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::ExactModelRoute,
                    domain,
                    ConsumerRouteOmissionSemantics::Required,
                ));
            self.selection.push(row);
        }
        self
    }

    pub(crate) fn build(
        self,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(rejection) = self.rejected {
            return Err(rejection);
        }
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            [self.source],
            self.selection,
            self.session_start,
            self.active_session,
        )
    }
}
