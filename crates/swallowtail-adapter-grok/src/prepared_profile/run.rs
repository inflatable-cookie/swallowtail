#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one prepared Grok structured run.
pub struct GrokRunProfileInput {
    request_id: RequestId,
    model: GrokModelSelection,
    content: swallowtail_runtime::OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Option<swallowtail_runtime::Deadline>,
}

impl GrokRunProfileInput {
    /// Creates a Grok run profile with an optional deadline.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: GrokModelSelection,
        content: swallowtail_runtime::OperationContent,
        working_resource: WorkingResourceRef,
        deadline: Option<swallowtail_runtime::Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            content,
            working_resource,
            deadline,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Prepared one-shot Grok ACP structured run.
pub struct GrokPreparedRun {
    evidence: GrokPreparedEvidence,
    request: swallowtail_runtime::StructuredRunRequest,
}

/// Future returned when a prepared Grok run starts.
pub type GrokPreparedRunFuture =
    BoxFuture<'static, Result<Box<dyn swallowtail_runtime::RunHandle>, RuntimeFailure>>;

impl GrokPreparedRun {
    /// Returns portable evidence for the prepared run.
    #[must_use]
    pub const fn evidence(&self) -> &GrokPreparedEvidence {
        &self.evidence
    }

    /// Returns the validated preflight plan.
    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    /// Returns the bound structured-run request.
    #[must_use]
    pub const fn request(&self) -> &swallowtail_runtime::StructuredRunRequest {
        &self.request
    }

    /// Creates the low-level driver bound to this run.
    #[must_use]
    pub fn low_level_driver(&self) -> GrokAcpDriver {
        self.evidence.low_level_driver()
    }

    /// Starts the prepared run with caller-supplied host services.
    pub fn start_run(&self, services: HostServices) -> GrokPreparedRunFuture {
        use swallowtail_runtime::StructuredRunDriver;
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    /// Splits the prepared run into evidence, plan, and request.
    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        GrokPreparedEvidence,
        PreflightPlan,
        swallowtail_runtime::StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl GrokPreparedIntegration {
    /// Prepares a structured run from the admitted integration.
    pub fn prepare_run(
        &self,
        input: GrokRunProfileInput,
    ) -> Result<GrokPreparedRun, PreparationFailure> {
        validate_prepared_model(self, input.model.model_id.as_str())?;
        let activity_profile = activity_profile(self)?;
        let capabilities = with_activity(crate::prepared::run_capabilities(), &activity_profile);
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = ModelRoute::new(
            input.model.route_id,
            input.model.route_revision,
            self.instance().id().clone(),
            input.model.model_id,
            capabilities.clone(),
        );
        let requirements = run_requirements(self, profile_requirements(&capabilities));
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let policy = swallowtail_runtime::OperationPolicy::offline()
            .with_provider_retention(swallowtail_runtime::ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
        let mut request = swallowtail_runtime::StructuredRunRequest::new(
            input.request_id,
            input.content,
            policy,
        )
        .with_working_resource(input.working_resource);
        if let Some(deadline) = input.deadline {
            request = request.with_deadline(deadline);
        }
        Ok(GrokPreparedRun {
            evidence: GrokPreparedEvidence::from_prepared(self, plan, activity_profile)?,
            request,
        })
    }
}
