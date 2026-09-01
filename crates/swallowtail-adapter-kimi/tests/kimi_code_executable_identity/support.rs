use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY_0_36_1: &str = include_str!("../fixtures/kimi-code-0.36.1/identity.json");
pub(super) const PROTOCOL_0_36_1: &str = include_str!("../fixtures/kimi-code-0.36.1/protocol.json");
pub(super) const IDENTITY_0_37_2: &str = include_str!("../fixtures/kimi-code-0.37.2/identity.json");
pub(super) const PROTOCOL_0_37_2: &str = include_str!("../fixtures/kimi-code-0.37.2/protocol.json");
pub(super) const IDENTITY_0_38_0: &str = include_str!("../fixtures/kimi-code-0.38.0/identity.json");
pub(super) const PROTOCOL_0_38_0: &str = include_str!("../fixtures/kimi-code-0.38.0/protocol.json");
pub(super) const IDENTITY_0_38_0_HEADLESS_V2: &str =
    include_str!("../fixtures/kimi-code-0.38.0-headless-v2/identity.json");
pub(super) const PROTOCOL_0_38_0_HEADLESS_V2: &str =
    include_str!("../fixtures/kimi-code-0.38.0-headless-v2/protocol.json");

pub(super) fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version text is non-empty")
}
