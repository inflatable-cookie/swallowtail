use super::*;
use crate::sidecar::failure::failure;
use crate::sidecar::wire::PiSdkSidecarCommand;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, HostServices, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    RuntimeFailure,
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
