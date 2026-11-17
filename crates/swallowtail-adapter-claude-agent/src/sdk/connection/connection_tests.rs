use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{BoxFuture, ProcessExit, ProcessOutputChunk, RuntimeFailure};

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
        ExecutionHostId::new("claude-agent-sdk.connection-test").expect("fixture host id is valid");
    let connection = SdkConnection::new(Arc::new(process), HostServices::new(host));
    let gate = Arc::new((Barrier::new(2), Barrier::new(2)));
    *connection
        .send_registration_gate
        .lock()
        .expect("SDK test send gate lock poisoned") = Some(Arc::clone(&gate));

    // Pause after send's fast closed check. This forces the exact ordering
    // where the pump drains pending commands first. Without the locked recheck,
    // send registers afterward and close waits on a response the stopped pump
    // can never deliver while the fixture clock remains unadvanced.
    let sender_connection = Arc::clone(&connection);
    let sender = thread::spawn(move || {
        futures_executor::block_on(sender_connection.send(
            "close:raced".to_owned(),
            ClaudeAgentSdkCommand::Close,
            serde_json::json!({}),
        ))
        .map(|_| ())
        .map_err(|error| error.diagnostic().code().to_owned())
    });
    gate.0.wait();

    let error = RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.claude-agent.sdk.event_without_turn",
        "Claude Agent SDK sidecar emitted a turn event outside an active turn",
    ));
    connection.fail_connection(&error);
    gate.1.wait();

    assert_eq!(
        sender.join().expect("command registration worker joins"),
        Err("swallowtail.claude-agent.sdk.event_without_turn".to_owned())
    );
    assert_eq!(writes.load(Ordering::SeqCst), 0);
    assert!(
        connection
            .pending
            .lock()
            .expect("SDK sidecar pending lock poisoned")
            .is_empty()
    );
}

#[test]
fn deadline_recipient_retains_until_drain_and_interrupt_command_settle() {
    use crate::sdk::wire::ClaudeAgentSdkEvent;
    for drain_first in [true, false] {
        let connection = SdkConnection::new(
            Arc::new(RegistrationGateProcess {
                writes: Arc::new(AtomicUsize::new(0)),
            }),
            HostServices::new(ExecutionHostId::new("sdk.deadline-race").expect("host")),
        );
        let make = |id| {
            SdkActiveTurn::new(
                swallowtail_runtime::RuntimeTurnId::new(id).expect("id"),
                Arc::downgrade(&connection),
                None,
                crate::sdk::profile::ClaudeAgentSdkSessionProfile::default(),
                Vec::new(),
            )
            .expect("turn")
        };
        let (old, _events, _callbacks, terminal) = make("old");
        let (next, _next_events, _next_callbacks, _next_terminal) = make("next");
        connection
            .set_active_turn(old.clone())
            .expect("old recipient");
        assert!(!connection.rollback_pre_query_turn(&next), "identity must match");
        assert!(connection.expire_turn(&old));
        assert!(
            !connection.rollback_pre_query_turn(&old),
            "pre-query rollback cannot discard a timed-out recipient"
        );
        assert_eq!(
            futures_executor::block_on(terminal).status(),
            &swallowtail_runtime::TerminalStatus::TimedOut
        );
        let abort = || ClaudeAgentSdkEvent::TurnEnded {
            usage: crate::sdk::wire::ClaudeAgentSdkUsage {
                input_tokens: 0,
                output_tokens: 0,
                cache_read_input_tokens: None,
                cache_write_input_tokens: None,
            },
            stop_reason: "error_during_execution".into(),
            failed: true,
            subtype: None,
            num_turns: None,
            duration_ms: None,
            error_text_present: true,
            error_text_type: "array".into(),
            result_field_presence: BTreeMap::new(),
            api_error_status: None,
            terminal_reason: Some("aborted_streaming".into()),
            rate_limit_status: None,
        };
        if drain_first {
            connection
                .dispatch_turn_event(abort())
                .expect("exact drain");
        } else {
            old.finish_deadline_interrupt();
        }
        connection.clear_active_turn(&old);
        assert!(
            connection.set_active_turn(next.clone()).is_err(),
            "one half alone cannot release"
        );
        if drain_first {
            old.finish_deadline_interrupt();
        } else {
            connection
                .dispatch_turn_event(abort())
                .expect("exact drain");
        }
        connection
            .set_active_turn(next.clone())
            .expect("both halves permit replacement");
        assert!(
            !connection.expire_turn(&old),
            "late deadline claim cannot mutate old or next recipient"
        );
        assert!(!next.is_finished());
        assert_eq!(
            connection
                .dispatch_turn_event(abort())
                .expect_err("old terminal before next start refuses")
                .diagnostic()
                .code(),
            "swallowtail.claude-agent.sdk.terminal_before_turn_start"
        );
        assert!(
            !next.is_finished(),
            "stale terminal is never the next outcome"
        );
    }
}
