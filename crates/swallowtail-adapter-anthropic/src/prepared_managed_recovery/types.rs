#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedRunReconciliationInput {
    request_id: RequestId,
    model: AnthropicManagedModelSelection,
    checkpoint: PersistedProviderRunCheckpoint,
    maximum_output_bytes: NonZeroU64,
    deadline: Option<Deadline>,
}

impl AnthropicManagedRunReconciliationInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: AnthropicManagedModelSelection,
        checkpoint: PersistedProviderRunCheckpoint,
        maximum_output_bytes: NonZeroU64,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            checkpoint,
            maximum_output_bytes,
            deadline,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnthropicPreparedManagedRunReconciliation {
    evidence: PreparedProviderRunReconciliationEvidence,
    request: ProviderRunReconciliationRequest,
}

impl AnthropicPreparedManagedRunReconciliation {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderRunReconciliationEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderRunReconciliationPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderRunReconciliationRequest {
        &self.request
    }

    pub fn reconcile(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderRunReconciliationOutcome, RuntimeFailure>> {
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            AnthropicManagedAgentDriver::new()
                .reconcile_provider_run(plan, request, services)
                .await
        })
    }
}

impl WorkingStateRestorationOperation for AnthropicPreparedManagedRunReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderRunReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.reconcile(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::RunReconciled)
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicManagedRecoveredCleanupInput {
    request_id: RequestId,
    model: AnthropicManagedModelSelection,
    binding: PersistedProviderRecoveredResourceCleanupBinding,
    deadline: Option<Deadline>,
}

impl AnthropicManagedRecoveredCleanupInput {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: AnthropicManagedModelSelection,
        binding: PersistedProviderRecoveredResourceCleanupBinding,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            model,
            binding,
            deadline,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnthropicPreparedManagedRecoveredCleanup {
    evidence: PreparedProviderRecoveredResourceCleanupEvidence,
    request: ProviderRecoveredResourceCleanupRequest,
}

impl AnthropicPreparedManagedRecoveredCleanup {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderRecoveredResourceCleanupEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderRecoveredResourceCleanupPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ProviderRecoveredResourceCleanupRequest {
        &self.request
    }

    pub fn cleanup(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderRecoveredResourceCleanupOutcome, RuntimeFailure>> {
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            AnthropicManagedAgentDriver::new()
                .cleanup_recovered_resources(plan, request, services)
                .await
        })
    }
}
