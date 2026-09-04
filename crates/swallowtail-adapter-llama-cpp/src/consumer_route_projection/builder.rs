use super::{availability, bounded, exact, namespaced_control, route_local};
use swallowtail_core::{Capability, InstanceOwnership, PreflightPlan};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueKind,
};

/// Collects contributed rows from one exact prepared llama.cpp plan.
pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: &'static str,
    feature_for: fn(Capability) -> Option<ConsumerRouteFeatureId>,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    pub(super) fn new(
        plan: &'a PreflightPlan,
        route: &'static str,
        source_id: ConsumerRouteProjectionSourceId,
        feature_for: fn(Capability) -> Option<ConsumerRouteFeatureId>,
    ) -> Self {
        Self {
            plan,
            route,
            feature_for,
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

    fn segment(&self) -> &'a str {
        self.plan.protocol_facade_id().as_str()
    }

    fn row(
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

    pub(super) fn with_prepared_capabilities(mut self) -> Self {
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
            let Some(feature) = (self.feature_for)(requirement.capability()) else {
                continue;
            };
            if matches!(feature, ConsumerRouteFeatureId::ActivityObservation) {
                self.active_session.push(
                    self.row(
                        ConsumerRouteRowIdentity::Feature(feature),
                        ConsumerRouteSourceClass::PreparedOperationRecord,
                        ConsumerRouteEvidenceStrength::PreparedOperation,
                        ConsumerRouteLifecycle::PostOpenObservationOnly,
                    )
                    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                    .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
                );
                continue;
            }
            self.selection.push(
                self.row(
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

    pub(super) fn with_owned_runtime_lifecycle(mut self) -> Self {
        if self.plan.ownership() != InstanceOwnership::HostOwnedEphemeral {
            return self;
        }
        match route_local(
            self.route,
            self.segment(),
            "feature.owned-runtime-lifecycle",
        ) {
            Ok(extension) => self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(
                        extension,
                    )),
                    ConsumerRouteSourceClass::PreparedOperationRecord,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            ),
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
            }
        }
        self
    }

    pub(super) fn with_model_selection(mut self) -> Self {
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
            .row(
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

    pub(super) fn with_maximum_output_tokens(
        mut self,
        maximum: Option<std::num::NonZeroU64>,
    ) -> Self {
        let Some(maximum) = maximum else {
            return self;
        };
        self.push_enumerated(
            ConsumerRouteControlId::MaximumOutputTokens,
            ConsumerRouteValueKind::BoundedInteger,
            &maximum.get().to_string(),
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }

    pub(super) fn with_serving_model_artifact(mut self) -> Self {
        let control = match namespaced_control(
            self.route,
            self.segment(),
            "control.serving-model-artifact",
        ) {
            Ok(control) => control,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        let domain = match bounded("exact model artifact binding and model route") {
            Ok(domain) => domain,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        self.push_session_control(
            control,
            ConsumerRouteValueKind::StructuredDeclarations,
            domain,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }

    pub(super) fn with_serving_context_size(
        mut self,
        context_size: Option<crate::LlamaCppContextSize>,
    ) -> Self {
        let Some(context_size) = context_size else {
            return self;
        };
        let control =
            match namespaced_control(self.route, self.segment(), "control.serving-context-size") {
                Ok(control) => control,
                Err(rejection) => {
                    self.rejected.get_or_insert(rejection);
                    return self;
                }
            };
        self.push_enumerated(
            control,
            ConsumerRouteValueKind::BoundedInteger,
            &context_size.as_u32().to_string(),
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        );
        self
    }

    pub(super) fn with_serving_reasoning(
        mut self,
        reasoning: Option<crate::LlamaCppReasoningSelection>,
    ) -> Self {
        let Some(reasoning) = reasoning else {
            return self;
        };
        let control =
            match namespaced_control(self.route, self.segment(), "control.serving-reasoning") {
                Ok(control) => control,
                Err(rejection) => {
                    self.rejected.get_or_insert(rejection);
                    return self;
                }
            };
        self.push_enumerated(
            control,
            ConsumerRouteValueKind::BoundedEnumeration,
            reasoning.as_argument_value(),
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        );
        self
    }

    fn push_enumerated(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let domain = match exact(value) {
            Ok(domain) => domain,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return;
            }
        };
        self.push_session_control(control, kind, domain, omission);
    }

    fn push_session_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: swallowtail_runtime::ConsumerRouteValueDomain,
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
            .with_mutation_authority(self.prepared_authority())
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.session_start.push(row);
    }

    fn prepared_authority(&self) -> ConsumerRouteMutationAuthority {
        ConsumerRouteMutationAuthority::PreparedSessionStart(self.source.id().clone())
    }

    pub(super) fn build(
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
