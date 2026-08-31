//! Contract 061 contributions emitted by prepared Codex app-server operations.
//!
//! Every row is proved by one exact prepared plan or bound request. A row
//! backed only by a documentation matrix or route-wide posture is withheld.

#[path = "consumer_route_projection/facades.rs"]
mod facades;

use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
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

/// Maps one exact prepared capability to portable feature identity.
///
/// A capability without a portable feature in this tranche stays withheld
/// rather than inventing a projected row.
pub(crate) const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::RealtimeMedia => ConsumerRouteFeatureId::RealtimeMediaSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::StructuredOutput => ConsumerRouteFeatureId::StructuredOutput,
        Capability::Attachments => ConsumerRouteFeatureId::Attachments,
        Capability::ToolCalls => ConsumerRouteFeatureId::ConsumerToolExchange,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
        Capability::Resume => ConsumerRouteFeatureId::ResumeSession,
        Capability::ProviderSessionCatalogue => ConsumerRouteFeatureId::ProviderSessionCatalogue,
        Capability::ProviderSessionImport => ConsumerRouteFeatureId::ProviderSessionImport,
        Capability::ProviderSessionArchive => ConsumerRouteFeatureId::ProviderSessionArchive,
        Capability::ProviderSessionRestore => ConsumerRouteFeatureId::ProviderSessionRestore,
        Capability::ProviderSessionDelete => ConsumerRouteFeatureId::ProviderSessionDelete,
        Capability::ProviderSessionReconciliation => {
            ConsumerRouteFeatureId::ProviderSessionReconciliation
        }
        Capability::ProviderSessionHistory => ConsumerRouteFeatureId::ProviderSessionHistory,
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::WorkingResourceTextWrite => ConsumerRouteFeatureId::BoundedWorkspaceTextWrite,
        Capability::ExternalSearch => ConsumerRouteFeatureId::ExternalSearch,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        _ => return None,
    })
}

const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(status.credential(), CredentialState::Ready)
        && matches!(status.entitlement(), EntitlementState::Available)
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

/// Collects contributed rows from one exact prepared Codex plan.
pub(crate) struct CodexProjectionBuilder<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> CodexProjectionBuilder<'a> {
    pub(crate) fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
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

    /// Emits one selection-summary feature row per exact prepared capability.
    pub(crate) fn with_prepared_capabilities(mut self) -> Self {
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
                let row = self.row(
                    ConsumerRouteRowIdentity::Feature(feature),
                    ConsumerRouteSourceClass::PreparedOperationRecord,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                );
                self.active_session.push(
                    row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                        .with_state_support(
                            ConsumerRouteStateSupport::descriptor_only().with_observed(),
                        ),
                );
                continue;
            }
            let row = self.row(
                ConsumerRouteRowIdentity::Feature(feature),
                ConsumerRouteSourceClass::CapabilityProfile,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            );
            self.selection
                .push(row.with_actor_posture(ConsumerRouteActorPosture::Informational));
        }
        self
    }

    /// Emits the consumer-mediated question-exchange feature when admitted.
    pub(crate) fn with_question_exchange(mut self) -> Self {
        if self
            .plan
            .requirements()
            .session_access_policy()
            .is_some_and(|policy| policy.provider_requests().exchanged_extensions().len() > 0)
        {
            self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange),
                    ConsumerRouteSourceClass::AdapterPreparedInput,
                    ConsumerRouteEvidenceStrength::RouteValidation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            );
        }
        self
    }

    /// Emits the exact selected model route as a selection-time control.
    pub(crate) fn with_model_selection(mut self) -> Self {
        if let Some(model) = self.applicability.model() {
            let admitted = match ConsumerRouteEnumerableValue::new(model.model_id().as_str()) {
                Ok(value) => value,
                Err(rejection) => {
                    self.rejected.get_or_insert(rejection);
                    return self;
                }
            };
            let domain = ConsumerRouteValueDomain::Enumerated(
                ConsumerRouteEnumeratedValues::new([admitted])
                    .unwrap_or_else(|_| unreachable!("one admitted value is within the maximum")),
            );
            let row = self
                .row(
                    ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection),
                    ConsumerRouteSourceClass::PreparedOperationRecord,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
                .with_mutation_authority(prepared_authority(&self.source))
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
        }
        self
    }

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

    pub(crate) fn push_per_turn_exchange(&mut self) {
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::UserInputExchange),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::PerTurn,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(prepared_authority(&self.source))
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

    pub(crate) fn build(
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

fn prepared_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::PreparedSessionStart(source.id().clone())
}

pub(crate) fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)
            .unwrap_or_else(|_| unreachable!("static bound text is admissible")),
    )
}
