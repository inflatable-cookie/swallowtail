use super::support::block_on;
use super::{operation_data, runtime_turn, watcher_host, watcher_owning_turn};
use swallowtail_core::{WatcherLifecyclePhase, WatcherRequester, WatcherTerminalCause};
use swallowtail_host_local::LocalProcessLimits;
use swallowtail_runtime::WatcherWaitRepresentation;

#[test]
fn watcher_registration_and_unapproved_start_do_no_work() {
    let local = watcher_host("exit-zero", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let operation = operation_data("unapproved-operation");
    let turn = runtime_turn("turn-unapproved");
    let owning_turn = watcher_owning_turn("turn-unapproved");

    let failure = block_on(watcher.accept_start(turn, WatcherRequester::Model, operation))
        .expect_err("unapproved watcher operation must be rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.operation_not_approved"
    );
    let failure = block_on(watcher.list(owning_turn)).expect_err("rejected start leaves no turn");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_not_found"
    );
}

#[test]
fn watcher_start_is_host_bound_and_wait_requires_joined_truth() {
    let local = watcher_host("exit-zero", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let operation = operation_data("exit-zero-operation");
    let turn = runtime_turn("turn-complete");
    let owning_turn = watcher_owning_turn("turn-complete");

    let accepted = block_on(watcher.accept_start(turn, WatcherRequester::Model, operation.clone()))
        .expect("host-approved watcher starts");
    assert_eq!(accepted.phase(), WatcherLifecyclePhase::Running);
    assert_eq!(accepted.accepted_by(), WatcherRequester::Model);
    assert!(accepted.summary().is_none());
    assert!(!format!("{accepted:?}").contains(operation.as_str()));

    let wait = block_on(watcher.wait(owning_turn.clone(), accepted.watcher_id().clone()))
        .expect("watcher waits until terminal and joined");
    assert_eq!(
        wait,
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Completed)
    );
    let joined = block_on(watcher.inspect(owning_turn, accepted.watcher_id().clone()))
        .expect("joined watcher remains inspectable");
    assert_eq!(joined.phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        joined
            .summary()
            .expect("host selects terminal summary")
            .as_str(),
        "completed"
    );
}

#[test]
fn watcher_output_overflow_is_bounded_and_becomes_host_failure() {
    let local = super::watcher_host_with_limits(
        "overflow",
        2,
        LocalProcessLimits::new(8, 1024, 64, 16, 1024),
    );
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let owning_turn = watcher_owning_turn("turn-overflow");
    let watcher_id = block_on(watcher.accept_start(
        runtime_turn("turn-overflow"),
        WatcherRequester::Model,
        operation_data("overflow-operation"),
    ))
    .expect("overflow watcher starts")
    .watcher_id()
    .clone();

    assert_eq!(
        block_on(watcher.wait(owning_turn.clone(), watcher_id.clone()))
            .expect("bounded output watcher joins"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Failed)
    );
    let failed = block_on(watcher.inspect(owning_turn, watcher_id))
        .expect("failed watcher remains inspectable");
    assert_eq!(failed.phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        failed
            .summary()
            .expect("host records bounded output failure")
            .as_str(),
        "output_limit_exceeded"
    );
}
