use super::http::post_json;
use super::support::{
    close_lease, handshake, open_lease, tool_call, tools_list_body, watcher_host,
};
use serde_json::json;
use swallowtail_host_local::WatcherBridgeProofKind;
use swallowtail_runtime::{
    RuntimeTurnId, WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_START,
};

#[test]
fn reserved_bridge_operations_record_names_only() {
    let local = watcher_host("proof", "sleep");
    let lease = open_lease(&local, "turn-proof");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, _) = post_json(&endpoint, Some(&bearer), &tools_list_body(2));
    assert_eq!(status, 200);
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(3),
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "sleep-operation"}),
        ),
    );
    assert_eq!(status, 200, "{body}");
    let (status, _) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(json!(4), WATCHER_BRIDGE_TOOL_COMPLETION_GATE, json!({})),
    );
    assert_eq!(status, 200);
    let turn = RuntimeTurnId::new("turn-proof").expect("turn");
    let proof = local.watcher_bridge_proof(&turn);
    assert_eq!(
        proof,
        [
            WatcherBridgeProofKind::Initialize,
            WatcherBridgeProofKind::ToolsList,
            WatcherBridgeProofKind::Start,
            WatcherBridgeProofKind::CompletionGateActive,
        ]
    );
    let rendered = format!("{proof:?}");
    assert!(!rendered.contains(&bearer));
    assert!(!rendered.contains(&endpoint));
    assert!(!rendered.contains("127.0.0.1"));
    close_lease(&local, lease);
    assert_eq!(
        local.watcher_bridge_proof(&turn),
        proof,
        "proof survives lease close"
    );
}

#[test]
fn bridge_proof_is_scoped_to_the_owning_turn() {
    let local = watcher_host("proof-scope", "sleep");
    let first = open_lease(&local, "turn-a");
    let second = open_lease(&local, "turn-b");
    handshake(first.endpoint().expose(), first.bearer().expose());
    handshake(second.endpoint().expose(), second.bearer().expose());
    let (status, _) = post_json(
        first.endpoint().expose(),
        Some(first.bearer().expose()),
        &tools_list_body(2),
    );
    assert_eq!(status, 200);
    let turn_a = RuntimeTurnId::new("turn-a").expect("turn");
    let turn_b = RuntimeTurnId::new("turn-b").expect("turn");
    assert_eq!(
        local.watcher_bridge_proof(&turn_a),
        [
            WatcherBridgeProofKind::Initialize,
            WatcherBridgeProofKind::ToolsList,
        ]
    );
    assert_eq!(
        local.watcher_bridge_proof(&turn_b),
        [WatcherBridgeProofKind::Initialize]
    );
    close_lease(&local, first);
    close_lease(&local, second);
}
