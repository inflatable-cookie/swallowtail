use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.unsupported_input",
        format!("Kimi Platform direct inference does not support {feature}"),
    )
}
