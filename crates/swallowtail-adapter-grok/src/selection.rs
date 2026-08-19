use swallowtail_core::{
    AccessProfile, AccessProfileId, CredentialMechanism, CredentialRef, EndpointAudience,
    EntitlementMetering, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceNewerVersionPosture, InterfaceSupportStatus,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, InterfaceVersionScheme,
    InterfaceVersionSegment, PreflightPlan, SupportAuthority,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Executable-version interface axis used by Grok Build ACP.
pub const GROK_BUILD_ACP_AXIS: &str = "grok-build.executable";
/// Oldest qualified Grok Build version.
pub const GROK_BUILD_ACP_BASELINE_VERSION: &str = "0.2.114";
/// Most recent qualified Grok Build version.
pub const GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION: &str = "1.0.5";
/// Stable identifier for Grok Build delegated subscription access.
pub const GROK_BUILD_SUBSCRIPTION_ACCESS_PROFILE_ID: &str =
    "grok-build.subscription.delegated-oauth";
/// Endpoint audience for Grok Build delegated subscription access.
pub const GROK_BUILD_SUBSCRIPTION_AUDIENCE: &str = "grok-build.subscription";

pub(crate) const GROK_BUILD_ACP_BEHAVIOR: &str = "grok-build.acp-v1.cached-token-activation-v1";
pub(crate) const GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR: &str =
    "grok-build.acp-v1.cached-token-task-control-v2";
pub(crate) const GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR: &str =
    "grok-build.acp-v1.cached-token-model-4-6-v3";
pub(crate) const GROK_BUILD_MODEL_4_5: &str = "grok-4.5";
pub(crate) const GROK_BUILD_MODEL_4_6: &str = "grok-4.6";
const MAX_VERSION_BYTES: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GrokPlanSelection {
    version: InterfaceVersion,
    expected_model: &'static str,
}

impl GrokPlanSelection {
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }

    pub(crate) const fn expected_model(&self) -> &'static str {
        self.expected_model
    }
}

#[must_use]
/// Builds Grok Build's delegated subscription OAuth access profile.
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
/// Parses a stable Grok Build release into its interface binding.
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
/// Returns the qualified compatibility claim for Grok Build ACP.
pub fn grok_build_acp_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("grok-build.acp.executable-window-2")
            .expect("static Grok claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                version(GROK_BUILD_ACP_BASELINE_VERSION).expect("static Grok version is valid"),
                version("0.2.116").expect("static Grok version is valid"),
                behavior(GROK_BUILD_ACP_BEHAVIOR),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::exact(
                version("0.2.117").expect("static Grok version is valid"),
                behavior(GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR),
                InterfaceSupportStatus::Deprecated,
            ),
            InterfaceVersionSegment::new(
                version("1.0.4").expect("static Grok version is valid"),
                version(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION)
                    .expect("static Grok version is valid"),
                behavior(GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [],
    )
    .expect("static Grok compatibility claim is valid")
}

#[must_use]
/// Returns the qualified model id bound to a Grok behavior revision.
pub fn grok_build_model_for_behavior(behavior_revision: &str) -> Option<&'static str> {
    match behavior_revision {
        GROK_BUILD_ACP_BEHAVIOR | GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR => {
            Some(GROK_BUILD_MODEL_4_5)
        }
        GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR => Some(GROK_BUILD_MODEL_4_6),
        _ => None,
    }
}

#[must_use]
/// Returns the qualified model id for a permitted Grok executable version.
pub fn grok_build_model_for_version(version: &InterfaceVersion) -> Option<&'static str> {
    grok_build_acp_claim()
        .assess(version)
        .behavior_revision()
        .and_then(|revision| grok_build_model_for_behavior(revision.as_str()))
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
    let behavior_revision = assessment.behavior_revision().ok_or_else(|| {
        failure(
            "swallowtail.grok.acp.behavior_incompatible",
            "Grok Build ACP behavior is not mapped by this driver",
        )
    })?;
    let expected_model =
        grok_build_model_for_behavior(behavior_revision.as_str()).ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.behavior_incompatible",
                "Grok Build ACP behavior is not mapped by this driver",
            )
        })?;
    Ok(GrokPlanSelection {
        version: binding.version().clone(),
        expected_model,
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("static Grok axis is valid")
}

fn version(value: &str) -> Option<InterfaceVersion> {
    InterfaceVersion::new(value).ok()
}

fn behavior(value: &str) -> InterfaceBehaviorRevision {
    InterfaceBehaviorRevision::new(value).expect("static Grok behavior is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        GROK_BUILD_ACP_AXIS, GROK_BUILD_ACP_BEHAVIOR, GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR,
        GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR, GROK_BUILD_MODEL_4_5, GROK_BUILD_MODEL_4_6,
        grok_build_acp_binding, grok_build_acp_claim, grok_build_model_for_version,
    };
    use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

    #[test]
    fn segments_cover_0_2_windows_and_1_0_4_through_1_0_5() {
        let claim = grok_build_acp_claim();
        for candidate in ["0.2.114", "0.2.115", "0.2.116", "0.2.117", "1.0.4", "1.0.5"] {
            assert!(claim.supports(&version(candidate)), "missing {candidate}");
        }
        for (candidate, behavior) in [
            ("0.2.114", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.115", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.116", GROK_BUILD_ACP_BEHAVIOR),
            ("0.2.117", GROK_BUILD_ACP_TASK_CONTROL_BEHAVIOR),
            ("1.0.4", GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR),
            ("1.0.5", GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR),
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
        for rejected in [
            "0.2.0",
            "0.2.111",
            "0.2.112",
            "0.2.113",
            "0.2.114-alpha.1",
            "0.2.118",
            "0.2.121",
            "1.0.0",
            "1.0.3",
        ] {
            assert!(
                !claim.permits(&version(rejected)),
                "unexpected permit {rejected}"
            );
        }
        let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) =
            claim.assess(&version("1.0.6"))
        else {
            panic!("later stable release remains unverified");
        };
        assert_eq!(
            newer.behavior_revision().as_str(),
            GROK_BUILD_ACP_MODEL_4_6_BEHAVIOR
        );
        assert_eq!(
            grok_build_model_for_version(&version("0.2.117")),
            Some(GROK_BUILD_MODEL_4_5)
        );
        assert_eq!(
            grok_build_model_for_version(&version("1.0.4")),
            Some(GROK_BUILD_MODEL_4_6)
        );
        assert_eq!(
            grok_build_model_for_version(&version("1.0.5")),
            Some(GROK_BUILD_MODEL_4_6)
        );
        assert_eq!(
            grok_build_model_for_version(&version("1.0.6")),
            Some(GROK_BUILD_MODEL_4_6)
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
        assert!(grok_build_acp_binding("1.0.4").is_some());
        assert!(grok_build_acp_binding("1.0.5").is_some());
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
