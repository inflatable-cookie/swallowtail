use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.cursor.acp.response_malformed",
        "Cursor Agent returned a malformed ACP response",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.cursor.acp.protocol_failed",
        "Cursor Agent ACP framing or correlation failed",
    )
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.cursor.acp.feature_unsupported",
        format!("Cursor Agent ACP does not support {feature}"),
    )
}
