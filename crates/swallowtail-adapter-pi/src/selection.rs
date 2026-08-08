use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Semantic-version axis reported by the installed Pi package.
pub const PI_PACKAGE_AXIS: &str = "pi.package";
/// Oldest Pi package version qualified for the RPC route.
pub const PI_PACKAGE_BASELINE_VERSION: &str = "0.80.10";
/// Newest Pi package version behaviorally qualified for the RPC route.
pub const PI_PACKAGE_LATEST_QUALIFIED_VERSION: &str = "0.83.0";

const BASELINE_BEHAVIOR: &str = "pi.rpc.strict-lf-v0.80.10";
const THINKING_USAGE_BEHAVIOR: &str = "pi.rpc.strict-lf-v0.81.0-thinking-usage";
const SUMMARY_RETRY_BEHAVIOR: &str = "pi.rpc.strict-lf-v0.81.1-summary-retry";
const BASH_CORRELATION_BEHAVIOR: &str = "pi.rpc.strict-lf-v0.82.0-bash-correlation";
const BASH_EXTENSION_BEHAVIOR: &str = "pi.rpc.strict-lf-v0.83.0-bash-extension-hook";
/// Parses one exact Pi package semantic-version binding.
#[must_use]
pub fn pi_package_binding(value: &str) -> Option<InterfaceVersionBinding> {
    swallowtail_runtime::parse_semantic_version_binding(&axis(), value)
}

/// Returns the qualified Pi RPC package compatibility window.
#[must_use]
pub fn pi_rpc_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("pi.rpc.package-window-2")
            .expect("static Pi claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            segment(
                "0.80.10",
                "0.80.10",
                BASELINE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.81.0",
                "0.81.0",
                THINKING_USAGE_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.81.1",
                "0.81.1",
                SUMMARY_RETRY_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.82.0",
                "0.82.1",
                BASH_CORRELATION_BEHAVIOR,
                InterfaceSupportStatus::Deprecated,
            ),
            segment(
                "0.83.0",
                "0.83.0",
                BASH_EXTENSION_BEHAVIOR,
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Pi compatibility claim is valid")
}

pub(crate) fn validate_pi_plan_version(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let claim = pi_rpc_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.pi.rpc.version_missing",
            "Pi RPC plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.pi.rpc.version_ambiguous",
            "Pi RPC plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment.behavior_revision().is_none_or(|revision| {
            !matches!(
                revision.as_str(),
                BASELINE_BEHAVIOR
                    | THINKING_USAGE_BEHAVIOR
                    | SUMMARY_RETRY_BEHAVIOR
                    | BASH_CORRELATION_BEHAVIOR
                    | BASH_EXTENSION_BEHAVIOR
            )
        })
    {
        return Err(failure(
            "swallowtail.pi.rpc.version_incompatible",
            "Pi RPC executable version is incompatible with this driver",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(PI_PACKAGE_AXIS).expect("static Pi axis is valid")
}

fn segment(
    start: &str,
    end: &str,
    behavior: &str,
    status: InterfaceSupportStatus,
) -> InterfaceVersionSegment {
    InterfaceVersionSegment::new(
        version(start).expect("static Pi version is valid"),
        version(end).expect("static Pi version is valid"),
        InterfaceBehaviorRevision::new(behavior).expect("static Pi behavior revision is valid"),
        status,
    )
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

#[cfg(test)]
mod tests {
    use super::{pi_package_binding, pi_rpc_claim};
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn claim_qualifies_exact_milestones_and_keeps_later_stable_unverified() {
        let claim = pi_rpc_claim();
        for candidate in ["0.80.10", "0.81.0", "0.81.1", "0.82.0", "0.82.1", "0.83.0"] {
            assert!(claim.supports(&version(candidate)), "missing {candidate}");
        }
        for (candidate, behavior) in [
            ("0.80.10", "pi.rpc.strict-lf-v0.80.10"),
            ("0.81.0", "pi.rpc.strict-lf-v0.81.0-thinking-usage"),
            ("0.81.1", "pi.rpc.strict-lf-v0.81.1-summary-retry"),
            ("0.82.0", "pi.rpc.strict-lf-v0.82.0-bash-correlation"),
            ("0.82.1", "pi.rpc.strict-lf-v0.82.0-bash-correlation"),
            ("0.83.0", "pi.rpc.strict-lf-v0.83.0-bash-extension-hook"),
        ] {
            assert_eq!(
                claim
                    .assess(&version(candidate))
                    .behavior_revision()
                    .unwrap()
                    .as_str(),
                behavior
            );
        }
        for unsupported in ["0.80.11", "0.81.2", "0.82.2"] {
            assert!(!claim.permits(&version(unsupported)));
        }
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.83.1"))
        else {
            panic!("later stable Pi remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            "pi.rpc.strict-lf-v0.83.0-bash-extension-hook"
        );
        assert!(!claim.permits(&version("0.83.1-rc.1")));
    }

    #[test]
    fn binding_accepts_only_one_bare_semver() {
        assert!(pi_package_binding("0.80.10").is_some());
        assert!(pi_package_binding("0.83.0").is_some());
        for value in ["", " 0.80.10", "pi 0.80.10", "latest"] {
            assert!(pi_package_binding(value).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}
