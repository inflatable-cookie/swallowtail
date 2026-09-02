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

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.unsupported_input",
        format!("Claude Agent SDK sidecar does not support {feature}"),
    )
}
