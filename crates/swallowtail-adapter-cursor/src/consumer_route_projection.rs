//! Contract 061 projection contributions for prepared Cursor agent operations.

use crate::headless_command::CursorHeadlessReadMode;
use crate::headless_model_parameters::{
    CursorHeadlessContext, CursorHeadlessFast, parse_plan_model_id,
};
use crate::{CursorPreparedAcpSession, CursorPreparedCatalogue, CursorPreparedHeadlessRun};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

const CURSOR_HEADLESS_ROUTE: &str = "cursor-agent.headless";

impl CursorPreparedAcpSession {
    /// Emits only the interactive-session truth this prepared ACP session proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Activity observation stays a post-open descriptor-only row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .build()
    }
}

impl CursorPreparedCatalogue {
    /// Emits only the authenticated model-catalogue truth this prepared catalogue proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Catalogue operations attach no activity profile and prove no execution
    /// controls, so those rows are withheld at construction.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .build()
    }
}

impl CursorPreparedHeadlessRun {
    /// Emits only the stream-JSON structured-run truth this prepared run proves.
    ///
    /// The supplied source identity is preserved as one adapter contribution.
    /// Activity observation stays a post-open descriptor-only row.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let segment = self.plan().protocol_facade_id().as_str();
        let mut builder = ProjectionBuilder::new(self.plan(), source_id)
            .with_prepared_facade()
            .with_prepared_capabilities()
            .with_model_selection()?;

        if let Some(parsed) = self
            .plan()
            .model_id()
            .and_then(|model_id| parse_plan_model_id(model_id.as_str()).ok())
        {
            if let Some(fast) = parsed.parameters.fast() {
                let fast_str = match fast {
                    CursorHeadlessFast::Standard => "Standard",
                };
                let control = route_local(segment, "control.fast")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(fast_str)?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }

            if let Some(context) = parsed.parameters.context() {
                let context_str = match context {
                    CursorHeadlessContext::ThreeHundredK => "ThreeHundredK",
                    CursorHeadlessContext::OneMillion => "OneMillion",
                };
                let control = route_local(segment, "control.context-window")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(context_str)?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }

            if let Some(effort) = parsed.parameters.effort() {
                let control = route_local(segment, "control.reasoning-effort")?;
                builder.push_session_start_control(
                    ConsumerRouteControlId::Namespaced(control),
                    ConsumerRouteValueKind::BoundedEnumeration,
                    exact(effort.as_str())?,
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
                );
            }
        }

        if let Some(read_mode) = self.read_mode() {
            let mode_str = match read_mode {
                CursorHeadlessReadMode::Plan => "Plan",
                CursorHeadlessReadMode::Ask => "Ask",
            };
            let control = route_local(segment, "control.read-mode")?;
            builder.push_session_start_control(
                ConsumerRouteControlId::Namespaced(control),
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(mode_str)?,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }

        builder.build()
    }
}

struct ProjectionBuilder<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> ProjectionBuilder<'a> {
    fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            source: ConsumerRouteProjectionSourceIdentity::new(
                source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            availability: availability(plan.access_status()),
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

    fn prepared_authority(&self) -> ConsumerRouteMutationAuthority {
        ConsumerRouteMutationAuthority::PreparedSessionStart(self.source.id().clone())
    }

    fn with_prepared_facade(mut self) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        self
    }

    fn with_prepared_capabilities(mut self) -> Self {
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

    fn with_model_selection(mut self) -> Result<Self, ConsumerRouteProjectionFailure> {
        let Some(model) = self.applicability.model() else {
            return Ok(self);
        };
        let domain = exact(model.model_id().as_str())?;
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
        Ok(self)
    }

    fn push_session_start_control(
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
            .with_mutation_authority(self.prepared_authority())
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission));
        self.session_start.push(row);
    }

    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            [self.source],
            self.selection,
            self.session_start,
            self.active_session,
        )
    }
}

fn route_local(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(CURSOR_HEADLESS_ROUTE, segment, semantic_id)
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
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
