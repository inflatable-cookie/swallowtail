use super::Projection;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteValueDomain,
};

impl Projection<'_> {
    pub(in crate::consumer_route_projection) fn active_feature_row(
        &mut self,
        feature: ConsumerRouteFeatureId,
        state: ConsumerRouteStateSupport,
        value: ConsumerRouteControlValue,
    ) -> Option<ConsumerRouteProjectionRow> {
        let source = self.active_source.clone()?;
        Some(
            self.row(
                ConsumerRouteRowIdentity::Feature(feature),
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
            .with_control_value(value),
        )
    }

    pub(in crate::consumer_route_projection) fn local_feature(
        &mut self,
        semantic: &str,
    ) -> Option<ConsumerRouteFeatureId> {
        match ConsumerRouteNamespacedExtension::new(
            self.route.id(),
            self.plan.protocol_facade_id().as_str(),
            semantic,
        ) {
            Ok(extension) => Some(ConsumerRouteFeatureId::Namespaced(extension)),
            Err(error) => {
                self.rejected = Some(error);
                None
            }
        }
    }

    pub(in crate::consumer_route_projection) fn exact(
        &mut self,
        value: &str,
    ) -> Option<ConsumerRouteValueDomain> {
        match ConsumerRouteEnumerableValue::new(value)
            .and_then(|value| ConsumerRouteEnumeratedValues::new([value]))
        {
            Ok(values) => Some(ConsumerRouteValueDomain::Enumerated(values)),
            Err(error) => {
                self.rejected = Some(error);
                None
            }
        }
    }

    pub(in crate::consumer_route_projection) fn bounded(
        &mut self,
        value: &str,
    ) -> Option<ConsumerRouteValueDomain> {
        match ConsumerRouteEnumerableValue::new(value) {
            Ok(value) => Some(ConsumerRouteValueDomain::Unenumerated(value)),
            Err(error) => {
                self.rejected = Some(error);
                None
            }
        }
    }

    pub(in crate::consumer_route_projection) fn build(
        self,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(error) = self.rejected {
            return Err(error);
        }
        let sources = std::iter::once(self.prepared_source).chain(
            self.active_source
                .filter(|source| self.active.iter().any(|row| row.source() == source)),
        );
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            sources,
            self.selection,
            self.session_start,
            self.active,
        )
    }
}
