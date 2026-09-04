use super::*;

pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejection: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    pub(super) fn new(plan: &'a PreflightPlan, source: ConsumerRouteProjectionSourceId) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            source: ConsumerRouteProjectionSourceIdentity::new(
                source,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            availability: availability(plan.access_status()),
            rejection: None,
            selection: Vec::new(),
            session_start: Vec::new(),
            active_session: Vec::new(),
        }
    }

    pub(super) fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        class: ConsumerRouteSourceClass,
        strength: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
            class,
            strength,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    pub(super) fn with_prepared_facade(mut self) -> Self {
        let row = self.row(
            ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
            ConsumerRouteSourceClass::PreparedOperationRecord,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            ConsumerRouteLifecycle::SelectionSummary,
        );
        self.selection.push(row);
        self
    }

    pub(super) fn with_feature(mut self, feature: ConsumerRouteFeatureId) -> Self {
        let row = self.row(
            ConsumerRouteRowIdentity::Feature(feature),
            ConsumerRouteSourceClass::CapabilityProfile,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            ConsumerRouteLifecycle::SelectionSummary,
        );
        self.selection.push(row);
        self
    }

    pub(super) fn with_prepared_capabilities(mut self) -> Self {
        self = self.with_prepared_facade();
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability()) else {
                continue;
            };
            let lifecycle = if feature == ConsumerRouteFeatureId::ActivityObservation {
                ConsumerRouteLifecycle::PostOpenObservationOnly
            } else {
                ConsumerRouteLifecycle::SelectionSummary
            };
            let mut row = self.row(
                ConsumerRouteRowIdentity::Feature(feature.clone()),
                ConsumerRouteSourceClass::CapabilityProfile,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                lifecycle,
            );
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                row = row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly);
                self.active_session.push(row);
            } else {
                self.selection.push(row);
            }
        }
        self
    }

    pub(super) fn with_model_selection(self) -> Self {
        let model = self
            .applicability
            .model()
            .expect("prepared DeepSeek run is model-bound");
        let value = model.model_id().as_str().to_owned();
        self.with_selection_control(
            ConsumerRouteControlId::ModelSelection,
            ConsumerRouteValueKind::ExactModelRoute,
            &value,
            ConsumerRouteOmissionSemantics::Required,
        )
    }

    pub(super) fn with_selection_control(
        mut self,
        id: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        let Some(domain) = self.domain(value) else {
            return self;
        };
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(id),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
                self.source.id().clone(),
            ))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.selection.push(row);
        self
    }

    pub(super) fn with_control(
        mut self,
        id: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        let Some(domain) = self.domain(value) else {
            return self;
        };
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(id),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SessionStartOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
                self.source.id().clone(),
            ))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.session_start.push(row);
        self
    }

    pub(super) fn with_namespaced_control(
        mut self,
        semantic: &str,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        match ConsumerRouteNamespacedExtension::new(
            ROUTE,
            self.plan.protocol_facade_id().as_str(),
            semantic,
        ) {
            Ok(extension) => self.with_control(
                ConsumerRouteControlId::Namespaced(extension),
                kind,
                value,
                omission,
            ),
            Err(error) => {
                self.rejection = Some(error);
                self
            }
        }
    }

    pub(super) fn domain(&mut self, value: &str) -> Option<ConsumerRouteValueDomain> {
        match ConsumerRouteEnumerableValue::new(value)
            .and_then(|value| ConsumerRouteEnumeratedValues::new([value]))
        {
            Ok(values) => Some(ConsumerRouteValueDomain::Enumerated(values)),
            Err(error) => {
                self.rejection = Some(error);
                None
            }
        }
    }

    pub(super) fn build(
        self,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(error) = self.rejection {
            return Err(error);
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
