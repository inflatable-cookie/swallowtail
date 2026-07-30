use super::input::OllamaInferenceAttemptInput;
use super::plan::{
    OllamaPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{OllamaNativeAttachedDriver, OllamaPreparedIntegration};
use swallowtail_core::{
    AttachedRuntimeResidency, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, DriverRole, PreflightPlan, ReasoningMode, StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, SchemaDocument, StructuredOutputDescriptor, StructuredRunDriver,
    StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OllamaPreparedInferenceAttempt {
    evidence: OllamaPreparedEvidence,
    request: StructuredRunRequest,
}

impl OllamaPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &OllamaPreparedEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &PreflightPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &StructuredRunRequest {
        &self.request
    }

    #[must_use]
    pub fn low_level_driver(&self) -> OllamaNativeAttachedDriver {
        OllamaNativeAttachedDriver::new()
    }

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
    pub fn into_parts(self) -> (OllamaPreparedEvidence, PreflightPlan, StructuredRunRequest) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl OllamaPreparedIntegration {
    pub fn prepare_inference_attempt(
        &self,
        input: OllamaInferenceAttemptInput,
    ) -> Result<OllamaPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, content, maximum, reasoning, structured_output, deadline) =
            input.into_parts();
        if maximum.get() > u64::from(u32::MAX) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.ollama.preparation.output_limit_invalid",
                "Ollama maximum output tokens exceed the supported request range",
            ));
        }
        if let Some(reasoning) = reasoning.as_ref() {
            validate_reasoning(self, reasoning)?;
        }
        if let Some(output) = structured_output.as_ref() {
            validate_structured_output(output)?;
        }
        let capability_requirements = inference_capabilities(
            maximum.get(),
            reasoning.as_ref(),
            structured_output.as_ref(),
        );
        let activity = crate::activity::profile::activity_profile(self);
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
        let route = model_route(self, self.model_selection().clone(), capabilities);
        let requirements = requirements(
            self,
            &route,
            swallowtail_core::OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            capability_requirements,
        );
        let plan = build_plan(self, &instance, &route, &requirements)?;
        let mut policy = OperationPolicy::offline()
            .with_attached_runtime_residency(AttachedRuntimeResidency::RuntimeManaged);
        if let Some(reasoning) = reasoning {
            policy = policy.with_reasoning_mode(reasoning);
        }
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum);
        if let Some(output) = structured_output {
            request = request.with_structured_output(output);
        }
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(OllamaPreparedInferenceAttempt {
            evidence: OllamaPreparedEvidence::from_prepared_with_activity(self, plan, activity)?,
            request,
        })
    }
}

fn inference_capabilities(
    maximum: u64,
    reasoning: Option<&ReasoningMode>,
    structured_output: Option<&StructuredOutputDescriptor>,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::OutputTokenLimit,
            [CapabilityConstraint::OutputTokenMaximum(maximum)],
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
    capabilities
}

fn validate_reasoning(
    prepared: &OllamaPreparedIntegration,
    reasoning: &ReasoningMode,
) -> Result<(), PreparationFailure> {
    if prepared
        .runtime()
        .selected_model_supports(crate::OllamaModelCapability::Thinking)
        && matches!(reasoning.as_str(), "off" | "low" | "medium" | "high")
    {
        Ok(())
    } else {
        Err(failure(
            PreparationStage::Preflight,
            "swallowtail.ollama.preparation.reasoning_unsupported",
            "The selected Ollama model does not support the exact requested reasoning mode",
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
            "swallowtail.ollama.preparation.schema_unsupported",
            "Ollama structured output requires one inline JSON Schema 2020-12 object",
        ))
    }
}
