use crate::claude_code_handle::ClaudeCodeCancellation;
use crate::claude_code_pump_common::{
    self as common, RESPONSE_ONLY_PUMP_ROUTE, cleanup_from_wait, emit_host_process_debug,
    emit_protocol_debug, event_delivery_failed, force_cleanup, process_wait_failed, send_all,
};
use crate::claude_code_response_events::ClaudeCodeResponseEventParser;
use std::sync::Arc;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DeadlineObservation, HostServices, ProcessHandle,
    ProcessOutputStream, RuntimeEventSender, TerminalOutcome, TerminalStatus,
};

const ROUTE: common::PumpRoute = RESPONSE_ONLY_PUMP_ROUTE;

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<ClaudeCodeCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    mut parser: ClaudeCodeResponseEventParser,
    services: HostServices,
) -> TerminalOutcome {
    let mut deadline = Some(deadline);
    loop {
        match common::next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            common::NextOutput::Deadline => {
                let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            common::NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
                            return event_delivery_failed(&ROUTE, cleanup);
                        }
                    }
                    Err(error) => {
                        emit_protocol_debug(&ROUTE, &services, &error, ROUTE.decode_stage);
                        let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
            common::NextOutput::Process(Ok(Some(_))) => {}
            common::NextOutput::Process(Ok(None)) => break,
            common::NextOutput::Process(Err(error)) => {
                emit_host_process_debug(&ROUTE, &services, &error, ROUTE.read_stage);
                let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
                return TerminalOutcome::new(
                    TerminalStatus::HostFailed(error.diagnostic().clone()),
                    cleanup,
                );
            }
        }
    }
    let exit = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&ROUTE, &exit));
    }
    match exit {
        Ok(exit) => match parser.finish(exit) {
            Ok((trailing, outcome)) => {
                if send_all(&events, trailing).is_err() {
                    event_delivery_failed(&ROUTE, CleanupOutcome::Clean)
                } else {
                    outcome
                }
            }
            Err(error) => {
                emit_protocol_debug(&ROUTE, &services, &error, ROUTE.finish_stage);
                TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                )
            }
        },
        Err(_) => process_wait_failed(&ROUTE, &services),
    }
}
