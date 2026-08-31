use super::CodexProjectionBuilder;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEnumerableValue, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

impl CodexProjectionBuilder<'_> {
    pub(crate) fn push_session_start_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(control),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SessionStartOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(prepared_authority(&self.source))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.session_start.push(row);
    }

    /// Publishes the consumer-mediated per-turn user-input exchange.
    ///
    /// The row stays per-turn under `ConsumerMediatedPerTurn`. It never claims
    /// prepared session-start state and never implies provider mutation or
    /// acknowledgement.
    pub(crate) fn push_per_turn_exchange(&mut self) {
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::UserInputExchange),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::PerTurn,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(per_turn_authority(&self.source))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_observed(),
            )
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::ExchangeCallback,
                ConsumerRouteValueDomain::Unenumerated(
                    ConsumerRouteEnumerableValue::new(
                        "consumer-mediated question and user-input exchange",
                    )
                    .unwrap_or_else(|_| unreachable!("static bound text is admissible")),
                ),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            ));
        self.session_start.push(row);
    }

    pub(crate) fn push_observed_query(&mut self, control: ConsumerRouteControlId, bound: &str) {
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(control),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
            .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::BoundedQuery,
                ConsumerRouteValueDomain::Unenumerated(
                    ConsumerRouteEnumerableValue::new(bound)
                        .unwrap_or_else(|_| unreachable!("static bound text is admissible")),
                ),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            ));
        self.active_session.push(row);
    }
}

pub(super) fn prepared_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::PreparedSessionStart(source.id().clone())
}

pub(super) fn per_turn_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(source.id().clone())
}

pub(crate) fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)
            .unwrap_or_else(|_| unreachable!("static bound text is admissible")),
    )
}
