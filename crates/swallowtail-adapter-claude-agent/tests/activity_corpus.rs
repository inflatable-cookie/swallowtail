use serde_json::Value;
use std::collections::BTreeSet;

const RANGE: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity-range.json");
const ACTIVITY: &str = include_str!("fixtures/claude-agent-acp-v0.53.0-v0.61.0/activity.jsonl");

const EXPECTED_EMITTED_UPDATES: &[&str] = &[
    "agent_message_chunk",
    "agent_thought_chunk",
    "async_task_progress",
    "async_task_spawned",
    "async_task_state_update",
    "available_commands_update",
    "config_option_update",
    "current_mode_update",
    "plan",
    "session_info_update",
    "subagent_spawned",
    "subagent_state_update",
    "tool_call",
    "tool_call_update",
    "usage_update",
    "user_message_chunk",
];

#[test]
fn every_qualified_claude_segment_has_exact_activity_provenance() {
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let segments = range["qualified_segments"]
        .as_array()
        .expect("segments are an array");
    assert_eq!(segments.len(), 7);
    for segment in segments {
        assert!(segment["range"].is_string());
        assert!(segment["acp_sdk"].is_string());
        assert!(segment["stable_schema"].is_string());
        assert_sha(segment, "tag_commit", 40);
        assert_sha(segment, "source_sha256", 64);
    }
    assert_eq!(range["qualified_segments"][6]["range"], "0.66.0..=0.98.0");
    assert_eq!(range["current_external_releases"][2]["version"], "0.64.0");
    assert_eq!(
        range["current_external_releases"][2]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][2]["profile"],
        "0.64.0-guarantee"
    );
    assert_eq!(range["current_external_releases"][3]["version"], "0.69.0");
    assert_eq!(
        range["current_external_releases"][3]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][3]["profile"],
        "0.69.0-guarantee"
    );
    assert_eq!(range["current_external_releases"][4]["version"], "0.70.0");
    assert_eq!(
        range["current_external_releases"][4]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][4]["profile"],
        "0.70.0-guarantee"
    );
    assert_eq!(range["current_external_releases"][5]["version"], "0.71.0");
    assert_eq!(
        range["current_external_releases"][5]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][5]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][5]["activity_delta"],
        "session-titles-subagents-modes-steering-unmapped"
    );
    assert_eq!(range["current_external_releases"][6]["version"], "0.72.0");
    assert_eq!(
        range["current_external_releases"][6]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][6]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][6]["activity_delta"],
        "effort-result-attribution-model-switch-hooks-unmapped"
    );
    assert_eq!(range["current_external_releases"][7]["version"], "0.73.0");
    assert_eq!(
        range["current_external_releases"][7]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][7]["profile"],
        "0.73.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][7]["activity_delta"],
        "agent-sdk-pin-unmapped"
    );
    assert_eq!(range["current_external_releases"][8]["version"], "0.74.0");
    assert_eq!(
        range["current_external_releases"][8]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][8]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][8]["activity_delta"],
        "hide-claude-auth-guard-and-session-failure-unmapped"
    );
    assert_eq!(range["current_external_releases"][9]["version"], "0.75.0");
    assert_eq!(
        range["current_external_releases"][9]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][9]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][9]["activity_delta"],
        "auth-status-compaction-and-usage-markdown-unmapped"
    );
    assert_eq!(range["current_external_releases"][10]["version"], "0.75.1");
    assert_eq!(
        range["current_external_releases"][10]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][10]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][10]["activity_delta"],
        "resumed-transcript-and-fork-meta-unmapped"
    );
    assert_eq!(range["current_external_releases"][11]["version"], "0.76.0");
    assert_eq!(
        range["current_external_releases"][11]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][11]["profile"],
        "0.76.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][11]["activity_delta"],
        "recommended-config-values-and-clear-context-unmapped"
    );
    assert_eq!(range["current_external_releases"][12]["version"], "0.77.0");
    assert_eq!(
        range["current_external_releases"][12]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][12]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][12]["activity_delta"],
        "agent-config-removal-and-multi-select-other-description"
    );
    assert_eq!(range["current_external_releases"][13]["version"], "0.78.0");
    assert_eq!(
        range["current_external_releases"][13]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][13]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][13]["activity_delta"],
        "compaction-update-and-single-select-other-description"
    );
    assert_eq!(range["current_external_releases"][14]["version"], "0.79.0");
    assert_eq!(
        range["current_external_releases"][14]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][14]["profile"],
        "0.79.0-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][14]["activity_delta"],
        "agent-sdk-pin-and-shell-permission-title-unmapped"
    );
    assert_eq!(range["current_external_releases"][15]["version"], "0.80.0");
    assert_eq!(
        range["current_external_releases"][15]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][15]["profile"],
        "0.81.2-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][15]["activity_delta"],
        "compaction-cancel-interrupt-and-acp-sdk-pin"
    );
    assert_eq!(range["current_external_releases"][16]["version"], "0.81.0");
    assert_eq!(
        range["current_external_releases"][16]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][16]["profile"],
        "0.81.2-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][16]["activity_delta"],
        "session-notices-capability-gated"
    );
    assert_eq!(range["current_external_releases"][17]["version"], "0.81.1");
    assert_eq!(
        range["current_external_releases"][17]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][17]["profile"],
        "0.81.2-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][17]["activity_delta"],
        "managed-policy-extract-and-usage-model-meta"
    );
    assert_eq!(range["current_external_releases"][18]["version"], "0.81.2");
    assert_eq!(
        range["current_external_releases"][18]["classification"],
        "qualified"
    );
    assert_eq!(
        range["current_external_releases"][18]["profile"],
        "0.81.2-guarantee"
    );
    assert_eq!(
        range["current_external_releases"][18]["activity_delta"],
        "native-subagent-and-exit-plan-unmapped"
    );
    let expected_current_hops = [
        (
            "0.82.0",
            "2026-09-28T14:06:22.315Z",
            "18de37624071b48e95aed9ec5382823e2d72cd39",
            "f6592f34d2bb0c247695049ce02daf6153b6faaf62e31adfd9bea91478a1da1b",
            "partial-tool-call-update-fields-supported-by-existing-decoder",
        ),
        (
            "0.83.0",
            "2026-09-28T16:01:42.778Z",
            "691328a9190d8729387149afad9bdb012450028b",
            "118db0410ec19acd855155da1c71e1a3cdf4926a0df7ddc82058d05c580015bc",
            "selected-acp-v1-activity-contract-unchanged",
        ),
        (
            "0.84.0",
            "2026-09-28T19:18:31.748Z",
            "bdb50ad984336e62dde1d41339f04071f6617085",
            "118db0410ec19acd855155da1c71e1a3cdf4926a0df7ddc82058d05c580015bc",
            "selected-acp-v1-activity-contract-unchanged",
        ),
        (
            "0.85.0",
            "2026-10-01T11:40:57.777Z",
            "c84845272fe3c55c1f97759f00ee48a1356fccae",
            "d5ef615bbfabf27397e24e22d45dacde98aa2f11f07ccb68d228c8e7a893a068",
            "selected-acp-v1-activity-contract-unchanged",
        ),
        (
            "0.85.1",
            "2026-10-02T09:00:37.749Z",
            "686c0c99b3b89217b74d1f5de8272e7c9ef1aab4",
            "644daa80157fcdfe7b806cbd23b56f75be4fed146a890aa5e219204e689aca13",
            "cancel-handler-rework-keeps-selected-method",
        ),
        (
            "0.86.0",
            "2026-10-05T13:18:45.542Z",
            "7b5c61a4ed55c03028ac60e1768bc57d92df24a4",
            "fc5b393d5b5f5b17dc796275581dd00eef60dd6b660c4892f067c19ac4565141",
            "load-replay-rework-keeps-selected-response",
        ),
        (
            "0.87.0",
            "2026-10-07T13:30:51.218Z",
            "b2dbc8f5a1b84f48cc1512d06f50a9d85d6e757b",
            "7d0d20d1c558c68fafeaf81f4d6f92f86ececb7c28b4679d0873b1b29b4ac8e5",
            "failed-tool-call-update-maps-to-existing-status",
        ),
        (
            "0.88.0",
            "2026-10-08T12:19:49.210Z",
            "d43fec3fa3118e88ff8a8ed99b85d2a5f444e538",
            "790992f7c7387d78a5324fcfcbf9eb79a811ebd68eb3d6161a93de34b28327a0",
            "agent-sdk-bump-keeps-selected-activity-contract",
        ),
        (
            "0.89.0",
            "2026-10-09T22:06:29.180Z",
            "96b6b4f478c5276b727c0a1d3cc3717da2d9e9ae",
            "f00fea811d7144360a6e70bc87e38e36d00a807d15825a8179e2458520b71784",
            "air-session-index-unmapped-missing-close-empty-success",
        ),
        (
            "0.89.1",
            "2026-10-10T09:11:59.285Z",
            "4e1fc1e8df2b9af3b1b2d6880ba90c5862283ba8",
            "f00fea811d7144360a6e70bc87e38e36d00a807d15825a8179e2458520b71784",
            "selected-acp-v1-activity-contract-unchanged",
        ),
        (
            "0.90.0",
            "2026-10-13T16:57:17.025Z",
            "7f7a1404a15024a45e62b699ced4de3fb7a814a2",
            "e3bbdc3399a2d702839d6e350b5b798fbe6e61d9ef916d38ccc1bc3abab15e3e",
            "cancel-closes-open-tool-calls-keeps-cancelled",
        ),
        (
            "0.91.0",
            "2026-10-15T12:25:22.910Z",
            "c0af592d252e6bd072b53407f618a9dd4cd02797",
            "2801cde1baea8dd2e69e0d9903cfeee1097750b00d52147cfc48368f504935ee",
            "unstored-delete-empty-success-and-gateway-narrowing-unmapped",
        ),
        (
            "0.92.0",
            "2026-10-16T12:42:02.948Z",
            "af3d7f9335dfe8fc2ea5320649f72b34d3cbf410",
            "8e2fca3efc966c4ff78047c3aa49189ea5628b95412917a566a2e2ce62257276",
            "prompt-suggestions-meta-unmapped",
        ),
        (
            "0.93.0",
            "2026-10-16T19:21:32.910Z",
            "dbf386d8d35d2bc30f17f11f15d1a4305c9d391f",
            "cc9c6a29633a0219077228733480d15f4c79ce9946b546c66752874a28069a15",
            "remote-control-command-unmapped",
        ),
        (
            "0.94.0",
            "2026-10-19T21:31:04.558Z",
            "7bcbfc7a5ca7ab9c7eece4f0749ccaf49c7ba7e1",
            "4fc4f4902ad4859666620635963abec6eb87263cf78b9fd4adb11708620745a4",
            "acp-sdk-2-import-path-keeps-v1-wire",
        ),
        (
            "0.95.0",
            "2026-10-23T20:00:57.929Z",
            "ebfbfbdaa9c2a6a8bd9eb0a4965b521009152350",
            "be0ebf68bb422a7c0867db8092e99e601eca0390d22f33f385f76b123e296243",
            "v1-client-stays-v1-async-tasks-capability-gated",
        ),
        (
            "0.96.0",
            "2026-10-26T09:53:45.910Z",
            "f6958db1ee8d3e9ead6767a3a294f98b9a21adb9",
            "a748438ac05553478285a1bb1bfe47c6adea1d47b97b93b4a55a411ea69bbeaa",
            "native-subagent-model-reports-unmapped",
        ),
        (
            "0.97.0",
            "2026-10-26T12:48:22.882Z",
            "d52c7c64d8e4b3e5bb988658fed4cd487b9f935c",
            "c569a914b2b9c75127193139a69bca3ca4ae6b2ebd6728de0330a16689fdc268",
            "ultracode-option-unmapped-advisor-tool-result-existing-shape",
        ),
        (
            "0.98.0",
            "2026-10-27T18:53:50.104Z",
            "c90f856c11b959d40328f86accc48f3a537aae96",
            "12c05702b3295d80a838135ee15802dd25739f8871a19eae6e664efcf3e2ed0f",
            "air-batch-get-unmapped-sandbox-network-settles-existing-status",
        ),
    ];
    let releases = range["current_external_releases"]
        .as_array()
        .expect("releases");
    assert_eq!(releases.len(), 19 + expected_current_hops.len());
    for (release, (version, published_at, tag_commit, source_sha256, activity_delta)) in
        releases.iter().skip(19).zip(expected_current_hops)
    {
        assert_eq!(release["version"], version);
        assert_eq!(release["published_at"], published_at);
        assert_eq!(release["tag_commit"], tag_commit);
        assert_eq!(release["source_sha256"], source_sha256);
        assert_eq!(release["classification"], "qualified");
        assert_eq!(release["profile"], "0.98.0-guarantee");
        assert_eq!(release["activity_delta"], activity_delta);
        assert_sha(release, "tag_commit", 40);
        assert_sha(release, "source_sha256", 64);
    }
}

#[test]
fn claude_activity_emitted_updates_are_the_exact_known_set() {
    let expected: BTreeSet<_> = EXPECTED_EMITTED_UPDATES.iter().copied().collect();
    assert_eq!(expected.len(), 16);
    for kind in [
        "subagent_spawned",
        "subagent_state_update",
        "async_task_spawned",
        "async_task_progress",
        "async_task_state_update",
    ] {
        assert!(expected.contains(kind), "missing emitted update {kind}");
    }
    let range: Value = serde_json::from_str(RANGE).expect("range fixture is valid JSON");
    let actual: BTreeSet<_> = range["emitted_updates"]
        .as_array()
        .expect("emitted_updates is an array")
        .iter()
        .map(|value| value.as_str().expect("emitted update is text"))
        .collect();
    assert_eq!(actual, expected);
    assert_eq!(
        range["emitted_updates"],
        serde_json::json!(EXPECTED_EMITTED_UPDATES)
    );
}

#[test]
fn claude_activity_keeps_display_tool_and_provider_metadata_boundaries() {
    let cases = json_lines(ACTIVITY);
    let names: BTreeSet<_> = cases
        .iter()
        .map(|case| case["case"].as_str().expect("case name is text"))
        .collect();
    for required in [
        "assistant-message",
        "thought-display",
        "plan-replacement",
        "tool-create",
        "tool-denied",
        "usage",
        "mode",
        "commands",
        "config",
        "permission",
        "completion",
        "unknown-safe",
        "malformed-tool",
    ] {
        assert!(names.contains(required), "missing case {required}");
    }
    assert_eq!(
        case(&cases, "thought-display")["expected"]["disclosure"],
        "provider_display_content"
    );
    assert_eq!(
        case(&cases, "tool-create")["expected"]["rawInput"],
        "excluded"
    );
    assert_eq!(
        case(&cases, "malformed-tool")["expected"]["semantics"],
        "fail_closed"
    );
}

fn assert_sha(value: &Value, field: &str, length: usize) {
    let hash = value[field].as_str().expect("hash is text");
    assert_eq!(hash.len(), length);
    assert!(hash.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn case<'a>(cases: &'a [Value], name: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| case["case"] == name)
        .expect("fixture case exists")
}

fn json_lines(value: &str) -> Vec<Value> {
    value
        .lines()
        .map(|line| serde_json::from_str(line).expect("fixture line is valid JSON"))
        .collect()
}
