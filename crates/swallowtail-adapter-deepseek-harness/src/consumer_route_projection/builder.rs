use super::*;

pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: &'static str,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejection: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    pub(super) fn new(
        plan: &'a PreflightPlan,
        source: ConsumerRouteProjectionSourceId,
        route: &'static str,
    ) -> Self {
        Self {
            plan,
            route,
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

    pub(super) fn with_model_selection(mut self) -> Self {
        let model = self
            .applicability
            .model()
            .expect("prepared Harness run is model-bound");
        let domain = match exact(model.model_id().as_str()) {
            Ok(domain) => domain,
            Err(error) => {
                self.rejection = Some(error);
                return self;
            }
        };
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection),
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
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::ExactModelRoute,
                domain,
                ConsumerRouteOmissionSemantics::Required,
            ));
        self.selection.push(row);
        self
    }

    pub(super) fn with_namespaced_control(
        mut self,
        semantic: &str,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        let id = match ConsumerRouteNamespacedExtension::new(
            self.route,
            self.plan.protocol_facade_id().as_str(),
            semantic,
        ) {
            Ok(id) => ConsumerRouteControlId::Namespaced(id),
            Err(error) => {
                self.rejection = Some(error);
                return self;
            }
        };
        let domain = match exact(value) {
            Ok(domain) => domain,
            Err(error) => {
                self.rejection = Some(error);
                return self;
            }
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

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

pub(super) fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ProviderSessionCatalogue => ConsumerRouteFeatureId::ProviderSessionCatalogue,
        Capability::ProviderSessionArchive => ConsumerRouteFeatureId::ProviderSessionArchive,
        _ => return None,
    })
}

const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(
        status.credential(),
        CredentialState::Ready | CredentialState::NotRequired
    ) && matches!(status.entitlement(), EntitlementState::Available)
        && matches!(
            status.endpoint_authorization(),
            EndpointAuthorization::Allowed
        )
        && matches!(status.runtime_readiness(), RuntimeReadiness::Ready)
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}
