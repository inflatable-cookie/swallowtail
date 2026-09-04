use super::{availability, bounded, exact, feature_for, namespaced_control};
use swallowtail_core::PreflightPlan;
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

/// Collects contributed rows from one exact prepared Ollama plan.
pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    pub(super) fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
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
            let Some(feature) = feature_for(requirement.capability()) else {
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

    pub(super) fn with_reasoning_selection(mut self, mode: &str) -> Self {
        self.push_enumerated(
            ConsumerRouteControlId::ReasoningSelection,
            ConsumerRouteValueKind::BoundedEnumeration,
            mode,
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        );
        self
    }

    pub(super) fn with_structured_output(mut self) -> Self {
        let control = match namespaced_control(self.segment(), "control.structured-output") {
            Ok(control) => control,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        let domain = match bounded("inline JSON Schema 2020-12 document the exact route accepts") {
            Ok(domain) => domain,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        self.push_control(
            control,
            ConsumerRouteValueKind::StructuredDeclarations,
            domain,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    pub(super) fn with_context_window(
        mut self,
        context_window: Option<crate::OllamaContextWindow>,
    ) -> Self {
        let Some(context_window) = context_window else {
            return self;
        };
        let control = match namespaced_control(self.segment(), "control.context-window") {
            Ok(control) => control,
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
                return self;
            }
        };
        self.push_enumerated(
            control,
            ConsumerRouteValueKind::BoundedInteger,
            &context_window.as_u32().to_string(),
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
        self.push_control(control, kind, domain, omission);
    }

    fn push_control(
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
