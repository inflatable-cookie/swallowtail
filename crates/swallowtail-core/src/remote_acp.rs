use crate::SafeDiagnostic;
use std::error::Error;
use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};

mod version;

pub use version::{
    RemoteAcpCoreSdkVersion, RemoteAcpRfdRevision, RemoteAcpRfdStatus,
    RemoteAcpTransportSdkVersion, RemoteAcpVersionEvidence,
};

pub const REMOTE_ACP_WIRE_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RemoteAcpTransport {
    StreamableHttpSse,
    WebSocket,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteAcpAffinityPolicy {
    None,
    ConnectionScopedCookies {
        maximum_cookie_count: NonZeroU32,
        maximum_cookie_bytes: NonZeroU64,
    },
}

impl RemoteAcpAffinityPolicy {
    #[must_use]
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
pub struct RemoteAcpConnectionBounds {
    maximum_frame_bytes: NonZeroU64,
    maximum_pending_requests: NonZeroU32,
    maximum_pending_callbacks: NonZeroU32,
    maximum_connection_stream_events: NonZeroU32,
    maximum_session_stream_events: NonZeroU32,
}

impl RemoteAcpConnectionBounds {
    #[must_use]
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
    pub const fn maximum_frame_bytes(self) -> NonZeroU64 {
        self.maximum_frame_bytes
    }

    #[must_use]
    pub const fn maximum_pending_requests(self) -> NonZeroU32 {
        self.maximum_pending_requests
    }

    #[must_use]
    pub const fn maximum_pending_callbacks(self) -> NonZeroU32 {
        self.maximum_pending_callbacks
    }

    #[must_use]
    pub const fn maximum_connection_stream_events(self) -> NonZeroU32 {
        self.maximum_connection_stream_events
    }

    #[must_use]
    pub const fn maximum_session_stream_events(self) -> NonZeroU32 {
        self.maximum_session_stream_events
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteAcpRequirements {
    transport: RemoteAcpTransport,
    affinity: RemoteAcpAffinityPolicy,
    bounds: RemoteAcpConnectionBounds,
    versions: RemoteAcpVersionEvidence,
}

impl RemoteAcpRequirements {
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
    pub const fn transport(&self) -> RemoteAcpTransport {
        self.transport
    }

    #[must_use]
    pub const fn affinity(&self) -> RemoteAcpAffinityPolicy {
        self.affinity
    }

    #[must_use]
    pub const fn bounds(&self) -> RemoteAcpConnectionBounds {
        self.bounds
    }

    #[must_use]
    pub const fn versions(&self) -> &RemoteAcpVersionEvidence {
        &self.versions
    }

    #[must_use]
    pub const fn maximum_connections(&self) -> NonZeroU32 {
        NonZeroU32::MIN
    }

    #[must_use]
    pub const fn maximum_active_sessions(&self) -> NonZeroU32 {
        NonZeroU32::MIN
    }

    #[must_use]
    pub const fn permits_redirect(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn permits_retry(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn permits_reconnect(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn permits_replay_or_resumption(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn permits_transport_fallback(&self) -> bool {
        false
    }

    #[must_use]
    pub const fn permits_pooling_or_multiplexing(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
