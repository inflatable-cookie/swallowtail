use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_runtime::{
    BoxEventStream, CleanupOutcome, ProviderObservation, RuntimeEventKind, TerminalStatus,
    TokenUsage,
};

fn usage_snapshots(events: &mut BoxEventStream) -> Vec<TokenUsage> {
    let mut usage = Vec::new();
    while let Some(event) = block_on(events.next()) {
        let event = event.expect("fixture event stream stays healthy");
        if let RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(snapshot)) =
            event.kind()
        {
            usage.push(*snapshot);
        }
    }
    usage
}

#[test]
fn usage_snapshots_keep_each_turn_independent_across_failure_and_reset() {
    let host = host_id("claude-agent-sdk.fixture.usage-snapshots");
    let fixture = SdkFixtureHost::new(SdkScenario::TurnErrorThenComplete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session =
        block_on(prepared.open_route_session(services.clone())).expect("SDK sidecar session opens");

    let mut failed_turn = block_on(session.start_turn(
        turn_request("turn-1", "first fixture turn"),
        services.clone(),
    ))
    .expect("first SDK turn starts");
    let mut failed_events = failed_turn
        .take_events()
        .expect("failed turn exposes events");
    assert_eq!(
        usage_snapshots(&mut failed_events),
        vec![TokenUsage::new(Some(10), Some(4)).with_cache_tokens(Some(2), Some(3))]
    );
    let failed = block_on(
        failed_turn
            .take_terminal_outcome()
            .expect("failed turn terminal outcome exists"),
    );
    assert!(matches!(failed.status(), TerminalStatus::ProviderFailed(_)));
    let _ = block_on(failed_turn.close());

    let mut next_turn =
        block_on(session.start_turn(turn_request("turn-2", "second fixture turn"), services))
            .expect("session remains usable after a failed result");
    let mut next_events = next_turn.take_events().expect("next turn exposes events");
    assert_eq!(
        usage_snapshots(&mut next_events),
        vec![TokenUsage::new(Some(20), Some(8)).with_cache_tokens(Some(4), Some(6))]
    );
    let completed = block_on(
        next_turn
            .take_terminal_outcome()
            .expect("next turn terminal outcome exists"),
    );
    assert_eq!(completed.status(), &TerminalStatus::Completed);
    let _ = block_on(next_turn.close());
    let outcome = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    let CleanupOutcome::Degraded(diagnostic) = outcome else {
        panic!("unattested fixture tree closes degraded: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_root_only_degraded"
    );
}

#[test]
fn usage_duplicate_result_after_turn_end_is_not_correlated_or_counted_twice() {
    let host = host_id("claude-agent-sdk.fixture.usage-duplicate");
    let fixture = SdkFixtureHost::new(SdkScenario::DuplicateTurnEnd);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session =
        block_on(prepared.open_route_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn =
        block_on(session.start_turn(turn_request("turn-1", "duplicate result fixture"), services))
            .expect("SDK turn starts");
    let mut events = turn.take_events().expect("turn exposes events");
    assert_eq!(
        usage_snapshots(&mut events),
        vec![TokenUsage::new(Some(10), Some(4)).with_cache_tokens(Some(2), Some(3))]
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("turn terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    let _ = block_on(turn.close());
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}
