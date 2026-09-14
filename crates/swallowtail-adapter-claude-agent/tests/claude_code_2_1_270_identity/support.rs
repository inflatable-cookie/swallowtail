use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/claude-code-2.1.270/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/claude-code-2.1.270/protocol.json");
pub(super) const RESPONSE_ONLY: &str =
    include_str!("../fixtures/claude-code-2.1.270/response-only.json");
pub(super) const DIST_INVENTORY: &str =
    include_str!("../fixtures/claude-code-2.1.270/dist-inventory.json");
pub(super) const FROZEN_2_1_257_IDENTITY: &str =
    include_str!("../fixtures/claude-code-2.1.257/identity.json");
pub(super) const FROZEN_2_1_257_PROTOCOL: &str =
    include_str!("../fixtures/claude-code-2.1.257/protocol.json");

/// Help digest frozen for both official `2.1.257` and official `2.1.258`.
pub(super) const FROZEN_2_1_257_HELP_SHA256: &str =
    "a0ab4f1df36388fba86563a10839c020cc7dcb13cec2311c336aebe6963db0a1";
pub(super) const OFFICIAL_2_1_258_HELP_SHA256: &str =
    "a0ab4f1df36388fba86563a10839c020cc7dcb13cec2311c336aebe6963db0a1";
/// Normalized `init` record shape shared by every compared version once the
/// embedded version, build timestamp, and build commit are blanked.
pub(super) const INIT_RECORD_SHAPE_SHA256: &str =
    "3a19423278f50a72bcbc4335a38d568f52e0f628fff7bae9d63f157838b5d187";

pub(super) const COMPARED_VERSIONS: &[&str] = &[
    "2.1.257", "2.1.258", "2.1.259", "2.1.260", "2.1.261", "2.1.263", "2.1.265", "2.1.266",
    "2.1.267", "2.1.268", "2.1.269", "2.1.270",
];

pub(super) const PUBLISHED_HOPS: &[&str] = &[
    "2.1.258", "2.1.259", "2.1.260", "2.1.261", "2.1.263", "2.1.265", "2.1.266", "2.1.267",
    "2.1.268", "2.1.269", "2.1.270",
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
