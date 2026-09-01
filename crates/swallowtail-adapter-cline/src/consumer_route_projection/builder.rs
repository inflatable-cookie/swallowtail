use super::{ProjectionBuilder, bounded, exact, feature_for};
use swallowtail_core::{Capability, CapabilityConstraint, HarnessMode};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionFailure, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass, ConsumerRouteStateSupport, ConsumerRouteValueKind,
};

impl ProjectionBuilder<'_> {
    pub(super) fn with_prepared_capabilities(mut self) -> Self {
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
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active_session.push(
                    self.prepared_row(
                        ConsumerRouteRowIdentity::Feature(feature),
                        ConsumerRouteSourceClass::PreparedOperationRecord,
                        ConsumerRouteEvidenceStrength::PreparedOperation,
                        ConsumerRouteLifecycle::PostOpenObservationOnly,
                    )
                    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                    .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
                );
            } else {
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
        }
        self
    }

    pub(super) fn with_harness_mode(mut self, pending: bool) -> Self {
        let Some(mode) = self
            .plan
            .requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == Capability::HarnessModeSelection)
            .and_then(|requirement| {
                requirement
                    .constraints()
                    .find_map(|constraint| match constraint {
                        CapabilityConstraint::HarnessMode(mode) => Some(*mode),
                        _ => None,
                    })
            })
        else {
            return self;
        };
        let control = match self.local_control("control.harness-mode") {
            Ok(control) => control,
            Err(rejection) => {
                self.rejected = Some(rejection);
                return self;
            }
        };
        if mode != HarnessMode::Plan {
            return self;
        }
        let domain = match exact("plan") {
            Ok(domain) => domain,
            Err(rejection) => {
                self.rejected = Some(rejection);
                return self;
            }
        };
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
            .with_control_value(ConsumerRouteControlValue::new(
                if pending {
                    ConsumerRouteValueKind::AcknowledgedEnumeration
                } else {
                    ConsumerRouteValueKind::BoundedEnumeration
                },
                domain,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            ));
        self.session_start.push(row);
        self
    }

    pub(super) fn with_plan_acknowledgement(
        mut self,
        value: &str,
        rejected: bool,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let source = self
            .active_source
            .clone()
            .expect("Plan acknowledgement names an active-session source");
        let state = if rejected {
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_rejected()
        } else {
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_provider_effective()
        };
        let identity = ConsumerRouteRowIdentity::Feature(
            self.local_feature("feature.active-session-plan-ack")?,
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
            .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
                source.id().clone(),
            ))
            .with_state_support(state)
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::AcknowledgementState,
                exact(value)?,
                ConsumerRouteOmissionSemantics::NotSelectable,
            ));
        self.active_session.push(row);
        Ok(self)
    }

    pub(super) fn with_model_observation(mut self) -> Result<Self, ConsumerRouteProjectionFailure> {
        let source = self
            .active_source
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
