use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::RuntimeFailure;

use crate::stream::ProviderFailureKind;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.bedrock.unsupported_input",
        format!("Bedrock Runtime direct inference does not support {feature}"),
    )
}

pub(crate) fn provider_failure(kind: ProviderFailureKind) -> RuntimeFailure {
    let (code, message) = match kind {
        ProviderFailureKind::AuthenticationOrPermissionDenied => (
            "swallowtail.bedrock.access_denied",
            "Bedrock Runtime rejected authentication or authorization",
        ),
        ProviderFailureKind::InvalidRequest => (
            "swallowtail.bedrock.invalid_request",
            "Bedrock Runtime rejected the request",
        ),
        ProviderFailureKind::ModelUnavailable => (
            "swallowtail.bedrock.model_unavailable",
            "Bedrock Runtime model was unavailable",
        ),
        ProviderFailureKind::ModelTimedOut => (
            "swallowtail.bedrock.model_timed_out",
            "Bedrock Runtime model timed out",
        ),
        ProviderFailureKind::RateLimited => (
            "swallowtail.bedrock.rate_limited",
            "Bedrock Runtime rate limit was reached",
        ),
        ProviderFailureKind::ResourceNotFound => (
            "swallowtail.bedrock.resource_not_found",
            "Bedrock Runtime resource was not found",
        ),
        ProviderFailureKind::ProviderOverloaded => (
            "swallowtail.bedrock.provider_overloaded",
            "Bedrock Runtime was unavailable",
        ),
        ProviderFailureKind::ProviderFailed => (
            "swallowtail.bedrock.provider_failed",
            "Bedrock Runtime failed while producing output",
        ),
        ProviderFailureKind::ProtocolFailed => (
            "swallowtail.bedrock.protocol_failed",
            "Bedrock Runtime returned an unknown failure",
        ),
        ProviderFailureKind::TransportFailed => (
            "swallowtail.bedrock.transport_failed",
            "Bedrock Runtime transport failed",
        ),
    };
    failure(code, message)
}
