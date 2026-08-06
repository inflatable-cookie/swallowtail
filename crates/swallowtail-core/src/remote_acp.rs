use crate::SafeDiagnostic;
use std::error::Error;
use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};

mod version;

pub use version::{
    RemoteAcpCoreSdkVersion, RemoteAcpRfdRevision, RemoteAcpRfdStatus,
    RemoteAcpTransportSdkVersion, RemoteAcpVersionEvidence,
};

/// Remote ACP wire version supported by this contract.
pub const REMOTE_ACP_WIRE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Explicit transport selected for a remote ACP connection.
pub enum RemoteAcpTransport {
    /// Streamable HTTP requests with server-sent event responses.
    StreamableHttpSse,
    /// WebSocket connection.
    WebSocket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Connection affinity required by the remote ACP endpoint.
pub enum RemoteAcpAffinityPolicy {
    /// No affinity state; invalid for currently supported transports.
    None,
    /// Bounded cookies retained only for the connection.
    ConnectionScopedCookies {
        /// Maximum cookies retained for affinity.
        maximum_cookie_count: NonZeroU32,
        /// Maximum combined cookie bytes.
        maximum_cookie_bytes: NonZeroU64,
    },
}

impl RemoteAcpAffinityPolicy {
    #[must_use]
    /// Returns maximum affinity cookie count, when cookie affinity is used.
    pub const fn maximum_cookie_count(self) -> Option<NonZeroU32> {
        match self {
            Self::None => None,
            Self::ConnectionScopedCookies {
                maximum_cookie_count,
                ..
            } => Some(maximum_cookie_count),
        }
    }

    #[must_use]
    /// Returns maximum combined affinity-cookie bytes.
    pub const fn maximum_cookie_bytes(self) -> Option<NonZeroU64> {
        match self {
            Self::None => None,
            Self::ConnectionScopedCookies {
                maximum_cookie_bytes,
                ..
            } => Some(maximum_cookie_bytes),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Positive framing, pending-work, and event-buffer bounds for remote ACP.
pub struct RemoteAcpConnectionBounds {
    maximum_frame_bytes: NonZeroU64,
    maximum_pending_requests: NonZeroU32,
    maximum_pending_callbacks: NonZeroU32,
    maximum_connection_stream_events: NonZeroU32,
    maximum_session_stream_events: NonZeroU32,
}

impl RemoteAcpConnectionBounds {
    #[must_use]
    /// Creates exact remote ACP connection bounds.
    pub const fn new(
        maximum_frame_bytes: NonZeroU64,
        maximum_pending_requests: NonZeroU32,
        maximum_pending_callbacks: NonZeroU32,
        maximum_connection_stream_events: NonZeroU32,
        maximum_session_stream_events: NonZeroU32,
    ) -> Self {
        Self {
            maximum_frame_bytes,
            maximum_pending_requests,
            maximum_pending_callbacks,
            maximum_connection_stream_events,
            maximum_session_stream_events,
        }
    }

    #[must_use]
    /// Returns maximum bytes in one wire frame.
    pub const fn maximum_frame_bytes(self) -> NonZeroU64 {
        self.maximum_frame_bytes
    }

    #[must_use]
    /// Returns maximum pending client requests.
    pub const fn maximum_pending_requests(self) -> NonZeroU32 {
        self.maximum_pending_requests
    }

    #[must_use]
    /// Returns maximum pending provider callbacks.
    pub const fn maximum_pending_callbacks(self) -> NonZeroU32 {
        self.maximum_pending_callbacks
    }

    #[must_use]
    /// Returns maximum buffered connection-stream events.
    pub const fn maximum_connection_stream_events(self) -> NonZeroU32 {
        self.maximum_connection_stream_events
    }

    #[must_use]
    /// Returns maximum buffered events per session stream.
    pub const fn maximum_session_stream_events(self) -> NonZeroU32 {
        self.maximum_session_stream_events
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Exact transport, affinity, bounds, and version evidence for remote ACP.
pub struct RemoteAcpRequirements {
    transport: RemoteAcpTransport,
    affinity: RemoteAcpAffinityPolicy,
    bounds: RemoteAcpConnectionBounds,
    versions: RemoteAcpVersionEvidence,
}

impl RemoteAcpRequirements {
    /// Creates requirements after validating wire version and affinity.
    pub fn new(
        transport: RemoteAcpTransport,
        affinity: RemoteAcpAffinityPolicy,
        bounds: RemoteAcpConnectionBounds,
        versions: RemoteAcpVersionEvidence,
    ) -> Result<Self, InvalidRemoteAcpRequirements> {
        if versions.wire_version().get() != REMOTE_ACP_WIRE_VERSION {
            return Err(InvalidRemoteAcpRequirements::wire_version());
        }
        let affinity_matches = matches!(
            (transport, affinity),
            (
                RemoteAcpTransport::StreamableHttpSse,
                RemoteAcpAffinityPolicy::ConnectionScopedCookies { .. }
            ) | (
                RemoteAcpTransport::WebSocket,
                RemoteAcpAffinityPolicy::ConnectionScopedCookies { .. }
            )
        );
        if !affinity_matches {
            return Err(InvalidRemoteAcpRequirements::affinity());
        }
        Ok(Self {
            transport,
            affinity,
            bounds,
            versions,
        })
    }

    #[must_use]
    /// Returns selected transport.
    pub const fn transport(&self) -> RemoteAcpTransport {
        self.transport
    }

    #[must_use]
    /// Returns connection-affinity policy.
    pub const fn affinity(&self) -> RemoteAcpAffinityPolicy {
        self.affinity
    }

    #[must_use]
    /// Returns exact connection bounds.
    pub const fn bounds(&self) -> RemoteAcpConnectionBounds {
        self.bounds
    }

    #[must_use]
    /// Returns exact protocol and SDK version evidence.
    pub const fn versions(&self) -> &RemoteAcpVersionEvidence {
        &self.versions
    }

    #[must_use]
    /// Returns the fixed one-connection bound.
    pub const fn maximum_connections(&self) -> NonZeroU32 {
        NonZeroU32::MIN
    }

    #[must_use]
    /// Returns the fixed one-active-session bound.
    pub const fn maximum_active_sessions(&self) -> NonZeroU32 {
        NonZeroU32::MIN
    }

    #[must_use]
    /// Reports redirect authority; always false.
    pub const fn permits_redirect(&self) -> bool {
        false
    }

    #[must_use]
    /// Reports automatic retry authority; always false.
    pub const fn permits_retry(&self) -> bool {
        false
    }

    #[must_use]
    /// Reports automatic reconnect authority; always false.
    pub const fn permits_reconnect(&self) -> bool {
        false
    }

    #[must_use]
    /// Reports replay or resumption authority; always false.
    pub const fn permits_replay_or_resumption(&self) -> bool {
        false
    }

    #[must_use]
    /// Reports transport fallback authority; always false.
    pub const fn permits_transport_fallback(&self) -> bool {
        false
    }

    #[must_use]
    /// Reports pooling or multiplexing authority; always false.
    pub const fn permits_pooling_or_multiplexing(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Rejection raised for invalid remote ACP requirements.
pub struct InvalidRemoteAcpRequirements {
    diagnostic: SafeDiagnostic,
}

impl InvalidRemoteAcpRequirements {
    fn wire_version() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.remote_acp_wire_version_rejected",
                "Remote ACP requires the supported wire version",
            ),
        }
    }

    fn affinity() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.remote_acp_affinity_rejected",
                "Remote ACP affinity does not match the selected transport",
            ),
        }
    }

    #[must_use]
    /// Returns the redacted requirement diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidRemoteAcpRequirements {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidRemoteAcpRequirements {}

#[cfg(test)]
#[path = "remote_acp/tests.rs"]
mod tests;
