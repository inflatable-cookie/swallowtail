//! Safe Contract 063 failure vocabulary for the registered-tool profile.

use crate::RuntimeFailure;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

/// Machine-distinct class of one registered-tool failure.
///
/// Every variant is safe: it never carries tool arguments, results, prompt or
/// skill bodies, credentials, endpoints, or host path material.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolFailureKind {
    /// The registration snapshot itself is not supported.
    UnsupportedRegistration,
    /// A selected namespaced tool identity is not in the snapshot.
    UnsupportedTool,
    /// A declared schema namespace, dialect, media type, or digest is unsupported.
    UnsupportedSchema,
    /// The selected transport is declared but not qualified for this slice.
    UnsupportedTransport,
    /// The negotiated protocol version is outside the qualified subset.
    UnsupportedProtocolVersion,
    /// A required host service port is absent from the registry.
    MissingHostService,
    /// A required credential reference is unavailable.
    CredentialReferenceUnavailable,
    /// A required process or environment recipe reference is unavailable.
    ProcessRecipeUnavailable,
    /// The lease is not ready for the requested phase.
    NotReady,
    /// Consumer policy denied one exact call.
    ConsumerDenied,
    /// The provider rejected the call.
    ProviderRejected,
    /// The call or lease was cancelled.
    Cancelled,
    /// The call or lease reached its deadline.
    DeadlineExceeded,
    /// An issued call was abandoned without a settled result.
    Abandoned,
    /// The server executed the call and failed.
    ServerExecutionFailed,
    /// A result was returned but is not a valid bounded result.
    InvalidResult,
    /// The transport was lost.
    TransportLost,
    /// The execution outcome of an issued call is unknown.
    UnknownExecutionOutcome,
    /// Joined teardown did not complete within its bounded budget.
    TeardownFailed,
    /// A correlation value is stale for the current generation.
    StaleCorrelation,
    /// A correlation value was already accepted.
    DuplicateCorrelation,
    /// A correlation value belongs to another lease, call, or turn.
    ForeignCorrelation,
    /// Work arrived after the terminal barrier or after close.
    PostTerminalCorrelation,
    /// A positive declared bound was exceeded.
    LimitExceeded,
    /// Consumer admission is revoked for this binding.
    Revoked,
    /// A required identity value is blank, oversized, or duplicated.
    IdentityRejected,
    /// A declared required reference is missing or host-inaccessible.
    RequiredReferenceUnavailable,
    /// Resolved selected content disagrees with its declared descriptor.
    SelectedContentMismatch,
    /// The host supplied selected content the bundle never declared.
    ForeignSelectedContent,
}

impl RegisteredToolFailureKind {
    /// Returns the stable diagnostic code for this failure class.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnsupportedRegistration => "swallowtail.registered_tool.unsupported_registration",
            Self::UnsupportedTool => "swallowtail.registered_tool.unsupported_tool",
            Self::UnsupportedSchema => "swallowtail.registered_tool.unsupported_schema",
            Self::UnsupportedTransport => "swallowtail.registered_tool.unsupported_transport",
            Self::UnsupportedProtocolVersion => {
                "swallowtail.registered_tool.unsupported_protocol_version"
            }
            Self::MissingHostService => "swallowtail.registered_tool.missing_host_service",
            Self::CredentialReferenceUnavailable => {
                "swallowtail.registered_tool.credential_reference_unavailable"
            }
            Self::ProcessRecipeUnavailable => {
                "swallowtail.registered_tool.process_recipe_unavailable"
            }
            Self::NotReady => "swallowtail.registered_tool.not_ready",
            Self::ConsumerDenied => "swallowtail.registered_tool.consumer_denied",
            Self::ProviderRejected => "swallowtail.registered_tool.provider_rejected",
            Self::Cancelled => "swallowtail.registered_tool.cancelled",
            Self::DeadlineExceeded => "swallowtail.registered_tool.deadline_exceeded",
            Self::Abandoned => "swallowtail.registered_tool.abandoned",
            Self::ServerExecutionFailed => "swallowtail.registered_tool.server_execution_failed",
            Self::InvalidResult => "swallowtail.registered_tool.invalid_result",
            Self::TransportLost => "swallowtail.registered_tool.transport_lost",
            Self::UnknownExecutionOutcome => {
                "swallowtail.registered_tool.unknown_execution_outcome"
            }
            Self::TeardownFailed => "swallowtail.registered_tool.teardown_failed",
            Self::StaleCorrelation => "swallowtail.registered_tool.stale_correlation",
            Self::DuplicateCorrelation => "swallowtail.registered_tool.duplicate_correlation",
            Self::ForeignCorrelation => "swallowtail.registered_tool.foreign_correlation",
            Self::PostTerminalCorrelation => {
                "swallowtail.registered_tool.post_terminal_correlation"
            }
            Self::LimitExceeded => "swallowtail.registered_tool.limit_exceeded",
            Self::Revoked => "swallowtail.registered_tool.revoked",
            Self::IdentityRejected => "swallowtail.registered_tool.identity_rejected",
            Self::RequiredReferenceUnavailable => {
                "swallowtail.registered_tool.required_reference_unavailable"
            }
            Self::SelectedContentMismatch => {
                "swallowtail.registered_tool.selected_content_mismatch"
            }
            Self::ForeignSelectedContent => "swallowtail.registered_tool.foreign_selected_content",
        }
    }

    /// Returns the safe operator-facing message for this failure class.
    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::UnsupportedRegistration => "Registration snapshot is not supported",
            Self::UnsupportedTool => "Selected tool identity is not in the registration snapshot",
            Self::UnsupportedSchema => "Declared tool schema is not supported",
            Self::UnsupportedTransport => "Selected registered-tool transport is not qualified",
            Self::UnsupportedProtocolVersion => {
                "Selected registered-tool protocol version is not qualified"
            }
            Self::MissingHostService => "A required host service port is not registered",
            Self::CredentialReferenceUnavailable => {
                "A required credential reference is not available"
            }
            Self::ProcessRecipeUnavailable => {
                "A required process or environment recipe reference is not available"
            }
            Self::NotReady => "Registered tool lease is not ready for this work",
            Self::ConsumerDenied => "Consumer policy denied this exact call",
            Self::ProviderRejected => "The provider rejected this call",
            Self::Cancelled => "The registered tool call was cancelled",
            Self::DeadlineExceeded => "The registered tool call reached its deadline",
            Self::Abandoned => "The registered tool call was abandoned without a result",
            Self::ServerExecutionFailed => "The registered tool server failed while executing",
            Self::InvalidResult => "The registered tool result was not a valid bounded result",
            Self::TransportLost => "The registered tool transport was lost",
            Self::UnknownExecutionOutcome => {
                "The registered tool execution outcome is unknown and must not replay"
            }
            Self::TeardownFailed => "Registered tool teardown did not join within its budget",
            Self::StaleCorrelation => "Correlation belongs to an earlier generation",
            Self::DuplicateCorrelation => "Correlation was already accepted",
            Self::ForeignCorrelation => "Correlation does not match the bound lease or call",
            Self::PostTerminalCorrelation => "Work arrived after the terminal barrier",
            Self::LimitExceeded => "A positive registered-tool bound was exceeded",
            Self::Revoked => "Consumer admission is revoked for this binding",
            Self::IdentityRejected => "A required registered-tool identity was rejected",
            Self::RequiredReferenceUnavailable => {
                "A declared required reference is missing or inaccessible"
            }
            Self::SelectedContentMismatch => {
                "Resolved selected content does not match its declared descriptor"
            }
            Self::ForeignSelectedContent => {
                "The host supplied selected content this bundle did not declare"
            }
        }
    }
}

/// Safe typed failure returned by the registered-tool profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredToolFailure {
    kind: RegisteredToolFailureKind,
}

impl RegisteredToolFailure {
    /// Creates one safe failure of an exact class.
    #[must_use]
    pub const fn new(kind: RegisteredToolFailureKind) -> Self {
        Self { kind }
    }

    /// Returns the exact failure class.
    #[must_use]
    pub const fn kind(&self) -> RegisteredToolFailureKind {
        self.kind
    }

    /// Returns the safe diagnostic for this failure.
    #[must_use]
    pub fn diagnostic(&self) -> SafeDiagnostic {
        SafeDiagnostic::new(self.kind.code(), self.kind.message())
    }

    /// Converts this failure into the shared runtime failure type.
    #[must_use]
    pub fn into_runtime_failure(self) -> RuntimeFailure {
        RuntimeFailure::new(self.diagnostic())
    }
}

impl fmt::Display for RegisteredToolFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.kind.message())
    }
}

impl std::error::Error for RegisteredToolFailure {}

impl From<RegisteredToolFailure> for RuntimeFailure {
    fn from(failure: RegisteredToolFailure) -> Self {
        failure.into_runtime_failure()
    }
}

pub(super) fn reject(kind: RegisteredToolFailureKind) -> RegisteredToolFailure {
    RegisteredToolFailure::new(kind)
}

pub(super) fn fail(kind: RegisteredToolFailureKind) -> RuntimeFailure {
    RegisteredToolFailure::new(kind).into_runtime_failure()
}
