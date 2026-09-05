use super::*;

impl<'a> Projection<'a> {
    pub(super) fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
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

    pub(super) fn row(
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

    pub(super) fn with_prepared_capabilities(mut self) -> Self {
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

    pub(super) fn with_model_selection(mut self) -> Self {
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

    pub(super) fn with_question_and_permission(mut self) -> Self {
        if !self.callback_namespaces() {
            return self;
        }
        match namespaced_feature(PERMISSION_NAMESPACE, "feature.permission-exchange") {
            Ok(extension) => self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(
                        extension,
                    )),
                    ConsumerRouteSourceClass::AdapterPreparedInput,
                    ConsumerRouteEvidenceStrength::RouteValidation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            ),
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
            }
        }
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

    pub(super) fn with_run_controls(mut self, run: &OpenCodePreparedRun) -> Self {
        if run.request().policy().reasoning_mode().is_some() {
            self.push_session_start_control(
                namespaced_control(self.plan, "control.reasoning-selection"),
                ConsumerRouteValueKind::BoundedEnumeration,
                bounded("the selected OpenCode reasoning mode accepted by the prepared route"),
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            );
        }
        if run.request().structured_output().is_some() {
            self.push_session_start_control(
                namespaced_control(self.plan, "control.structured-output"),
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("the bounded JSON Schema document accepted by the prepared route"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if has_capability(self.plan, Capability::Attachments) {
            self.push_session_start_control(
                namespaced_control(self.plan, "control.attachments"),
                ConsumerRouteValueKind::StructuredDeclarations,
                bounded("bounded PNG attachment media, count, and byte constraints"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if self.callback_namespaces() {
            self.push_per_turn_control(
                namespaced_control(self.plan, "control.provider-callbacks"),
                "consumer-mediated OpenCode permission and question callbacks",
            );
        }
        self
    }

    pub(super) fn with_owned_remote_cleanup(mut self) -> Self {
        if !has_capability(self.plan, Capability::OwnedRemoteResourceDeletion) {
            return self;
        }
        match ConsumerRouteNamespacedExtension::new(
            ROUTE_ID,
            self.plan.protocol_facade_id().as_str(),
            "feature.owned-remote-resource-cleanup",
        ) {
            Ok(extension) => self.selection.push(
                self.row(
                    ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(
                        extension,
                    )),
                    ConsumerRouteSourceClass::CapabilityProfile,
                    ConsumerRouteEvidenceStrength::PreparedOperation,
                    ConsumerRouteLifecycle::SelectionSummary,
                )
                .with_actor_posture(ConsumerRouteActorPosture::Informational),
            ),
            Err(rejection) => {
                self.rejected.get_or_insert(rejection);
            }
        }
        self
    }

    pub(super) fn with_session_controls(mut self) -> Self {
        if has_capability(self.plan, Capability::Attachments) {
            self.push_per_turn_control(
                namespaced_control(self.plan, "control.attachments"),
                "consumer-mediated bounded PNG attachment exchange",
            );
        }
        if has_capability(self.plan, Capability::ActiveOperationDetachment) {
            self.push_session_start_control(
                namespaced_control(self.plan, "control.active-turn-detachment"),
                ConsumerRouteValueKind::LifecycleAction,
                bounded("active-turn detachment with durable OpenCode session binding"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            );
        }
        if self.callback_namespaces() {
            self.push_per_turn_control(
                namespaced_control(self.plan, "control.provider-callbacks"),
                "consumer-mediated OpenCode permission and question callbacks",
            );
        }
        self.push_session_start_control(
            ConsumerRouteControlId::LoadSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact provider session identity and prepared route binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self.push_session_start_control(
            ConsumerRouteControlId::ResumeSession,
            ConsumerRouteValueKind::LifecycleAction,
            bounded("exact provider session identity and prepared route binding"),
            ConsumerRouteOmissionSemantics::SuppliesNothing,
        );
        self
    }

    pub(super) fn with_provider_session_query(mut self) -> Self {
        self.active_session.push(
            self.row(
                ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::SessionCatalogueBounds),
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteEvidenceStrength::RouteValidation,
                ConsumerRouteLifecycle::PostOpenObservationOnly,
            )
            .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
            .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::BoundedQuery,
                bounded("bounded working-resource-scoped retained-session catalogue query"),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            )),
        );
        self
    }

    pub(super) fn push_session_start_control(
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

    pub(super) fn push_per_turn_control(&mut self, control: ConsumerRouteControlId, bound: &str) {
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
                bounded(bound),
                ConsumerRouteOmissionSemantics::SuppliesNothing,
            )),
        );
    }

    pub(super) fn callback_namespaces(&self) -> bool {
        let has = |namespaces: &mut dyn Iterator<Item = &ExtensionNamespace>| {
            let namespaces = namespaces.collect::<Vec<_>>();
            namespaces.len() == 2
                && namespaces
                    .iter()
                    .any(|item| item.as_str() == PERMISSION_NAMESPACE)
                && namespaces
                    .iter()
                    .any(|item| item.as_str() == QUESTION_NAMESPACE)
        };
        if self.plan.requirements().operation_shape() == OperationShape::InteractiveSession {
            self.plan
                .requirements()
                .session_access_policy()
                .is_some_and(|policy| {
                    policy.approval_policy() == ProviderApprovalPolicy::ConsumerMediated
                        && has(&mut policy.provider_requests().exchanged_extensions())
                })
        } else {
            has(&mut self.plan.requirements().extension_namespaces())
        }
    }

    pub(super) fn build(self) -> Contribution {
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
