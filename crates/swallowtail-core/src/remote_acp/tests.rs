use super::{
    REMOTE_ACP_WIRE_VERSION, RemoteAcpAffinityPolicy, RemoteAcpConnectionBounds,
    RemoteAcpCoreSdkVersion, RemoteAcpRequirements, RemoteAcpRfdRevision, RemoteAcpRfdStatus,
    RemoteAcpTransport, RemoteAcpTransportSdkVersion, RemoteAcpVersionEvidence,
};
use std::num::{NonZeroU32, NonZeroU64};

fn versions(wire_version: u32) -> RemoteAcpVersionEvidence {
    RemoteAcpVersionEvidence::new(
        NonZeroU32::new(wire_version).expect("wire version is non-zero"),
        RemoteAcpRfdRevision::new("2026-07-21-active").expect("revision is valid"),
        RemoteAcpRfdStatus::Active,
        RemoteAcpTransportSdkVersion::new("2.0.0").expect("version is valid"),
        RemoteAcpCoreSdkVersion::new("2.0.0").expect("version is valid"),
    )
}

fn bounds() -> RemoteAcpConnectionBounds {
    RemoteAcpConnectionBounds::new(
        NonZeroU64::new(65_536).unwrap(),
        NonZeroU32::new(8).unwrap(),
        NonZeroU32::new(4).unwrap(),
        NonZeroU32::new(256).unwrap(),
        NonZeroU32::new(512).unwrap(),
    )
}

#[test]
fn exact_version_axes_and_positive_bounds_remain_separate() {
    let requirements = RemoteAcpRequirements::new(
        RemoteAcpTransport::StreamableHttpSse,
        RemoteAcpAffinityPolicy::ConnectionScopedCookies {
            maximum_cookie_count: NonZeroU32::new(8).unwrap(),
            maximum_cookie_bytes: NonZeroU64::new(8_192).unwrap(),
        },
        bounds(),
        versions(REMOTE_ACP_WIRE_VERSION),
    )
    .expect("HTTP/SSE requirements are valid");

    assert_eq!(requirements.versions().wire_version().get(), 1);
    assert_eq!(
        requirements.versions().rfd_revision().as_str(),
        "2026-07-21-active"
    );
    assert_eq!(
        requirements.versions().transport_sdk_version().as_str(),
        "2.0.0"
    );
    assert_eq!(requirements.versions().core_sdk_version().as_str(), "2.0.0");
    assert_eq!(requirements.maximum_connections().get(), 1);
    assert_eq!(requirements.maximum_active_sessions().get(), 1);
    assert_eq!(requirements.bounds().maximum_pending_requests().get(), 8);
}

#[test]
fn transport_and_affinity_mismatch_or_wrong_wire_version_fail_closed() {
    assert!(
        RemoteAcpRequirements::new(
            RemoteAcpTransport::StreamableHttpSse,
            RemoteAcpAffinityPolicy::None,
            bounds(),
            versions(REMOTE_ACP_WIRE_VERSION),
        )
        .is_err()
    );
    assert!(
        RemoteAcpRequirements::new(
            RemoteAcpTransport::WebSocket,
            RemoteAcpAffinityPolicy::None,
            bounds(),
            versions(REMOTE_ACP_WIRE_VERSION),
        )
        .is_err()
    );
    assert!(
        RemoteAcpRequirements::new(
            RemoteAcpTransport::WebSocket,
            RemoteAcpAffinityPolicy::None,
            bounds(),
            versions(2),
        )
        .is_err()
    );
}

#[test]
fn first_remote_transport_grants_no_implicit_recovery_or_fallback() {
    let requirements = RemoteAcpRequirements::new(
        RemoteAcpTransport::WebSocket,
        RemoteAcpAffinityPolicy::ConnectionScopedCookies {
            maximum_cookie_count: NonZeroU32::new(8).unwrap(),
            maximum_cookie_bytes: NonZeroU64::new(8_192).unwrap(),
        },
        bounds(),
        versions(REMOTE_ACP_WIRE_VERSION),
    )
    .expect("WebSocket requirements are valid");

    assert!(!requirements.permits_redirect());
    assert!(!requirements.permits_retry());
    assert!(!requirements.permits_reconnect());
    assert!(!requirements.permits_replay_or_resumption());
    assert!(!requirements.permits_transport_fallback());
    assert!(!requirements.permits_pooling_or_multiplexing());
}

#[test]
fn blank_version_evidence_is_rejected() {
    assert!(RemoteAcpRfdRevision::new(" ").is_err());
    assert!(RemoteAcpTransportSdkVersion::new("").is_err());
    assert!(RemoteAcpCoreSdkVersion::new("\t").is_err());
}
