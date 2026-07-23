use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.ollama.unsupported_input",
        format!("Ollama native attached inference does not support {feature}"),
    )
}
