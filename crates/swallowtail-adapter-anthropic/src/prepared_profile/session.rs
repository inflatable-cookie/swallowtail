use super::input::AnthropicSessionProfileInput;
use super::plan::{
    AnthropicPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{AnthropicDirectDriver, AnthropicPreparedIntegration};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    DirectAttemptTransport, DirectContinuationConfig, DirectContinuationRequirements,
    DirectToolSelection, DriverRole, ProviderInferenceCachePolicy,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle,
    OpenDirectContinuationSessionRequest, PreparationFailure, PreparationStage, RuntimeFailure,
    SessionOptions,
};

#[must_use]
pub fn anthropic_messages_continuation_config() -> DirectContinuationConfig {
    DirectContinuationConfig::new(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(3).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(1).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU64::new(262_144).unwrap(),
        NonZeroU64::new(1_048_576).unwrap(),
        NonZeroU32::new(4_096).unwrap(),
        NonZeroU64::new(8_192).unwrap(),
        DirectAttemptTransport::ServerSentEvents,
        DirectAttemptTransport::ServerSentEvents,
        DirectToolSelection::ProviderAutomatic,
        ProviderInferenceCachePolicy::Prohibited,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicPreparedSession {
    evidence: AnthropicPreparedEvidence,
    request: OpenDirectContinuationSessionRequest,
}

impl AnthropicPreparedSession {
    #[must_use]
    pub const fn evidence(&self) -> &AnthropicPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &swallowtail_core::PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &OpenDirectContinuationSessionRequest {
        &self.request
    }

    pub fn open_session(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let driver = AnthropicDirectDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .open_direct_continuation_session(plan, request, services)
                .await
        })
    }
}

impl AnthropicPreparedIntegration {
    pub fn prepare_session(
        &self,
        input: AnthropicSessionProfileInput,
    ) -> Result<AnthropicPreparedSession, PreparationFailure> {
        let (request_id, model, tools) = input.into_parts();
        if tools.is_empty() || tools.len() > 8 {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.preparation.tools_rejected",
                "Anthropic continuation requires one to eight declared consumer tools",
            ));
        }
        let config = anthropic_messages_continuation_config();
        let mut capabilities = vec![
            CapabilityRequirement::new(Capability::InteractiveSession, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            CapabilityRequirement::new(Capability::ToolCalls, []),
            CapabilityRequirement::new(Capability::UsageReporting, []),
            CapabilityRequirement::new(Capability::OutputTokenLimit, []),
            CapabilityRequirement::new(
                Capability::Interruption,
                [CapabilityConstraint::CancellationScope(
                    CancellationScope::ActiveTurn,
                )],
            ),
        ];
        capabilities.extend(config.capability_requirements());
        let profile = CapabilityProfile::new(capabilities.clone());
        let instance = instance_with_capabilities(self, profile.clone());
        let route = model_route(self, model, profile);
        let requirements = requirements(self, DriverRole::InteractiveSession, capabilities, [])
            .with_direct_continuation(DirectContinuationRequirements::new(
                route.model_id().clone(),
                config.clone(),
            ))
            .require_model_route();
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let request = OpenDirectContinuationSessionRequest::new(request_id, config)
            .with_options(SessionOptions::default().with_tools(tools));
        swallowtail_runtime::validate_direct_continuation_plan(&plan, &request).map_err(
            |error| {
                PreparationFailure::new(
                    PreparationStage::Preflight,
                    swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
                )
            },
        )?;
        Ok(AnthropicPreparedSession {
            evidence: AnthropicPreparedEvidence::from_prepared(self, plan)?,
            request,
        })
    }
}
