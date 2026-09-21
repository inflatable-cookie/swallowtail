use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/claude-code-2.1.278/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/claude-code-2.1.278/protocol.json");
pub(super) const RESPONSE_ONLY: &str =
    include_str!("../fixtures/claude-code-2.1.278/response-only.json");
pub(super) const DIST_INVENTORY: &str =
    include_str!("../fixtures/claude-code-2.1.278/dist-inventory.json");
pub(super) const FROZEN_2_1_270_IDENTITY: &str =
    include_str!("../fixtures/claude-code-2.1.270/identity.json");

pub(super) const COMPARED_VERSIONS: &[&str] = &[
    "2.1.270", "2.1.271", "2.1.272", "2.1.273", "2.1.274", "2.1.275", "2.1.276", "2.1.277",
    "2.1.278",
];

pub(super) const PUBLISHED_HOPS: &[&str] = &[
    "2.1.271", "2.1.272", "2.1.273", "2.1.274", "2.1.275", "2.1.276", "2.1.277", "2.1.278",
];

pub(super) const SELECTED_FLAGS: &[&str] = &[
    "--input-format",
    "--output-format",
    "--verbose",
    "--no-session-persistence",
    "--model",
    "--effort",
    "--permission-mode",
    "--tools",
    "--setting-sources",
    "--mcp-config",
    "--strict-mcp-config",
    "--max-turns",
    "--safe-mode",
    "--disable-slash-commands",
    "--no-chrome",
    "--prompt-suggestions",
];

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
