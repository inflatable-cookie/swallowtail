use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/codex-cli-0.152.1/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/codex-cli-0.152.1/protocol.json");
pub(super) const DIST_INVENTORY: &str =
    include_str!("../fixtures/codex-cli-0.152.1/dist-inventory.json");
pub(super) const FROZEN_0_152_0_PROTOCOL: &str =
    include_str!("../fixtures/codex-cli-0.152.0/protocol.json");

pub(super) const FROZEN_0_152_0_EXEC_HELP_SHA256: &str =
    "e504bac5a6364566fbe408132dec7993639def9258ece34e8352f51f8d43687c";
pub(super) const FROZEN_0_152_0_APP_SERVER_HELP_SHA256: &str =
    "95d290035d274e91e6f85b9af63e9a3fd2cf70a2295d9eedbfc23a2ee82d4383";

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

pub(super) fn assert_exact_string_set(value: &Value, expected: &[&str]) {
    assert_eq!(strings(value), expected);
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
