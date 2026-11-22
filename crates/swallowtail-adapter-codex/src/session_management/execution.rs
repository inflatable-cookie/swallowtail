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
                let _ = connection.cancel_session().await;
                let _ = task.join().await;
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
    match connection.cancel_session().await {
        Ok(()) => {
            if task.join().await.is_err() {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.lifecycle.cancel_cleanup_failed",
                    "Codex lifecycle cancellation cleanup failed",
                ))
            } else {
                connection.cleanup_outcome()
            }
        }
        Err(_) => {
            // If `force_stop` was refused while stdout is still open, joining
            // unconditionally would hang forever. Fall back to bounded close
            // (stdin EOF + escalation) and still report cancellation failure.
            let _ = close_connection(connection, task, budget).await;
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.codex.lifecycle.cancel_cleanup_failed",
                "Codex lifecycle cancellation cleanup failed",
            ))
        }
    }
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
        force_stops: AtomicUsize,
        stdin_closes: AtomicUsize,
        eof: AtomicBool,
        waker: Mutex<Option<Waker>>,
    }

    impl CancelTestProcess {
        fn new(refuse_force_stop: bool) -> Self {
            Self {
                refuse_force_stop,
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
            self.signal_eof();
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

    struct ThreadJoined(std::thread::JoinHandle<()>);

    impl JoinedTask for ThreadJoined {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move {
                self.0.join().map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new("test.panicked", "task panicked"))
                })
            })
        }
    }

    impl ScopedTaskService for ThreadTaskService {
        fn spawn(
            &self,
            _scope: ScopeId,
            task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Ok(Box::new(ThreadJoined(std::thread::spawn(move || {
                futures_executor::block_on(task);
            }))))
        }
    }

    #[test]
    fn successful_cancel_stops_once_without_closing_stdin() {
        let host = ExecutionHostId::new("codex.lifecycle.cancel-ok").expect("host id");
        let services = HostServices::new(host);
        let process = Arc::new(CancelTestProcess::new(false));
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
    fn refused_force_stop_falls_back_to_bounded_close_and_reports_failure() {
        let host = ExecutionHostId::new("codex.lifecycle.cancel-refused").expect("host id");
        let services = HostServices::new(host);
        let process = Arc::new(CancelTestProcess::new(true));
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
            "fallback must close stdin so the still-running pump can exit"
        );
    }
}
