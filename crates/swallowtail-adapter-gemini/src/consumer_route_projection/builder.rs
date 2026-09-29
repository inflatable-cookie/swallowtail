use super::{Projection, Route, bounded, exact, feature_for, namespaced};
use crate::{
    GEMINI_ACP_HTTP_MCP_PLACEMENT, GEMINI_ACP_MCP_SERVER_NAME, GeminiAcpHttpMcpPlacement,
    GeminiPreparedLiveSession,
};
use swallowtail_core::Capability;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteControlValue,
    ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteNamespacedExtension, ConsumerRouteOmissionSemantics, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass, ConsumerRouteStateSupport, ConsumerRouteValueDomain,
    ConsumerRouteValueKind,
};

impl Projection<'_> {
    pub(super) fn prepared(mut self) -> Self {
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
    pub(super) fn harness_mode(mut self) -> Self {
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
    pub(super) fn model_selection(mut self) -> Self {
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
    pub(super) fn live_controls(mut self, session: &GeminiPreparedLiveSession) -> Self {
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
    /// Publishes Contract 063's consumer-supplied streamable-HTTP placement
    /// when the session binds one: one namespaced row naming the placement
    /// token and the route-owned server name. Omission stays silent, and no
    /// URL or header value is ever carried.
    pub(super) fn http_mcp_placement(
        mut self,
        placement: Option<&GeminiAcpHttpMcpPlacement>,
    ) -> Self {
        let Some(_) = placement else {
            return self;
        };
        let identity = match ConsumerRouteNamespacedExtension::new(
            self.route.id(),
            MCP_PLACEMENT_VERSION,
            MCP_PLACEMENT_SEMANTIC_ID,
        ) {
            Ok(extension) => {
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(extension))
            }
            Err(error) => {
                self.rejected = Some(error);
                return self;
            }
        };
        let (token, name) = (
            ConsumerRouteEnumerableValue::new(GEMINI_ACP_HTTP_MCP_PLACEMENT),
            ConsumerRouteEnumerableValue::new(GEMINI_ACP_MCP_SERVER_NAME),
        );
        let values = match (token, name) {
            (Ok(token), Ok(name)) => ConsumerRouteEnumeratedValues::new([token, name]),
            (Err(error), _) | (_, Err(error)) => {
                self.rejected = Some(error);
                return self;
            }
        };
        let values = match values {
            Ok(values) => values,
            Err(error) => {
                self.rejected = Some(error);
                return self;
            }
        };
        self.selection.push(
            self.row(
                identity,
                &self.prepared_source,
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational)
            .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
            .with_state_support(
                ConsumerRouteStateSupport::descriptor_only()
                    .with_requested()
                    .with_prepared(),
            )
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::BoundedEnumeration,
                ConsumerRouteValueDomain::Enumerated(values),
                ConsumerRouteOmissionSemantics::NotSelectable,
            )),
        );
        self
    }
    pub(super) fn model_observation(&mut self) {
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
}

const MCP_PLACEMENT_SEMANTIC_ID: &str = "mcp.placement";
const MCP_PLACEMENT_VERSION: &str = "acp-v1";
