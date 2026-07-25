use std::error::Error;
use std::fmt;
use swallowtail_core::SafeDiagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteAcpErrorKind {
    BindingRejected,
    EndpointRejected,
    HostServiceMissing,
    CapacityExceeded,
    ProtocolRejected,
    TransportFailed,
    Cancelled,
    DeadlineExceeded,
    CleanupFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAcpError {
    kind: RemoteAcpErrorKind,
    diagnostic: SafeDiagnostic,
}

impl RemoteAcpError {
    pub(crate) fn new(kind: RemoteAcpErrorKind, code: &'static str, message: &'static str) -> Self {
        Self {
            kind,
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> RemoteAcpErrorKind {
        self.kind
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for RemoteAcpError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for RemoteAcpError {}

pub(crate) fn binding_error() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::BindingRejected,
        "swallowtail.remote_acp.binding_rejected",
        "Remote ACP transport binding did not match preflight",
    )
}

pub(crate) fn endpoint_error() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::EndpointRejected,
        "swallowtail.remote_acp.endpoint_rejected",
        "Remote ACP endpoint was not an allowed exact transport endpoint",
    )
}

pub(crate) fn capacity_error() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::CapacityExceeded,
        "swallowtail.remote_acp.capacity_exceeded",
        "Remote ACP bounded transport capacity was exceeded",
    )
}

pub(crate) fn protocol_error() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::ProtocolRejected,
        "swallowtail.remote_acp.protocol_rejected",
        "Remote ACP peer violated the selected protocol boundary",
    )
}

pub(crate) fn transport_error() -> RemoteAcpError {
    RemoteAcpError::new(
        RemoteAcpErrorKind::TransportFailed,
        "swallowtail.remote_acp.transport_failed",
        "Remote ACP transport failed",
    )
}
