use super::ROUTE;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

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
            availability: availability(plan),
            rejected: None,
            selection: Vec::new(),
            session_start: Vec::new(),
            active_session: Vec::new(),
        }
    }

    fn row(
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
            if feature == ConsumerRouteFeatureId::ActivityObservation {
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
            } else {
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
        }
        self
    }

    pub(super) fn with_persistent_session_posture(mut self) -> Self {
        if self.plan.requirements().session_provider_state_policy()
            != Some(SessionProviderStatePolicy::Prohibited)
        {
            return self;
        }
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PersistentSessionPosture),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        self
    }

    pub(super) fn with_model_selection(mut self) -> Self {
        let Some(model) = self.applicability.model() else {
            return self;
        };
        let domain = match exact(model.model_id().as_str()) {
            Ok(domain) => domain,
            Err(error) => {
                self.rejected.get_or_insert(error);
                return self;
            }
        };
        self.selection.push(
            self.row(
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
            )),
        );
        self
    }

    pub(super) fn with_reasoning(mut self, mode: Option<&swallowtail_core::ReasoningMode>) -> Self {
        if let Some(mode) = mode {
            self.push_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(mode.as_str()),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        self
    }

    pub(super) fn with_maximum_output_tokens(
        mut self,
        maximum: Option<std::num::NonZeroU64>,
    ) -> Self {
        if let Some(maximum) = maximum {
            self.push_control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                exact(&maximum.get().to_string()),
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        self
    }

    fn push_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure>,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let domain = match domain {
            Ok(domain) => domain,
            Err(error) => {
                self.rejected.get_or_insert(error);
                return;
            }
        };
        self.session_start.push(
            self.row(
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
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission)),
        );
    }

    fn prepared_authority(&self) -> ConsumerRouteMutationAuthority {
        ConsumerRouteMutationAuthority::PreparedSessionStart(self.source.id().clone())
    }

    pub(super) fn build(
        self,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(error) = self.rejected {
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

fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::BilledCostReporting => ConsumerRouteFeatureId::Namespaced(
            swallowtail_runtime::ConsumerRouteNamespacedExtension::new(
                ROUTE,
                "xai-responses-websocket-2026-04-23",
                "feature.billed-cost-evidence",
            )
            .expect("static xAI feature identity is valid"),
        ),
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

fn availability(plan: &PreflightPlan) -> ConsumerRouteAvailability {
    let status: &AccessStatus = plan.access_status();
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

#[allow(dead_code)]
const _: &str = ROUTE;
