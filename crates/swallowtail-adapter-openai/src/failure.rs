use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.openai.protocol_malformed",
        "OpenAI Responses returned malformed protocol data",
    )
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.openai.unsupported_input",
        format!("OpenAI background inference does not support {feature}"),
    ))
}
