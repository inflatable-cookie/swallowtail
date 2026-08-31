use super::{Feature, ProjectionBuilder, feature_for};
use swallowtail_core::Capability;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport,
};

impl ProjectionBuilder<'_> {
    pub(crate) fn with_prepared_capabilities(mut self) -> Self {
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
            let Some(feature) = feature_for(self.route, requirement.capability()) else {
                continue;
            };
            let feature = match feature {
                Feature::Portable(feature) => feature,
                Feature::Local(semantic_id) => match self.local_feature(semantic_id) {
                    Ok(feature) => feature,
                    Err(rejection) => {
                        self.rejected.get_or_insert(rejection);
                        continue;
                    }
                },
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

    pub(crate) fn with_additional_capability(mut self, capability: Capability) -> Self {
        let Some(feature) = feature_for(self.route, capability) else {
            return self;
        };
        let feature = match feature {
            Feature::Portable(feature) => feature,
            Feature::Local(semantic_id) => match self.local_feature(semantic_id) {
                Ok(feature) => feature,
                Err(rejection) => {
                    self.rejected.get_or_insert(rejection);
                    return self;
                }
            },
        };
        self.selection.push(
            self.prepared_row(
                ConsumerRouteRowIdentity::Feature(feature),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        self
    }

    pub(crate) fn with_callback_features(mut self) -> Self {
        if self.plan.requirements().extension_namespaces().len() == 0 {
            return self;
        }
        match self.local_feature("feature.permission-exchange") {
            Ok(permission) => self.selection.push(
                self.prepared_row(
                    ConsumerRouteRowIdentity::Feature(permission),
                    ConsumerRouteSourceClass::AdapterPreparedInput,
                    ConsumerRouteEvidenceStrength::RouteValidation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            ),
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
            }
        }
        self.selection.push(
            self.prepared_row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        self
    }
}
