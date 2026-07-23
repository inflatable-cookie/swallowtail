use crate::failure::unsupported;
use swallowtail_runtime::{
    ProviderExecutionPolicy, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    StreamReattachmentPolicy, validate_attached_runtime_residency_policy,
};

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.model_id().is_none() || plan.provider_id().is_some() {
        return Err(failure(
            "swallowtail.ollama.model_binding_rejected",
            "Ollama run requires one exact model without a provider identity",
        ));
    }
    if request.maximum_output_tokens().is_none()
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::OutputTokenLimit)
    {
        return Err(failure(
            "swallowtail.ollama.output_limit_missing",
            "Ollama run requires a preflight-bound maximum output-token input",
        ));
    }
    validate_attached_runtime_residency_policy(plan, request.policy())
        .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))?;
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
        || request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
        || request.policy().harness_isolation().is_some()
    {
        return Err(unsupported(
            "tools, reasoning, network, background, retention, recovery, or reattachment policy",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.ollama.deadline_elapsed",
            "Ollama deadline elapsed before provider work",
        ));
    }
    Ok(())
}
