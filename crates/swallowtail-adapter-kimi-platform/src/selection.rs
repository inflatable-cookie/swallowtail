pub const KIMI_PLATFORM_ENDPOINT: &str = "https://api.moonshot.ai";
pub const KIMI_PLATFORM_ENDPOINT_AUDIENCE: &str = "api.moonshot.ai";
pub const KIMI_PLATFORM_FACADE_REVISION: &str = "kimi-platform-chat-2026-07-21";
pub const KIMI_PLATFORM_MODEL_ID: &str = "kimi-k3";
pub const KIMI_PLATFORM_PROVIDER_ID: &str = "moonshot";
pub(crate) const KIMI_PLATFORM_MAXIMUM_OUTPUT_TOKENS: u64 = 1_048_576;

#[must_use]
pub fn kimi_platform_facade_binding() -> swallowtail_core::InterfaceVersionBinding {
    swallowtail_core::InterfaceVersionBinding::new(
        swallowtail_core::InterfaceVersionAxis::new("kimi-platform.chat-facade")
            .expect("static Kimi Platform interface axis is valid"),
        swallowtail_core::InterfaceVersion::new(KIMI_PLATFORM_FACADE_REVISION)
            .expect("static Kimi Platform facade revision is valid"),
    )
}

#[must_use]
pub fn kimi_platform_facade_claim() -> swallowtail_core::InterfaceCompatibilityClaim {
    swallowtail_core::InterfaceCompatibilityClaim::new(
        swallowtail_core::InterfaceCompatibilityClaimId::new("kimi-platform-chat-window-1")
            .expect("static Kimi Platform compatibility claim is valid"),
        swallowtail_core::InterfaceVersionAxis::new("kimi-platform.chat-facade")
            .expect("static Kimi Platform interface axis is valid"),
        swallowtail_core::InterfaceVersionScheme::Opaque,
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
        [swallowtail_core::InterfaceVersionSegment::exact(
            swallowtail_core::InterfaceVersion::new(KIMI_PLATFORM_FACADE_REVISION)
                .expect("static Kimi Platform facade revision is valid"),
            swallowtail_core::InterfaceBehaviorRevision::new("kimi-platform-k3-stream-v1")
                .expect("static Kimi Platform behavior revision is valid"),
            swallowtail_core::InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Kimi Platform compatibility claim is valid")
}
