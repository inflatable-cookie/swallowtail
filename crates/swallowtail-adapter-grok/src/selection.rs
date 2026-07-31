use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, CredentialRef, EndpointAudience,
    EntitlementMetering, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan, SupportAuthority,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

pub const GROK_BUILD_ACP_AXIS: &str = "grok-build.executable";
pub const GROK_BUILD_ACP_BASELINE_VERSION: &str = "0.2.114";
pub const GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION: &str = "0.2.117";
pub const GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID: &str =
    "grok-build.subscription.delegated-oauth";
pub const GROK_BUILD_SUBSCRIPTION_AUDIENCE: &str = "grok-build.subscription";

pub(crate) const GROK_BUILD_ACP_BEHAVIOR: &str = "grok-build.acp-v1.cached-token-activation-v1";
pub(crate) const GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR: &str =
    "grok-build.acp-v1.cached-token-task-control-v2";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GrokPlanSelection {
    version: InterfaceVersion,
}

impl GrokPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[must_use]
pub fn grok_build_subscription_access_profile(credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new(GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID)
            .expect("static Grok access profile id is valid"),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new(GROK_BUILD_SUBSCRIPTION_AUDIENCE)
            .expect("static Grok endpoint audience is valid"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

#[must_use]
pub fn grok_build_acp_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty()
        || value.len() > MAX_VERSION_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return None;
    }
    let parsed = semver::Version::parse(value).ok()?;
    if !parsed.pre.is_empty() {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
pub fn grok_build_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("grok-build.acp.executable-window-2")
            .expect("static Grok claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(GROK_BUILD_ACP_BASELINE_VERSION),
                version("0.2.116"),
                behavior(GROK_BUILD_ACP_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                version(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION),
                behavior(GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Grok compatibility claim is valid")
}

pub(crate) fn select_grok_acp_plan(
    plan: &PreflightPlan,
) -> Result<GrokPlanSelection, RuntimeFailure> {
    let claim = grok_build_acp_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.grok.acp.version_missing",
            "Grok Build plan is missing its exact executable version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.grok.acp.version_ambiguous",
            "Grok Build plan contains more than one executable version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding) || !assessment.is_permitted() {
        return Err(failure(
            "swallowtail.grok.acp.version_incompatible",
            "Grok Build executable version is incompatible with this driver",
        ));
    }
    if assessment.behavior_revision().is_none_or(|revision| {
        !matches!(
            revision.as_str(),
            GROK_BUILD_ACP_BEHAVIOR | GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR
        )
    }) {
        return Err(failure(
            "swallowtail.grok.acp.behavior_incompatible",
            "Grok Build ACP behavior is not mapped by this driver",
        ));
    }
    Ok(GrokPlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("static Grok axis is valid")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("static Grok version is valid")
}

fn behavior(value: &str) -> InterfaceBehaviorRevision {
    InterfaceBehaviorRevision::new(value).expect("static Grok behavior is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        GROK_BUILD_ACP_AXIS, GROK_BUILD_ACP_BEHAVIOR, GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR,
        grok_build_acp_binding, grok_build_acp_claim,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn both_segments_are_qualified_and_later_stable_is_unverified() {
        let claim = grok_build_acp_claim();
        for candidate in ["0.2.114", "0.2.115", "0.2.116", "0.2.117"] {
            assert!(claim.supports(&version(candidate)), "missing {candidate}");
        }
        for (candidate, behavior) in [
            ("0.2.114", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.115", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.116", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.117", GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR),
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
        for rejected in ["0.2.0", "0.2.111", "0.2.112", "0.2.113", "0.2.114-alpha.1"] {
            assert!(!claim.permits(&version(rejected)));
        }
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("0.2.118"))
        else {
            panic!("later stable release remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR
        );
    }

    #[test]
    fn binding_accepts_stable_semver_only() {
        assert_eq!(
            grok_build_acp_binding("0.2.114")
                .expect("binding parses")
                .axis()
                .as_str(),
            GROK_BUILD_ACP_AXIS
        );
        for rejected in [
            "",
            " 0.2.114",
            "grok 0.2.114",
            "0.2.114 extra",
            "0.2.115-alpha.1",
            "latest",
        ] {
            assert!(grok_build_acp_binding(rejected).is_none());
        }
    }

    fn version(value: &str) -> InterfaceVersion {
        InterfaceVersion::new(value).expect("fixture version is valid")
    }
}
