use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
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
    failure(code, message).with_failure_classification(classification(kind))
}

const fn classification(kind: ProviderFailureKind) -> FailureClassification {
    let (origin, failure_kind, recovery) = match kind {
        ProviderFailureKind::AuthenticationOrPermissionDenied => (
            FailureOrigin::Provider,
            FailureKind::AuthorizationDenied,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        ProviderFailureKind::InvalidRequest => (
            FailureOrigin::Provider,
            FailureKind::InvalidRequest,
            FailureRecovery::InputChangeRequired,
        ),
        ProviderFailureKind::ModelUnavailable => (
            FailureOrigin::Provider,
            FailureKind::ModelUnavailable,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        ProviderFailureKind::ModelTimedOut | ProviderFailureKind::ProviderOverloaded => (
            FailureOrigin::Provider,
            FailureKind::ProviderUnavailable,
            FailureRecovery::RetryMaySucceed,
        ),
        ProviderFailureKind::RateLimited => (
            FailureOrigin::Provider,
            FailureKind::RateLimited,
            FailureRecovery::RetryMaySucceed,
        ),
        ProviderFailureKind::ResourceNotFound => (
            FailureOrigin::Provider,
            FailureKind::ResourceNotFound,
            FailureRecovery::ConfigurationChangeRequired,
        ),
        ProviderFailureKind::ProviderFailed => (
            FailureOrigin::Provider,
            FailureKind::Unknown,
            FailureRecovery::Unknown,
        ),
        ProviderFailureKind::ProtocolFailed => (
            FailureOrigin::Protocol,
            FailureKind::MalformedData,
            FailureRecovery::Unknown,
        ),
        ProviderFailureKind::TransportFailed => (
            FailureOrigin::Transport,
            FailureKind::TransportInterrupted,
            FailureRecovery::RetryMaySucceed,
        ),
    };
    FailureClassification::new(origin, failure_kind, recovery)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_failure_classes_preserve_origin_and_recovery() {
        let rate = provider_failure(ProviderFailureKind::RateLimited);
        let transport = provider_failure(ProviderFailureKind::TransportFailed);

        assert_eq!(
            rate.diagnostic().failure_classification(),
            FailureClassification::new(
                FailureOrigin::Provider,
                FailureKind::RateLimited,
                FailureRecovery::RetryMaySucceed,
            )
        );
        assert_eq!(
            transport.diagnostic().failure_classification().origin(),
            FailureOrigin::Transport
        );
        assert_eq!(
            transport.diagnostic().code(),
            "swallowtail.bedrock.transport_failed"
        );
    }
}
