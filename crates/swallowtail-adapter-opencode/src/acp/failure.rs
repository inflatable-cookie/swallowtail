use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.malformed_message",
        "OpenCode returned a malformed ACP message",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.protocol_failed",
        "OpenCode ACP transport failed",
    )
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.opencode.acp.unsupported",
        format!("OpenCode ACP does not support requested {feature}"),
    ))
}
