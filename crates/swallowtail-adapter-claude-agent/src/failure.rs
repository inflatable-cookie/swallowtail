use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.malformed_response",
        "Claude Agent returned malformed ACP data",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.protocol_failure",
        "Claude Agent ACP transport failed",
    )
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.unsupported",
        format!("Claude Agent ACP does not support the requested {feature}"),
    )
}
