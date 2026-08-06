use swallowtail_core::{
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding,
    InterfaceVersionScheme, InterfaceVersionSegment,
};

/// Exact opaque Anthropic Messages facade revision.
pub const ANTHROPIC_MESSAGES_FACADE_REVISION: &str = "anthropic-2023-06-01";
const FACADE_AXIS: &str = "anthropic.messages-facade";

#[must_use]
/// Returns the exact Anthropic Messages facade version binding.
pub fn anthropic_messages_facade_binding() -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(FACADE_AXIS).expect("static Anthropic axis is valid"),
        InterfaceVersion::new(ANTHROPIC_MESSAGES_FACADE_REVISION)
            .expect("static Anthropic facade revision is valid"),
    )
}

#[must_use]
/// Returns the qualified-only compatibility claim for the Messages facade.
pub fn anthropic_messages_facade_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("anthropic.messages-window-1")
            .expect("static Anthropic claim id is valid"),
        InterfaceVersionAxis::new(FACADE_AXIS).expect("static Anthropic axis is valid"),
        InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [InterfaceVersionSegment::exact(
            InterfaceVersion::new(ANTHROPIC_MESSAGES_FACADE_REVISION)
                .expect("static Anthropic facade revision is valid"),
            InterfaceBehaviorRevision::new("anthropic.messages-streaming-tools-v1")
                .expect("static Anthropic behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Anthropic Messages facade claim is valid")
}
