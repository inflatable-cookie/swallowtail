mod containment;
mod wakeups;

use super::{LocalWatcherHostService, LocalWatcherState, MAX_RETIRED_TURNS};
use crate::host::LocalProcessHost;
use crate::task::LocalScopedTaskService;
use containment::TestContainmentBackend;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{
    ExecutionHostId, SafeDiagnostic, WatcherOperationData, WatcherOwningTurn, WatcherRequester,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, ProcessRequest, RuntimeFailure, RuntimeTurnId, ScopeId,
    ScopedTaskService, WatcherHostService,
};

struct FailFirstTaskService {
    failed: AtomicBool,
    delegate: LocalScopedTaskService,
}

impl ScopedTaskService for FailFirstTaskService {
    fn spawn(
        &self,
        scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn swallowtail_runtime::JoinedTask>, RuntimeFailure> {
        if !self.failed.swap(true, Ordering::AcqRel) {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.task_spawn_failed",
                "Fixture task service rejected one spawn",
            )))
        } else {
            self.delegate.spawn(scope, task)
        }
    }
}

#[test]
fn monitor_spawn_failure_rolls_back_without_a_phantom_watcher() {
    let execution_host =
        ExecutionHostId::new("fixture.watcher.rollback").expect("host id is valid");
    let executable =
        swallowtail_runtime::ExecutableRef::new("fixture.sleep").expect("executable is valid");
    let operation =
        WatcherOperationData::new("fixture.sleep.operation").expect("operation is valid");
    let request = ProcessRequest::new(executable.clone()).with_arguments(["30".to_owned()]);
    let exit_operation =
        WatcherOperationData::new("fixture.watcher.rollback.exit").expect("operation is valid");
    let exit_request = ProcessRequest::new(executable.clone()).with_arguments(["0".to_owned()]);
    let process_host = Arc::new(
        LocalProcessHost::builder(crate::limits::LocalProcessLimits::default())
            .approve_executable(executable, "/bin/sleep")
            .approve_watcher_operation(operation.clone(), request)
            .approve_watcher_operation(exit_operation.clone(), exit_request)
            .with_process_containment_factory(|host| Arc::new(TestContainmentBackend::new(host)))
            .build(),
    );
    let task_service = Arc::new(FailFirstTaskService {
        failed: AtomicBool::new(false),
        delegate: LocalScopedTaskService::new(execution_host),
    });
    let containment = process_host.process_containment().cloned();
    let watcher =
        LocalWatcherHostService::new_with_task_service(process_host, task_service, 2, containment);
    let turn = RuntimeTurnId::new("fixture.watcher.rollback.turn").expect("turn is valid");
    let owning_turn = WatcherOwningTurn::new(turn.as_str()).expect("owning turn is valid");

    let failure = futures_executor::block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation.clone(),
    ))
    .expect_err("first monitor spawn must fail");
    assert_eq!(failure.diagnostic().code(), "fixture.task_spawn_failed");
    let failure = futures_executor::block_on(watcher.list(owning_turn.clone()))
        .expect_err("failed start must leave no phantom turn");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_watcher.turn_not_found"
    );

    let accepted = futures_executor::block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Operator,
        exit_operation,
    ))
    .expect("the next start can reuse the rolled-back turn");
    assert!(accepted.watcher_id().as_str().contains("watcher-2-1"));
    assert_eq!(
        futures_executor::block_on(watcher.wait(
            owning_turn,
            accepted.watcher_id().clone(),
            swallowtail_runtime::WatcherWaitOptions::default(),
        ))
        .expect("subsequent watcher joins"),
        swallowtail_runtime::WatcherWaitRepresentation::Satisfied(
            swallowtail_core::WatcherTerminalCause::Completed,
        )
    );
    assert_eq!(
        futures_executor::block_on(watcher.finalize_turn(turn))
            .expect("subsequent turn finalization succeeds"),
        CleanupOutcome::Clean
    );
}

#[test]
fn retired_turn_tombstones_are_bounded() {
    let mut state = LocalWatcherState::default();
    for index in 0..(MAX_RETIRED_TURNS + 8) {
        let turn =
            RuntimeTurnId::new(format!("fixture.retired.{index}")).expect("retired turn is valid");
        state.retire(&turn);
    }
    assert_eq!(state.retired.len(), MAX_RETIRED_TURNS);
}

#[test]
fn monitor_spawn_rollback_preserves_containment_cleanup_failure() {
    let execution_host =
        ExecutionHostId::new("fixture.watcher.rollback.cleanup").expect("host id is valid");
    let executable =
        swallowtail_runtime::ExecutableRef::new("fixture.sleep").expect("executable is valid");
    let operation =
        WatcherOperationData::new("fixture.sleep.operation").expect("operation is valid");
    let request = ProcessRequest::new(executable.clone()).with_arguments(["30".to_owned()]);
    let backend_slot = Arc::new(std::sync::Mutex::new(None));
    let slot = Arc::clone(&backend_slot);
    let process_host = Arc::new(
        LocalProcessHost::builder(crate::limits::LocalProcessLimits::default())
            .approve_executable(executable, "/bin/sleep")
            .approve_watcher_operation(operation.clone(), request)
            .with_process_containment_factory(move |host| {
                let backend = Arc::new(TestContainmentBackend::new(host));
                backend.fail_force_stop();
                *slot.lock().expect("backend slot poisoned") = Some(Arc::clone(&backend));
                backend
            })
            .build(),
    );
    let task_service = Arc::new(FailFirstTaskService {
        failed: AtomicBool::new(false),
        delegate: LocalScopedTaskService::new(execution_host),
    });
    let containment = process_host.process_containment().cloned();
    let watcher =
        LocalWatcherHostService::new_with_task_service(process_host, task_service, 2, containment);
    let turn = RuntimeTurnId::new("fixture.watcher.rollback.cleanup.turn").expect("turn is valid");
    let owning_turn = WatcherOwningTurn::new(turn.as_str()).expect("owning turn is valid");

    let failure = futures_executor::block_on(watcher.accept_start(
        turn.clone(),
        WatcherRequester::Model,
        operation,
    ))
    .expect_err("failed containment cleanup must surface");
    assert_eq!(
        failure.diagnostic().code(),
        "fixture.containment.force_stop_failed"
    );
    let listed = futures_executor::block_on(watcher.list(owning_turn.clone()))
        .expect("failed cleanup retains the watcher identity");
    assert_eq!(listed.len(), 1);
    let stop = futures_executor::block_on(
        watcher.stop_and_join_all(turn, swallowtail_core::WatcherCleanupCause::Cancelled),
    )
    .expect("retained identity remains reachable for cleanup");
    assert!(matches!(stop.1, CleanupOutcome::Failed(_)));
    let backend = backend_slot
        .lock()
        .expect("backend slot poisoned")
        .clone()
        .expect("factory installed backend");
    assert!(
        backend.calls().contains(&"lease.force_stop"),
        "rollback must attempt lease force-stop: {:?}",
        backend.calls()
    );
}
