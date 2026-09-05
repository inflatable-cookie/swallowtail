//! Contract 061 contributions for the prepared Qwen Code facades.

use crate::{QwenPreparedCatalogue, QwenPreparedRun, QwenPreparedSession};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    HarnessMode, PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

const ROUTE: &str = "qwen.headless";

pub(crate) fn catalogue(
    prepared: &QwenPreparedCatalogue,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Projection::new(prepared.plan(), source_id)
        .with_prepared_capabilities()
        .build()
}

pub(crate) fn run(
    prepared: &QwenPreparedRun,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Projection::new(prepared.plan(), source_id)
        .with_prepared_capabilities()
        .with_model_selection()
        .with_reasoning(prepared.request().policy().reasoning_mode())
        .with_harness(prepared.request().policy().harness_mode())
        .build()
}

pub(crate) fn session(
    prepared: &QwenPreparedSession,
    source_id: ConsumerRouteProjectionSourceId,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    Projection::new(prepared.plan(), source_id)
        .with_prepared_capabilities()
        .with_model_selection()
        .with_reasoning(prepared.request().options().reasoning_mode())
        .with_harness(prepared.request().options().harness_mode())
        .build()
}

struct Projection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
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
            active_session: Vec::new(),
        }
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
            source_class,
            evidence,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    fn with_prepared_capabilities(mut self) -> Self {
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
                    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly),
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

    fn with_model_selection(mut self) -> Self {
        let Some(model) = self.applicability.model() else {
            return self;
        };
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::ModelSelection),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteEvidenceStrength::PreparedOperation,
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
                exact(model.model_id().as_str()),
                ConsumerRouteOmissionSemantics::Required,
            )),
        );
        self
    }

    fn with_reasoning(mut self, reasoning: Option<&swallowtail_core::ReasoningMode>) -> Self {
        let Some(reasoning) = reasoning else {
            return self;
        };
        self.session_start.push(self.control(
            ConsumerRouteControlId::ReasoningSelection,
            ConsumerRouteValueKind::BoundedEnumeration,
            exact(reasoning.as_str()),
            ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
        ));
        self
    }

    fn with_harness(mut self, harness: Option<HarnessMode>) -> Self {
        let Some(harness) = harness else {
            return self;
        };
        self.session_start.push(
            self.control(
                ConsumerRouteControlId::Namespaced(
                    swallowtail_runtime::ConsumerRouteNamespacedExtension::new(
                        ROUTE,
                        self.plan.protocol_facade_id().as_str(),
                        "control.harness-mode",
                    )
                    .expect("static harness identity is bounded"),
                ),
                ConsumerRouteValueKind::BoundedEnumeration,
                exact(match harness {
                    HarnessMode::Plan => "plan",
                    _ => "unknown",
                }),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            ),
        );
        self
    }

    fn control(
        &self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) -> ConsumerRouteProjectionRow {
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
        .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission))
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
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn exact(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Enumerated(
        swallowtail_runtime::ConsumerRouteEnumeratedValues::new([{
            swallowtail_runtime::ConsumerRouteEnumerableValue::new(value)
                .expect("prepared value is bounded")
        }])
        .expect("one prepared value is unique"),
    )
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
