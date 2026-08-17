use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.zcode.app_server.unsupported_request",
        format!("ZCode app-server does not support the requested {dimension}"),
    )
}
