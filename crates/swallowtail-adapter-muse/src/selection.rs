use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Exact opaque installed Muse Code payload revision.
pub const MUSE_CODE_RELEASE_REVISION: &str = "0.1.0-R708.1";
/// Basename of the signed versioned payload selected for execution.
pub const MUSE_CODE_PAYLOAD_BASENAME: &str = "muse-bin-0.1.0-R708.1";
/// Opaque version axis for signed Muse Code payloads.
pub const MUSE_CODE_RELEASE_AXIS: &str = "muse-code.signed-payload";
/// Exact Meta model qualified through the installed route.
pub const MUSE_SPARK_MODEL_ID: &str = "muse-spark-1.2";

pub(crate) const MUSE_HEADLESS_BEHAVIOR: &str = "muse-code.events-v1";

#[must_use]
/// Parses the one qualified opaque release revision.
pub fn muse_code_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != MUSE_CODE_RELEASE_REVISION {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
/// Returns the qualified-only exact headless protocol claim.
pub fn muse_headless_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("muse-code.headless-window-1")
            .expect("static Muse claim id is valid"),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(MUSE_CODE_RELEASE_REVISION)
                .expect("static Muse revision is valid"),
            InterfaceBehaviorRevision::new(MUSE_HEADLESS_BEHAVIOR)
                .expect("static Muse behavior is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Muse claim is valid")
}

pub(crate) fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    if !is_versioned_payload_target(plan.instance_target_ref().as_host_value()) {
        return Err(crate::failure::failure(
            "swallowtail.muse_code.headless.launcher_rejected",
            "Muse Code execution requires the exact versioned payload, not its mutable launcher",
        ));
    }
    let claim = muse_headless_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.muse_code.headless.version_missing",
            "Muse Code plan is missing its exact signed payload revision",
        )
    })?;
    if bindings.next().is_some() {
        return Err(crate::failure::failure(
            "swallowtail.muse_code.headless.version_ambiguous",
            "Muse Code plan contains more than one signed payload revision",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != MUSE_HEADLESS_BEHAVIOR)
    {
        return Err(crate::failure::failure(
            "swallowtail.muse_code.headless.version_incompatible",
            "Muse Code payload is incompatible with the headless driver",
        ));
    }
    Ok(())
}

pub(crate) fn is_versioned_payload_target(value: &str) -> bool {
    std::path::Path::new(value)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        == Some(MUSE_CODE_PAYLOAD_BASENAME)
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(MUSE_CODE_RELEASE_AXIS).expect("static Muse release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_opaque_payload_is_qualified() {
        assert!(muse_code_release_binding(MUSE_CODE_RELEASE_REVISION).is_some());
        for rejected in ["0.1.0", "0.1.0-R708.2", "Muse Code 0.1.0", " 0.1.0-R708.1"] {
            assert!(muse_code_release_binding(rejected).is_none());
        }
        let claim = muse_headless_claim();
        assert!(claim.supports(&InterfaceVersion::new(MUSE_CODE_RELEASE_REVISION).unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("0.1.0-R708.2").unwrap()));
        assert!(is_versioned_payload_target(&format!(
            "/fixture/bin/{MUSE_CODE_PAYLOAD_BASENAME}"
        )));
        assert!(!is_versioned_payload_target("/fixture/bin/muse"));
    }
}
