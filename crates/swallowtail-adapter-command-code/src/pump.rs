use crate::events::CommandCodeHeadlessEventParser;
use crate::handle::CommandCodeCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservation,
    DebugObservationKind, HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream,
    RequestId, RuntimeEventSender, RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "command-code.headless";

pub(crate) struct CommandCodePumpResult {
    pub(crate) outcome: TerminalOutcome,
    pub(crate) session_id: Option<String>,
}

pub(crate) struct PumpContext {
    operation_id: ActivityOperationId,
    model_selection: ModelSelectionDebugContext,
    services: HostServices,
}

impl PumpContext {
    pub(crate) fn new(
        operation_id: ActivityOperationId,
        model_selection: ModelSelectionDebugContext,
        services: HostServices,
    ) -> Self {
        Self {
            operation_id,
            model_selection,
            services,
        }
    }
}

#[derive(Clone)]
pub(crate) struct ModelSelectionDebugContext {
    requested_model_id: ModelId,
    correlation: ModelSelectionCorrelation,
    enabled: bool,
}

#[derive(Clone)]
enum ModelSelectionCorrelation {
    Request(RequestId),
    Turn(RuntimeTurnId),
}

impl ModelSelectionDebugContext {
    pub(crate) fn for_request(
        requested_model_id: ModelId,
        request_id: RequestId,
        enabled: bool,
    ) -> Self {
        Self {
            requested_model_id,
            correlation: ModelSelectionCorrelation::Request(request_id),
            enabled,
        }
    }

    pub(crate) fn for_turn(
        requested_model_id: ModelId,
        turn_id: RuntimeTurnId,
        enabled: bool,
    ) -> Self {
        Self {
            requested_model_id,
            correlation: ModelSelectionCorrelation::Turn(turn_id),
            enabled,
        }
    }
}

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<CommandCodeCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    context: PumpContext,
) -> TerminalOutcome {
    pump_with_session(process, events, cancellation, deadline, None, context)
        .await
        .outcome
}

pub(crate) async fn pump_with_session(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<CommandCodeCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    expected_session_id: Option<String>,
    context: PumpContext,
) -> CommandCodePumpResult {
    let mut parser = CommandCodeHeadlessEventParser::with_expected_session(
        context.operation_id,
        expected_session_id,
        context.model_selection.enabled,
    );
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(TerminalStatus::TimedOut, cleanup));
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        emit_model_selection_debug(
                            &context.services,
                            &context.model_selection,
                            parser.take_model_request_models(),
                        );
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return result(event_delivery_failed(cleanup));
                        }
                    }
                    Err(failure) => {
                        emit_protocol_debug(&context.services, &failure, "headless.pump.decode");
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return result(TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                            cleanup,
                        ));
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(failure)) => {
                emit_host_process_debug(&context.services, &failure, "headless.pump.read");
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(
                    TerminalStatus::HostFailed(failure.diagnostic().clone()),
                    cleanup,
                ));
            }
        }
    }

    let exit = process.wait().await;
    if cancellation.is_requested() {
        return result(TerminalOutcome::new(
            TerminalStatus::Cancelled,
            cleanup_from_wait(&exit),
        ));
    }
    match (parser.finish(), exit) {
        (Ok(finished), Ok(exit)) => {
            emit_model_selection_debug(
                &context.services,
                &context.model_selection,
                finished.model_request_models,
            );
            if send_all(&events, finished.events).is_err() {
                result(event_delivery_failed(CleanupOutcome::Clean))
            } else {
                CommandCodePumpResult {
                    outcome: finished.terminal.outcome(exit),
                    session_id: finished.session_id,
                }
            }
        }
        (Err(failure), exit) => {
            emit_protocol_debug(&context.services, &failure, "headless.pump.finish");
            result(TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                cleanup_from_wait(&exit),
            ))
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.command_code.headless.process_wait_failed",
                "Command Code process wait failed",
            );
            context.services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "headless.pump.wait",
                diagnostic.code(),
                diagnostic.message(),
            );
            result(TerminalOutcome::new(
                TerminalStatus::HostFailed(diagnostic),
                process_cleanup_failed(),
            ))
        }
    }
}

fn emit_model_selection_debug(
    services: &HostServices,
    context: &ModelSelectionDebugContext,
    effective_model_ids: impl IntoIterator<Item = Option<String>>,
) {
    if !context.enabled {
        return;
    }
    for effective_model_id in effective_model_ids {
        let observation = model_selection_debug_observation(
            &context.correlation,
            &context.requested_model_id,
            effective_model_id.as_deref(),
        );
        services.emit_debug_observation(&observation);
    }
}

fn model_selection_debug_observation(
    correlation: &ModelSelectionCorrelation,
    requested_model_id: &ModelId,
    effective_model_id: Option<&str>,
) -> DebugObservation {
    let requested_model_id = bounded_model_id(requested_model_id.as_str());
    let effective_model_id = effective_model_id.and_then(bounded_model_id);
    let detail = serde_json::json!({
        "requested_model_id": requested_model_id,
        "effective_model_id": effective_model_id,
        "effective_source": "command-code.model_request_start",
        "effective_scope": "cli-selected-model-before-sdk-request",
    })
    .to_string();
    let observation = DebugObservation::new(DebugObservationKind::InterfaceVersion, detail)
        .with_route(ROUTE)
        .with_stage("model-selection");
    match correlation {
        ModelSelectionCorrelation::Request(request_id) => {
            observation.with_request_id(request_id.clone())
        }
        ModelSelectionCorrelation::Turn(turn_id) => observation.with_turn_id(turn_id.clone()),
    }
}

fn bounded_model_id(value: &str) -> Option<&str> {
    if value.is_empty()
        || value.len() > 128
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        None
    } else {
        Some(value)
    }
}

fn result(outcome: TerminalOutcome) -> CommandCodePumpResult {
    CommandCodePumpResult {
        outcome,
        session_id: None,
    }
}

fn emit_protocol_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::ProtocolParse,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

fn emit_host_process_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &CommandCodeCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| {
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextOutput::Deadline);
        }
        read.as_mut().poll(context).map(NextOutput::Process)
    })
    .await
}

fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = swallowtail_runtime::RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = force_cleanup(process).await;
}

async fn force_cleanup(process: &dyn ProcessHandle) -> CleanupOutcome {
    let force = process.force_stop().await;
    let wait = process.wait().await;
    if force.is_err() || wait.is_err() {
        process_cleanup_failed()
    } else {
        CleanupOutcome::Clean
    }
}

fn cleanup_from_wait(
    exit: &Result<swallowtail_runtime::ProcessExit, RuntimeFailure>,
) -> CleanupOutcome {
    if exit.is_ok() {
        CleanupOutcome::Clean
    } else {
        process_cleanup_failed()
    }
}

fn event_delivery_failed(cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.command_code.headless.event_delivery_failed",
            "Command Code event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.command_code.headless.process_cleanup_failed",
        "Command Code process cleanup failed",
    ))
}

#[cfg(test)]
mod tests {
    use super::{ModelSelectionCorrelation, ROUTE, model_selection_debug_observation};
    use serde_json::{Value, json};
    use swallowtail_core::ModelId;
    use swallowtail_runtime::{DebugObservationKind, RequestId};

    #[test]
    fn model_selection_observation_contains_only_bounded_requested_and_cli_selected_ids() {
        let request_id = RequestId::new("request-1").expect("request id is valid");
        let requested_model_id = ModelId::new("caller/requested-model").expect("model id");
        let observation = model_selection_debug_observation(
            &ModelSelectionCorrelation::Request(request_id.clone()),
            &requested_model_id,
            Some("configured/planning-model"),
        );

        assert_eq!(observation.kind(), DebugObservationKind::InterfaceVersion);
        assert_eq!(observation.route(), Some(ROUTE));
        assert_eq!(observation.stage(), Some("model-selection"));
        assert_eq!(observation.request_id(), Some(&request_id));
        let detail: Value = serde_json::from_str(observation.detail()).expect("JSON detail");
        assert_eq!(
            detail,
            json!({
                "requested_model_id": "caller/requested-model",
                "effective_model_id": "configured/planning-model",
                "effective_source": "command-code.model_request_start",
                "effective_scope": "cli-selected-model-before-sdk-request",
            })
        );
        assert_eq!(detail.as_object().unwrap().len(), 4);
    }

    #[test]
    fn missing_or_unbounded_cli_model_id_is_not_inferred_from_the_request() {
        let request_id = RequestId::new("request-1").expect("request id is valid");
        let requested_model_id = ModelId::new("caller/requested-model").expect("model id");
        let unbounded_model_id = "x".repeat(129);
        for effective_model_id in [None, Some(unbounded_model_id.as_str())] {
            let observation = model_selection_debug_observation(
                &ModelSelectionCorrelation::Request(request_id.clone()),
                &requested_model_id,
                effective_model_id,
            );
            let detail: Value = serde_json::from_str(observation.detail()).expect("JSON detail");
            assert_eq!(detail["requested_model_id"], "caller/requested-model");
            assert_eq!(detail["effective_model_id"], Value::Null);
        }
    }

    #[test]
    fn model_selection_observation_uses_turn_correlation_for_interactive_turns() {
        let turn_id = swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("turn id");
        let requested_model_id = ModelId::new("caller/requested-model").expect("model id");
        let observation = model_selection_debug_observation(
            &ModelSelectionCorrelation::Turn(turn_id.clone()),
            &requested_model_id,
            Some("configured/planning-model"),
        );
        assert_eq!(observation.turn_id(), Some(&turn_id));
        assert_eq!(observation.request_id(), None);
    }
}
