fn validate_evidence(
    evidence: &DeploymentEvidence,
    expected_model: Option<&str>,
) -> Result<(), RuntimeFailure> {
    let ChatTemplateCapabilities {
        supports_string_content,
        supports_typed_content,
        supports_tools,
        supports_tool_calls,
        supports_parallel_tool_calls,
        supports_system_role,
        supports_preserve_reasoning,
        supports_object_arguments,
    } = evidence.chat_template_capabilities;
    if evidence.chat_template != "chatml"
        || !supports_string_content
        || !supports_system_role
        || supports_typed_content
        || supports_tools
        || supports_tool_calls
        || supports_parallel_tool_calls
        || supports_preserve_reasoning
        || supports_object_arguments
        || expected_model.is_some_and(|model| model != evidence.model_alias)
    {
        return Err(failure(
            "swallowtail.llama_cpp.capability_evidence_mismatch",
            "llama.cpp deployment capabilities differ from the observed fixture",
        ));
    }
    Ok(())
}

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.model_id().is_none() || plan.provider_id().is_some() {
        return Err(failure(
            "swallowtail.llama_cpp.model_binding_rejected",
            "llama.cpp run requires one exact deployment model without a provider identity",
        ));
    }
    if request.maximum_output_tokens().is_none()
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::OutputTokenLimit)
    {
        return Err(failure(
            "swallowtail.llama_cpp.output_limit_missing",
            "llama.cpp run requires a preflight-bound maximum output-token input",
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
        || request.policy().provider_execution()
            != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || request.policy().provider_retention()
            != swallowtail_runtime::ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery()
            != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment()
            != swallowtail_runtime::StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            "tools, reasoning, network, background, retention, or reattachment policy",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.llama_cpp.deadline_elapsed",
            "llama.cpp deadline elapsed before provider work",
        ));
    }
    Ok(())
}
