use super::ROUTE;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness, SessionProviderStatePolicy,
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

    pub(super) fn with_prepared_capabilities(mut self) -> Self {
        self = self.with_prepared_facade();
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

    pub(super) fn with_model_selection(mut self) -> Self {
        let Some(model) = self.applicability.model() else {
            return self;
        };
        let Ok(domain) = exact(model.model_id().as_str()) else {
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

    pub(super) fn with_provider_state_policy(mut self) -> Self {
        let Some(policy) = self.plan.requirements().session_provider_state_policy() else {
            return self;
        };
        if policy != SessionProviderStatePolicy::DurableProviderSessionPreserved {
            return self;
        }
        self.push_namespaced_control(
            "control.provider-state-policy",
            ConsumerRouteValueKind::BoundedPolicy,
            "durable provider session preserved",
            ConsumerRouteOmissionSemantics::Required,
        );
        self
    }

    pub(super) fn with_fixed_wire_turn_options(mut self) -> Self {
        self.push_namespaced_control(
            "control.fixed-wire-turn-options",
            ConsumerRouteValueKind::FixedStructuredConfig,
            "stream=true; store=false; reasoning_effort=none; tools=0; session_cache=false; background=false; retries=0; previous_response=false; maximum_output_tokens=absent; fallback=false",
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    pub(super) fn with_load_session(mut self) -> Self {
        self.push_control(
            ConsumerRouteControlId::LoadSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("load an exact retained Alibaba provider session"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    fn push_namespaced_control(
        &mut self,
        semantic_id: &str,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let extension = match ConsumerRouteNamespacedExtension::new(
            ROUTE,
            self.plan.protocol_facade_id().as_str(),
            semantic_id,
        ) {
            Ok(extension) => extension,
            Err(error) => {
                self.rejected.get_or_insert(error);
                return;
            }
        };
        self.push_control(
            ConsumerRouteControlId::Namespaced(extension),
            kind,
            bounded(value),
            omission,
        );
    }

    fn push_control(
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
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::LoadSession => ConsumerRouteFeatureId::LoadSession,
        Capability::ProviderSessionDelete => ConsumerRouteFeatureId::ProviderSessionDelete,
        Capability::OwnedRemoteResourceDeletion => ConsumerRouteFeatureId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                ROUTE,
                "model-studio-2026-07-22",
                "feature.owned-remote-resource-cleanup",
            )
            .expect("static Alibaba feature identity is valid"),
        ),
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn exact(value: &str) -> Result<ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(ConsumerRouteValueDomain::Enumerated(
        ConsumerRouteEnumeratedValues::new([ConsumerRouteEnumerableValue::new(value)?])?,
    ))
}

fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)
            .unwrap_or_else(|_| unreachable!("static Alibaba projection bound is admissible")),
    )
}

fn availability(plan: &PreflightPlan) -> ConsumerRouteAvailability {
    let status: &AccessStatus = plan.access_status();
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
