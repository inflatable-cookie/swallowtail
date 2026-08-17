use std::path::Path;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Exact npm package version qualified by the Web route.
pub const DEEPSEEK_HARNESS_WEB_RELEASE_VERSION: &str = "0.1.0-rc.6";
/// Version axis for the DeepSeek Harness Web `/api` host.
pub const DEEPSEEK_HARNESS_WEB_RELEASE_AXIS: &str = "deepseek-harness.web";
/// Exact published CLI basename admitted by the route.
pub const DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME: &str = "dsh";
pub(crate) const DEEPSEEK_HARNESS_WEB_COMPATIBILITY_REVISION: &str = "deepseek-harness.web-rc6-1";
pub(crate) const DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION: &str =
    "deepseek-harness.apiproxy-v1";

#[must_use]
/// Returns the qualified-only Web `/api` compatibility claim.
pub fn deepseek_harness_web_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(DEEPSEEK_HARNESS_WEB_COMPATIBILITY_REVISION)
            .expect("static DeepSeek Harness Web claim id is valid"),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION)
                .expect("static DeepSeek Harness Web version is valid"),
            InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                .expect("static DeepSeek Harness Web protocol revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static DeepSeek Harness Web claim is valid")
}

pub(crate) fn web_claim() -> InterfaceCompatibilityClaim {
    deepseek_harness_web_claim()
}

pub(crate) fn target_is_exact(value: &str) -> bool {
    Path::new(value)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        == Some(DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME)
}

pub(crate) fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    if !target_is_exact(plan.instance_target_ref().as_host_value()) {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.web.target_not_pinned",
            "DeepSeek Harness Web execution requires the exact dsh CLI target",
        ));
    }
    let claim = web_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.deepseek_harness.web.version_missing",
            "DeepSeek Harness Web plan is missing its exact npm version",
        )
    })?;
    if bindings.next().is_some()
        || !claim.permits(binding.version())
        || binding.version().as_str() != DEEPSEEK_HARNESS_WEB_RELEASE_VERSION
        || plan
            .assess_interface_version(binding)
            .behavior_revision()
            .is_none_or(|revision| {
                revision.as_str() != DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION
            })
    {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.web.version_incompatible",
            "DeepSeek Harness Web npm version is incompatible with the route",
        ));
    }
    Ok(())
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS)
        .expect("static DeepSeek Harness Web axis is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, DEEPSEEK_HARNESS_WEB_RELEASE_VERSION,
        deepseek_harness_web_claim, target_is_exact,
    };
    use swallowtail_core::{InterfaceVersion, InterfaceVersionAxis};

    #[test]
    fn web_pin_is_npm_identity_not_jsonrpc_or_host_metadata() {
        assert_eq!(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION, "0.1.0-rc.6");
        assert_eq!(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, "deepseek-harness.web");
        let claim = deepseek_harness_web_claim();
        assert!(claim.permits(&InterfaceVersion::new("0.1.0-rc.6").expect("version")));
        assert!(!claim.permits(&InterfaceVersion::new("0.1.0-rc.7").expect("version")));
        assert!(!claim.permits(&InterfaceVersion::new("0.1.0rc6").expect("version")));
        assert!(target_is_exact("/fixture/bin/dsh"));
        assert!(!target_is_exact(
            "/fixture/bin/dsh-jsonrpc-agent-pkg-macos-arm64"
        ));
        assert!(InterfaceVersionAxis::new(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS).is_ok());
    }
}
