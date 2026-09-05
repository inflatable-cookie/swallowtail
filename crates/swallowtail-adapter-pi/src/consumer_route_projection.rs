//! Contract 061 contributions for prepared Pi operations.
//!
//! The RPC and SDK-sidecar routes keep their projections local to the Pi
//! adapter. Every emitted row is bound to the immutable prepared plan or
//! public request; no matrix-only posture is promoted into runtime evidence.

use crate::{PiPreparedCatalogue, PiPreparedRun, PiPreparedSession, PiSdkSidecarPreparedSession};
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

const RPC_ROUTE_ID: &str = "pi.rpc";
const SIDECAR_ROUTE_ID: &str = "pi.sdk-sidecar";

type Contribution = Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure>;

struct Projection<'a> {
    plan: &'a PreflightPlan,
    route_id: &'static str,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    fn new(
        plan: &'a PreflightPlan,
        route_id: &'static str,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self {
            plan,
            route_id,
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
            let row = self.row(
                ConsumerRouteRowIdentity::Feature(feature.clone()),
                ConsumerRouteSourceClass::CapabilityProfile,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                if feature == ConsumerRouteFeatureId::ActivityObservation {
                    ConsumerRouteLifecycle::PostOpenObservationOnly
                } else {
                    ConsumerRouteLifecycle::SelectionSummary
                },
            );
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active_session.push(
                    row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                        .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
                );
            } else {
                self.selection
                    .push(row.with_actor_posture(ConsumerRouteActorPosture::Informational));
            }
        }
        self
    }

    fn with_model_selection(mut self) -> Self {
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
        self.selection.push(
            self.row(
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
            )),
        );
        self
    }

    fn with_question_exchange(mut self) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::QuestionExchange),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        self
    }

    fn with_rpc_attachment_control(mut self, per_turn: bool) -> Self {
        if !has_capability(self.plan, Capability::Attachments) {
            return self;
        }
        let control = namespaced_control(self.route_id, self.plan, "control.attachments");
        if per_turn {
            self.session_start.push(
                self.row(
                    ConsumerRouteRowIdentity::Control(control),
                    ConsumerRouteSourceClass::AdapterPreparedInput,
                    ConsumerRouteEvidenceStrength::RouteValidation,
                    ConsumerRouteLifecycle::PerTurn,
                )
                .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
                .with_mutation_authority(per_turn_authority(&self.source))
                .with_state_support(
                    ConsumerRouteStateSupport::descriptor_only()
                        .with_requested()
                        .with_observed(),
                )
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::ExchangeCallback,
                    bounded("consumer-mediated bounded PNG attachment exchange"),
                    ConsumerRouteOmissionSemantics::SuppliesNothing,
                )),
            );
        } else {
            self.session_start.push(
                self.row(
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
                .with_control_value(ConsumerRouteControlValue::new(
                    ConsumerRouteValueKind::StructuredDeclarations,
                    bounded("bounded PNG attachment media, count, and byte constraints"),
                    ConsumerRouteOmissionSemantics::SuppliesNothing,
                )),
            );
        }
        self
    }

    fn with_sidecar_controls(mut self, session: &PiSdkSidecarPreparedSession) -> Self {
        self.push_session_start_control(
            namespaced_control(self.route_id, self.plan, "control.session-options"),
            ConsumerRouteValueKind::StructuredOptions,
            bounded("bounded Pi SDK sidecar session options"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        if session.request().options().reasoning_mode().is_some() {
            self.push_session_start_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                bounded("the selected Pi SDK sidecar reasoning mode"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        self = self.with_rpc_attachment_control(true);
        self.push_session_start_control(
            ConsumerRouteControlId::LoadSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact durable Pi SDK sidecar session binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self.push_session_start_control(
            ConsumerRouteControlId::ResumeSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact durable Pi SDK sidecar session binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    fn push_session_start_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        self.session_start.push(
            self.row(
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
            .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission)),
        );
    }

    fn build(self) -> Contribution {
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

impl PiPreparedCatalogue {
    /// Emits the Pi RPC model-catalogue and prepared-facade rows.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), RPC_ROUTE_ID, source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl PiPreparedRun {
    /// Emits the Pi RPC structured-run rows proved by the prepared plan.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), RPC_ROUTE_ID, source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_rpc_attachment_control(false)
            .build()
    }
}

impl PiPreparedSession {
    /// Emits the Pi RPC interactive-session rows proved by the prepared plan.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), RPC_ROUTE_ID, source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_question_exchange()
            .with_rpc_attachment_control(true)
            .build()
    }
}

impl PiSdkSidecarPreparedSession {
    /// Emits the Pi SDK-sidecar rows proved by the prepared sidecar session.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Contribution {
        Projection::new(self.plan(), SIDECAR_ROUTE_ID, source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_sidecar_controls(self)
            .build()
    }
}

fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::Attachments => ConsumerRouteFeatureId::Attachments,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
        Capability::Resume => ConsumerRouteFeatureId::ResumeSession,
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn has_capability(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

fn namespaced_control(
    route_id: &str,
    plan: &PreflightPlan,
    semantic_id: &str,
) -> ConsumerRouteControlId {
    ConsumerRouteControlId::Namespaced(
        ConsumerRouteNamespacedExtension::new(
            route_id,
            plan.protocol_facade_id().as_str(),
            semantic_id,
        )
        .expect("static Pi projection identity is bounded"),
    )
}

fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value).expect("static Pi projection bound is admissible"),
    )
}

fn prepared_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::PreparedSessionStart(source.id().clone())
}

fn per_turn_authority(
    source: &ConsumerRouteProjectionSourceIdentity,
) -> ConsumerRouteMutationAuthority {
    ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(source.id().clone())
}

fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
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

#[cfg(test)]
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
