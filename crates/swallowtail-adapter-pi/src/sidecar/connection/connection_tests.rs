use super::*;
use crate::sidecar::failure::failure;
use crate::sidecar::wire::{PiSdkSidecarCommand, PiSdkSidecarRecord, PiSdkSidecarResponse};
use futures_util::FutureExt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, MonotonicInstant, ProcessExit,
    ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, TimeService,
};

struct RegistrationGateProcess {
    writes: Arc<AtomicUsize>,
}

impl ProcessHandle for RegistrationGateProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.writes.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }
}

#[test]
fn a_pump_failure_between_command_check_and_registration_rejects_the_command() {
    let writes = Arc::new(AtomicUsize::new(0));
    let process = RegistrationGateProcess {
        writes: Arc::clone(&writes),
    };
    let host =
        ExecutionHostId::new("pi.sdk-sidecar.connection-test").expect("fixture host id is valid");
    let connection = SidecarConnection::new(Arc::new(process), HostServices::new(host));
    let gate = Arc::new((Barrier::new(2), Barrier::new(2)));
    *connection
        .send_registration_gate
        .lock()
        .expect("sidecar test send gate lock poisoned") = Some(Arc::clone(&gate));

    // Pause after command's fast closed check. This forces the exact ordering
    // where the pump drains pending commands first. Without the locked recheck,
    // command registers afterward and load waits on a response the stopped pump
    // can never deliver.
    let sender_connection = Arc::clone(&connection);
    let sender = thread::spawn(move || {
        futures_executor::block_on(sender_connection.command(
            "state-raced".to_owned(),
            PiSdkSidecarCommand::State,
            serde_json::json!({}),
        ))
        .map(|_| ())
        .map_err(|error| error.diagnostic().code().to_owned())
    });
    gate.0.wait();

    let error = failure(
        "swallowtail.pi.sdk-sidecar.replay_unexpected",
        "Pi SDK sidecar emitted replay evidence outside a load replay phase",
    );
    connection.fail_connection(&error);
    gate.1.wait();

    assert_eq!(
        sender.join().expect("command registration worker joins"),
        Err("swallowtail.pi.sdk-sidecar.replay_unexpected".to_owned())
    );
    assert_eq!(writes.load(Ordering::SeqCst), 0);
    assert!(
        connection
            .pending
            .lock()
            .expect("sidecar pending lock poisoned")
            .is_empty()
    );
}

/// Records the shutdown stages and never answers anything.
struct SilentProcess {
    stdin_closed: Arc<AtomicBool>,
    stop_requested: Arc<AtomicBool>,
}

impl ProcessHandle for SilentProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop_requested.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }
}

struct ImmediateDeadlineClock;

impl TimeService for ImmediateDeadlineClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

fn silent_connection(
    stdin_closed: &Arc<AtomicBool>,
    stop_requested: &Arc<AtomicBool>,
) -> Arc<SidecarConnection> {
    let process = SilentProcess {
        stdin_closed: Arc::clone(stdin_closed),
        stop_requested: Arc::clone(stop_requested),
    };
    let host =
        ExecutionHostId::new("pi.sdk-sidecar.connection-test").expect("fixture host id is valid");
    SidecarConnection::new(
        Arc::new(process),
        HostServices::new(host).with_time(Arc::new(ImmediateDeadlineClock)),
    )
}

#[test]
fn a_record_that_cannot_be_encoded_leaves_no_pending_slot() {
    let stdin_closed = Arc::new(AtomicBool::new(false));
    let stop_requested = Arc::new(AtomicBool::new(false));
    let connection = silent_connection(&stdin_closed, &stop_requested);
    // One record larger than the protocol's maximum cannot be encoded.
    let oversized = serde_json::json!({"blob": "x".repeat(1024 * 1024 + 1)});

    for index in 0..(MAXIMUM_PENDING_COMMANDS + 4) {
        let result = futures_executor::block_on(connection.command(
            format!("oversized-{index}"),
            PiSdkSidecarCommand::State,
            oversized.clone(),
        ));
        match result {
            Ok(_) => panic!("an unencodable command must not be sent"),
            Err(error) => assert_eq!(
                error.diagnostic().code(),
                "swallowtail.pi.sdk-sidecar.protocol_failed"
            ),
        }
    }

    assert!(
        connection
            .pending
            .lock()
            .expect("sidecar pending lock poisoned")
            .is_empty(),
        "an encoding failure must not consume a bounded pending slot"
    );
}

#[test]
fn a_silent_close_command_finishes_without_waiting_for_eof() {
    let stdin_closed = Arc::new(AtomicBool::new(false));
    let stop_requested = Arc::new(AtomicBool::new(false));
    let connection = silent_connection(&stdin_closed, &stop_requested);

    connection
        .close_session("close:fixture".to_owned())
        .now_or_never()
        .expect("close finishes after the acknowledgement grace expires");
    assert!(stop_requested.load(Ordering::SeqCst));
    assert!(
        stdin_closed.load(Ordering::SeqCst),
        "stdin must close even when the harness never answers close"
    );
    assert!(connection.closed.load(Ordering::SeqCst));

    assert!(connection.pending.lock().expect("pending lock").is_empty());
    let late_close = PiSdkSidecarRecord::Response(PiSdkSidecarResponse {
        id: "close:fixture".to_owned(),
        command: "close".to_owned(),
        success: true,
        data: None,
        failure: None,
    });
    assert!(
        connection
            .dispatch(late_close)
            .expect("late close reply is valid")
            .is_none(),
        "a late acknowledgement cannot complete an abandoned waiter"
    );
}
