use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Semantic-version axis reported by the installed Oh My Pi package.
pub const OH_MY_PI_PACKAGE_AXIS: &str = "oh-my-pi.package";
/// Oldest Oh My Pi package version qualified for RPC v2.
pub const OH_MY_PI_PACKAGE_BASELINE_VERSION: &str = "17.2.9";
/// Newest `17.x` Oh My Pi package version qualified on the `17.x` behavior.
///
/// The `17.x` segment ends here. The npm-unpublished GitHub tags `v17.4.3`
/// and `v17.4.4` stay outside every segment and therefore incompatible.
const OH_MY_PI_PACKAGE_17X_LATEST_QUALIFIED_VERSION: &str = "17.4.2";
/// Oldest `18.x` Oh My Pi package version, opening the private major segment.
const OH_MY_PI_PACKAGE_18_BASELINE_VERSION: &str = "18.0.0";
/// Newest Oh My Pi package version behaviorally qualified for RPC v2.
pub const OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION: &str = "18.2.7";

/// Behavior revision for the retained `17.2.9..=17.4.2` segment.
///
/// The `17.4.1` and `17.4.2` hops are additive or advisory only: an unmapped
/// `runCommandInBackground` host callback, an accurate `agentInvoked` field on
/// the consumed-builtin `prompt` response, and optional select
/// `optionDetails` that `docs/rpc.md` documents as ignorable by hosts reading
/// `options` alone. No mapped flag, command, frame, response key, event, or UI
/// field moves, so the frozen `17.2.9` decoder contract still holds.
const RETAINED_17X_BEHAVIOR: &str = "oh-my-pi.rpc-v2-v17.2.9";
/// Adapter-private behavior revision for the `18.x` major line.
///
/// The selected `--mode rpc` flags, commands, framing, response shape,
/// lifecycle, usage, terminal, failure, retention, cancellation, and cleanup
/// behavior are unchanged through `18.2.7`, but the `17` to `18` boundary is
/// a major-line reset. Contract 029 and the g05.079 task require a distinct
/// adapter-private behavior revision and segment rather than silent
/// inheritance of the prior major window. The later `18.2.0..=18.2.7` hops
/// are a compatible extension of this same revision.
const MAJOR_18X_BEHAVIOR: &str = "oh-my-pi.rpc-v2-v18.0.0";
/// npm-unpublished stable points inside the admitted `18.x` segment.
///
/// `18.0.2` and `18.1.7` exist as GitHub tags only. They stay explicitly
/// excluded so an unpublished point can never be inferred compatible.
const UNPUBLISHED_18X_GAPS: [&str; 2] = ["18.0.2", "18.1.7"];

/// Parses one exact Oh My Pi package semantic-version binding.
#[must_use]
pub fn oh_my_pi_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(&axis(), value)
}

/// Returns the qualified Oh My Pi RPC package compatibility window.
#[must_use]
pub fn oh_my_pi_rpc_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("oh-my-pi.rpc.package-window-2")
            .expect("static OhMyPi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            segment(
                OH_MY_PI_PACKAGE_BASELINE_VERSION,
                OH_MY_PI_PACKAGE_17X_LATEST_QUALIFIED_VERSION,
                RETAINED_17X_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                OH_MY_PI_PACKAGE_18_BASELINE_VERSION,
                OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION,
                MAJOR_18X_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        UNPUBLISHED_18X_GAPS
            .into_iter()
            .map(|version| self::version(version).expect("static OhMyPi gap version is valid")),
    )
    .expect("static OhMyPi compatibility claim is valid")
}

pub(crate) fn validate_oh_my_pi_plan_version(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = oh_my_pi_rpc_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.oh_my_pi.rpc.version_missing",
            "OhMyPi RPC plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.version_ambiguous",
            "OhMyPi RPC plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment.behavior_revision().is_none_or(|revision| {
            !matches!(
                revision.as_str(),
                RETAINED_17X_BEHAVIOR | MAJOR_18X_BEHAVIOR
            )
        })
    {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.version_incompatible",
            "OhMyPi RPC executable version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(OH_MY_PI_PACKAGE_AXIS).expect("static OhMyPi axis is valid")
}

fn segment(
    start: &str,
    end: &str,
    behavior: &str,
    support_status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(start).expect("static OhMyPi version is valid"),
        version(end).expect("static OhMyPi version is valid"),
        InterfaceBehaviorRevision::new(behavior).expect("static OhMyPi behavior revision is valid"),
        support_status,
    )
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{
        MAJOR_18X_BEHAVIOR, OH_MY_PI_PACKAGE_18_BASELINE_VERSION,
        OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, RETAINED_17X_BEHAVIOR, oh_my_pi_package_binding,
        oh_my_pi_rpc_claim,
    };
    use swallowtail_core::{
        InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
    };

    #[test]
    fn claim_qualifies_both_segments_and_keeps_later_stable_unverified() {
        let claim = oh_my_pi_rpc_claim();
        assert_eq!(claim.id().as_str(), "oh-my-pi.rpc.package-window-2");
        for candidate in [
            "17.2.9", "17.2.15", "17.3.7", "17.3.8", "17.4.0", "17.4.1", "17.4.2",
        ] {
            assert!(claim.supports(&version(candidate)), "missing {candidate}");
        }
        for candidate in [
            "18.0.0", "18.0.1", "18.0.3", "18.0.11", "18.1.0", "18.1.16", "18.1.21", "18.1.22",
            "18.2.0", "18.2.1", "18.2.6", "18.2.7",
        ] {
            assert!(claim.supports(&version(candidate)), "missing {candidate}");
        }

        assert_eq!(OH_MY_PI_PACKAGE_18_BASELINE_VERSION, "18.0.0");
        assert_eq!(OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, "18.2.7");

        // The 17.x segment is retained but deprecated by the newest revision.
        for candidate in ["17.2.9", "17.4.0", "17.4.2"] {
            let matched = claim
                .assess(&version(candidate))
                .behavior_revision()
                .expect("17.x version has a behavior")
                .as_str()
                .to_owned();
            assert_eq!(matched, RETAINED_17X_BEHAVIOR);
        }
        let matched = claim
            .classify(&version("17.4.2"))
            .expect("17.4.2 is qualified");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Deprecated);
        let matched = claim
            .classify(&version("18.0.0"))
            .expect("18.0.0 is qualified");
        assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
        assert_eq!(matched.behavior_revision().as_str(), MAJOR_18X_BEHAVIOR);

        // Major-line boundary and the unpublished tags stay incompatible.
        for stopped in ["17.4.3", "17.4.4", "17.2.8"] {
            assert!(
                matches!(
                    claim.assess(&version(stopped)),
                    InterfaceCompatibilityAssessment::Incompatible
                ),
                "{stopped} stays incompatible"
            );
        }
        for gap in ["18.0.2", "18.1.7"] {
            assert!(
                matches!(
                    claim.assess(&version(gap)),
                    InterfaceCompatibilityAssessment::Incompatible
                ),
                "unpublished {gap} stays incompatible"
            );
        }

        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("18.2.8"))
        else {
            panic!("later stable OhMyPi remains unverified");
        };
        assert_eq!(newer.behavior_revision().as_str(), MAJOR_18X_BEHAVIOR);
        assert!(!claim.permits(&version("18.2.8-rc.1")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(oh_my_pi_package_binding("17.2.9").is_some());
        assert!(oh_my_pi_package_binding("18.1.22").is_some());
        assert!(oh_my_pi_package_binding("18.2.7").is_some());
        for value in ["", " 17.2.9", "omp 17.2.9", "latest"] {
            assert!(oh_my_pi_package_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}
