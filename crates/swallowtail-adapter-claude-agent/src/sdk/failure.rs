use super::wire::ClaudeAgentSdkFailureCode;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    crate::failure::failure(code, message)
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.protocol_failed",
        "Claude Agent SDK sidecar stream did not match the qualified protocol",
    )
}

pub(crate) fn command_rejected(
    route_code: &'static str,
    message: &'static str,
    sidecar_code: ClaudeAgentSdkFailureCode,
) -> RuntimeFailure {
    failure(route_code, format!("{message}: {}", sidecar_code.as_str()))
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.unsupported_input",
        format!("Claude Agent SDK sidecar does not support {feature}"),
    )
}
