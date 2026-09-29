use super::{Projection, bounded, exact, feature_for};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

impl Projection<'_> {
    pub(super) fn prepared(mut self, session: bool) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                &self.prepared_source,
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability(), session) else {
                continue;
            };
            let row = self
                .row(
                    ConsumerRouteRowIdentity::Feature(feature.clone()),
                    &self.prepared_source,
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    if feature == ConsumerRouteFeatureId::ActivityObservation {
                        ConsumerRouteLifecycle::PostOpenObservationOnly
                    } else {
                        ConsumerRouteLifecycle::SelectionSummary
                    },
                )
                .with_actor_posture(if feature == ConsumerRouteFeatureId::ActivityObservation {
                    ConsumerRouteActorPosture::ObservationOnly
                } else {
                    ConsumerRouteActorPosture::Informational
                });
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active
                    .push(row.with_state_support(ConsumerRouteStateSupport::descriptor_only()));
            } else {
                self.selection.push(row);
            }
        }
        if !self.selection.iter().any(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence)
        }) {
            self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::UsageEvidence),
                    &self.prepared_source,
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            );
        }
        self
    }
    pub(super) fn model_selection(mut self) -> Self {
        if let Some(model) = self.applicability.model() {
            let domain = exact(model.model_id().as_str(), &mut self.rejected);
            self.push_control(
                ConsumerRouteControlId::ModelSelection,
                ConsumerRouteValueKind::ExactModelRoute,
                domain,
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        self
    }
    pub(super) fn session_options(mut self) -> Self {
        let domain = bounded(
            "validated empty Grok ACP session options",
            &mut self.rejected,
        );
        self.push_control(
            ConsumerRouteControlId::SessionOptions,
            ConsumerRouteValueKind::StructuredOptions,
            domain,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }
    pub(super) fn model_observation(&mut self) {
        let Some(source) = self.active_source.clone() else {
            return;
        };
        let identity = match ConsumerRouteNamespacedExtension::new(
            "grok-build.acp",
            self.plan.protocol_facade_id().as_str(),
            "feature.negotiated-model-options-observation",
        ) {
            Ok(extension) => {
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(extension))
            }
            Err(error) => {
                self.rejected = Some(error);
                return;
            }
        };
        let value = bounded(
            "exact bounded negotiated model options on the open session",
            &mut self.rejected,
        );
        if let Some(value) = value {
            self.active.push(
                self.row(
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
                    value,
                    ConsumerRouteOmissionSemantics::NotSelectable,
                )),
            );
        }
    }
    fn push_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: Option<ConsumerRouteValueDomain>,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let Some(domain) = domain else { return };
        self.session_start.push(
            self.row(
                ConsumerRouteRowIdentity::Control(control),
                &self.prepared_source,
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
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission)),
        );
    }
}
