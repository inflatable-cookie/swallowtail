use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.grok.acp.response_malformed",
        "Grok Build returned a malformed ACP response",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.grok.acp.protocol_failed",
        "Grok Build ACP framing or correlation failed",
    )
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.grok.acp.feature_unsupported",
        format!("Grok Build ACP does not support {feature}"),
    ))
}
