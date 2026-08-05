use crate::protocol::{ProtocolFailure, ProtocolFailureKind, ProviderFailureKind};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};
use swallowtail_runtime::RuntimeFailure;

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn protocol(error: ProtocolFailure) -> RuntimeFailure {
    let (code, message) = match error.kind() {
        ProtocolFailureKind::InvalidStructure => (
            "swallowtail.deepseek.protocol_invalid",
            "DeepSeek response structure was invalid",
        ),
        ProtocolFailureKind::BoundExceeded => (
            "swallowtail.deepseek.protocol_bound_exceeded",
            "DeepSeek response exceeded a selected bound",
        ),
        ProtocolFailureKind::ModelMismatch => (
            "swallowtail.deepseek.model_mismatch",
            "DeepSeek response used a different model",
        ),
        ProtocolFailureKind::UnknownSemanticField => (
            "swallowtail.deepseek.semantic_drift",
            "DeepSeek response contained unsupported semantics",
        ),
        ProtocolFailureKind::IncompleteStream => (
            "swallowtail.deepseek.stream_incomplete",
            "DeepSeek stream ended before completion",
        ),
        ProtocolFailureKind::ProviderFailure => (
            "swallowtail.deepseek.stream_provider_failure",
            "DeepSeek stream reported a provider failure",
        ),
    };
    let (origin, failure_kind) = match error.kind() {
        ProtocolFailureKind::BoundExceeded => {
            (FailureOrigin::Protocol, FailureKind::InputLimitExceeded)
        }
        ProtocolFailureKind::ModelMismatch => {
            (FailureOrigin::Protocol, FailureKind::ProtocolIncompatible)
        }
        ProtocolFailureKind::IncompleteStream => {
            (FailureOrigin::Transport, FailureKind::TransportInterrupted)
        }
        ProtocolFailureKind::InvalidStructure | ProtocolFailureKind::UnknownSemanticField => {
            (FailureOrigin::Protocol, FailureKind::MalformedData)
        }
        ProtocolFailureKind::ProviderFailure => (FailureOrigin::Provider, FailureKind::Unknown),
    };
    failure(code, message).with_failure_classification(FailureClassification::new(
        origin,
        failure_kind,
        FailureRecovery::Unknown,
    ))
}

pub(crate) fn provider(kind: ProviderFailureKind) -> RuntimeFailure {
    let (code, message) = match kind {
        ProviderFailureKind::InvalidRequest => (
            "swallowtail.deepseek.invalid_request",
            "DeepSeek rejected the request or continuation",
        ),
        ProviderFailureKind::Authentication => (
            "swallowtail.deepseek.authentication_rejected",
            "DeepSeek rejected the API credential",
        ),
        ProviderFailureKind::InsufficientBalance => (
            "swallowtail.deepseek.insufficient_balance",
            "DeepSeek reported insufficient account balance",
        ),
        ProviderFailureKind::AccountConcurrency => (
            "swallowtail.deepseek.account_concurrency",
            "DeepSeek rejected the request at the account concurrency boundary",
        ),
        ProviderFailureKind::Provider => (
            "swallowtail.deepseek.provider_failure",
            "DeepSeek failed the request",
        ),
        ProviderFailureKind::Overloaded => (
            "swallowtail.deepseek.overloaded",
            "DeepSeek reported temporary overload",
        ),
    };
    let (failure_kind, recovery) = match kind {
        ProviderFailureKind::InvalidRequest => (
            FailureKind::InvalidRequest,
            FailureRecovery::InputChangeRequired,
        ),
        ProviderFailureKind::Authentication => (
            FailureKind::AuthenticationRejected,
            FailureRecovery::ReauthenticationRequired,
        ),
        ProviderFailureKind::InsufficientBalance => (
            FailureKind::EntitlementUnavailable,
            FailureRecovery::SameRequestNotRetryable,
        ),
        ProviderFailureKind::AccountConcurrency => {
            (FailureKind::RateLimited, FailureRecovery::RetryMaySucceed)
        }
        ProviderFailureKind::Provider => (FailureKind::Unknown, FailureRecovery::Unknown),
        ProviderFailureKind::Overloaded => (
            FailureKind::ProviderUnavailable,
            FailureRecovery::RetryMaySucceed,
        ),
    };
    failure(code, message).with_failure_classification(FailureClassification::new(
        FailureOrigin::Provider,
        failure_kind,
        recovery,
    ))
}

pub(crate) fn unsupported(subject: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.unsupported",
        format!("DeepSeek V4 continuation does not support {subject}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_status_classes_are_portable_without_losing_codes() {
        let authentication = provider(ProviderFailureKind::Authentication);
        let overloaded = provider(ProviderFailureKind::Overloaded);

        assert_eq!(
            authentication.diagnostic().failure_classification().kind(),
            FailureKind::AuthenticationRejected
        );
        assert_eq!(
            authentication.diagnostic().code(),
            "swallowtail.deepseek.authentication_rejected"
        );
        assert_eq!(
            overloaded.diagnostic().failure_classification().recovery(),
            FailureRecovery::RetryMaySucceed
        );
    }
}
