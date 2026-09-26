use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment,
};
use swallowtail_runtime::RuntimeFailure;

use crate::{KIMI_CODE_AXIS, failure::failure, kimi_code_binding};

/// Oldest qualified Kimi local-server version.
pub const KIMI_LOCAL_SERVER_BASELINE_VERSION: &str = "0.28.1";
/// Most recent qualified Kimi local-server version.
///
/// Official `@moonshot-ai/kimi-code` `2.1.1` (Research 354 and 357, Q-004 B).
/// From `0.40.0` the Bash tool no longer checks `cwd` against workspace
/// roots. The route declares Contract 023 `AmbientHost`, which never
/// claimed workspace containment. Later stables run as unverified newer.
/// Consumers who want no shell send the existing `disabled_tools` control
/// with the exact name `Bash`.
pub const KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION: &str = "2.1.1";
/// Last 0.x heartbeat-ping point that still called `assertAllowed` from
/// `RuntimeWorkspaceView.resolve`.
const HEARTBEAT_PING_WITH_BASH_CWD_CHECK: &str = "0.39.1";
/// Last 0.x heartbeat-ping point after that check was removed.
const HEARTBEAT_PING_0X_WITHOUT_BASH_CWD_CHECK: &str = "0.43.1";

const REST_WS_V2_BASELINE_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-baseline";
const REST_WS_V2_PROFILE_TOOLS_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-profile-tools";
const REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR: &str =
    "kimi.local-server.rest-ws-v2-global-events-catalog-filter";
const REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR: &str =
    "kimi.local-server.rest-ws-v2-full-subagent-status";
const REST_WS_V2_REFRESH_STABLE_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-refresh-stable";
const REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR: &str =
    "kimi.local-server.rest-ws-v2-optional-meta-flags";
const REST_WS_V2_HEARTBEAT_PING_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-heartbeat-ping";

#[must_use]
/// Returns the qualified compatibility claim for Kimi local-server.
pub fn kimi_local_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.local-server.executable-window-6")
            .expect("static Kimi local-server claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            exact_segment(
                KIMI_LOCAL_SERVER_BASELINE_VERSION,
                REST_WS_V2_BASELINE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            )
            .expect("static Kimi exact segment is valid"),
            exact_segment(
                "0.29.0",
                REST_WS_V2_PROFILE_TOOLS_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            )
            .expect("static Kimi exact segment is valid"),
            segment(
                "0.29.1",
                "0.30.0",
                REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            )
            .expect("static Kimi segment is valid"),
            exact_segment(
                "0.31.0",
                REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            )
            .expect("static Kimi exact segment is valid"),
            exact_segment(
                "0.31.1",
                REST_WS_V2_REFRESH_STABLE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            )
            .expect("static Kimi exact segment is valid"),
            segment(
                "0.32.0",
                "0.34.0",
                REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            )
            .expect("static Kimi segment is valid"),
            segment(
                "0.35.0",
                HEARTBEAT_PING_WITH_BASH_CWD_CHECK,
                REST_WS_V2_HEARTBEAT_PING_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            )
            .expect("static Kimi segment is valid"),
            segment(
                "0.40.0",
                HEARTBEAT_PING_0X_WITHOUT_BASH_CWD_CHECK,
                REST_WS_V2_HEARTBEAT_PING_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            )
            .expect("static Kimi segment is valid"),
            segment(
                "2.0.0",
                KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
                REST_WS_V2_HEARTBEAT_PING_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            )
            .expect("static Kimi segment is valid"),
        ],
        [
            InterfaceVersion::new("0.40.2").expect("static Kimi unpublished gap is valid"),
            InterfaceVersion::new("0.41.1").expect("static Kimi unpublished gap is valid"),
            InterfaceVersion::new("0.42.1").expect("static Kimi unpublished gap is valid"),
            InterfaceVersion::new("2.0.3").expect("static Kimi unpublished gap is valid"),
        ],
    )
    .expect("static Kimi local-server compatibility claim is valid")
}

pub(super) fn corroborate_versions(
    executable: &InterfaceVersionBinding,
    server_reported: &str,
) -> Result<InterfaceCompatibilityAssessment, RuntimeFailure> {
    let Some(server) = kimi_code_binding(server_reported) else {
        return Err(version_failure());
    };
    if executable.axis() != server.axis() || executable.version() != server.version() {
        return Err(version_failure());
    }

    let assessment = kimi_local_server_claim().assess(executable.version());
    if !assessment.is_permitted() {
        return Err(version_failure());
    }
    Ok(assessment)
}

pub(super) fn supports_profile_tools(assessment: &InterfaceCompatibilityAssessment) -> bool {
    assessment.behavior_revision().is_some_and(|revision| {
        matches!(
            revision.as_str(),
            REST_WS_V2_PROFILE_TOOLS_BEHAVIOR
                | REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR
                | REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR
                | REST_WS_V2_REFRESH_STABLE_BEHAVIOR
                | REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR
                | REST_WS_V2_HEARTBEAT_PING_BEHAVIOR
        )
    })
}

fn version_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.version_incompatible",
        "Kimi executable and local-server versions are not compatible",
    )
}

fn exact_segment(
    value: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> Option<InterfaceVersionSegment> {
    Some(InterfaceVersionSegment::exact(
        InterfaceVersion::new(value).ok()?,
        InterfaceBehaviorRevision::new(behavior).expect("static Kimi behavior is valid"),
        status,
    ))
}

fn segment(
    minimum: &str,
    maximum: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> Option<InterfaceVersionSegment> {
    Some(InterfaceVersionSegment::new(
        InterfaceVersion::new(minimum).ok()?,
        InterfaceVersion::new(maximum).ok()?,
        InterfaceBehaviorRevision::new(behavior).expect("static Kimi behavior is valid"),
        status,
    ))
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("static Kimi axis is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
        REST_WS_V2_BASELINE_BEHAVIOR, REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR,
        REST_WS_V2_HEARTBEAT_PING_BEHAVIOR, REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR,
        REST_WS_V2_PROFILE_TOOLS_BEHAVIOR, REST_WS_V2_REFRESH_STABLE_BEHAVIOR,
        REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR, corroborate_versions, kimi_local_server_claim,
        supports_profile_tools,
    };
    use crate::kimi_code_binding;
    use swallowtail_core::InterfaceCompatibilityAssessment;

    #[test]
    fn claim_qualifies_published_hops_through_2_1_1_and_keeps_gaps() {
        use swallowtail_core::InterfaceNewerVersionPosture;

        let claim = kimi_local_server_claim();
        assert_eq!(
            claim.baseline().as_str(),
            KIMI_LOCAL_SERVER_BASELINE_VERSION
        );
        assert_eq!(
            claim.latest_qualified().as_str(),
            KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION
        );
        assert_eq!(KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION, "2.1.1");
        assert_eq!(claim.id().as_str(), "kimi.local-server.executable-window-6");
        assert_eq!(
            claim.newer_version_posture(),
            InterfaceNewerVersionPosture::AllowUnverified
        );
        assert_eq!(claim.milestones().len(), 9);

        for (qualified, behavior) in [
            ("0.28.1", REST_WS_V2_BASELINE_BEHAVIOR),
            ("0.29.0", REST_WS_V2_PROFILE_TOOLS_BEHAVIOR),
            ("0.29.1", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.29.2", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.30.0", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.31.0", REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR),
            ("0.31.1", REST_WS_V2_REFRESH_STABLE_BEHAVIOR),
            ("0.32.0", REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR),
            ("0.33.0", REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR),
            ("0.34.0", REST_WS_V2_OPTIONAL_META_FLAGS_BEHAVIOR),
            ("0.35.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.36.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.36.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.37.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.37.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.37.2", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.38.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.39.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.39.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.40.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.40.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.41.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.42.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.43.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("0.43.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("2.0.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("2.0.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("2.0.2", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("2.1.0", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
            ("2.1.1", REST_WS_V2_HEARTBEAT_PING_BEHAVIOR),
        ] {
            let binding = kimi_code_binding(qualified).expect("fixture version binds");
            let InterfaceCompatibilityAssessment::Qualified(matched) =
                corroborate_versions(&binding, qualified).expect("versions corroborate")
            else {
                panic!("exact release must remain qualified");
            };
            assert_eq!(matched.behavior_revision().as_str(), behavior);
            assert_eq!(
                supports_profile_tools(&InterfaceCompatibilityAssessment::Qualified(matched)),
                qualified != "0.28.1"
            );
        }

        for gap in [
            "0.39.2", "0.40.2", "0.41.1", "0.42.1", "0.43.2", "1.0.0", "2.0.3",
        ] {
            let binding = kimi_code_binding(gap).expect("fixture version binds");
            assert_eq!(
                corroborate_versions(&binding, gap)
                    .expect_err("unpublished and excluded hops stay incompatible")
                    .diagnostic()
                    .code(),
                "swallowtail.kimi.local_server.version_incompatible"
            );
            assert_eq!(
                claim.assess(binding.version()),
                InterfaceCompatibilityAssessment::Incompatible
            );
        }

        let later = kimi_code_binding("2.1.2").expect("synthetic later binds");
        assert!(matches!(
            claim.assess(later.version()),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
        assert!(claim.permits(later.version()));
    }

    #[test]
    fn mismatched_and_unqualified_versions_fail_without_echoing_inputs() {
        for (executable, reported) in [
            ("0.29.0", "0.28.1"),
            ("0.28.0", "0.28.0"),
            ("0.29.0", "0.30.0"),
            ("0.29.0", "0.29.0-rc.1"),
            ("0.29.0", "not-a-version"),
        ] {
            let binding = kimi_code_binding(executable).expect("fixture executable binds");
            let failure =
                corroborate_versions(&binding, reported).expect_err("versions must fail closed");
            assert_eq!(
                failure.diagnostic().code(),
                "swallowtail.kimi.local_server.version_incompatible"
            );
            let rendered = format!("{failure:?}");
            assert!(!rendered.contains(reported));
        }
    }
}
