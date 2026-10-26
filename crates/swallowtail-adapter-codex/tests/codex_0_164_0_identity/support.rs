use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const IDENTITY: &str = include_str!("../fixtures/codex-cli-0.164.0/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/codex-cli-0.164.0/protocol.json");
pub(super) const DIST_INVENTORY: &str =
    include_str!("../fixtures/codex-cli-0.164.0/dist-inventory.json");
pub(super) const SOURCE_INVENTORY: &str =
    include_str!("../fixtures/codex-cli-0.164.0/source-inventory.json");
pub(super) const SOURCE_STATUS: &str =
    include_str!("../fixtures/codex-cli-0.164.0/source-name-status.tsv");

pub(super) const HOPS: &[&str] = &["0.161.0", "0.162.0", "0.162.1", "0.163.0", "0.164.0"];

pub(super) fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

pub(super) fn fixture_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/codex-cli-0.164.0")
}

pub(super) fn assert_sha256(value: &Value) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

pub(super) fn table_rows(value: &str) -> impl Iterator<Item = Vec<&str>> {
    value
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| line.split('\t').collect::<Vec<_>>())
}
