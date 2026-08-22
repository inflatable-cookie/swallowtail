use super::input::OpenAiBackgroundRunProfileInput;
use super::plan::{
    OpenAiBackgroundPreparedEvidence, build_plan, instance_with_capabilities, model_route,
};
use crate::prepared::failure;
use crate::{OpenAiBackgroundDriver, OpenAiBackgroundPreparedIntegration};
use std::num::NonZeroU32;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    PreflightPlan, ReasoningMode, StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage,
    ProviderExecutionPolicy, ProviderRetentionPolicy, RunHandle, RuntimeFailure, SchemaDocument,
    StreamReattachmentPolicy, StructuredOutputDescriptor, StructuredRunDriver,
    StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Executable OpenAI background run with exact plan and request agreement.
pub struct OpenAiPreparedBackgroundRun {
    evidence: OpenAiBackgroundPreparedEvidence,
    request: StructuredRunRequest,
}

impl OpenAiPreparedBackgroundRun {
    #[must_use]
    /// Returns the route-specific prepared evidence.
    pub const fn evidence(&self) -> &OpenAiBackgroundPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    /// Returns the plan-derived structured-run request.
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    /// Returns the public low-level background driver.
    pub fn low_level_driver(&self) -> OpenAiBackgroundDriver {
        OpenAiBackgroundDriver::new()
    }

    /// Starts the single provider inference attempt.
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
    /// Splits the prepared value into evidence, plan, and request.
    pub fn into_parts(
        self,
    ) -> (
        OpenAiBackgroundPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OpenAiBackgroundPreparedIntegration {
    /// Validates and prepares one explicitly retained background run.
    pub fn prepare_background_run(
        &self,
        input: OpenAiBackgroundRunProfileInput,
    ) -> Result<OpenAiPreparedBackgroundRun, PreparationFailure> {
        let (
            request_id,
            model,
            content,
            maximum,
            reasoning,
            structured_output,
            deadline,
            provider_execution,
            provider_retention,
            stream_reattachment,
            active_run_detachment,
        ) = input.into_parts();
        if provider_execution != ProviderExecutionPolicy::Background {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.background_not_selected",
                "OpenAI provider-owned background execution must be selected explicitly",
            ));
        }
        if provider_retention != ProviderRetentionPolicy::TemporaryAllowed {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.retention_not_accepted",
                "OpenAI background temporary provider retention must be accepted explicitly",
            ));
        }
        if stream_reattachment
            != StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.reattachment_rejected",
                "OpenAI background preparation permits exactly one stream reattachment",
            ));
        }
        if maximum.get() > u64::from(u32::MAX) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.output_limit_invalid",
                "OpenAI maximum output tokens exceed the supported request range",
            ));
        }
        if let Some(reasoning) = reasoning.as_ref() {
            validate_reasoning(reasoning)?;
        }
        if let Some(output) = structured_output.as_ref() {
            validate_structured_output(output)?;
        }
        let capability_requirements = run_capabilities(
            maximum.get(),
            reasoning.as_ref(),
            structured_output.as_ref(),
            active_run_detachment,
        );
        let activity = crate::activity::profile::activity_profile();
        let capabilities = crate::activity::profile::with_activity(
            CapabilityProfile::new(capability_requirements),
            &activity,
        );
        let capability_requirements = capabilities
            .iter()
            .map(|(capability, constraints)| {
                CapabilityRequirement::new(capability, constraints.iter().cloned())
            })
            .collect::<Vec<_>>();
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        if route.id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ROUTE_ID
            || route.model_id().as_str() != crate::OPENAI_BACKGROUND_MODEL_ID
        {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.openai.preparation.route_rejected",
                "OpenAI background preparation requires the exact GPT-5.6 route",
            ));
        }
        let plan = build_plan(self, &instance, &route, capability_requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_provider_execution(provider_execution)
            .with_provider_retention(provider_retention)
            .with_stream_reattachment(stream_reattachment);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum)
            .with_deadline(deadline);
        if let Some(output) = structured_output {
            request = request.with_structured_output(output);
        }
        Ok(OpenAiPreparedBackgroundRun {
            evidence: OpenAiBackgroundPreparedEvidence::from_prepared(self, plan, activity)?,
            request,
        })
    }
}

fn run_capabilities(
    maximum: u64,
    reasoning: Option<&ReasoningMode>,
    structured_output: Option<&StructuredOutputDescriptor>,
    active_run_detachment: bool,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(maximum)],
        ),
        CapabilityRequirement::new(Capability::ProviderBackgroundExecution, []),
        CapabilityRequirement::new(Capability::ProviderTemporaryRetention, []),
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            [CapabilityConstraint::OwnedRemoteResource(
                swallowtail_core::OwnedRemoteResourceKind::Response,
            )],
        ),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
    ];
    if let Some(reasoning) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::ReasoningMode(reasoning.clone())],
        ));
    }
    if let Some(output) = structured_output {
        capabilities.push(CapabilityRequirement::new(
            Capability::StructuredOutput,
            [
                CapabilityConstraint::SchemaDialect(output.dialect().to_owned()),
                CapabilityConstraint::StructuredOutputEnforcement(
                    StructuredOutputEnforcement::ProviderNative,
                ),
            ],
        ));
    }
    if active_run_detachment {
        capabilities.push(CapabilityRequirement::new(
            Capability::ActiveOperationDetachment,
            [CapabilityConstraint::OperationDetachmentScope(
                swallowtail_core::OperationDetachmentScope::StructuredRun,
            )],
        ));
    }
    capabilities
}

fn validate_reasoning(reasoning: &ReasoningMode) -> Result<(), PreparationFailure> {
    if matches!(
        reasoning.as_str(),
        "none" | "low" | "medium" | "high" | "xhigh" | "max"
    ) {
        Ok(())
    } else {
        Err(failure(
            PreparationStage::Preflight,
            "swallowtail.openai.preparation.reasoning_unsupported",
            "OpenAI background reasoning selection is unsupported for the exact GPT-5.6 route",
        ))
    }
}

fn validate_structured_output(
    output: &StructuredOutputDescriptor,
) -> Result<(), PreparationFailure> {
    let valid_document = match output.document() {
        SchemaDocument::Inline(bytes) => serde_json::from_slice::<serde_json::Value>(bytes)
            .is_ok_and(|schema| schema.is_object()),
        SchemaDocument::Reference(_) => false,
    };
    if output.media_type() == "application/schema+json"
        && output.dialect() == "json-schema-2020-12"
        && valid_document
    {
        Ok(())
    } else {
        Err(failure(
            PreparationStage::Preflight,
            "swallowtail.openai.preparation.schema_unsupported",
            "OpenAI background structured output requires one inline JSON Schema 2020-12 object",
        ))
    }
}
