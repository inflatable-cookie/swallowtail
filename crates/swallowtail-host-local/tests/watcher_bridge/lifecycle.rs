use super::http::{post_json, post_json_result};
use super::support::{
    close_lease, default_host, handshake, initialize_body, open_lease, tool_call, tool_payload,
    tools_list_body, watcher_host,
};
use serde_json::json;
use swallowtail_core::{HostServiceKind, WatcherOwningTurn, WatcherRequester};
use swallowtail_runtime::{
    WATCHER_BRIDGE_HTTP_PATH, WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_WAIT,
};

#[test]
fn registration_exposes_the_bridge_kind_without_opening_a_lease() {
    let local = default_host("registration");
    assert!(
        local
            .services()
            .available_kinds()
            .contains(&HostServiceKind::WatcherBridge)
    );
    assert!(local.services().watcher_bridge().is_some());
}

#[test]
fn open_binds_ready_loopback_authority_and_redacts_it() {
    let local = default_host("open-ready");
    let lease = open_lease(&local, "turn-open");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    assert!(endpoint.starts_with("http://127.0.0.1:"));
    assert!(endpoint.ends_with(WATCHER_BRIDGE_HTTP_PATH));
    assert_eq!(bearer.len(), 64);
    assert!(!format!("{lease:?}").contains(&endpoint));
    assert!(!format!("{lease:?}").contains(&bearer));
    handshake(&endpoint, &bearer);
    close_lease(&local, lease);
}

#[test]
fn missing_wrong_and_duplicate_requests_fail_before_extra_watcher_work() {
    let local = watcher_host("fail-closed", "sleep");
    let lease = open_lease(&local, "turn-fail");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    let (status, _) = post_json(&endpoint, None, &initialize_body(1));
    assert_eq!(status, 401);
    let (status, _) = post_json(&endpoint, Some("deadbeef"), &initialize_body(1));
    assert_eq!(status, 401);
    handshake(&endpoint, &bearer);
    let start = tool_call(
        json!(2),
        WATCHER_BRIDGE_TOOL_START,
        json!({"operation_data": "sleep-operation"}),
    );
    let (status, _) = post_json(&endpoint, Some(&bearer), &start);
    assert_eq!(status, 200);
    let (status, body) = post_json(&endpoint, Some(&bearer), &start);
    assert_eq!(status, 200);
    assert!(body.contains("correlation"));
    let listed = futures_executor::block_on(
        local
            .services()
            .watcher()
            .expect("watcher")
            .list(WatcherOwningTurn::new("turn-fail").expect("owning turn")),
    )
    .expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].accepted_by(), WatcherRequester::Model);
    close_lease(&local, lease);
}

#[test]
fn reserved_family_freezes_idle_admission() {
    let local = watcher_host("same-registry", "exit-zero");
    let lease = open_lease(&local, "turn-registry");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let (status, body) = post_json(&endpoint, Some(&bearer), &tools_list_body(2));
    assert_eq!(status, 200);
    assert!(body.contains(WATCHER_BRIDGE_TOOL_START));
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(3),
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200);
    let watcher_id = tool_payload(&body)["watcher_id"]
        .as_str()
        .expect("id")
        .to_owned();
    let (status, _) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(4),
            WATCHER_BRIDGE_TOOL_WAIT,
            json!({"watcher_id": watcher_id}),
        ),
    );
    assert_eq!(status, 200);
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(json!(5), WATCHER_BRIDGE_TOOL_COMPLETION_GATE, json!({})),
    );
    assert_eq!(status, 200);
    assert!(body.contains("frozen"));
    let (status, body) = post_json(
        &endpoint,
        Some(&bearer),
        &tool_call(
            json!(6),
            WATCHER_BRIDGE_TOOL_START,
            json!({"operation_data": "exit-zero-operation"}),
        ),
    );
    assert_eq!(status, 200);
    assert!(body.contains("frozen"));
    close_lease(&local, lease);
}

#[test]
fn cross_lease_bearer_fails_and_close_releases_the_listener() {
    let local = default_host("cross-lease");
    let first = open_lease(&local, "turn-a");
    let second = open_lease(&local, "turn-b");
    let first_endpoint = first.endpoint().expose().to_owned();
    let first_bearer = first.bearer().expose().to_owned();
    let second_bearer = second.bearer().expose().to_owned();
    let (status, _) = post_json(&first_endpoint, Some(&second_bearer), &initialize_body(1));
    assert_eq!(status, 401);
    close_lease(&local, first);
    let closed = post_json_result(&first_endpoint, Some(&first_bearer), &initialize_body(2));
    assert!(closed.is_err() || closed.ok().is_some_and(|(status, _)| status != 200));
    close_lease(&local, second);
}

#[test]
fn dropped_lease_joins_without_leaving_a_listener() {
    let local = default_host("drop-join");
    let lease = open_lease(&local, "turn-drop");
    let endpoint = lease.endpoint().expose().to_owned();
    let bearer = lease.bearer().expose().to_owned();
    drop(lease);
    let closed = post_json_result(&endpoint, Some(&bearer), &initialize_body(1));
    assert!(closed.is_err() || closed.ok().is_some_and(|(status, _)| status != 200));
}
