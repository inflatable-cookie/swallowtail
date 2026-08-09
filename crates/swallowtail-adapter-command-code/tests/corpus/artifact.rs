use super::common::{ARTIFACT, PROTOCOL};
use serde_json::Value;
use swallowtail_adapter_command_code::{
    COMMAND_CODE_RELEASE_AXIS, COMMAND_CODE_RELEASE_VERSION, command_code_headless_claim,
    command_code_release_binding,
};
use swallowtail_core::InterfaceVersion;

#[test]
fn exact_artifact_and_protocol_revisions_are_bound_together() {
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
    assert_eq!(artifact["release"], COMMAND_CODE_RELEASE_VERSION);
    assert_eq!(protocol["artifact_revision"], COMMAND_CODE_RELEASE_VERSION);
    assert_eq!(protocol["route_id"], "command-code.headless");
    assert_eq!(
        protocol["protocol_facade_revision"],
        "command-code.agent-event-ndjson-v1"
    );
    assert_eq!(
        command_code_release_binding(COMMAND_CODE_RELEASE_VERSION)
            .expect("binding")
            .axis()
            .as_str(),
        COMMAND_CODE_RELEASE_AXIS
    );
    assert!(
        command_code_headless_claim()
            .supports(&InterfaceVersion::new(COMMAND_CODE_RELEASE_VERSION).expect("version"))
    );
}

#[test]
fn protocol_never_ingests_run_end_next_state() {
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
    assert!(
        protocol["known_event_types"]
            .as_array()
            .expect("known event types")
            .iter()
            .any(|value| value == "run_end")
    );
    assert!(
        !protocol["projected_event_types"]
            .as_array()
            .expect("projected event types")
            .iter()
            .any(|value| value == "run_end")
    );
}
