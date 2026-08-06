use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.muse_code.headless.unsupported_request",
        format!("Muse Code headless does not support the requested {dimension}"),
    )
}
