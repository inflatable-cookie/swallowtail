use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.acp.malformed_message",
        "Gemini CLI returned a malformed ACP message",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.acp.protocol_failed",
        "Gemini CLI ACP transport failed",
    )
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.gemini.acp.unsupported",
        format!("Gemini ACP does not support requested {feature}"),
    ))
}
