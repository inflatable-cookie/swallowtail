use crate::failure::unsupported;
use std::num::NonZeroU32;
use swallowtail_core::{
    CancellationScope, CapabilityConstraint, ExternalNetworkPolicy, ExternalSearchPolicy,
};
use swallowtail_runtime::{
    ProviderExecutionPolicy, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    StreamReattachmentPolicy, StructuredRunRequest,
};

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.model_route_id().is_none()
        || plan.model_id().is_none()
        || plan.provider_id().is_none_or(|provider| provider.as_str() != crate::INTEGRATION_FAMILY)
    {
        return Err(failure(
            "swallowtail.openai.model_binding_rejected",
            "OpenAI background inference requires one exact OpenAI model route",
        ));
    }
    for capability in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
        Capability::ProviderBackgroundExecution,
        Capability::ProviderTemporaryRetention,
        Capability::StreamReattachment,
        Capability::Interruption,
    ] {
        if !requires(plan, capability) {
            return Err(failure(
                "swallowtail.openai.capability_binding_rejected",
                "OpenAI background capability requirements did not match the driver subset",
            ));
        }
    }
    let supported = [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
        Capability::OutputTokenLimit,
        Capability::ProviderBackgroundExecution,
        Capability::ProviderTemporaryRetention,
        Capability::StreamReattachment,
        Capability::Interruption,
    ];
    if plan
        .requirements()
        .capabilities()
        .any(|requirement| !supported.contains(&requirement.capability()))
    {
        return Err(failure(
            "swallowtail.openai.capability_binding_rejected",
            "OpenAI background requirements included an unsupported capability",
        ));
    }
    let reattachment = CapabilityConstraint::ReattachmentMaximumCount(1);
    let cancellation = CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun);
    if !plan.requirements().capabilities().all(|requirement| {
        let constraints: Vec<_> = requirement.constraints().collect();
        match requirement.capability() {
            Capability::StreamReattachment => constraints == [&reattachment],
            Capability::Interruption => constraints == [&cancellation],
            _ => constraints.is_empty(),
        }
    }) {
        return Err(failure(
            "swallowtail.openai.capability_constraint_rejected",
            "OpenAI background reattachment or cancellation bounds did not match",
        ));
    }
    if request.maximum_output_tokens().is_none() {
        return Err(failure(
            "swallowtail.openai.output_limit_missing",
            "OpenAI background inference requires a positive output-token limit",
        ));
    }
    if request.deadline().is_none() {
        return Err(failure(
            "swallowtail.openai.deadline_missing",
            "OpenAI background inference requires an operation deadline",
        ));
    }
    if request.working_resource().is_some() {
        return Err(unsupported("a working resource"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("attachments"));
    }
    if request.tools().len() != 0 {
        return Err(unsupported("structured-run tools"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    if request.policy().reasoning_mode().is_some()
        || request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().provider_execution() != ProviderExecutionPolicy::Background
        || request.policy().provider_retention() != ProviderRetentionPolicy::TemporaryAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment()
            != StreamReattachmentPolicy::Bounded(
                NonZeroU32::new(1).expect("one is non-zero"),
            )
    {
        return Err(unsupported(
            "tools, reasoning, network, storage, background, or reattachment policy drift",
        ));
    }
    if services.time().expect("validated time").now()
        >= request.deadline().expect("validated deadline").instant()
    {
        return Err(failure(
            "swallowtail.openai.deadline_elapsed",
            "OpenAI deadline elapsed before provider work",
        ));
    }
    Ok(())
}
