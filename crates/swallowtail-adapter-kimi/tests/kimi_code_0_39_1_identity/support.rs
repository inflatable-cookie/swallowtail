use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/kimi-code-0.39.1/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/kimi-code-0.39.1/protocol.json");
pub(super) const ROUTING_IDENTITY: &str =
    include_str!("../fixtures/kimi-code-0.33.0-headless-routing/identity.json");
pub(super) const ROUTING_PROTOCOL: &str =
    include_str!("../fixtures/kimi-code-0.33.0-headless-routing/protocol.json");
pub(super) const AUTHORITY: &str =
    include_str!("../fixtures/kimi-code-0.39.0-acp-authority/identity.json");
pub(super) const FROZEN_0_38_0_IDENTITY: &str =
    include_str!("../fixtures/kimi-code-0.38.0/identity.json");
pub(super) const FROZEN_0_38_0_README: &str =
    include_str!("../fixtures/kimi-code-0.38.0/README.md");
pub(super) const FROZEN_0_37_2_PROTOCOL: &str =
    include_str!("../fixtures/kimi-code-0.37.2/protocol.json");
pub(super) const FROZEN_0_37_2_README: &str =
    include_str!("../fixtures/kimi-code-0.37.2/README.md");
pub(super) const FROZEN_0_38_0_HEADLESS_V2_PROTOCOL: &str =
    include_str!("../fixtures/kimi-code-0.38.0-headless-v2/protocol.json");
pub(super) const FROZEN_0_30_0_0_31_0_RANGE: &str =
    include_str!("../fixtures/kimi-code-0.30.0-0.31.0/installed-range.json");
pub(super) const FROZEN_0_31_1_RELEASE: &str =
    include_str!("../fixtures/kimi-code-0.31.1/release.json");

pub(super) fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

pub(super) fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

pub(super) fn text<'a>(value: &'a Value, path: &[&str]) -> &'a str {
    let mut cursor = value;
    for key in path {
        cursor = &cursor[*key];
    }
    cursor
        .as_str()
        .unwrap_or_else(|| panic!("{} is text", path.join(".")))
}

pub(super) fn assert_sha256(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

pub(super) fn assert_sha1(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 40);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
