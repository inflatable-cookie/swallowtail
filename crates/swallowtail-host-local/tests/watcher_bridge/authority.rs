use super::http::post_json;
use super::support::{close_lease, default_host, handshake, open_lease};
use futures_executor::block_on;
use swallowtail_core::{ExecutionHostId, WatcherCleanupCause};
use swallowtail_runtime::{
    ScopeId, WatcherBridgeBearer, WatcherBridgeEndpoint, WatcherBridgeGeneration,
    WatcherBridgeLease,
};

#[test]
fn forged_and_cross_identity_handles_cannot_close_a_live_lease() {
    let local = default_host("forge");
    let live = open_lease(&local, "turn-live");
    let endpoint = live.endpoint().expose().to_owned();
    let bearer = live.bearer().expose().to_owned();
    handshake(&endpoint, &bearer);
    let bridge = local.services().watcher_bridge().expect("bridge").clone();

    let forged = WatcherBridgeLease::new(
        live.execution_host_id().clone(),
        live.scope().clone(),
        live.turn().clone(),
        live.generation(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("forged-bearer").expect("bearer"),
    );
    let failure = block_on(bridge.close(forged, WatcherCleanupCause::Cancelled))
        .expect_err("forged close fails");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.watcher_bridge.foreign_lease"
    );

    let other_host = WatcherBridgeLease::new(
        ExecutionHostId::new("fixture.host.other").expect("host"),
        live.scope().clone(),
        live.turn().clone(),
        live.generation(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("forged-bearer").expect("bearer"),
    );
    assert!(block_on(bridge.close(other_host, WatcherCleanupCause::Cancelled)).is_err());

    let other_scope = WatcherBridgeLease::new(
        live.execution_host_id().clone(),
        ScopeId::new("scope-other").expect("scope"),
        live.turn().clone(),
        live.generation(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("forged-bearer").expect("bearer"),
    );
    assert!(block_on(bridge.close(other_scope, WatcherCleanupCause::Cancelled)).is_err());

    let other_generation = WatcherBridgeLease::new(
        live.execution_host_id().clone(),
        live.scope().clone(),
        live.turn().clone(),
        WatcherBridgeGeneration::new(99).expect("generation"),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("forged-bearer").expect("bearer"),
    );
    assert!(block_on(bridge.close(other_generation, WatcherCleanupCause::Cancelled)).is_err());

    let dropped = WatcherBridgeLease::new(
        live.execution_host_id().clone(),
        live.scope().clone(),
        live.turn().clone(),
        live.generation(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("forged-bearer").expect("bearer"),
    );
    drop(dropped);

    let (status, _) = post_json(
        &endpoint,
        Some(&bearer),
        &super::support::tools_list_body(2),
    );
    assert_eq!(status, 200);
    close_lease(&local, live);
}

#[test]
fn second_open_uses_a_new_generation_and_stale_handles_fail() {
    let local = default_host("stale-generation");
    let first = open_lease(&local, "turn-stale");
    let first_generation = first.generation();
    close_lease(&local, first);
    let second = open_lease(&local, "turn-stale");
    assert_ne!(second.generation(), first_generation);
    let stale = WatcherBridgeLease::new(
        second.execution_host_id().clone(),
        second.scope().clone(),
        second.turn().clone(),
        first_generation,
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint"),
        WatcherBridgeBearer::new("stale-bearer").expect("bearer"),
    );
    let bridge = local.services().watcher_bridge().expect("bridge").clone();
    assert!(block_on(bridge.close(stale, WatcherCleanupCause::Cancelled)).is_err());
    close_lease(&local, second);
}
