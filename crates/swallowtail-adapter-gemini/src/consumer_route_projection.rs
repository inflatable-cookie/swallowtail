//! Contract 061 contributions for the prepared Gemini routes.
//!
//! Rows are constructed only from the exact prepared plan and bound request.
//! Negotiated model options stay withheld until the additive open path observes
//! them on a successfully opened ACP session.

use crate::{GeminiHeadlessPreparedRun, GeminiPreparedLiveSession, GeminiPreparedSession};
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

/// Result of opening Gemini ACP through the additive projection seam.
pub struct GeminiProjectionOpenOutcome {
    session: Box<dyn InteractiveSessionHandle>,
    contribution: ConsumerRouteProjectionContribution,
}

impl GeminiProjectionOpenOutcome {
    /// Returns the open session.
    #[must_use]
    pub fn session(&self) -> &dyn InteractiveSessionHandle {
        self.session.as_ref()
    }
    /// Returns the prepared and active-session contribution.
    #[must_use]
    pub const fn contribution(&self) -> &ConsumerRouteProjectionContribution {
        &self.contribution
    }
    /// Returns model options retained and validated during open.
    #[must_use]
    pub fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.session.negotiated_model_options()
    }
    /// Splits the opened session from its contribution.
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

/// Failure returned by the additive Gemini projected-open seam.
pub enum GeminiProjectionOpenFailure {
    /// The underlying route or projection failure.
    Runtime(RuntimeFailure),
}
impl GeminiProjectionOpenFailure {
    /// Returns the underlying route failure.
    #[must_use]
    pub const fn failure(&self) -> &RuntimeFailure {
        match self {
            Self::Runtime(failure) => failure,
        }
    }
}
/// Future returned by the additive Gemini projected-open seam.
pub type GeminiProjectionOpenFuture =
    BoxFuture<'static, Result<GeminiProjectionOpenOutcome, GeminiProjectionOpenFailure>>;

#[derive(Clone, Copy)]
enum Route {
    Acp,
    Headless,
    Live,
}
impl Route {
    const fn id(self) -> &'static str {
        match self {
            Self::Acp => "gemini-cli.acp",
            Self::Headless => "gemini-cli.headless",
            Self::Live => "gemini.live",
        }
    }
}

impl GeminiPreparedSession {
    /// Emits only the ACP rows this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Acp, source)
            .prepared()
            .harness_mode()
            .build()
    }
    /// Opens ACP and publishes retained negotiated options only after success.
    pub fn open_session_with_projection(
        &self,
        prepared_source: ConsumerRouteProjectionSourceId,
        active_source: ConsumerRouteProjectionSourceId,
        cleanup: SessionCleanupRequest,
        services: HostServices,
    ) -> GeminiProjectionOpenFuture {
        if prepared_source == active_source {
            return Box::pin(async {
                Err(GeminiProjectionOpenFailure::Runtime(
                    crate::failure::failure(
                        "swallowtail.gemini.projection_source_identity_invalid",
                        "Gemini prepared and active-session projection sources must differ",
                    ),
                ))
            });
        }
        let prepared = self.clone();
        Box::pin(async move {
            let session = prepared
                .open_session(services.clone())
                .await
                .map_err(GeminiProjectionOpenFailure::Runtime)?;
            let contribution = Projection::observed(
                prepared.plan(),
                Route::Acp,
                prepared_source,
                active_source,
                session.negotiated_model_options().is_some(),
            )
            .prepared()
            .harness_mode()
            .build();
            match contribution {
                Ok(contribution) => Ok(GeminiProjectionOpenOutcome {
                    session,
                    contribution,
                }),
                Err(rejection) => {
                    let _ = session.close(cleanup, services).await;
                    Err(GeminiProjectionOpenFailure::Runtime(RuntimeFailure::new(
                        rejection.diagnostic().clone(),
                    )))
                }
            }
        })
    }
}
impl GeminiHeadlessPreparedRun {
    /// Emits only the headless rows this prepared run proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Headless, source)
            .prepared()
            .model_selection()
            .build()
    }
}
impl GeminiPreparedLiveSession {
    /// Emits only the Live rows this prepared session and request prove.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), Route::Live, source)
            .prepared()
            .live_controls(self)
            .build()
    }
}

struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: Route,
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
    fn new(plan: &'a PreflightPlan, route: Route, source: ConsumerRouteProjectionSourceId) -> Self {
        Self::sources(plan, route, source, None)
    }
    fn observed(
        plan: &'a PreflightPlan,
        route: Route,
        prepared: ConsumerRouteProjectionSourceId,
        active: ConsumerRouteProjectionSourceId,
        has_options: bool,
    ) -> Self {
        let mut projection = Self::sources(plan, route, prepared, Some(active));
        if has_options {
            projection.model_observation();
        }
        projection
    }
    fn sources(
        plan: &'a PreflightPlan,
        route: Route,
        prepared: ConsumerRouteProjectionSourceId,
        active: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            route,
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
    fn prepared(mut self) -> Self {
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
            let Some(feature) = feature_for(self.route, requirement.capability()) else {
                continue;
            };
            let row = self
                .row(
                    ConsumerRouteRowIdentity::Feature(feature.clone()),
                    &self.prepared_source,
                    if feature == ConsumerRouteFeatureId::PreparedFacade {
                        ConsumerRouteSourceClass::PreparedOperationRecord
                    } else {
                        ConsumerRouteSourceClass::CapabilityProfile
                    },
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
        if matches!(self.route, Route::Live)
            && !self.active.iter().any(|row| {
                row.identity()
                    == &ConsumerRouteRowIdentity::Feature(
                        ConsumerRouteFeatureId::ActivityObservation,
                    )
            })
        {
            self.active.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation),
                    &self.prepared_source,
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
            );
        }
        self
    }
    fn harness_mode(mut self) -> Self {
        if self
            .plan
            .requirements()
            .capabilities()
            .any(|r| r.capability() == Capability::HarnessModeSelection)
        {
            let control = namespaced(
                self.route,
                self.plan,
                "control.harness-mode",
                &mut self.rejected,
            );
            let domain = exact("plan", &mut self.rejected);
            self.push_control(
                control,
                ConsumerRouteValueKind::BoundedEnumeration,
                domain,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        self
    }
    fn model_selection(mut self) -> Self {
        if let Some(model) = self.applicability.model() {
            let domain = exact(model.model_id().as_str(), &mut self.rejected);
            self.push_control(
                Some(ConsumerRouteControlId::ModelSelection),
                ConsumerRouteValueKind::ExactModelRoute,
                domain,
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        self
    }
    fn live_controls(mut self, session: &GeminiPreparedLiveSession) -> Self {
        let request = session.request();
        if let Some(mode) = request.reasoning_mode() {
            let domain = exact(mode.as_str(), &mut self.rejected);
            self.push_control(
                Some(ConsumerRouteControlId::ReasoningSelection),
                ConsumerRouteValueKind::BoundedEnumeration,
                domain,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if let Some(maximum) = request.maximum_output_tokens() {
            let domain = exact(&maximum.get().to_string(), &mut self.rejected);
            self.push_control(
                Some(ConsumerRouteControlId::MaximumOutputTokens),
                ConsumerRouteValueKind::BoundedInteger,
                domain,
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        let media_config = bounded("validated realtime media configuration", &mut self.rejected);
        self.push_control(
            Some(ConsumerRouteControlId::RealtimeMediaConfig),
            ConsumerRouteValueKind::FixedStructuredConfig,
            media_config,
            ConsumerRouteOmissionSemantics::Required,
        );
        let compression = namespaced(
            self.route,
            self.plan,
            "control.context-window-compression",
            &mut self.rejected,
        );
        let compression_domain = bounded("provider-default sliding-window", &mut self.rejected);
        self.push_control(
            compression,
            ConsumerRouteValueKind::BoundedPolicy,
            compression_domain,
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        let rollover = exact(
            &request
                .planned_connection_rollover()
                .maximum_count()
                .map_or(0, |value| value.get())
                .to_string(),
            &mut self.rejected,
        );
        self.push_control(
            Some(ConsumerRouteControlId::PlannedConnectionRollover),
            ConsumerRouteValueKind::BoundedInteger,
            rollover,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }
    fn model_observation(&mut self) {
        let Some(source) = self.active_source.clone() else {
            return;
        };
        let identity = match ConsumerRouteNamespacedExtension::new(
            self.route.id(),
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
        let Some(value) = value else { return };
        let row = self
            .row(
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
            ));
        self.active.push(row);
    }
    fn push_control(
        &mut self,
        control: Option<ConsumerRouteControlId>,
        kind: ConsumerRouteValueKind,
        domain: Option<ConsumerRouteValueDomain>,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let (Some(control), Some(domain)) = (control, domain) else {
            return;
        };
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
fn feature_for(route: Route, capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::InteractiveSession if !matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::InteractiveSession
        }
        Capability::RealtimeMedia if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::RealtimeMediaSession
        }
        Capability::StructuredRun if matches!(route, Route::Headless) => {
            ConsumerRouteFeatureId::StructuredRun
        }
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource if !matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::WorkingResource
        }
        Capability::OutputTokenLimit if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::OutputTokenLimit
        }
        Capability::ReasoningSelection if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::ReasoningSelection
        }
        Capability::PlannedConnectionRollover if matches!(route, Route::Live) => {
            ConsumerRouteFeatureId::Namespaced(
                ConsumerRouteNamespacedExtension::new(
                    route.id(),
                    "gemini.live",
                    "feature.planned-connection-rollover",
                )
                .ok()?,
            )
        }
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}
fn namespaced(
    route: Route,
    plan: &PreflightPlan,
    semantic: &str,
    rejected: &mut Option<ConsumerRouteProjectionFailure>,
) -> Option<ConsumerRouteControlId> {
    match ConsumerRouteNamespacedExtension::new(
        route.id(),
        plan.protocol_facade_id().as_str(),
        semantic,
    ) {
        Ok(extension) => Some(ConsumerRouteControlId::Namespaced(extension)),
        Err(error) => {
            *rejected = Some(error);
            None
        }
    }
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
