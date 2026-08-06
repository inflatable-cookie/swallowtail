use super::input::AnthropicManagedAgentRunInput;
use super::plan::{AnthropicManagedPreparedEvidence, build_plan, model_route};
use crate::prepared_managed::failure;
use crate::{AnthropicManagedAgentDriver, AnthropicManagedPreparedIntegration};
use std::num::NonZeroU32;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RunHandle, RuntimeFailure,
    StreamReattachmentPolicy, StructuredRunDriver, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bound prepared Anthropic Managed Agents run.
pub struct AnthropicPreparedManagedAgentRun {
    evidence: AnthropicManagedPreparedEvidence,
    request: StructuredRunRequest,
}

impl AnthropicPreparedManagedAgentRun {
    #[must_use]
    /// Returns the prepared operation evidence.
    pub const fn evidence(&self) -> &AnthropicManagedPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable managed-run plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the low-level Managed Agents driver.
    pub fn low_level_driver(&self) -> AnthropicManagedAgentDriver {
        AnthropicManagedAgentDriver::new()
    }

    /// Starts the bound provider-managed run.
    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.start_run(plan, request, services).await })
    }

    #[must_use]
    /// Splits the prepared operation into evidence, plan, and request.
    pub fn into_parts(
        self,
    ) -> (
        AnthropicManagedPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AnthropicManagedPreparedIntegration {
    /// Prepares a durable managed-agent run with explicit policy acceptance.
    pub fn prepare_managed_run(
        &self,
        input: AnthropicManagedAgentRunInput,
    ) -> Result<AnthropicPreparedManagedAgentRun, PreparationFailure> {
        let (
            request_id,
            model,
            content,
            deadline,
            tools,
            provider_retention,
            provider_recovery,
            stream_reattachment,
            cross_process_recovery,
        ) = input.into_parts();
        if provider_retention != ProviderRetentionPolicy::DurableAllowed {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.managed.preparation.retention_not_accepted",
                "Anthropic Managed Agents durable provider retention must be accepted explicitly",
            ));
        }
        if provider_recovery != ProviderRecoveryPolicy::ManagedAllowed {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.managed.preparation.recovery_not_accepted",
                "Anthropic Managed Agents provider-managed recovery must be accepted explicitly",
            ));
        }
        if stream_reattachment
            != StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.managed.preparation.reattachment_rejected",
                "Anthropic Managed Agents preparation permits one authoritative-history reattachment",
            ));
        }
        if tools.len() > 8 {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.managed.preparation.tool_limit_exceeded",
                "Anthropic Managed Agents preparation permits at most eight custom tools",
            ));
        }
        let route = model_route(self, model);
        let plan = build_plan(self, &route, cross_process_recovery)?;
        let policy = OperationPolicy::offline()
            .with_provider_retention(provider_retention)
            .with_provider_recovery(provider_recovery)
            .with_stream_reattachment(stream_reattachment);
        let request = StructuredRunRequest::new(request_id, content, policy)
            .with_deadline(deadline)
            .with_tools(tools);
        Ok(AnthropicPreparedManagedAgentRun {
            evidence: AnthropicManagedPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}
