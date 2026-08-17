use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, PreflightPlan,
};
use swallowtail_runtime::RuntimeFailure;

/// Exact published runtime-bin identity qualified by the route.
pub const DEEPSEEK_HARNESS_RELEASE_VERSION: &str = "0.1.0rc6";
/// Interface-version axis for the bundled DeepSeek Harness runtime payload.
pub const DEEPSEEK_HARNESS_RELEASE_AXIS: &str = "deepseek-harness.runtime-bin";
/// Exact packaged executable basename admitted as the process target.
pub const DEEPSEEK_HARNESS_EXECUTABLE_BASENAME: &str = "dsh-jsonrpc-agent-pkg-macos-arm64";
/// Exact executable payload digest from Research 124.
///
/// Discovery and run validation stream-hash the host-approved executable and
/// require this exact value before the process can be admitted.
pub const DEEPSEEK_HARNESS_PAYLOAD_SHA256: &str =
    "ac1c91462518427467bd0a0ca3bf1049df62be0dbe8b0ee8014c6761cb8f80bf";
/// Exact spawn-helper digest from Research 124, retained as artifact provenance.
/// The helper is not the selected process target and is not a separate
/// executable admission axis.
pub const DEEPSEEK_HARNESS_SPAWN_HELPER_SHA256: &str =
    "21c589109bca43e287df884f3c34ab888033a83927ea7d273949ac5030583f26";

const PAYLOAD_HASH_BUFFER_BYTES: usize = 64 * 1024;

pub(crate) const DEEPSEEK_HARNESS_COMPATIBILITY_REVISION: &str = "deepseek-harness.jsonrpc-rc6-1";
pub(crate) const DEEPSEEK_HARNESS_PROTOCOL_FACADE_REVISION: &str =
    "deepseek-harness.sdk-jsonrpc-v1";

#[must_use]
/// Parses only the exact qualified runtime-bin release text.
pub fn deepseek_harness_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != DEEPSEEK_HARNESS_RELEASE_VERSION
        || value.len() > 32
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).ok()?,
    ))
}

#[must_use]
/// Returns the qualified-only exact JSON-RPC compatibility claim.
pub fn deepseek_harness_jsonrpc_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(DEEPSEEK_HARNESS_COMPATIBILITY_REVISION)
            .expect("static DeepSeek Harness claim id is valid"),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(DEEPSEEK_HARNESS_RELEASE_VERSION)
                .expect("static DeepSeek Harness release is valid"),
            InterfaceBehaviorRevision::new(DEEPSEEK_HARNESS_PROTOCOL_FACADE_REVISION)
                .expect("static DeepSeek Harness protocol revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static DeepSeek Harness claim is valid")
}

pub(crate) fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    validate_target_payload(plan.instance_target_ref().as_host_value())?;
    let claim = deepseek_harness_jsonrpc_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.deepseek_harness.version_missing",
            "DeepSeek Harness plan is missing its exact runtime-bin version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.version_ambiguous",
            "DeepSeek Harness plan contains more than one runtime-bin version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != DEEPSEEK_HARNESS_PROTOCOL_FACADE_REVISION)
    {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.version_incompatible",
            "DeepSeek Harness runtime-bin version is incompatible with the JSON-RPC driver",
        ));
    }
    Ok(())
}

pub(crate) fn target_is_exact(value: &str) -> bool {
    Path::new(value)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        == Some(DEEPSEEK_HARNESS_EXECUTABLE_BASENAME)
}

pub(crate) fn validate_target_payload(value: &str) -> Result<(), RuntimeFailure> {
    if !target_is_exact(value) {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.target_not_pinned",
            "DeepSeek Harness execution requires the exact packaged runtime target",
        ));
    }
    let actual = payload_sha256(value).map_err(|_| {
        crate::failure::failure(
            "swallowtail.deepseek_harness.payload_digest_unavailable",
            "DeepSeek Harness executable payload could not be hashed",
        )
    })?;
    if actual != DEEPSEEK_HARNESS_PAYLOAD_SHA256 {
        return Err(crate::failure::failure(
            "swallowtail.deepseek_harness.payload_digest_mismatch",
            "DeepSeek Harness executable payload does not match the qualified digest",
        ));
    }
    Ok(())
}

fn payload_sha256(value: &str) -> std::io::Result<String> {
    let mut file = File::open(value)?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; PAYLOAD_HASH_BUFFER_BYTES];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new(DEEPSEEK_HARNESS_RELEASE_AXIS)
        .expect("static DeepSeek Harness release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_rc6_runtime_is_qualified() {
        assert!(deepseek_harness_release_binding(DEEPSEEK_HARNESS_RELEASE_VERSION).is_some());
        for rejected in [
            "",
            "0.1.0",
            "0.1.0rc5",
            "0.1.0rc7",
            "v0.1.0rc6",
            "0.1.0rc6\n",
            " 0.1.0rc6",
        ] {
            assert!(
                deepseek_harness_release_binding(rejected).is_none(),
                "{rejected}"
            );
        }
        let claim = deepseek_harness_jsonrpc_claim();
        assert!(claim.supports(&InterfaceVersion::new(DEEPSEEK_HARNESS_RELEASE_VERSION).unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("0.1.0rc7").unwrap()));
        assert!(target_is_exact(
            "/fixture/bin/dsh-jsonrpc-agent-pkg-macos-arm64"
        ));
        assert!(!target_is_exact("/fixture/bin/dsh-jsonrpc-agent"));
    }

    #[test]
    fn payload_drift_is_rejected_before_process_admission() {
        let directory = std::env::temp_dir().join(format!(
            "swallowtail-deepseek-harness-selection-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("fixture directory is created");
        let path = directory.join(DEEPSEEK_HARNESS_EXECUTABLE_BASENAME);
        std::fs::write(&path, b"not the qualified executable").expect("fixture payload is written");

        let error = validate_target_payload(path.to_str().expect("fixture path is UTF-8"))
            .expect_err("payload drift is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.deepseek_harness.payload_digest_mismatch"
        );
        assert!(
            !error
                .diagnostic()
                .message()
                .contains(path.to_string_lossy().as_ref())
        );

        std::fs::remove_dir_all(directory).expect("fixture directory is removed");
    }
}
