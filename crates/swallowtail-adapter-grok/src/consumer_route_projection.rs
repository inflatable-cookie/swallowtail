//! Contract 061 contributions for prepared Grok Build ACP operations.
//!
//! Prepared sessions and runs publish only their own plan-backed rows. The
//! negotiated model-options row is produced only by the additive successful-open seam.

use crate::{GrokPreparedRun, GrokPreparedSession};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    BoxFuture, ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind, HostServices,
    InteractiveSessionHandle, NegotiatedSessionModelOptions, RuntimeFailure, SessionCleanupRequest,
};

/// Result of opening Grok ACP through the additive projection seam.
pub struct GrokProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}
impl GrokProjectionOpenOutcome {
    /// Returns the open session.
    #[must_use]
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
        self.session.as_ref()
    }
    /// Returns the exact prepared and active-session contribution.
    #[must_use]
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }
    /// Returns options retained and validated during the successful open.
    #[must_use]
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }
    /// Splits session and contribution.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        Box<dyn InteractiveSessionHandle>,
        ConsumerRouteProjectionContribution,
    ) {
        (self.session, self.contribution)
    }
}
/// Failure returned by the additive Grok projected-open seam.
pub enum GrokProjectionOpenFailure {
    /// The underlying route or projection failure.
    Runtime(RuntimeFailure),
}
impl GrokProjectionOpenFailure {
    /// Returns the underlying route failure.
    #[must_use]
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) => failure,
        }
    }
}
/// Future returned by the additive Grok projected-open seam.
pub type GrokProjectionOpenFuture =
    BoxFuture<'static, Result<GrokProjectionOpenOutcome, GrokProjectionOpenFailure>>;

impl GrokPreparedSession {
    /// Emits only the interactive-session rows this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source)
            .prepared(true)
            .model_selection()
            .session_options()
            .build()
    }
    /// Opens ACP and publishes negotiated options only when the open retained them.
    pub fn open_session_with_projection(
        &self,
        prepared_source: ConsumerRouteProjectionSourceId,
        active_source: ConsumerRouteProjectionSourceId,
        cleanup: SessionCleanupRequest,
        services: HostServices,
    ) -> GrokProjectionOpenFuture {
        if prepared_source == active_source {
            return Box::pin(async {
                Err(GrokProjectionOpenFailure::Runtime(crate::failure::failure(
                    "swallowtail.grok.projection_source_identity_invalid",
                    "Grok prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        Box::pin(async move {
            let session = prepared
                .open_session(services.clone())
                .await
                .map_err(GrokProjectionOpenFailure::Runtime)?;
            let contribution = Projection::observed(
                prepared.plan(),
                prepared_source,
                active_source,
                session.negotiated_model_options().is_some(),
            )
            .prepared(true)
            .model_selection()
            .session_options()
            .build();
            match contribution {
                Ok(contribution) => Ok(GrokProjectionOpenOutcome {
                    session,
                    contribution,
                }),
                Err(rejection) => {
                    let _ = session.close(cleanup, services).await;
                    Err(GrokProjectionOpenFailure::Runtime(RuntimeFailure::new(
                        rejection.diagnostic().clone(),
                    )))
                }
            }
        })
    }
}
impl GrokPreparedRun {
    /// Emits only the structured-run rows this prepared run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source)
            .prepared(false)
            .model_selection()
            .build()
    }
}

struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    active_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active: Vec<ConsumerRouteProjectionRow>,
    rejected: Option<ConsumerRouteProjectionFailure>,
}
impl<'a> Projection<'a> {
    fn new(plan: &'a PreflightPlan, source: ConsumerRouteProjectionSourceId) -> Self {
        Self::sources(plan, source, None)
    }
    fn observed(
        plan: &'a PreflightPlan,
        prepared: ConsumerRouteProjectionSourceId,
        active: ConsumerRouteProjectionSourceId,
        options: bool,
    ) -> Self {
        let mut projection = Self::sources(plan, prepared, Some(active));
        if options {
            projection.model_observation();
        }
        projection
    }
    fn sources(
        plan: &'a PreflightPlan,
        prepared: ConsumerRouteProjectionSourceId,
        active: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: ConsumerRouteProjectionSourceIdentity::new(
                prepared,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            active_source: active.map(|id| {
                ConsumerRouteProjectionSourceIdentity::new(
                    id,
                    ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
                )
            }),
            availability: availability(plan.access_status()),
            selection: Vec::new(),
            session_start: Vec::new(),
            active: Vec::new(),
            rejected: None,
        }
    }
    fn prepared(mut self, session: bool) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                &self.prepared_source,
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability(), session) else {
                continue;
            };
            let row = self
                .row(
                    ConsumerRouteRowIdentity::Feature(feature.clone()),
                    &self.prepared_source,
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    if feature == ConsumerRouteFeatureId::ActivityObservation {
                        ConsumerRouteLifecycle::PostOpenObservationOnly
                    } else {
                        ConsumerRouteLifecycle::SelectionSummary
                    },
                )
                .with_actor_posture(if feature == ConsumerRouteFeatureId::ActivityObservation {
                    ConsumerRouteActorPosture::ObservationOnly
                } else {
                    ConsumerRouteActorPosture::Informational
                });
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active
                    .push(row.with_state_support(ConsumerRouteStateSupport::descriptor_only()));
            } else {
                self.selection.push(row);
            }
        }
        self
    }
    fn model_selection(mut self) -> Self {
        if let Some(model) = self.applicability.model() {
            let domain = exact(model.model_id().as_str(), &mut self.rejected);
            self.push_control(
                ConsumerRouteControlId::ModelSelection,
                ConsumerRouteValueKind::ExactModelRoute,
                domain,
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        self
    }
    fn session_options(mut self) -> Self {
        let domain = bounded(
            "validated empty Grok ACP session options",
            &mut self.rejected,
        );
        self.push_control(
            ConsumerRouteControlId::SessionOptions,
            ConsumerRouteValueKind::StructuredOptions,
            domain,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }
    fn model_observation(&mut self) {
        let Some(source) = self.active_source.clone() else {
            return;
        };
        let identity = match ConsumerRouteNamespacedExtension::new(
            "grok-build.acp",
            self.plan.protocol_facade_id().as_str(),
            "feature.negotiated-model-options-observation",
        ) {
            Ok(extension) => {
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(extension))
            }
            Err(error) => {
                self.rejected = Some(error);
                return;
            }
        };
        let value = bounded(
            "exact bounded negotiated model options on the open session",
            &mut self.rejected,
        );
        if let Some(value) = value {
            self.active.push(
                self.row(
                    identity,
                    &source,
                    ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
                    ConsumerRouteEvidenceStrength::WireAcknowledgement,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::Observation,
                    value,
                    ConsumerRouteOmissionSemantics::NotSelectable,
                )),
            );
        }
    }
    fn push_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: Option<ConsumerRouteValueDomain>,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let Some(domain) = domain else { return };
        self.session_start.push(
            self.row(
                ConsumerRouteRowIdentity::Control(control),
                &self.prepared_source,
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SessionStartOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
                self.prepared_source.id().clone(),
            ))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission)),
        );
    }
    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source: &ConsumerRouteProjectionSourceIdentity,
        class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            source.clone(),
            class,
            evidence,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }
    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        if let Some(error) = self.rejected {
            return Err(error);
        };
        let sources =
            std::iter::once(self.prepared_source).chain(self.active_source.filter(|source| {
                self.active
                    .iter()
                    .any(|row| row.source().id() == source.id())
            }));
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            sources,
            self.selection,
            self.session_start,
            self.active,
        )
    }
}
const fn feature_for(capability: Capability, session: bool) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::InteractiveSession if session => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StructuredRun if !session => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}
fn exact(
    value: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteValueDomain> {
    match ConsumerRouteEnumerableValue::new(value)
        .and_then(|value| ConsumerRouteEnumeratedValues::new([value]))
    {
        Ok(values) => Some(ConsumerRouteValueDomain::Enumerated(values)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
}
fn bounded(
    value: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteValueDomain> {
    match ConsumerRouteEnumerableValue::new(value) {
        Ok(value) => Some(ConsumerRouteValueDomain::Unenumerated(value)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
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
