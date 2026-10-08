use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment,
};

/// Qualified service behavior revision for Runtime `ConverseStream`.
pub const BEDROCK_RUNTIME_SERVICE_REVISION: &str = "bedrock-runtime-converse-stream";
/// Qualified service behavior revision for `ListFoundationModels`.
pub const BEDROCK_CATALOGUE_SERVICE_REVISION: &str = "bedrock-list-foundation-models";

const RUNTIME_SDK_AXIS: &str = "amazon-bedrock.runtime-rust-sdk";
const RUNTIME_SERVICE_AXIS: &str = "amazon-bedrock.runtime-service-api";
const CATALOGUE_SDK_AXIS: &str = "amazon-bedrock.control-plane-rust-sdk";
const CATALOGUE_SERVICE_AXIS: &str = "amazon-bedrock.control-plane-service-api";

#[must_use]
/// Returns separate SDK and service bindings for Bedrock Runtime.
pub fn bedrock_runtime_interface_bindings() -> [InterfaceVersionBinding; 2] {
    [
        binding(RUNTIME_SDK_AXIS, crate::SDK_VERSION),
        binding(RUNTIME_SERVICE_AXIS, BEDROCK_RUNTIME_SERVICE_REVISION),
    ]
}

#[must_use]
/// Returns separate SDK and service bindings for the Bedrock catalogue.
pub fn bedrock_catalogue_interface_bindings() -> [InterfaceVersionBinding; 2] {
    [
        binding(CATALOGUE_SDK_AXIS, crate::CATALOGUE_SDK_VERSION),
        binding(CATALOGUE_SERVICE_AXIS, BEDROCK_CATALOGUE_SERVICE_REVISION),
    ]
}

#[must_use]
/// Returns Runtime SDK and service compatibility claims.
pub fn bedrock_runtime_interface_claims() -> [InterfaceCompatibilityClaim; 2] {
    [
        runtime_sdk_claim(),
        exact_claim(
            "amazon-bedrock.runtime-service-window-1",
            RUNTIME_SERVICE_AXIS,
            BEDROCK_RUNTIME_SERVICE_REVISION,
            "amazon-bedrock.runtime-service-1",
        ),
    ]
}

fn runtime_sdk_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(
            InterfaceCompatibilityClaimId::new,
            "amazon-bedrock.runtime-sdk-window-1",
        ),
        valid(InterfaceVersionAxis::new, RUNTIME_SDK_AXIS),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::exact(
                valid(InterfaceVersion::new, "1.136.0"),
                valid(
                    InterfaceBehaviorRevision::new,
                    "amazon-bedrock.runtime-sdk-1",
                ),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::new(
                valid(InterfaceVersion::new, "1.139.0"),
                valid(InterfaceVersion::new, crate::SDK_VERSION),
                valid(
                    InterfaceBehaviorRevision::new,
                    "amazon-bedrock.runtime-sdk-1",
                ),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [valid(InterfaceVersion::new, "1.144.0")],
    )
    .expect("static Bedrock Runtime SDK compatibility claim is valid")
}

#[must_use]
/// Returns the semantic SDK and exact service claims for the Bedrock catalogue.
pub fn bedrock_catalogue_interface_claims() -> [InterfaceCompatibilityClaim; 2] {
    [
        catalogue_sdk_claim(),
        exact_claim(
            "amazon-bedrock.catalogue-service-window-1",
            CATALOGUE_SERVICE_AXIS,
            BEDROCK_CATALOGUE_SERVICE_REVISION,
            "amazon-bedrock.catalogue-service-1",
        ),
    ]
}

fn catalogue_sdk_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(
            InterfaceCompatibilityClaimId::new,
            "amazon-bedrock.catalogue-sdk-window-1",
        ),
        valid(InterfaceVersionAxis::new, CATALOGUE_SDK_AXIS),
        InterfaceVersionScheme::Semantic,
        InterfaceNewerVersionPosture::AllowUnverified,
        [
            InterfaceVersionSegment::exact(
                valid(InterfaceVersion::new, "1.148.0"),
                valid(
                    InterfaceBehaviorRevision::new,
                    "amazon-bedrock.catalogue-sdk-1",
                ),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::new(
                valid(InterfaceVersion::new, "1.150.0"),
                valid(InterfaceVersion::new, "1.155.0"),
                valid(
                    InterfaceBehaviorRevision::new,
                    "amazon-bedrock.catalogue-sdk-1",
                ),
                InterfaceSupportStatus::Maintained,
            ),
            InterfaceVersionSegment::new(
                valid(InterfaceVersion::new, "1.157.0"),
                valid(InterfaceVersion::new, crate::CATALOGUE_SDK_VERSION),
                valid(
                    InterfaceBehaviorRevision::new,
                    "amazon-bedrock.catalogue-sdk-1",
                ),
                InterfaceSupportStatus::Maintained,
            ),
        ],
        [
            valid(InterfaceVersion::new, "1.149.0"),
            valid(InterfaceVersion::new, "1.156.0"),
        ],
    )
    .expect("static Bedrock catalogue SDK compatibility claim is valid")
}

fn binding(axis: &str, version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        valid(InterfaceVersionAxis::new, axis),
        valid(InterfaceVersion::new, version),
    )
}

fn exact_claim(id: &str, axis: &str, version: &str, behavior: &str) -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        valid(InterfaceCompatibilityClaimId::new, id),
        valid(InterfaceVersionAxis::new, axis),
        InterfaceVersionScheme::Opaque,
        InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            valid(InterfaceVersion::new, version),
            valid(InterfaceBehaviorRevision::new, behavior),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Bedrock interface claim is valid")
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static Bedrock interface identity is valid")
}
