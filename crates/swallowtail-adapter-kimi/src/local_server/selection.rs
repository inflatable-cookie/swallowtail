use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityAssessment, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment,
};
use swallowtail_runtime::RuntimeFailure;

use crate::{KIMI_CODE_AXIS, failure::failure, kimi_code_binding};

pub const KIMI_LOCAL_SERVER_BASELINE_VERSION: &str = "0.28.1";
pub const KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION: &str = "0.31.0";

const REST_WS_V2_BASELINE_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-baseline";
const REST_WS_V2_PROFILE_TOOLS_BEHAVIOR: &str = "kimi.local-server.rest-ws-v2-profile-tools";
const REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR: &str =
    "kimi.local-server.rest-ws-v2-global-events-catalog-filter";
const REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR: &str =
    "kimi.local-server.rest-ws-v2-full-subagent-status";

#[must_use]
pub fn kimi_local_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("kimi.local-server.executable-window-3")
            .expect("static Kimi local-server claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            exact_segment(
                KIMI_LOCAL_SERVER_BASELINE_VERSION,
                REST_WS_V2_BASELINE_BEHAVIOR,
            ),
            exact_segment("0.29.0", REST_WS_V2_PROFILE_TOOLS_BEHAVIOR),
            segment("0.29.1", "0.30.0", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            exact_segment(
                KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
                REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR,
            ),
        ],
        [],
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
        )
    })
}

fn version_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.version_incompatible",
        "Kimi executable and local-server versions are not compatible",
    )
}

fn exact_segment(value: &str, behavior: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::exact(
        InterfaceVersion::new(value).expect("static Kimi version is valid"),
        InterfaceBehaviorRevision::new(behavior).expect("static Kimi behavior is valid"),
        InterfaceSupportStatus::Maintained,
    )
}

fn segment(minimum: &str, maximum: &str, behavior: &str) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        InterfaceVersion::new(minimum).expect("static Kimi minimum version is valid"),
        InterfaceVersion::new(maximum).expect("static Kimi maximum version is valid"),
        InterfaceBehaviorRevision::new(behavior).expect("static Kimi behavior is valid"),
        InterfaceSupportStatus::Maintained,
    )
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("static Kimi axis is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        KIMI_LOCAL_SERVER_BASELINE_VERSION, KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION,
        REST_WS_V2_BASELINE_BEHAVIOR, REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR,
        REST_WS_V2_PROFILE_TOOLS_BEHAVIOR, REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR,
        corroborate_versions, kimi_local_server_claim, supports_profile_tools,
    };
    use crate::kimi_code_binding;
    use swallowtail_core::InterfaceCompatibilityAssessment;

    #[test]
    fn claim_qualifies_exact_releases_and_permits_visible_newer_releases() {
        let claim = kimi_local_server_claim();
        assert_eq!(
            claim.baseline().as_str(),
            KIMI_LOCAL_SERVER_BASELINE_VERSION
        );
        assert_eq!(
            claim.latest_qualified().as_str(),
            KIMI_LOCAL_SERVER_LATEST_QUALIFIED_VERSION
        );
        assert_eq!(claim.milestones().len(), 4);

        for (qualified, behavior) in [
            ("0.28.1", REST_WS_V2_BASELINE_BEHAVIOR),
            ("0.29.0", REST_WS_V2_PROFILE_TOOLS_BEHAVIOR),
            ("0.29.1", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.29.2", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.30.0", REST_WS_V2_GLOBAL_EVENTS_BEHAVIOR),
            ("0.31.0", REST_WS_V2_SUBAGENT_STATUS_BEHAVIOR),
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

        let newer = kimi_code_binding("0.32.0").expect("fixture version binds");
        assert!(matches!(
            corroborate_versions(&newer, "0.32.0").expect("newer version remains permitted"),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
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
