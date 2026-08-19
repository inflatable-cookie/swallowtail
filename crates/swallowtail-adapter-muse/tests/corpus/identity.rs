use serde_json::Value;
use swallowtail_adapter_muse::{MUSE_CODE_RELEASE_REVISION, muse_headless_claim};
use swallowtail_core::InterfaceVersion;

const CANDIDATE: &str = include_str!("../fixtures/muse-code-0.2.1-R1215.1/artifact.json");
const ECHO_SUMMARY: &str = include_str!("../fixtures/muse-code-0.2.1-R1215.1/echo-summary.json");
const VERSION: &str = include_str!("../fixtures/muse-code-0.2.1-R1215.1/version.txt");
const OLD_ECHO: &str = include_str!("../fixtures/muse-code-0.1.0-R708.1/echo-success.jsonl");

#[test]
fn host_0_2_1_identity_is_the_opaque_qualified_pin() {
    let artifact: Value = serde_json::from_str(CANDIDATE).expect("candidate artifact parses");
    let echo: Value = serde_json::from_str(ECHO_SUMMARY).expect("echo summary parses");

    assert_eq!(VERSION.trim(), "Muse Code 0.2.1 (0.2.1-R1215.1)");
    assert_eq!(artifact["release"], "0.2.1-R1215.1");
    assert_eq!(
        artifact["reported_version"],
        "Muse Code 0.2.1 (0.2.1-R1215.1)"
    );
    assert_eq!(artifact["payload"]["basename"], "muse-bin-0.2.1-R1215.1");
    assert_eq!(
        artifact["payload"]["sha256"],
        "b67f181fb7a519007146104c56fad372f47428da9608ade59835899160f2d6e9"
    );
    assert_eq!(artifact["payload"]["team_identifier"], "V9WTTPBFK9");
    assert_eq!(artifact["payload"]["selected_as_runtime_artifact"], true);
    assert_eq!(artifact["launcher"]["selected_as_runtime_artifact"], false);
    assert_eq!(
        artifact["direct_payload_probe"]["echo_jsonl_succeeded"],
        true
    );
    assert_eq!(
        artifact["direct_payload_probe"]["meta_provider_used"],
        false
    );
    assert_eq!(
        artifact["direct_payload_probe"]["provider_prompt_sent"],
        false
    );

    assert_eq!(
        artifact["claim_at_observation"]["qualified_revision"],
        "0.1.0-R708.1"
    );
    assert_eq!(
        artifact["claim_at_observation"]["posture"],
        "qualified_only"
    );
    assert_eq!(artifact["claim_at_observation"]["scheme"], "opaque");
    assert_eq!(artifact["identity_decision"]["shape"], "opaque-pin-move");
    assert_eq!(
        artifact["identity_decision"]["keep_both_opaque_segments"],
        false
    );
    assert_eq!(
        artifact["identity_decision"]["reuse_behavior_revision"],
        "muse-code.events-v1"
    );
    assert_eq!(
        artifact["identity_decision"]["echo_payload_type_sequence_unchanged"],
        true
    );

    assert_eq!(echo["schema_version"], 1);
    assert_eq!(echo["record_count"], 23);
    assert_eq!(echo["terminal"], "completed");
    assert_eq!(echo["terminal_text_prefix"], "echo:");
    assert_eq!(echo["meta_provider_used"], false);

    let old_types = OLD_ECHO
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            serde_json::from_str::<Value>(line)
                .expect("old echo line")
                .get("payload_type")
                .and_then(Value::as_str)
                .expect("payload_type")
                .to_owned()
        })
        .collect::<Vec<_>>();
    let new_types = echo["payload_types"]
        .as_array()
        .expect("payload_types")
        .iter()
        .map(|value| value.as_str().expect("type").to_owned())
        .collect::<Vec<_>>();
    assert_eq!(new_types, old_types);

    assert_eq!(MUSE_CODE_RELEASE_REVISION, "0.2.1-R1215.1");
    assert!(
        muse_headless_claim().supports(&InterfaceVersion::new("0.2.1-R1215.1").expect("new pin"))
    );
    assert!(
        !muse_headless_claim().permits(&InterfaceVersion::new("0.1.0-R708.1").expect("old pin"))
    );
}
