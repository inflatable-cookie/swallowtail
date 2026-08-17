use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.unsupported_request",
        format!("DeepSeek Harness JSON-RPC does not support the requested {dimension}"),
    )
}
