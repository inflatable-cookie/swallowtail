#[derive(Clone, Debug, Eq, PartialEq)]
/// Request identity and optional deadline for Grok Build model discovery.
pub struct GrokCatalogueProfileInput {
    request_id: RequestId,
    deadline: Option<swallowtail_runtime::Deadline>,
}

impl GrokCatalogueProfileInput {
    /// Creates catalogue input with no deadline.
    #[must_use]
    pub const fn new(request_id: RequestId) -> Self {
        Self {
            request_id,
            deadline: None,
        }
    }

    /// Adds the operation deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: swallowtail_runtime::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared Grok Build model catalogue bound to exact installed `1.0.25`.
///
/// This is an authenticated non-inference metadata operation: it lists models
/// through one bounded `--no-auto-update models` process that may refresh
/// authentication or catalogue metadata but never sends a prompt, opens a
/// model session, or invokes inference.
pub struct GrokPreparedCatalogue {
    evidence: PreparedOperationEvidence,
    request: swallowtail_runtime::ModelCatalogRequest,
    environment: swallowtail_runtime::EnvironmentRef,
}

impl GrokPreparedCatalogue {
    /// Returns prepared operation and access evidence.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedOperationEvidence {
        &self.evidence
    }

    /// Returns the immutable catalogue preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the plan-derived catalogue request.
    #[must_use]
    pub const fn request(&self) -> &swallowtail_runtime::ModelCatalogRequest {
        &self.request
    }

    /// Creates the low-level driver bound to this catalogue.
    #[must_use]
    pub fn low_level_driver(&self) -> crate::GrokCatalogueDriver {
        crate::GrokCatalogueDriver::new(self.environment.clone())
    }

    /// Lists models through one bounded authenticated metadata process.
    pub fn list_models(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Vec<swallowtail_core::ModelCatalogEntry>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            swallowtail_runtime::ModelCatalogDriver::list_models(&driver, plan, request, services)
                .await
        })
    }

    /// Lists models and returns the redacted process output evidence captured
    /// before parsing.
    pub fn list_models_recorded(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<crate::GrokCatalogueListing, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.list_models_recorded(plan, request, services).await })
    }

    /// Splits the prepared catalogue into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        PreparedOperationEvidence,
        swallowtail_core::PreflightPlan,
        swallowtail_runtime::ModelCatalogRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GrokPreparedIntegration {
    /// Prepares a model catalogue from the admitted integration.
    ///
    /// Only an exact `1.0.25` observation prepares: every other executable
    /// version fails closed here, before any process starts. Access readiness
    /// was already proved while preparing the integration.
    pub fn prepare_catalogue(
        &self,
        input: GrokCatalogueProfileInput,
    ) -> Result<GrokPreparedCatalogue, PreparationFailure> {
        require_exact_catalogue_version(self)?;
        let capabilities = CapabilityProfile::new([CapabilityRequirement::new(
            swallowtail_core::Capability::ModelCatalog,
            [],
        )]);
        let instance = catalogue_instance(self, capabilities.clone());
        let requirements = catalogue_requirements(self, profile_requirements(&capabilities));
        let plan = build_catalogue_plan(self, &instance, &requirements)?;
        let request = match input.deadline {
            Some(deadline) => {
                swallowtail_runtime::ModelCatalogRequest::new(input.request_id)
                    .with_deadline(deadline)
            }
            None => swallowtail_runtime::ModelCatalogRequest::new(input.request_id),
        };
        Ok(GrokPreparedCatalogue {
            evidence: PreparedOperationEvidence::from_plan(plan, self.access_evidence().clone())?,
            request,
            environment: self.environment().clone(),
        })
    }
}

fn require_exact_catalogue_version(
    prepared: &GrokPreparedIntegration,
) -> Result<(), PreparationFailure> {
    let assessment =
        crate::grok_build_catalogue_claim().assess(prepared.observation().version().version());
    let qualified = matches!(
        assessment,
        swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
    ) && assessment.behavior_revision().is_some_and(|revision| {
        revision.as_str() == crate::selection::GROK_BUILD_CATALOGUE_BEHAVIOR
    });
    if !qualified {
        return Err(PreparationFailure::new(
            swallowtail_runtime::PreparationStage::CompatibilityClassification,
            swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.grok.preparation.catalogue_version_incompatible",
                "Grok Build catalogue requires exact installed version 1.0.25",
            )),
        ));
    }
    Ok(())
}

fn catalogue_instance(
    prepared: &GrokPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        prepared.instance().id().clone(),
        prepared.instance().revision().clone(),
        crate::grok_build_catalogue_descriptor()
            .identity()
            .id()
            .clone(),
        prepared.instance().execution_host_id().clone(),
        prepared.instance().target_reference().clone(),
        prepared.instance().ownership(),
        prepared.instance().access_profile_id().clone(),
        prepared.instance().support_authority(),
        swallowtail_core::ProtocolFacadeId::new("grok-cli-models-v1")
            .expect("static Grok catalogue facade is valid"),
        prepared.instance().policy_id().clone(),
        capabilities,
    )
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

fn catalogue_requirements(
    prepared: &GrokPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::ModelCatalog,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services([HostServiceKind::Process, HostServiceKind::Time])
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}
