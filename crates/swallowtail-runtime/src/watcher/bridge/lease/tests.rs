use super::WatcherBridgeLease;
use crate::{
    RuntimeTurnId, ScopeId, WatcherBridgeAdmission, WatcherBridgeBearer,
    WatcherBridgeCompletionState, WatcherBridgeEndpoint, WatcherBridgeGeneration,
    WatcherBridgeOpenRequest, WatcherBridgeToken,
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::ExecutionHostId;

#[test]
fn lease_values_redact_endpoint_bearer_and_token() {
    let endpoint = WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint is valid");
    let bearer = WatcherBridgeBearer::new("bridge-secret-token").expect("bearer is valid");
    let lease = WatcherBridgeLease::new(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ScopeId::new("scope-1").expect("scope is valid"),
        RuntimeTurnId::new("turn-1").expect("turn is valid"),
        WatcherBridgeGeneration::initial(),
        endpoint,
        bearer,
    )
    .bind(
        WatcherBridgeToken::new("lease-binding-token").expect("token is valid"),
        || {},
    );

    let debug = format!("{lease:?}");
    assert!(!debug.contains("127.0.0.1"));
    assert!(!debug.contains("bridge-secret-token"));
    assert!(!debug.contains("lease-binding-token"));
    assert!(!debug.contains("/mcp"));
    assert!(lease.bearer().matches("bridge-secret-token"));
    assert!(
        lease.binding_matches(
            &WatcherBridgeToken::new("lease-binding-token").expect("token is valid")
        )
    );
    assert!(
        !lease.binding_matches(
            &WatcherBridgeToken::new("other-binding-token").expect("token is valid")
        )
    );
}

#[test]
fn constructed_handles_cannot_authenticate_a_live_binding() {
    let lease = WatcherBridgeLease::new(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ScopeId::new("scope-1").expect("scope is valid"),
        RuntimeTurnId::new("turn-1").expect("turn is valid"),
        WatcherBridgeGeneration::initial(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint is valid"),
        WatcherBridgeBearer::new("bridge-secret-token").expect("bearer is valid"),
    );
    assert!(
        !lease.binding_matches(
            &WatcherBridgeToken::new("lease-binding-token").expect("token is valid")
        )
    );
}

#[test]
fn drop_always_runs_bound_cleanup() {
    let ran = Arc::new(AtomicBool::new(false));
    let ran_on_drop = Arc::clone(&ran);
    let lease = WatcherBridgeLease::new(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ScopeId::new("scope-1").expect("scope is valid"),
        RuntimeTurnId::new("turn-1").expect("turn is valid"),
        WatcherBridgeGeneration::initial(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint is valid"),
        WatcherBridgeBearer::new("bridge-secret-token").expect("bearer is valid"),
    )
    .bind(
        WatcherBridgeToken::new("lease-binding-token").expect("token is valid"),
        move || ran_on_drop.store(true, Ordering::SeqCst),
    );
    drop(lease);
    assert!(ran.load(Ordering::SeqCst));
}

#[test]
fn later_bind_cannot_disarm_cleanup_or_rebind_the_token() {
    let original = Arc::new(AtomicBool::new(false));
    let original_on_drop = Arc::clone(&original);
    let replacement = Arc::new(AtomicBool::new(false));
    let replacement_on_drop = Arc::clone(&replacement);
    let lease = WatcherBridgeLease::new(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ScopeId::new("scope-1").expect("scope is valid"),
        RuntimeTurnId::new("turn-1").expect("turn is valid"),
        WatcherBridgeGeneration::initial(),
        WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint is valid"),
        WatcherBridgeBearer::new("bridge-secret-token").expect("bearer is valid"),
    )
    .bind(
        WatcherBridgeToken::new("lease-binding-token").expect("token is valid"),
        move || original_on_drop.store(true, Ordering::SeqCst),
    )
    .bind(
        WatcherBridgeToken::new("forged-binding-token").expect("token is valid"),
        move || replacement_on_drop.store(true, Ordering::SeqCst),
    );
    assert!(
        lease.binding_matches(
            &WatcherBridgeToken::new("lease-binding-token").expect("token is valid")
        )
    );
    assert!(!lease.binding_matches(
        &WatcherBridgeToken::new("forged-binding-token").expect("token is valid")
    ));
    drop(lease);
    assert!(original.load(Ordering::SeqCst));
    assert!(!replacement.load(Ordering::SeqCst));
}

#[test]
fn open_request_keeps_scope_and_turn() {
    let request = WatcherBridgeOpenRequest::new(
        ScopeId::new("scope-1").expect("scope is valid"),
        RuntimeTurnId::new("turn-1").expect("turn is valid"),
    );
    assert_eq!(request.scope().as_str(), "scope-1");
    assert_eq!(request.turn().as_str(), "turn-1");
}

#[test]
fn frozen_empty_state_allows_successful_completion() {
    let blocked = WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Open, vec![]);
    let ready = WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Frozen, Vec::new());
    assert!(!blocked.allows_successful_completion());
    assert!(ready.allows_successful_completion());
}
