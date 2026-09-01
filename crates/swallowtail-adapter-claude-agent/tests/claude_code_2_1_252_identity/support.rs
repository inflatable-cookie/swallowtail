use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/claude-code-2.1.252/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/claude-code-2.1.252/protocol.json");
pub(super) const RESPONSE_ONLY: &str =
    include_str!("../fixtures/claude-code-2.1.252/response-only.json");
pub(super) const FROZEN_2_1_251_PROTOCOL: &str =
    include_str!("../fixtures/claude-code-2.1.251/protocol.json");
pub(super) const FROZEN_HELP_SHA256: &str =
    "5ff2e7a0bca8535fb9ec097fa0a21e9d6b735ed94104fa0d1f58ac73a841d52d";

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

pub(super) fn assert_sha256(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
