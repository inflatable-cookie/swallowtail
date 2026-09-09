use serde_json::Value;
use swallowtail_core::InterfaceVersion;

pub(super) const PRIOR_IDENTITY: &str = include_str!("../fixtures/pi-rpc-0.84.4/identity.json");
pub(super) const IDENTITY: &str = include_str!("../fixtures/pi-rpc-0.85.1/identity.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/pi-rpc-0.85.1/protocol.json");
pub(super) const DIST_INVENTORY: &str =
    include_str!("../fixtures/pi-rpc-0.85.1/dist-inventory.json");

pub(super) const HOST_SHA256: &str =
    "e6d7fcf36a239cf3746e67ddf4222081ac01a601b85a3ee688bdfe9c161d754c";
pub(super) const TARBALL_SHA256: &str =
    "1f498729649bdce647d1160993b4d92bf3c614cc819213bee2f91dd34f2a7af4";
pub(super) const GITHUB_TAG_COMMIT: &str = "d981de1229ef899957bbe968bc8dcda02a21f477";
pub(super) const INTERMEDIATE_GITHUB_TAG_COMMIT: &str = "107d79f11072bbc8a3a757ed7fd69596bee7d68c";
pub(super) const JSONL_BLOB: &str = "8962c734021eee0d39d3a35ad8b5c020b0c2c14b";
pub(super) const SESSION_CWD_BLOB: &str = "79960df1f7d4bebffa8fecfcb2aca3a3044bdd92";
pub(super) const JSON_EVENT_BLOB: &str = "c0c04fde0d7305bb8c07151c17498df8a25fb1d8";
pub(super) const ARGS_BLOB: &str = "8ad5da63e5cce1ee17476d061b3798359818fc97";
pub(super) const RPC_TYPES_BLOB: &str = "1cbd49a898382f0fbb409a7d241ad694b2f59e0d";
pub(super) const RPC_MODE_BLOB: &str = "fc8083bedc67824dd7ff1a5a154f1a08b28c4098";
pub(super) const RPC_DOCS_BLOB: &str = "d81c23bbf33917d1d319aa7bb460f05639cfb515";
pub(super) const AGENT_SESSION_BLOB: &str = "ac2bd4b18dbe4d888e48309ad7bb63f1166179b5";
pub(super) const FROZEN_RPC_TYPES_JS: &str =
    "bda239004694b7a087b3d9984e417d1cb4decf0da4483f1604881385240ff374";
pub(super) const FROZEN_RPC_MODE_JS: &str =
    "e7e4724aa55c5aac73cf36793653b26736200e5c59d58373990fc31028f86477";
pub(super) const FROZEN_ARGS_JS: &str =
    "bfb311d2c5d919fa4015d6aaa3c5a71a90b90011320e5e40f56b12e448c44dfc";

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
