use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

use crate::failure::failure;

/// Unambiguous executable name used for Vibe discovery.
pub const MISTRAL_VIBE_EXECUTABLE_NAME: &str = "vibe";
/// Opaque GitHub-release axis for Mistral Vibe headless.
pub const MISTRAL_VIBE_RELEASE_AXIS: &str = "mistral-vibe.release";
/// Latest qualified Vibe CLI release used by headless.
pub const MISTRAL_VIBE_RELEASE_VERSION: &str = "2.26.0";
/// First qualified Vibe CLI release retained by the headless claim.
const MISTRAL_VIBE_RELEASE_BASELINE_VERSION: &str = "2.25.4";

pub(crate) const MISTRAL_VIBE_HEADLESS_BEHAVIOR: &str = "mistral-vibe.headless.stdio-streaming-v1";
const MAX_VERSION_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VibePlanSelection {
    version: InterfaceVersion,
}

impl VibePlanSelection {
    #[allow(dead_code)]
    pub(crate) const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Parses installed `--version` stdout into a stable Vibe release binding.
#[must_use]
pub(crate) fn parse_vibe_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let exact = exact.strip_prefix("vibe ").unwrap_or(exact);
    mistral_vibe_release_binding(exact)
}

/// Parses one stable Vibe CLI release version into its interface binding.
///
/// The compatibility claim classifies the returned version as qualified,
/// unverified newer, or incompatible.
#[must_use]
pub fn mistral_vibe_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value.is_empty() || value.len() > MAX_VERSION_BYTES || value.trim() != value {
        return None;
    }
    let parsed = semver::Version::parse(value).ok()?;
    if !parsed.pre.is_empty() || !parsed.build.is_empty() {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

/// Returns the qualified Vibe headless protocol claim.
#[must_use]
pub fn mistral_vibe_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("mistral-vibe.headless.release-window-1")
            .expect("static Vibe claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new(MISTRAL_VIBE_RELEASE_BASELINE_VERSION)
                .expect("static Vibe baseline is valid"),
            InterfaceVersion::new(MISTRAL_VIBE_RELEASE_VERSION)
                .expect("static Vibe latest version is valid"),
            InterfaceBehaviorRevision::new(MISTRAL_VIBE_HEADLESS_BEHAVIOR)
                .expect("static Vibe behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [
            InterfaceVersion::new("2.25.6").expect("static Vibe 2.25.6 gap is valid"),
            InterfaceVersion::new("2.25.9").expect("static Vibe 2.25.9 gap is valid"),
        ],
    )
    .expect("static Vibe claim is valid")
}

pub(crate) fn select_mistral_vibe_headless_plan(
    plan: &PreflightPlan,
) -> Result<VibePlanSelection, RuntimeFailure> {
    let claim = mistral_vibe_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        failure(
            "swallowtail.mistral-vibe.headless.version_missing",
            "Mistral Vibe headless plan is missing its release version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(failure(
            "swallowtail.mistral-vibe.headless.version_ambiguous",
            "Mistral Vibe headless plan contains more than one release version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != MISTRAL_VIBE_HEADLESS_BEHAVIOR)
    {
        return Err(failure(
            "swallowtail.mistral-vibe.headless.version_incompatible",
            "Mistral Vibe release version is incompatible with the headless driver",
        ));
    }
    Ok(VibePlanSelection {
        version: binding.version().clone(),
    })
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(MISTRAL_VIBE_RELEASE_AXIS).expect("static Vibe release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_release_versions_are_bound_for_claim_classification() {
        for accepted in [
            "2.25.4", "2.25.5", "2.25.7", "2.25.8", "2.26.0", "2.26.1", "2.24.2",
        ] {
            assert!(
                mistral_vibe_release_binding(accepted).is_some(),
                "{accepted}"
            );
        }
        for rejected in [
            "",
            "2.25",
            "2.25.4.0",
            "v2.25.4",
            "2.25.4-beta",
            "2.25.4+build.1",
            "2.25.4\n",
            " 2.25.4",
            "2.25.4 ",
            "vibe 2.25.4",
        ] {
            assert!(
                mistral_vibe_release_binding(rejected).is_none(),
                "{rejected:?} must not bind"
            );
        }
    }

    #[test]
    fn published_hops_are_qualified_and_unpublished_holes_stay_excluded() {
        let older = InterfaceVersion::new("2.24.2").expect("prior baseline");
        let claim = mistral_vibe_headless_claim();
        for qualified in ["2.25.4", "2.25.5", "2.25.7", "2.25.8", "2.26.0"] {
            let version = InterfaceVersion::new(qualified).expect("qualified version");
            assert!(claim.assess(&version).is_permitted(), "{qualified}");
            assert!(claim.classify(&version).is_some(), "{qualified}");
        }
        for gap in ["2.25.6", "2.25.9"] {
            let version = InterfaceVersion::new(gap).expect("unpublished version");
            assert!(!claim.assess(&version).is_permitted(), "{gap}");
            assert!(claim.classify(&version).is_none(), "{gap}");
        }
        let later = InterfaceVersion::new("2.26.1").expect("later stable version");
        assert!(claim.assess(&later).is_permitted());
        assert!(claim.classify(&later).is_none());
        assert!(!claim.assess(&older).is_permitted());
    }

    #[test]
    fn version_stdout_parser_accepts_bare_or_named_stable_release() {
        assert_eq!(
            parse_vibe_version_output(b"2.25.4\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "2.25.4"
        );
        assert_eq!(
            parse_vibe_version_output(b"vibe 2.25.4\n")
                .expect("named version parses")
                .version()
                .as_str(),
            "2.25.4"
        );
        assert_eq!(
            parse_vibe_version_output(b"2.25.5\n")
                .expect("later stable version parses")
                .version()
                .as_str(),
            "2.25.5"
        );
        assert!(parse_vibe_version_output(b"2.25.6\n").is_some());
        assert!(parse_vibe_version_output(b"v2.25.4\n").is_none());
        assert!(parse_vibe_version_output(b"vibe  2.25.4\n").is_none());
    }
}
