use crate::profile_fixture::ProfilePreflightFixture;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    ExecutionHostId, PreflightFailure, PreflightPlan, REMOTE_ACP_WIRE_VERSION,
    RemoteAcpAffinityPolicy, RemoteAcpConnectionBounds, RemoteAcpCoreSdkVersion,
    RemoteAcpRequirements, RemoteAcpRfdRevision, RemoteAcpRfdStatus, RemoteAcpTransport,
    RemoteAcpTransportSdkVersion, RemoteAcpVersionEvidence,
};

/// Remote ACP RFD revision represented by this fixture.
pub const REMOTE_ACP_RFD_REVISION: &str = "2026-07-21-active";
/// Remote ACP transport SDK version represented by this fixture.
pub const REMOTE_ACP_TRANSPORT_SDK_VERSION: &str = "2.0.0";
/// Remote ACP core SDK version represented by this fixture.
pub const REMOTE_ACP_CORE_SDK_VERSION: &str = "2.0.0";

/// Preflight fixture for a remote ACP transport and execution host.
pub struct RemoteAcpPreflightFixture {
    inner: ProfilePreflightFixture,
}

impl RemoteAcpPreflightFixture {
    /// Builds a remote ACP fixture for the selected transport and host.
    #[must_use]
    pub fn new(transport: RemoteAcpTransport, execution_host_id: ExecutionHostId) -> Self {
        Self {
            inner: ProfilePreflightFixture::for_remote_acp(transport, execution_host_id),
        }
    }

    /// Runs provider-neutral remote ACP preflight.
    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        self.inner.preflight()
    }
}

#[must_use]
/// Returns the canonical remote ACP requirements for `transport`.
pub fn remote_acp_requirements(transport: RemoteAcpTransport) -> RemoteAcpRequirements {
    let affinity = match transport {
        RemoteAcpTransport::StreamableHttpSse | RemoteAcpTransport::WebSocket => {
            RemoteAcpAffinityPolicy::ConnectionScopedCookies {
                maximum_cookie_count: NonZeroU32::new(16).unwrap(),
                maximum_cookie_bytes: NonZeroU64::new(16 * 1024).unwrap(),
            }
        }
    };
    RemoteAcpRequirements::new(
        transport,
        affinity,
        RemoteAcpConnectionBounds::new(
            NonZeroU64::new(64 * 1024).unwrap(),
            NonZeroU32::new(16).unwrap(),
            NonZeroU32::new(8).unwrap(),
            NonZeroU32::new(1_024).unwrap(),
            NonZeroU32::new(2_048).unwrap(),
        ),
        RemoteAcpVersionEvidence::new(
            NonZeroU32::new(REMOTE_ACP_WIRE_VERSION).unwrap(),
            RemoteAcpRfdRevision::new(REMOTE_ACP_RFD_REVISION).unwrap(),
            RemoteAcpRfdStatus::Active,
            RemoteAcpTransportSdkVersion::new(REMOTE_ACP_TRANSPORT_SDK_VERSION).unwrap(),
            RemoteAcpCoreSdkVersion::new(REMOTE_ACP_CORE_SDK_VERSION).unwrap(),
        ),
    )
    .expect("static remote ACP fixture requirements are valid")
}
