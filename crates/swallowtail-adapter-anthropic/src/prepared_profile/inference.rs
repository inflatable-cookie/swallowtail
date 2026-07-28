use super::input::AnthropicInferenceAttemptInput;
use super::plan::{
    AnthropicPreparedEvidence, build_plan, instance_with_capabilities, model_route, requirements,
};
use crate::prepared::failure;
use crate::{AnthropicDirectDriver, AnthropicPreparedIntegration};
use swallowtail_core::{
    Capability, CapabilityProfile, CapabilityRequirement, DriverRole, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, HostServices, OperationPolicy, PreparationFailure, PreparationStage, RunHandle,
    RuntimeFailure, StructuredRunRequest,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnthropicPreparedInferenceAttempt {
    evidence: AnthropicPreparedEvidence,
    request: StructuredRunRequest,
    search_domains: Option<Vec<String>>,
}

impl AnthropicPreparedInferenceAttempt {
    #[must_use]
    pub const fn evidence(&self) -> &AnthropicPreparedEvidence {
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
    pub fn low_level_driver(&self) -> AnthropicDirectDriver {
        AnthropicDirectDriver::new()
    }

    pub fn start_run(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        let driver = self.low_level_driver();
        let plan = self.plan().clone();
        let request = self.request.clone();
        let search_domains = self.search_domains.clone();
        Box::pin(async move {
            driver
                .start_prepared_run(plan, request, search_domains, services)
                .await
        })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        AnthropicPreparedEvidence,
        PreflightPlan,
        StructuredRunRequest,
    ) {
        let plan = self.evidence.plan().clone();
        (self.evidence, plan, self.request)
    }
}

impl AnthropicPreparedIntegration {
    pub fn prepare_inference_attempt(
        &self,
        input: AnthropicInferenceAttemptInput,
    ) -> Result<AnthropicPreparedInferenceAttempt, PreparationFailure> {
        let (request_id, model, content, maximum, deadline, attachments, web_search) =
            input.into_parts();
        if maximum.get() > u64::from(u32::MAX) {
            return Err(failure(
                PreparationStage::Preflight,
                "swallowtail.anthropic.preparation.output_limit_invalid",
                "Anthropic maximum output tokens exceed the supported request range",
            ));
        }
        validate_attachments(&attachments)?;
        let search_domains =
            web_search.map(super::input::AnthropicWebSearchInput::into_allowed_domains);
        if let Some(domains) = search_domains.as_deref() {
            validate_search_domains(domains)?;
        }
        let capability_requirements =
            inference_capabilities(!attachments.is_empty(), search_domains.is_some());
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = instance_with_capabilities(self, capabilities.clone());
        let route = model_route(self, model, capabilities);
        let requirements = requirements(
            self,
            DriverRole::StructuredRun,
            capability_requirements,
            (!attachments.is_empty()).then_some(swallowtail_core::HostServiceKind::Attachment),
        )
        .require_model_route();
        let plan = build_plan(self, &instance, Some(&route), &requirements)?;
        let policy = if search_domains.is_some() {
            OperationPolicy::new(
                swallowtail_core::ExternalNetworkPolicy::HostApproved,
                swallowtail_core::ExternalSearchPolicy::Enabled,
            )
            .expect("host-approved search policy is valid")
        } else {
            OperationPolicy::offline()
        };
        let mut request = StructuredRunRequest::new(request_id, content, policy)
            .with_maximum_output_tokens(maximum)
            .with_attachments(attachments);
        if let Some(deadline) = deadline {
            request = request.with_deadline(deadline);
        }
        Ok(AnthropicPreparedInferenceAttempt {
            evidence: AnthropicPreparedEvidence::from_prepared(self, plan)?,
            request,
            search_domains,
        })
    }
}

fn inference_capabilities(image_attachments: bool, web_search: bool) -> Vec<CapabilityRequirement> {
    let mut capabilities: Vec<_> = [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
    ]
    .into_iter()
    .map(|capability| CapabilityRequirement::new(capability, []))
    .collect();
    if image_attachments {
        capabilities.push(CapabilityRequirement::new(
            Capability::Attachments,
            [
                swallowtail_core::CapabilityConstraint::attachment_media_type("image/png")
                    .expect("static media type is valid"),
                swallowtail_core::CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
                swallowtail_core::CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        ));
    }
    if web_search {
        capabilities.extend([
            CapabilityRequirement::new(Capability::ProviderExternalNetwork, []),
            CapabilityRequirement::new(Capability::ExternalSearch, []),
        ]);
    }
    capabilities
}

fn validate_attachments(
    attachments: &[swallowtail_runtime::AttachmentDescriptor],
) -> Result<(), PreparationFailure> {
    if attachments.len() > 1
        || attachments.iter().any(|attachment| {
            attachment.media_type() != "image/png"
                || attachment.role() != swallowtail_runtime::AttachmentRole::Input
                || attachment
                    .known_length()
                    .is_some_and(|length| length > 1024 * 1024)
        })
    {
        return Err(failure(
            PreparationStage::Preflight,
            "swallowtail.anthropic.preparation.attachments_unsupported",
            "Anthropic Messages supports one input PNG up to one MiB on this prepared role",
        ));
    }
    Ok(())
}

fn validate_search_domains(domains: &[String]) -> Result<(), PreparationFailure> {
    if domains.is_empty()
        || domains.len() > 10
        || domains.iter().any(|domain| {
            domain.trim().is_empty()
                || domain.contains('/')
                || domain.contains(':')
                || domain.chars().any(char::is_whitespace)
        })
    {
        return Err(failure(
            PreparationStage::Preflight,
            "swallowtail.anthropic.preparation.search_scope_rejected",
            "Anthropic web search requires one to ten bare allowed domains",
        ));
    }
    Ok(())
}
