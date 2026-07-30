#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokRunProfileInput {
    request_id: RequestId,
    model: GrokModelSelection,
    content: swallowtail_runtime::OperationContent,
    working_resource: WorkingResourceRef,
    deadline: Option<swallowtail_runtime::Deadline>,
}

impl GrokRunProfileInput {
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
pub struct GrokPreparedRun {
    evidence: GrokPreparedEvidence,
    request: swallowtail_runtime::StructuredRunRequest,
}

pub type GrokPreparedRunFuture =
    BoxFuture<'static, Result<Box<dyn swallowtail_runtime::RunHandle>, RuntimeFailure>>;

impl GrokPreparedRun {
    #[must_use]
    pub const fn evidence(&self) -> &GrokPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &swallowtail_runtime::StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> GrokAcpDriver {
        self.evidence.low_level_driver()
    }

    pub fn start_run(&self, services: HostServices) -> GrokPreparedRunFuture {
        use swallowtail_runtime::StructuredRunDriver;
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

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
    pub fn prepare_run(
        &self,
        input: GrokRunProfileInput,
    ) -> Result<GrokPreparedRun, PreparationFailure> {
        if input.model.model_id.as_str() != "grok-4.5" {
            return Err(preparation_failure(
                "swallowtail.grok.preparation.model_unsupported",
                "Grok prepared runs require the qualified grok-4.5 model",
            ));
        }
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
