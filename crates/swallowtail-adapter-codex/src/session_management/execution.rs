use super::protocol::{
    failure, lifecycle_assessment, lifecycle_request, notification_method, outcome,
    validate_lifecycle_response, with_cleanup,
};
use crate::CodexAppServerDriver;
use crate::app_server::{close_connection, scope};
use crate::control_wait::{Controlled, wait_controlled};
use crate::process_close::{AppServerTask, CloseBudget, close_budget};
use crate::rpc::RpcConnection;
use std::sync::Arc;
use swallowtail_core::{
    ProviderSessionEffectTruth, ProviderSessionManagementAction, ProviderSessionManagementEffect,
    SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ImmediateCancellation,
    ProviderSessionManagementAgreement, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RuntimeFailure,
};

impl CodexAppServerDriver {
    pub(super) async fn manage(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &ProviderSessionManagementAgreement,
        cancellation: &ImmediateCancellation,
        request_id: &str,
        services: HostServices,
    ) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
        let action = agreement.action();
        let preflight = plan.preflight();
        let app_behavior = self.validate_plan(preflight)?;
        let assessment = lifecycle_assessment(preflight)?;
        if !assessment.behavior.supports(action) {
            return Err(failure(
                "swallowtail.codex.lifecycle.action_unsupported",
                "Prepared Codex version does not support this thread lifecycle action",
            ));
        }
        if matches!(action, ProviderSessionManagementAction::Delete(_))
            && assessment.behavior.delete_action() != Some(action)
        {
            return Err(failure(
                "swallowtail.codex.lifecycle.deletion_strength_mismatch",
                "Codex management plan requests a different deletion strength",
            ));
        }
        if agreement.affected_scope() != assessment.behavior.affected_scope(action) {
            return Err(failure(
                "swallowtail.codex.lifecycle.affected_scope_mismatch",
                "Codex management plan requests a different affected scope",
            ));
        }
        if cancelled_or_expired(agreement, cancellation, &services)? {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }

        let request_id = swallowtail_runtime::RequestId::new(request_id)
            .expect("validated request id remains valid");
        let (connection, task) = self
            .spawn_connection(
                preflight,
                app_behavior,
                scope("management", &request_id),
                agreement.binding().working_resource().cloned(),
                &services,
            )
            .await?;
        let initialization = wait_controlled(
            connection.initialize(false),
            cancellation,
            deadline_wait(agreement, &services)?,
        )
        .await;
        match initialization {
            Controlled::Completed(Ok(())) => {}
            Controlled::Completed(Err(error)) => {
                let _ = cancel_connection(
                    &connection,
                    task,
                    close_budget(&services, agreement.deadline()),
                )
                .await;
                return Err(error);
            }
            Controlled::Cancelled | Controlled::Deadline => {
                let cleanup = cancel_connection(
                    &connection,
                    task,
                    close_budget(&services, agreement.deadline()),
                )
                .await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::failed_before_effect(action),
                    ),
                    cleanup,
                ));
            }
        }
        if cancelled_or_expired(agreement, cancellation, &services)? {
            let cleanup = close_connection(
                &connection,
                task,
                close_budget(&services, agreement.deadline()),
            )
            .await;
            return Ok(with_cleanup(
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ),
                cleanup,
            ));
        }

        let target = agreement
            .binding()
            .provider_session_ref()
            .as_provider_value();
        let (method, params) = lifecycle_request(action, target);
        let response = match connection.dispatch_request(method, params).await {
            Ok(response) => response,
            Err(error) => {
                let cleanup = close_connection(
                    &connection,
                    task,
                    close_budget(&services, agreement.deadline()),
                )
                .await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::failed_before_effect(action),
                    )
                    .with_diagnostic(error.diagnostic().clone()),
                    cleanup,
                ));
            }
        };

        let response =
            wait_controlled(response, cancellation, deadline_wait(agreement, &services)?).await;
        let result = match response {
            Controlled::Cancelled | Controlled::Deadline => {
                let cleanup = cancel_connection(
                    &connection,
                    task,
                    close_budget(&services, agreement.deadline()),
                )
                .await;
                return Ok(with_cleanup(
                    outcome(
                        agreement,
                        ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                    ),
                    cleanup,
                ));
            }
            Controlled::Completed(result) => result,
        };

        let cleanup = close_connection(
            &connection,
            task,
            close_budget(&services, agreement.deadline()),
        )
        .await;
        let mut result = match result {
            Ok(response) => match validate_lifecycle_response(action, target, &response) {
                Ok(()) => outcome(
                    agreement,
                    ProviderSessionManagementEffect::applied(
                        action,
                        assessment.behavior.affected_scope(action),
                    ),
                ),
                Err(error) => outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone()),
            },
            Err(error)
                if error.diagnostic().code() == "swallowtail.codex.app_server.request_failed" =>
            {
                outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone())
            }
            Err(error) => outcome(
                agreement,
                ProviderSessionManagementEffect::unconfirmed_after_effect(action),
            )
            .with_diagnostic(error.diagnostic().clone()),
        };
        if result.effect().truth() == ProviderSessionEffectTruth::Applied
            && assessment.behavior.expects_notification(action)
            && !connection
                .lifecycle_notifications()
                .iter()
                .any(|notification| notification.matches(notification_method(action), target))
        {
            result = result.with_diagnostic(SafeDiagnostic::new(
                "swallowtail.codex.lifecycle.notification_disagreement",
                "Codex lifecycle response succeeded without the expected matching notification",
            ));
        }
        Ok(with_cleanup(result, cleanup))
    }
}

async fn cancel_connection(
    connection: &Arc<RpcConnection>,
    task: AppServerTask,
    budget: CloseBudget,
) -> CleanupOutcome {
    crate::app_server::cancel_connection(
        connection,
        task,
        budget,
        "swallowtail.codex.lifecycle.cancel_cleanup_failed",
        "Codex lifecycle cancellation cleanup failed",
    )
    .await
}

fn cancelled_or_expired(
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
) -> Result<bool, RuntimeFailure> {
    if cancellation.is_requested() {
        return Ok(true);
    }
    let Some(deadline) = agreement.deadline() else {
        return Ok(false);
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.codex.lifecycle.time_service_missing",
            "Deadline-bound Codex lifecycle management requires a time service",
        )
    })?;
    Ok(time.now() >= deadline.instant())
}

fn deadline_wait(
    agreement: &ProviderSessionManagementAgreement,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>, RuntimeFailure> {
    agreement
        .deadline()
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.codex.lifecycle.time_service_missing",
                        "Deadline-bound Codex lifecycle management requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::cancel_connection;
    use crate::process_close::{AppServerTask, close_budget};
    use crate::rpc::RpcConnection;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Poll, Waker};
    use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, HostServices, JoinedTask, ProcessExit, ProcessHandle,
        ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, ScopeId, ScopedTaskService,
    };

    struct CancelTestProcess {
        refuse_force_stop: bool,
        close_stdin_signals_eof: bool,
        force_stops: AtomicUsize,
        stdin_closes: AtomicUsize,
        eof: AtomicBool,
        waker: Mutex<Option<Waker>>,
    }

    impl CancelTestProcess {
        fn new(refuse_force_stop: bool, close_stdin_signals_eof: bool) -> Self {
            Self {
                refuse_force_stop,
                close_stdin_signals_eof,
                force_stops: AtomicUsize::new(0),
                stdin_closes: AtomicUsize::new(0),
                eof: AtomicBool::new(false),
                waker: Mutex::new(None),
            }
        }

        fn signal_eof(&self) {
            self.eof.store(true, Ordering::SeqCst);
            if let Some(waker) = self.waker.lock().expect("waker lock").take() {
                waker.wake();
            }
        }
    }

    impl ProcessHandle for CancelTestProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.stdin_closes.fetch_add(1, Ordering::SeqCst);
            if self.close_stdin_signals_eof {
                self.signal_eof();
            }
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(std::future::poll_fn(|context| {
                if self.eof.load(Ordering::SeqCst) {
                    Poll::Ready(Ok(None))
                } else {
                    *self.waker.lock().expect("waker lock") = Some(context.waker().clone());
                    Poll::Pending
                }
            }))
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.force_stops.fetch_add(1, Ordering::SeqCst);
            if self.refuse_force_stop {
                Box::pin(async {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "test.force_stop_refused",
                        "force stop refused",
                    )))
                })
            } else {
                self.signal_eof();
                Box::pin(async { Ok(()) })
            }
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct ThreadTaskService;

    /// Models `LocalJoinedTask::drop`: synchronously joins the worker thread on
    /// drop if it was not explicitly joined, so dropping an unfinished pump
    /// would block forever instead of detaching.
    struct JoinOnDropThread(Option<std::thread::JoinHandle<()>>);

    impl Drop for JoinOnDropThread {
        fn drop(&mut self) {
            if let Some(handle) = self.0.take() {
                let _ = handle.join();
            }
        }
    }

    impl JoinedTask for JoinOnDropThread {
        fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let handle = self.0.take();
            Box::pin(async move {
                if let Some(handle) = handle {
                    handle.join().map_err(|_| {
                        RuntimeFailure::new(SafeDiagnostic::new("test.panicked", "task panicked"))
                    })?;
                }
                Ok(())
            })
        }
    }

    impl ScopedTaskService for ThreadTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Ok(Box::new(JoinOnDropThread(Some(std::thread::spawn(
                move || {
                    futures_executor::block_on(task);
                },
            )))))
        }
    }

    #[test]
    fn successful_cancel_stops_once_without_closing_stdin() {
        let host = ExecutionHostId::new("codex.lifecycle.cancel-ok").expect("host id");
        let services = HostServices::new(host);
        let process = Arc::new(CancelTestProcess::new(false, true));
        let connection = RpcConnection::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services.clone(),
        );
        let pump_connection = Arc::clone(&connection);
        let task = AppServerTask::spawn(
            Arc::new(ThreadTaskService),
            ScopeId::new("codex.lifecycle.cancel-ok").expect("scope"),
            Box::pin(async move { pump_connection.pump().await }),
        )
        .expect("task spawns");

        let cleanup = futures_executor::block_on(cancel_connection(
            &connection,
            task,
            close_budget(&services, None),
        ));
        assert_eq!(cleanup, CleanupOutcome::Clean);
        assert_eq!(process.force_stops.load(Ordering::SeqCst), 1);
        assert_eq!(process.stdin_closes.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn refused_force_stop_with_open_stdout_returns_failure_without_hanging() {
        let host = ExecutionHostId::new("codex.lifecycle.cancel-refused").expect("host id");
        let services = HostServices::new(host);
        // Keep stdout open even after `close_stdin` and refused `force_stop`
        // to prove the fallback does not unconditionally block on `task.join()`.
        let process = Arc::new(CancelTestProcess::new(true, false));
        let connection = RpcConnection::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services.clone(),
        );
        let pump_connection = Arc::clone(&connection);
        let task = AppServerTask::spawn(
            Arc::new(ThreadTaskService),
            ScopeId::new("codex.lifecycle.cancel-refused").expect("scope"),
            Box::pin(async move { pump_connection.pump().await }),
        )
        .expect("task spawns");

        let cleanup = futures_executor::block_on(cancel_connection(
            &connection,
            task,
            close_budget(&services, None),
        ));
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diagnostic) if diagnostic.code() == "swallowtail.codex.lifecycle.cancel_cleanup_failed"),
            "expected cancel_cleanup_failed, got {cleanup:?}"
        );
        assert_eq!(
            process.stdin_closes.load(Ordering::SeqCst),
            1,
            "fallback attempts stdin close before escalating"
        );
        // Note: stdout was never signalled EOF and `JoinOnDropThread` joins on
        // drop, proving the pump was aborted and joined cleanly before return.
    }

    struct PendingDispatchWriteProcess {
        initial_line: Mutex<Option<Vec<u8>>>,
        write_entered: Mutex<Option<futures_channel::oneshot::Sender<()>>>,
    }

    impl ProcessHandle for PendingDispatchWriteProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            if let Some(entered) = self
                .write_entered
                .lock()
                .expect("write_entered lock")
                .take()
            {
                let _ = entered.send(());
            }
            Box::pin(std::future::pending())
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            if let Some(line) = self.initial_line.lock().expect("initial_line lock").take() {
                Box::pin(async move {
                    Ok(Some(ProcessOutputChunk::new(
                        swallowtail_runtime::ProcessOutputStream::Stdout,
                        line,
                    )))
                })
            } else {
                Box::pin(std::future::pending())
            }
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.force_stop_refused",
                    "force stop refused",
                )))
            })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    #[test]
    fn refused_force_stop_aborts_and_joins_pump_stuck_in_dispatch_write() {
        let host = ExecutionHostId::new("codex.lifecycle.cancel-dispatch-write").expect("host id");
        let services = HostServices::new(host);
        let (entered_tx, entered_rx) = futures_channel::oneshot::channel();
        // Deliver a server request with no active turn so the pump calls
        // `reject_server_request` -> `write_stdin`, which remains pending.
        let process = Arc::new(PendingDispatchWriteProcess {
            initial_line: Mutex::new(Some(
                br#"{"id":99,"method":"item/tool/call","params":{}}"#
                    .iter()
                    .copied()
                    .chain(std::iter::once(b'\n'))
                    .collect(),
            )),
            write_entered: Mutex::new(Some(entered_tx)),
        });
        let connection = RpcConnection::new(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            services.clone(),
        );
        let pump_connection = Arc::clone(&connection);
        let task = AppServerTask::spawn(
            Arc::new(ThreadTaskService),
            ScopeId::new("codex.lifecycle.cancel-dispatch-write").expect("scope"),
            Box::pin(async move { pump_connection.pump().await }),
        )
        .expect("task spawns");

        // Wait until the pump is actually blocked inside `write_stdin`.
        futures_executor::block_on(entered_rx).expect("pump entered pending dispatch write");

        let cleanup = futures_executor::block_on(cancel_connection(
            &connection,
            task,
            close_budget(&services, None),
        ));
        assert!(
            matches!(cleanup, CleanupOutcome::Failed(ref diagnostic) if diagnostic.code() == "swallowtail.codex.lifecycle.cancel_cleanup_failed"),
            "expected cancel_cleanup_failed, got {cleanup:?}"
        );
    }
}
