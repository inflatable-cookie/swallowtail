use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.cline.acp.malformed_message",
        "Cline returned a malformed ACP message",
    )
}

pub(crate) fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.cline.acp.protocol_failed",
        "Cline ACP transport failed",
    )
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.cline.acp.unsupported",
        format!("Cline ACP does not support requested {feature}"),
    ))
}

pub(crate) fn headless_unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.cline.headless.unsupported",
        format!("Cline headless does not support requested {feature}"),
    ))
}
