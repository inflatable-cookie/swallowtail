#[cfg(test)]
mod tests {
    use super::PiConnection;
    use crate::failure::failure;
    use futures_executor::block_on;
    use serde_json::json;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::{Duration, Instant};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
        MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
        RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
    };

    fn fixture_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
        failure(code, message)
    }

    /// A process whose writes fail loudly: a command that reaches the write must
    /// be visible as a failure instead of hanging on a response that never comes.
    struct FixtureProcess;

    impl ProcessHandle for FixtureProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(fixture_failure(
                    "swallowtail.pi.fixture.write_unexpected",
                    "Fixture process refused a command write",
                ))
            })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(
            &self,
        ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    /// A process that ends its output cleanly and reports a chosen exit.
    struct ExitingProcess {
        success: bool,
    }

    impl ProcessHandle for ExitingProcess {
        fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            FixtureProcess.write_stdin(chunk)
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            FixtureProcess.close_stdin()
        }

        fn read_output(
            &self,
        ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            FixtureProcess.read_output()
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            FixtureProcess.request_stop()
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            FixtureProcess.force_stop()
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            let success = self.success;
            Box::pin(async move { Ok(ProcessExit::new(success, Some(if success { 0 } else { 1 }))) })
        }
    }

    struct FixtureTasks;

    impl ScopedTaskService for FixtureTasks {
        fn spawn(
            &self,
            _scope: ScopeId,
            _task: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            Err(fixture_failure(
                "swallowtail.pi.fixture.task_unsupported",
                "Fixture task service does not spawn",
            ))
        }
    }

    struct FixtureClock;

    impl TimeService for FixtureClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
        }
    }

    fn connection() -> Arc<PiConnection> {
        PiConnection::new(
            Arc::new(FixtureProcess),
            Arc::new(FixtureTasks),
            Arc::new(FixtureClock),
            HostServices::new(ExecutionHostId::new("pi.unit.fixture").expect("host id is valid")),
        )
    }

    struct StubTask {
        failure: Option<&'static str>,
        joined: Arc<AtomicUsize>,
    }

    impl JoinedTask for StubTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let joined = Arc::clone(&self.joined);
            let failure = self.failure;
            Box::pin(async move {
                joined.fetch_add(1, Ordering::SeqCst);
                match failure {
                    Some(code) => Err(fixture_failure(code, "Fixture callback task failed")),
                    None => Ok(()),
                }
            })
        }
    }

    #[test]
    fn a_command_registered_after_close_is_rejected_instead_of_waiting_forever() {
        let connection = connection();
        // Hold the pending lock the pump drains under, so the worker cannot
        // register while we prove it has already passed the fast closed check.
        let pending = connection.pending.lock().expect("Pi pending lock poisoned");
        let worker = {
            let connection = Arc::clone(&connection);
            thread::spawn(move || {
                block_on(connection.command(
                    "race-1".to_owned(),
                    "prompt",
                    json!({"id": "race-1", "type": "prompt", "message": "hello"}),
                ))
            })
        };
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if connection
                .used_ids
                .lock()
                .expect("Pi command id lock poisoned")
                .contains("race-1")
            {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "the worker never reached pending registration"
            );
            thread::sleep(Duration::from_millis(1));
        }
        // The pump marks the connection closed before draining pending work.
        connection.closed.store(true, Ordering::SeqCst);
        drop(pending);
        let result = worker.join().expect("worker thread joins");
        let error = match result {
            Ok(_) => panic!("a command registered after close must be rejected"),
            Err(error) => error,
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.rpc.connection_closed"
        );
        assert!(
            connection
                .pending
                .lock()
                .expect("Pi pending lock poisoned")
                .is_empty(),
            "a rejected command must not leave a pending sender behind"
        );
    }

    #[test]
    fn join_callback_tasks_joins_every_task_after_a_failure() {
        let connection = connection();
        let joined = Arc::new(AtomicUsize::new(0));
        {
            let mut tasks = connection
                .callback_tasks
                .lock()
                .expect("Pi callback-task lock poisoned");
            tasks.push(Box::new(StubTask {
                failure: Some("swallowtail.pi.fixture.callback_failed"),
                joined: Arc::clone(&joined),
            }));
            tasks.push(Box::new(StubTask {
                failure: None,
                joined: Arc::clone(&joined),
            }));
        }
        let outcome = block_on(connection.join_callback_tasks());
        assert!(matches!(outcome, CleanupOutcome::Failed(_)));
        assert_eq!(
            joined.load(Ordering::SeqCst),
            2,
            "one failed join must not strand the remaining callback tasks"
        );
    }

    #[test]
    fn join_callback_tasks_reports_clean_when_every_task_joins() {
        let connection = connection();
        let joined = Arc::new(AtomicUsize::new(0));
        connection
            .callback_tasks
            .lock()
            .expect("Pi callback-task lock poisoned")
            .push(Box::new(StubTask {
                failure: None,
                joined: Arc::clone(&joined),
            }));
        let outcome = block_on(connection.join_callback_tasks());
        assert!(matches!(outcome, CleanupOutcome::Clean));
        assert_eq!(joined.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn join_callback_tasks_is_not_applicable_without_tasks() {
        let connection = connection();
        let outcome = block_on(connection.join_callback_tasks());
        assert!(matches!(outcome, CleanupOutcome::NotApplicable));
    }

    /// Ends its output stream at once, then holds the process wait until the
    /// test releases it, so the pump's shutdown ordering is observable.
    struct BlockedWaitProcess {
        wait_started: Arc<AtomicBool>,
        gate: Mutex<Option<futures_channel::oneshot::Receiver<()>>>,
    }

    impl ProcessHandle for BlockedWaitProcess {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn read_output(
            &self,
        ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            self.wait_started.store(true, Ordering::SeqCst);
            let gate = self.gate.lock().expect("fixture gate lock poisoned").take();
            Box::pin(async move {
                if let Some(gate) = gate {
                    let _ = gate.await;
                }
                Ok(ProcessExit::new(true, Some(0)))
            })
        }
    }

    #[test]
    fn the_pump_closes_and_drains_pending_work_before_it_waits_for_the_process() {
        let wait_started = Arc::new(AtomicBool::new(false));
        let (gate, gate_receiver) = futures_channel::oneshot::channel();
        let process = BlockedWaitProcess {
            wait_started: Arc::clone(&wait_started),
            gate: Mutex::new(Some(gate_receiver)),
        };
        let connection = PiConnection::new(
            Arc::new(process),
            Arc::new(FixtureTasks),
            Arc::new(FixtureClock),
            HostServices::new(
                ExecutionHostId::new("pi.unit.pump-order").expect("host id is valid"),
            ),
        );
        let (sender, _response) = super::response_channel();
        connection
            .pending
            .lock()
            .expect("Pi pending lock poisoned")
            .insert(
                "pending-1".to_owned(),
                super::PendingCommand {
                    command: "state",
                    sender,
                    abort_turn: None,
                },
            );

        let pumped = {
            let connection = Arc::clone(&connection);
            thread::spawn(move || block_on(connection.pump()))
        };
        let deadline = Instant::now() + Duration::from_secs(5);
        while !wait_started.load(Ordering::SeqCst) {
            assert!(
                Instant::now() < deadline,
                "the pump never reached the process wait"
            );
            thread::sleep(Duration::from_millis(1));
        }
        assert!(
            connection.closed.load(Ordering::SeqCst),
            "the pump must close command admission before it waits for the process"
        );
        assert!(
            connection
                .pending
                .lock()
                .expect("Pi pending lock poisoned")
                .is_empty(),
            "the pump must drain pending waiters before it waits for the process"
        );

        let _ = gate.send(());
        pumped.join().expect("pump thread joins");
        assert!(matches!(
            connection.cleanup_outcome(),
            CleanupOutcome::Clean
        ));
    }

    #[test]
    fn a_non_zero_exit_fails_the_rpc_cleanup_like_the_sdk_sidecar() {
        for (success, clean) in [(true, true), (false, false)] {
            let connection = PiConnection::new(
                Arc::new(ExitingProcess { success }),
                Arc::new(FixtureTasks),
                Arc::new(FixtureClock),
                HostServices::new(ExecutionHostId::new("pi.unit.fixture").expect("host id is valid")),
            );

            block_on(Arc::clone(&connection).pump());

            assert_eq!(
                matches!(connection.cleanup_outcome(), CleanupOutcome::Clean),
                clean,
                "exit success {success} must map to clean={clean}"
            );
        }
    }
}
