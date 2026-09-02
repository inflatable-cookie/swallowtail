use super::{ProjectionBuilder, bounded, exact};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics, ConsumerRouteProjectionFailure,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

impl ProjectionBuilder<'_> {
    pub(crate) fn with_model_selection(mut self) -> Self {
        let Some(model) = self.applicability.model() else {
            return self;
        };
        let domain = match exact(model.model_id().as_str()) {
            Ok(domain) => domain,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        let row = self
            .prepared_row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(self.prepared_authority())
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
        self
    }

    pub(crate) fn push_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
        pending: bool,
    ) {
        let mut state = ConsumerRouteStateSupport::descriptor_only()
            .with_requested()
            .with_prepared();
        if pending {
            state = state.with_pending();
        }
        let row = self
            .prepared_row(
                ConsumerRouteRowIdentity::Control(control),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SessionStartOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(self.prepared_authority())
            .with_state_support(state)
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.session_start.push(row);
    }

    pub(crate) fn with_observed_reasoning(
        mut self,
        reasoning: &str,
        rejected: bool,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let source = self
            .observation_source
            .clone()
            .expect("observed reasoning names an active-session source");
        let state = if rejected {
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_rejected()
        } else {
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_provider_effective()
        };
        let row = self
            .row(
                ConsumerRouteRowIdentity::Feature(
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                ),
                &source,
                ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
                ConsumerRouteEvidenceStrength::WireAcknowledgement,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
            .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
                source.id().clone(),
            ))
            .with_state_support(state)
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::AcknowledgementState,
                exact(reasoning)?,
                ConsumerRouteOmissionSemantics::NotSelectable,
            ));
        self.active_session.push(row);
        Ok(self)
    }

    pub(crate) fn with_model_observation(mut self) -> Result<Self, ConsumerRouteProjectionFailure> {
        let source = self
            .observation_source
            .clone()
            .expect("model observation names an active-session source");
        let identity = ConsumerRouteRowIdentity::Feature(
            self.local_feature("feature.negotiated-model-options-observation")?,
        );
        let row = self
            .row(
                identity,
                &source,
                ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
                ConsumerRouteEvidenceStrength::WireAcknowledgement,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
            .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::Observation,
                bounded("exact bounded negotiated model options on the open session")?,
                ConsumerRouteOmissionSemantics::NotSelectable,
            ));
        self.active_session.push(row);
        Ok(self)
    }
}
