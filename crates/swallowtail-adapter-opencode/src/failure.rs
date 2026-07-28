use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(feature: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.opencode.unsupported",
        format!("OpenCode HTTP does not support {feature}"),
    ))
}
