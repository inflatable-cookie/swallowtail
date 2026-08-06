use super::common::{ARTIFACT, PROTOCOL};
use serde_json::Value;
use swallowtail_adapter_muse::{
    MUSE_CODE_PAYLOAD_BASENAME, MUSE_CODE_RELEASE_AXIS, MUSE_CODE_RELEASE_REVISION,
    MUSE_SPARK_MODEL_ID, muse_code_release_binding, muse_headless_claim,
};
use swallowtail_core::InterfaceVersion;

#[test]
fn exact_artifact_and_protocol_revisions_are_bound_together() {
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
    assert_eq!(artifact["release"], MUSE_CODE_RELEASE_REVISION);
    assert_eq!(artifact["payload"]["basename"], MUSE_CODE_PAYLOAD_BASENAME);
    assert_eq!(protocol["artifact_revision"], MUSE_CODE_RELEASE_REVISION);
    assert_eq!(protocol["protocol_facade_revision"], "muse-code.events-v1");
    assert_eq!(protocol["meta_success"]["model_id"], MUSE_SPARK_MODEL_ID);
    assert_eq!(
        muse_code_release_binding(MUSE_CODE_RELEASE_REVISION)
            .expect("binding")
            .axis()
            .as_str(),
        MUSE_CODE_RELEASE_AXIS
    );
    assert!(
        muse_headless_claim()
            .supports(&InterfaceVersion::new(MUSE_CODE_RELEASE_REVISION).expect("version"))
    );
}

#[test]
fn launcher_is_not_the_selected_runtime_artifact() {
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture parses");
    assert_eq!(artifact["launcher"]["selected_as_runtime_artifact"], false);
    assert_eq!(artifact["payload"]["selected_as_runtime_artifact"], true);
    assert_eq!(artifact["launcher"]["may_update_before_delegation"], true);
    assert_eq!(
        artifact["direct_payload_probe"]["launcher_update_path_executed"],
        false
    );
}
