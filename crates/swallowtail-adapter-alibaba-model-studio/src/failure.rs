use std::error::Error;
use std::fmt;
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, SafeDiagnostic,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Bounded protocol failure with an adapter-owned safe diagnostic.
pub struct AlibabaProtocolFailure {
    diagnostic: SafeDiagnostic,
}

impl AlibabaProtocolFailure {
    pub(crate) fn invalid(subject: &'static str) -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.protocol_invalid",
            format!("Alibaba Model Studio {subject} was invalid"),
            FailureClassification::new(
                FailureOrigin::Protocol,
                FailureKind::MalformedData,
                FailureRecovery::Unknown,
            ),
        )
    }

    pub(crate) fn unsupported(subject: &'static str) -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.unsupported_input",
            format!("Alibaba Model Studio does not support {subject} in the frozen route"),
            FailureClassification::new(
                FailureOrigin::Runtime,
                FailureKind::InvalidRequest,
                FailureRecovery::InputChangeRequired,
            ),
        )
    }

    pub(crate) fn provider() -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.provider_failed",
            "Alibaba Model Studio rejected the request",
            FailureClassification::new(
                FailureOrigin::Provider,
                FailureKind::Unknown,
                FailureRecovery::Unknown,
            ),
        )
    }

    fn new(
        code: &'static str,
        message: impl Into<String>,
        classification: FailureClassification,
    ) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(code, message)
                .with_failure_classification(classification),
        }
    }

    #[must_use]
    /// Returns the safe diagnostic and portable failure classification.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

pub(crate) fn failure(
    code: &'static str,
    message: impl Into<String>,
) -> swallowtail_runtime::RuntimeFailure {
    swallowtail_runtime::RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

pub(crate) fn protocol(error: AlibabaProtocolFailure) -> swallowtail_runtime::RuntimeFailure {
    swallowtail_runtime::RuntimeFailure::new(error.diagnostic)
}

pub(crate) fn unsupported(subject: &'static str) -> swallowtail_runtime::RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.unsupported_input",
        format!("Alibaba Model Studio does not support {subject} in the selected route"),
    )
}

impl fmt::Display for AlibabaProtocolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for AlibabaProtocolFailure {}
