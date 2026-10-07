use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, OperationShape,
    TransportFamilyId,
};

/// Oldest Ollama runtime release qualified for the native text facade.
pub const OLLAMA_BASELINE_VERSION: &str = "0.14.0";
/// Newest Ollama runtime release qualified for the native text facade.
///
/// Exact stable points `0.35.0` and `0.35.1` retain
/// `ollama.native-text-v1`. `0.40.0` adds an inference-triggered local
/// compatibility migration, so it remains visibly unverified pending an
/// adapter and authority ruling (Research 377).
pub const OLLAMA_LATEST_QUALIFIED_VERSION: &str = "0.35.1";
pub(crate) const OLLAMA_RUNTIME_AXIS: &str = "ollama.runtime";
pub(crate) const OLLAMA_DRIVER_ID: &str = "swallowtail.ollama.native-attached";
/// Exact text-only native API facade selected by this adapter.
pub const OLLAMA_NATIVE_FACADE: &str = "ollama.native-api.text-v1";
/// Maximum accepted observed Ollama runtime-version text.
const MAX_VERSION_BYTES: usize = 64;

/// Binds an observed Ollama runtime version to its semantic-version axis.
///
/// Returns `None` for blank, oversized, or control-character text, so a
/// malformed provider version can never panic a caller. Classification of
/// well-formed but out-of-window text stays with the compatibility claim.
#[must_use]
pub fn ollama_runtime_binding(version: &str) -> Option<InterfaceVersionBinding> {
    if version.is_empty()
        || version.len() > MAX_VERSION_BYTES
        || version.trim() != version
        || version.chars().any(char::is_control)
    {
        return None;
    }
    Some(InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(OLLAMA_RUNTIME_AXIS).expect("static version axis is valid"),
        InterfaceVersion::new(version).ok()?,
    ))
}

/// Returns the maintained runtime window and known excluded versions.
#[must_use]
pub fn ollama_runtime_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("ollama.native-runtime-window-2")
            .expect("static claim id is valid"),
        InterfaceVersionAxis::new(OLLAMA_RUNTIME_AXIS).expect("static version axis is valid"),
        InterfaceVersionScheme::Semantic,
        swallowtail_core::InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::new(
                InterfaceVersion::new(OLLAMA_BASELINE_VERSION).expect("baseline is valid"),
                InterfaceVersion::new("0.34.4").expect("previous ceiling is valid"),
                InterfaceBehaviorRevision::new("ollama.native-text-v1")
                    .expect("static behavior revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::new(
                InterfaceVersion::new("0.35.0").expect("first qualified point is valid"),
                InterfaceVersion::new(OLLAMA_LATEST_QUALIFIED_VERSION).expect("latest is valid"),
                InterfaceBehaviorRevision::new("ollama.native-text-v1")
                    .expect("static behavior revision is valid"),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [
            InterfaceVersion::new("0.32.2").expect("known exclusion is valid"),
            InterfaceVersion::new("0.32.10").expect("known exclusion is valid"),
        ],
    )
    .expect("static Ollama compatibility claim is valid")
}

/// Describes the attached runtime's catalogue, run, and session roles.
#[must_use]
pub fn ollama_native_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(OLLAMA_DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("ollama").expect("static family id is valid"),
        TransportFamilyId::new("http-ndjson-native").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::ModelCatalog,
        DriverRole::StructuredRun,
        DriverRole::InteractiveSession,
    ])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([
        OperationShape::StructuredRun,
        OperationShape::InteractiveSession,
    ])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
        ],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
        ],
    )
    .with_interface_compatibility(ollama_runtime_claim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::{DriverRole, InterfaceSupportStatus};

    #[test]
    fn descriptor_publishes_maintained_segments_without_filling_published_holes() {
        let descriptor = ollama_native_descriptor();
        assert!(descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        for version in [
            "0.14.0", "0.18.0", "0.30.0", "0.32.1", "0.32.9", "0.32.14", "0.32.15", "0.33.0",
            "0.33.1", "0.33.2", "0.33.3", "0.34.0", "0.34.1", "0.34.2", "0.34.3", "0.34.4",
            "0.35.0", "0.35.1",
        ] {
            assert_eq!(
                descriptor
                    .classify_interface_version(
                        &ollama_runtime_binding(version).expect("fixture Ollama version is valid"),
                    )
                    .expect("qualification point is supported")
                    .support_status(),
                InterfaceSupportStatus::Maintained
            );
        }
        for version in [
            "0.13.5",
            "0.18.0-rc.1",
            "0.32.2",
            "0.32.10",
            "0.32.3-rc.0",
            "0.34.5",
            "0.35.2",
            "0.39.0",
        ] {
            assert!(!descriptor.supports_interface_version(
                &ollama_runtime_binding(version).expect("fixture Ollama version is valid"),
            ));
        }
        assert!(matches!(
            descriptor.assess_interface_version(
                &ollama_runtime_binding("0.40.0").expect("fixture Ollama version is valid"),
            ),
            swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }

    #[test]
    fn claim_keeps_historical_and_new_exact_points_under_one_behavior() {
        let claim = ollama_runtime_claim();
        for version in ["0.14.0", "0.32.15", "0.34.4", "0.35.0", "0.35.1"] {
            let swallowtail_core::InterfaceCompatibilityAssessment::Qualified(matched) =
                claim.assess(&InterfaceVersion::new(version).unwrap())
            else {
                panic!("{version} must be qualified");
            };
            assert_eq!(matched.support_status(), InterfaceSupportStatus::Maintained);
            assert_eq!(
                matched.behavior_revision().as_str(),
                "ollama.native-text-v1"
            );
        }
        for version in ["0.32.2", "0.32.10", "0.34.5"] {
            assert!(!claim.permits(&InterfaceVersion::new(version).unwrap()));
        }
        for version in ["0.35.2", "0.39.0", "0.40.0"] {
            assert!(matches!(
                claim.assess(&InterfaceVersion::new(version).unwrap()),
                swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ));
        }
    }
}
