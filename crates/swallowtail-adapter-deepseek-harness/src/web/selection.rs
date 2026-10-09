use std::path::Path;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Original frozen npm package version qualified by the Web route.
pub const DEEPSEEK_HARNESS_WEB_RELEASE_VERSION: &str = "0.1.0-rc.6";
/// Version axis for the DeepSeek Harness Web `/api` host.
pub const DEEPSEEK_HARNESS_WEB_RELEASE_AXIS: &str = "deepseek-harness.web";
/// Exact published CLI basename admitted by the route.
pub const DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME: &str = "dsh";
pub(crate) const DEEPSEEK_HARNESS_WEB_COMPATIBILITY_REVISION: &str = "deepseek-harness.web-rc6-1";
pub(crate) const DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION: &str =
    "deepseek-harness.apiproxy-v1";

const QUALIFIED_WEB_RELEASES: &[&str] = &[
    DEEPSEEK_HARNESS_WEB_RELEASE_VERSION,
    "0.1.0-rc.7",
    "0.1.0-rc.8",
    "0.1.1-rc.1",
    "0.1.1-rc.2",
];

#[must_use]
/// Returns the qualified-only Web `/api` compatibility claim.
pub fn deepseek_harness_web_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(DEEPSEEK_HARNESS_WEB_COMPATIBILITY_REVISION)
            .expect("static DeepSeek Harness Web claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [
            InterfaceVersionSegment::exact(
                InterfaceVersion::new(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION)
                    .expect("static DeepSeek Harness Web baseline is valid"),
                InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                    .expect("static DeepSeek Harness Web protocol revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("0.1.0-rc.7")
                    .expect("static DeepSeek Harness Web version is valid"),
                InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                    .expect("static DeepSeek Harness Web protocol revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("0.1.0-rc.8")
                    .expect("static DeepSeek Harness Web version is valid"),
                InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                    .expect("static DeepSeek Harness Web protocol revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("0.1.1-rc.1")
                    .expect("static DeepSeek Harness Web version is valid"),
                InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                    .expect("static DeepSeek Harness Web protocol revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::exact(
                InterfaceVersion::new("0.1.1-rc.2")
                    .expect("static DeepSeek Harness Web version is valid"),
                InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_WEB_PROTOCOL_FACADE_REVISION)
                    .expect("static DeepSeek Harness Web protocol revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
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
    let assessment = claim.assess(binding.version());
    if bindings.next().is_some()
        || !assessment.is_permitted()
        || assessment != plan.assess_interface_version(binding)
        || assessment.behavior_revision().is_none_or(|revision| {
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

pub(crate) fn web_version(plan: &PreflightPlan) -> Option<&str> {
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == &axis());
    let version = bindings.next()?.version().as_str();
    bindings.next().is_none().then_some(version)
}

pub(crate) fn web_command_arguments(version: &str, patch_path: &str) -> Option<Vec<String>> {
    if !QUALIFIED_WEB_RELEASES.contains(&version) {
        return None;
    }

    let mut arguments = vec!["web".to_owned()];
    if matches!(version, "0.1.0-rc.8" | "0.1.1-rc.1" | "0.1.1-rc.2") {
        arguments.push("--no-open".to_owned());
    }
    arguments.push("--patch".to_owned());
    arguments.push(patch_path.to_owned());
    Some(arguments)
}

pub(crate) fn parse_web_version_output(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let version = output
        .strip_suffix("\r\n")
        .or_else(|| output.strip_suffix('\n'))
        .unwrap_or(output);
    if version.is_empty()
        || version.len() > 64
        || version.chars().any(char::is_control)
        || version.chars().any(char::is_whitespace)
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(version).ok()?,
    ))
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS)
        .expect("static DeepSeek Harness Web axis is valid")
}

#[cfg(test)]
mod tests {
    use super::{
        DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, DEEPSEEK_HARNESS_WEB_RELEASE_VERSION,
        deepseek_harness_web_claim, parse_web_version_output, target_is_exact,
        web_command_arguments,
    };
    use swallowtail_core::{InterfaceVersion, InterfaceVersionAxis};

    #[test]
    fn web_pin_is_npm_identity_not_jsonrpc_or_host_metadata() {
        assert_eq!(DEEPSEEK_HARNESS_WEB_RELEASE_VERSION, "0.1.0-rc.6");
        assert_eq!(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, "deepseek-harness.web");
        let claim = deepseek_harness_web_claim();
        for version in [
            "0.1.0-rc.6",
            "0.1.0-rc.7",
            "0.1.0-rc.8",
            "0.1.1-rc.1",
            "0.1.1-rc.2",
        ] {
            assert!(claim.permits(&InterfaceVersion::new(version).expect("version")));
        }
        for version in [
            "0.1.0-rc.5",
            "0.1.2-alpha.2",
            "0.1.2-rc.1",
            "0.2.0-rc.2",
            "0.2.1-alpha.1",
        ] {
            assert!(!claim.permits(&InterfaceVersion::new(version).expect("version")));
        }
        assert!(!claim.permits(&InterfaceVersion::new("0.1.0rc6").expect("version")));
        assert!(target_is_exact("/fixture/bin/dsh"));
        assert!(!target_is_exact(
            "/fixture/bin/dsh-jsonrpc-agent-pkg-macos-arm64"
        ));
        assert!(InterfaceVersionAxis::new(DEEPSEEK_HARNESS_WEB_RELEASE_AXIS).is_ok());
    }

    #[test]
    fn browser_handoff_is_suppressed_only_where_the_published_cli_supports_it() {
        assert_eq!(
            web_command_arguments("0.1.0-rc.6", "/host/cordis.patch.yml"),
            Some(vec![
                "web".to_owned(),
                "--patch".to_owned(),
                "/host/cordis.patch.yml".to_owned(),
            ])
        );
        assert_eq!(
            web_command_arguments("0.1.0-rc.7", "/host/cordis.patch.yml"),
            Some(vec![
                "web".to_owned(),
                "--patch".to_owned(),
                "/host/cordis.patch.yml".to_owned(),
            ])
        );
        for version in ["0.1.0-rc.8", "0.1.1-rc.1", "0.1.1-rc.2"] {
            assert_eq!(
                web_command_arguments(version, "/host/cordis.patch.yml"),
                Some(vec![
                    "web".to_owned(),
                    "--no-open".to_owned(),
                    "--patch".to_owned(),
                    "/host/cordis.patch.yml".to_owned(),
                ])
            );
        }
        assert!(web_command_arguments("0.1.2-rc.1", "/host/cordis.patch.yml").is_none());
    }

    #[test]
    fn web_version_probe_accepts_only_one_exact_package_version() {
        for version in ["0.1.0-rc.6", "0.1.1-rc.2", "0.2.0-rc.2"] {
            let output = format!("{version}\n");
            assert_eq!(
                parse_web_version_output(output.as_bytes())
                    .expect("one npm version is parsed")
                    .version()
                    .as_str(),
                version
            );
        }
        for output in [
            b"".as_slice(),
            b"dsh 0.1.0-rc.6",
            b"0.1.0-rc.6\nwarning",
            b" 0.1.0-rc.6",
            b"0.1.0-rc.6\n\n",
            b"0.1.0-rc.6\0",
        ] {
            assert!(parse_web_version_output(output).is_none());
        }
    }
}
