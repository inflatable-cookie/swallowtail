//! Contract 061 contributions for the prepared Kimi Platform route.

use crate::{KimiPlatformPreparedCatalogue, KimiPlatformPreparedInferenceAttempt};
use swallowtail_core::{AccessStatus, Capability, PreflightPlan};
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

impl KimiPlatformPreparedCatalogue {
    /// Emits exact prepared model-catalogue truth without model-selection authority.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .capabilities()
            .build()
    }
}

impl KimiPlatformPreparedInferenceAttempt {
    /// Emits exact prepared structured-inference truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut projection = Projection::new(self.plan(), source_id)
            .capabilities()
            .model_selection();
        if let Some(reasoning) = self.request().policy().reasoning_mode() {
            projection = projection.control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                reasoning.as_str(),
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        if let Some(maximum) = self.request().maximum_output_tokens() {
            projection = projection.control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                &maximum.get().to_string(),
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        projection
            .named_control(
                "control.reasoning-and-output-required",
                "reasoning and output maximum required",
            )
            .build()
    }
}

struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active: Vec<ConsumerRouteProjectionRow>,
    rejected: Option<ConsumerRouteProjectionFailure>,
}

impl<'a> Projection<'a> {
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
            active: Vec::new(),
            rejected: None,
        }
    }

    fn capabilities(mut self) -> Self {
        self.selection
            .push(self.feature_row(ConsumerRouteFeatureId::PreparedFacade));
        for requirement in self.plan.requirements().capabilities() {
            let feature = match requirement.capability() {
                Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
                Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
                Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
                Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
                Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
                Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
                Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
                _ => continue,
            };
            let row = if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.row(
                    ConsumerRouteRowIdentity::Feature(feature.clone()),
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::PostOpenObservationOnly,
                )
            } else {
                self.feature_row(feature.clone())
            };
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active
                    .push(row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly));
            } else {
                self.selection.push(row);
            }
        }
        self
    }

    fn model_selection(mut self) -> Self {
        let Some(model) = self
            .applicability
            .model()
            .map(|model| model.model_id().as_str().to_owned())
        else {
            return self;
        };
        self = self.control(
            ConsumerRouteControlId::ModelSelection,
            ConsumerRouteValueKind::ExactModelRoute,
            &model,
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }

    fn named_control(mut self, semantic: &str, value: &str) -> Self {
        match ConsumerRouteNamespacedExtension::new(
            "kimi-platform.chat",
            self.plan.protocol_facade_id().as_str(),
            semantic,
        ) {
            Ok(extension) => self.control(
                ConsumerRouteControlId::Namespaced(extension),
                ConsumerRouteValueKind::FixedStructuredConfig,
                value,
                ConsumerRouteOmissionSemantics::Required,
            ),
            Err(error) => {
                self.rejected = Some(error);
                self
            }
        }
    }

    fn control(
        mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        let domain = match ConsumerRouteEnumerableValue::new(value)
            .and_then(|value| ConsumerRouteEnumeratedValues::new([value]))
        {
            Ok(values) => ConsumerRouteValueDomain::Enumerated(values),
            Err(error) => {
                self.rejected = Some(error);
                return self;
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
            .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
                self.source.id().clone(),
            ))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission)),
        );
        self
    }

    fn feature_row(&self, feature: ConsumerRouteFeatureId) -> ConsumerRouteProjectionRow {
        self.row(
            ConsumerRouteRowIdentity::Feature(feature),
            ConsumerRouteSourceClass::CapabilityProfile,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            ConsumerRouteLifecycle::SelectionSummary,
        )
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
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
        }
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            [self.source],
            self.selection,
            self.session_start,
            self.active,
        )
    }
}

fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    use swallowtail_core::{
        CredentialState, EndpointAuthorization, EntitlementState, RuntimeReadiness,
    };
    if status.credential() == CredentialState::Ready
        && status.entitlement() == EntitlementState::Available
        && status.endpoint_authorization() == EndpointAuthorization::Allowed
        && status.runtime_readiness() == RuntimeReadiness::Ready
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}
