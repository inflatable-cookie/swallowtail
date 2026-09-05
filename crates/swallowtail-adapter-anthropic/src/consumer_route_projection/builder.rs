use super::MANAGED_ROUTE;
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

pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: &'static str,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    rejected: Option<ConsumerRouteProjectionFailure>,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> Projection<'a> {
    pub(super) fn new(
        plan: &'a PreflightPlan,
        route: &'static str,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self {
            plan,
            route,
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

    pub(super) fn with_prepared_capabilities(mut self, include_output_limit: bool) -> Self {
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
            let Some(feature) = feature_for(requirement.capability(), include_output_limit) else {
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
        if mode.is_some() {
            self.push_portable_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                "adaptive",
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
            self.push_portable_control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                &maximum.get().to_string(),
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        self
    }

    pub(super) fn with_attachments(mut self, present: bool) -> Self {
        if present {
            self.push_namespaced_control(
                "control.attachments",
                ConsumerRouteValueKind::StructuredDeclarations,
                "route-qualified image/png attachments, at most one and one MiB",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        self
    }

    pub(super) fn with_web_search(mut self) -> Self {
        if has_capability(self.plan, Capability::ExternalSearch) {
            self.push_namespaced_control(
                "control.web-search-allowlist",
                ConsumerRouteValueKind::BoundedPolicy,
                "one to ten bare allowed provider domains",
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        self
    }

    pub(super) fn with_session_tools_and_reasoning(mut self) -> Self {
        self.push_namespaced_control(
            "control.session-tools-and-reasoning",
            ConsumerRouteValueKind::StructuredOptions,
            "route-qualified tools, reasoning, and thinking setup",
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    pub(super) fn with_thinking(mut self, mode: Option<crate::AnthropicThinkingMode>) -> Self {
        if mode.is_some() {
            self.push_namespaced_control(
                "control.thinking-mode",
                ConsumerRouteValueKind::BoundedEnumeration,
                "adaptive",
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        self
    }

    pub(super) fn with_managed_policies(mut self) -> Self {
        self.push_namespaced_control(
            "control.provider-retention-policy",
            ConsumerRouteValueKind::BoundedPolicy,
            "durable managed-agent provider retention",
            ConsumerRouteOmissionSemantics::Required,
        );
        self.push_namespaced_control(
            "control.provider-recovery-policy",
            ConsumerRouteValueKind::BoundedPolicy,
            "provider-managed recovery",
            ConsumerRouteOmissionSemantics::Required,
        );
        self.push_namespaced_control(
            "control.stream-reattachment",
            ConsumerRouteValueKind::BoundedInteger,
            "one authoritative-history stream reattachment",
            ConsumerRouteOmissionSemantics::Required,
        );
        if has_capability(self.plan, Capability::ProviderRunReconciliation) {
            self.push_namespaced_control(
                "control.cross-process-recovery",
                ConsumerRouteValueKind::BoundedPolicy,
                "cross-process recovery accepted",
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        self
    }

    pub(super) fn with_per_turn_tool_exchange(mut self) -> Self {
        let row = self
            .row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::UserInputExchange),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::PerTurn,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_mutation_authority(ConsumerRouteMutationAuthority::ConsumerMediatedPerTurn(
                self.source.id().clone(),
            ))
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_observed(),
            )
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::ExchangeCallback,
                bounded("consumer-mediated Managed Agents tool exchange"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            ));
        self.session_start.push(row);
        self
    }

    fn push_portable_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        self.push_control(control, kind, bounded(value), omission);
    }

    fn push_namespaced_control(
        &mut self,
        semantic_id: &str,
        kind: ConsumerRouteValueKind,
        value: &str,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let extension = match ConsumerRouteNamespacedExtension::new(
            self.route,
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

fn feature_for(
    capability: Capability,
    include_output_limit: bool,
) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::OutputTokenLimit if include_output_limit => {
            ConsumerRouteFeatureId::OutputTokenLimit
        }
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::Attachments => ConsumerRouteFeatureId::Attachments,
        Capability::ToolCalls => ConsumerRouteFeatureId::ConsumerToolExchange,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::ExternalSearch => ConsumerRouteFeatureId::ExternalSearch,
        Capability::StreamReattachment => ConsumerRouteFeatureId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                MANAGED_ROUTE,
                "anthropic-managed-agents-2026-04-01",
                "feature.stream-reattachment",
            )
            .expect("static managed feature identity is valid"),
        ),
        Capability::ProviderManagedRecovery => ConsumerRouteFeatureId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                MANAGED_ROUTE,
                "anthropic-managed-agents-2026-04-01",
                "feature.provider-managed-recovery",
            )
            .expect("static managed feature identity is valid"),
        ),
        Capability::OwnedRemoteResourceDeletion => ConsumerRouteFeatureId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                MANAGED_ROUTE,
                "anthropic-managed-agents-2026-04-01",
                "feature.owned-remote-resource-cleanup",
            )
            .expect("static managed feature identity is valid"),
        ),
        Capability::ProviderDurableRetention => ConsumerRouteFeatureId::PersistentSessionPosture,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
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

fn bounded(value: &str) -> ConsumerRouteValueDomain {
    ConsumerRouteValueDomain::Unenumerated(
        ConsumerRouteEnumerableValue::new(value)
            .unwrap_or_else(|_| unreachable!("static Anthropic projection bound is admissible")),
    )
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
