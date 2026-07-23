use crate::protocol::{ProtocolFailure, ProtocolFailureKind, ProviderFailureKind};
use swallowtail_core::SafeDiagnostic;
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
    failure(code, message)
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
    failure(code, message)
}

pub(crate) fn unsupported(subject: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.unsupported",
        format!("DeepSeek V4 continuation does not support {subject}"),
    )
}
