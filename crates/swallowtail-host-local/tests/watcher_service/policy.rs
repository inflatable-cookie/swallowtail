use super::support::block_on;
use super::{
    default_watcher_host, operation_data, runtime_turn, watcher_host, watcher_owning_turn,
};
use swallowtail_core::{WatcherLifecyclePhase, WatcherRequester, WatcherTerminalCause};
use swallowtail_host_local::LocalProcessLimits;
use swallowtail_runtime::{WatcherWaitOptions, WatcherWaitRepresentation};

#[test]
fn default_composition_starts_an_approved_process_without_an_injected_backend() {
    let local = watcher_host("exit-zero", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("default composition registers the watcher service");
    let owning_turn = watcher_owning_turn("turn-default-start");
    let accepted = block_on(watcher.accept_start(
        runtime_turn("turn-default-start"),
        WatcherRequester::Model,
        operation_data("exit-zero-operation"),
    ))
    .expect("default host-local composition starts an approved process-backed watcher");
    assert_eq!(accepted.phase(), WatcherLifecyclePhase::Running);
    assert_eq!(
        block_on(watcher.wait(
            owning_turn,
            accepted.watcher_id().clone(),
            WatcherWaitOptions::default(),
        ))
        .expect("default composition joins through the owned process handle"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Completed)
    );
}

#[test]
fn default_composition_without_approval_rejects_before_work() {
    let local = default_watcher_host(2);
    let watcher = local
        .services()
        .watcher()
        .expect("default composition still registers the watcher service");
    let failure = block_on(watcher.accept_start(
        runtime_turn("turn-unapproved-default"),
        WatcherRequester::Model,
        operation_data("any-operation"),
    ))
    .expect_err("unapproved process-backed start must reject before work");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.operation_not_approved"
    );
}

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

    let accepted =
        block_on(watcher.accept_start(turn.clone(), WatcherRequester::Model, operation.clone()))
            .expect("host-approved watcher starts");
    assert_eq!(accepted.phase(), WatcherLifecyclePhase::Running);
    assert_eq!(accepted.accepted_by(), WatcherRequester::Model);
    assert!(accepted.summary().is_none());
    assert!(!format!("{accepted:?}").contains(operation.as_str()));

    let failure = block_on(watcher.finalize_turn(turn.clone()))
        .expect_err("finalization requires joined watcher truth");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_not_joined"
    );

    let wait = block_on(watcher.wait(
        owning_turn.clone(),
        accepted.watcher_id().clone(),
        WatcherWaitOptions::default(),
    ))
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
    assert_eq!(
        block_on(watcher.finalize_turn(turn)).expect("joined successful turn retires explicitly"),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let failure = block_on(watcher.list(watcher_owning_turn("turn-complete")))
        .expect_err("retired turn rejects stale list controls");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_retired"
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
        block_on(watcher.wait(
            owning_turn.clone(),
            watcher_id.clone(),
            WatcherWaitOptions::default(),
        ))
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

#[test]
fn non_zero_root_exit_is_failed_and_joined() {
    let local = watcher_host("exit-one", 2);
    let watcher = local
        .services()
        .watcher()
        .expect("local composition includes watcher");
    let owning_turn = watcher_owning_turn("turn-exit-one");
    let watcher_id = block_on(watcher.accept_start(
        runtime_turn("turn-exit-one"),
        WatcherRequester::Model,
        operation_data("exit-one-operation"),
    ))
    .expect("non-zero watcher starts")
    .watcher_id()
    .clone();

    assert_eq!(
        block_on(watcher.wait(
            owning_turn.clone(),
            watcher_id.clone(),
            WatcherWaitOptions::default()
        ))
        .expect("non-zero root still reaches joined cleanup"),
        WatcherWaitRepresentation::Satisfied(WatcherTerminalCause::Failed)
    );
    let failed = block_on(watcher.inspect(owning_turn, watcher_id))
        .expect("failed watcher remains inspectable");
    assert_eq!(failed.phase(), WatcherLifecyclePhase::Joined);
    assert_eq!(
        failed
            .summary()
            .expect("host records failed process result")
            .as_str(),
        "failed"
    );
}
