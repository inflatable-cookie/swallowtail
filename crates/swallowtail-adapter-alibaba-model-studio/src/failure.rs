use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlibabaProtocolFailure {
    diagnostic: SafeDiagnostic,
}

impl AlibabaProtocolFailure {
    pub(crate) fn invalid(subject: &'static str) -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.protocol_invalid",
            format!("Alibaba Model Studio {subject} was invalid"),
        )
    }

    pub(crate) fn unsupported(subject: &'static str) -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.unsupported_input",
            format!("Alibaba Model Studio does not support {subject} in the frozen route"),
        )
    }

    pub(crate) fn provider() -> Self {
        Self::new(
            "swallowtail.alibaba_model_studio.provider_failed",
            "Alibaba Model Studio rejected the request",
        )
    }

    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
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
