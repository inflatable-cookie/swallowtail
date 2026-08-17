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

/// Exact published ZCode runtime identity qualified by the route.
pub const ZCODE_RELEASE_VERSION: &str = "0.16.3";
/// Interface-version axis for the bundled `zcode.cjs` payload.
pub const ZCODE_RELEASE_AXIS: &str = "zcode.runtime";
/// Exact packaged executable basename admitted as the process target.
pub const ZCODE_EXECUTABLE_BASENAME: &str = "zcode.cjs";
/// Exact executable payload digest from Research 126.
pub const ZCODE_PAYLOAD_SHA256: &str =
    "3e3433d90fa502e5d02498dfde6c2090df898331359bcfe5f3dbc9a1d00b685f";
/// Launcher digest recorded as packaging provenance, not an admission axis.
pub const ZCODE_LAUNCHER_SHA256: &str =
    "36b9cb48bb79eab0c568909fb9830750f68f701a5aab16cb181c735909555362";

const PAYLOAD_HASH_BUFFER_BYTES: usize = 64 * 1024;

pub(crate) const ZCODE_COMPATIBILITY_REVISION: &str = "zcode.app-server-0.16.3-1";
pub(crate) const ZCODE_PROTOCOL_FACADE_REVISION: &str = "zcode.protocol-stdio-v1";

#[must_use]
/// Parses only the exact qualified runtime release text.
pub fn zcode_release_binding(value: &str) -> Option<InterfaceVersionBinding> {
    if value != ZCODE_RELEASE_VERSION
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
/// Returns the qualified-only exact app-server compatibility claim.
pub fn zcode_app_server_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new(ZCODE_COMPATIBILITY_REVISION)
            .expect("static ZCode claim id is valid"),
        axis(),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(ZCODE_RELEASE_VERSION).expect("static ZCode release is valid"),
            InterfaceBehaviorRevision::new(ZCODE_PROTOCOL_FACADE_REVISION)
                .expect("static ZCode protocol revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static ZCode claim is valid")
}

pub(crate) fn validate_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    validate_target_payload(plan.instance_target_ref().as_host_value())?;
    let claim = zcode_app_server_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.zcode.app_server.version_missing",
            "ZCode plan is missing its exact runtime version",
        )
    })?;
    if bindings.next().is_some() {
        return Err(crate::failure::failure(
            "swallowtail.zcode.app_server.version_ambiguous",
            "ZCode plan contains more than one runtime version",
        ));
    }
    let assessment = claim.assess(binding.version());
    if assessment != plan.assess_interface_version(binding)
        || !assessment.is_permitted()
        || assessment
            .behavior_revision()
            .is_none_or(|revision| revision.as_str() != ZCODE_PROTOCOL_FACADE_REVISION)
    {
        return Err(crate::failure::failure(
            "swallowtail.zcode.app_server.version_incompatible",
            "ZCode runtime version is incompatible with the app-server driver",
        ));
    }
    Ok(())
}

pub(crate) fn target_is_exact(value: &str) -> bool {
    Path::new(value)
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        == Some(ZCODE_EXECUTABLE_BASENAME)
}

pub(crate) fn validate_target_payload(value: &str) -> Result<(), RuntimeFailure> {
    if !target_is_exact(value) {
        return Err(crate::failure::failure(
            "swallowtail.zcode.app_server.target_not_pinned",
            "ZCode execution requires the exact packaged runtime target",
        ));
    }
    let actual = payload_sha256(value).map_err(|_| {
        crate::failure::failure(
            "swallowtail.zcode.app_server.payload_digest_unavailable",
            "ZCode executable payload could not be hashed",
        )
    })?;
    if actual != ZCODE_PAYLOAD_SHA256 {
        return Err(crate::failure::failure(
            "swallowtail.zcode.app_server.payload_digest_mismatch",
            "ZCode executable payload does not match the qualified digest",
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
    InterfaceVersionAxis::new(ZCODE_RELEASE_AXIS).expect("static ZCode release axis is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_exact_runtime_is_qualified() {
        assert!(zcode_release_binding(ZCODE_RELEASE_VERSION).is_some());
        for rejected in [
            "", "0.16.2", "0.16.4", "3.7.7", "3.7.7-13", "v0.16.3", "0.16.3\n", " 0.16.3",
        ] {
            assert!(zcode_release_binding(rejected).is_none(), "{rejected}");
        }
        let claim = zcode_app_server_claim();
        assert!(claim.supports(&InterfaceVersion::new(ZCODE_RELEASE_VERSION).unwrap()));
        assert!(!claim.permits(&InterfaceVersion::new("0.16.4").unwrap()));
        assert!(target_is_exact("/fixture/vendor/zcode.cjs"));
        assert!(!target_is_exact("/fixture/bin/zcode.js"));
        assert!(!target_is_exact("/Applications/ZCode.app"));
    }

    #[test]
    fn payload_drift_is_rejected_before_process_admission() {
        let directory = std::env::temp_dir().join(format!(
            "swallowtail-zcode-selection-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("fixture directory is created");
        let path = directory.join(ZCODE_EXECUTABLE_BASENAME);
        std::fs::write(&path, b"not the qualified executable").expect("fixture payload is written");

        let error = validate_target_payload(path.to_str().expect("fixture path is UTF-8"))
            .expect_err("payload drift is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.zcode.app_server.payload_digest_mismatch"
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
