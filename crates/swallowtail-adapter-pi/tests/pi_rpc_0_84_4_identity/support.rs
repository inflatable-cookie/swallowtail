use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const PRIOR_IDENTITY: &str = include_str!("../fixtures/pi-rpc-0.84.3/identity.json");
pub(super) const IDENTITY: &str = include_str!("../fixtures/pi-rpc-0.84.4/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/pi-rpc-0.84.4/protocol.json");

pub(super) const FROZEN_CLI_SHA256: &str =
    "840d1e8e689ed9e4937bcb00b9a810e02a8567d9afb10a47097f11ca93ea1521";
pub(super) const HOST_SHA256: &str =
    "af302f231437eaf6f37691bce4b34234fcb626bcb5eb3910d4fc3f6519bf78ca";
pub(super) const TARBALL_SHA256: &str =
    "5bce766d19c3ceba18f3fbaad91c449c9f9d73981f9e3400ecef932006f06968";
pub(super) const GITHUB_TAG_COMMIT: &str = "b79e4cc834970cca69daebffab7df1da7d1e52c4";
pub(super) const JSONL_BLOB: &str = "8962c734021eee0d39d3a35ad8b5c020b0c2c14b";
pub(super) const SESSION_CWD_BLOB: &str = "79960df1f7d4bebffa8fecfcb2aca3a3044bdd92";
pub(super) const JSON_EVENT_BLOB: &str = "c0c04fde0d7305bb8c07151c17498df8a25fb1d8";
pub(super) const ARGS_BLOB: &str = "8ad5da63e5cce1ee17476d061b3798359818fc97";
pub(super) const RPC_TYPES_BLOB: &str = "1cbd49a898382f0fbb409a7d241ad694b2f59e0d";
pub(super) const RPC_MODE_BLOB: &str = "fc8083bedc67824dd7ff1a5a154f1a08b28c4098";

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

pub(super) fn assert_blob(value: &Value, expected: &str) {
    let value = value.as_str().expect("blob is text");
    assert_eq!(value.len(), 40);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

pub(super) fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
