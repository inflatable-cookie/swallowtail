mod events;
mod failure;
mod signals;
mod tool_attempt;

use super::super::history::SessionHistory;
use super::super::lifecycle::TurnCancellation;
use super::super::session::DeepSeekSessionHandle;
use super::TurnWork;
use crate::failure::{failure, protocol, provider};
use crate::protocol::{
    FinalAttempt, FinalStreamParser, HttpRequest, PrivateContinuation, encode_after_tool,
    parse_tool_attempt, require_success,
};
use crate::transport::{CurlTransport, StreamItem};
use events::{emit, emit_output, emit_request, emit_update, emit_usage};
use failure::{runtime_failure, work_failure};
use futures_channel::oneshot;
use signals::{StopSignal, StreamSignal, WorkFailure, next_stream_signal, wait_results, wait_work};
use std::num::NonZeroU64;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use swallowtail_core::{DirectContinuationConfig, PreflightPlan};
use swallowtail_runtime::{
    CleanupOutcome, DirectContinuationBinding, DirectContinuationState, DirectInferenceAttempt,
    HostServices, OperationContent, ProviderPrivateContinuationRecord, RuntimeEventKind,
    RuntimeEventSender, RuntimeFailure, RuntimeSessionId, ScopeId, TerminalOutcome, TerminalStatus,
};
use tool_attempt::run_tool_turn;

pub(super) struct TurnContext {
    config: DirectContinuationConfig,
    plan: PreflightPlan,
    session_id: RuntimeSessionId,
    binding: DirectContinuationBinding,
    tools: Arc<Vec<crate::protocol::ToolSpec>>,
    state: Arc<Mutex<DirectContinuationState>>,
    history: Arc<Mutex<SessionHistory>>,
    private_records: Arc<Mutex<Vec<ProviderPrivateContinuationRecord>>>,
    transport: CurlTransport,
    services: HostServices,
    scope: ScopeId,
    cancelled: Arc<AtomicBool>,
    cancel_receiver: oneshot::Receiver<()>,
}

impl TurnContext {
    pub(super) fn from_session(
        session: &DeepSeekSessionHandle,
        cancelled: Arc<AtomicBool>,
        cancel_receiver: oneshot::Receiver<()>,
    ) -> Self {
        Self {
            config: session
                .plan
                .requirements()
                .direct_continuation()
                .expect("validated continuation")
                .config()
                .clone(),
            plan: session.plan.clone(),
            session_id: session.runtime_id.clone(),
            binding: session.binding.clone(),
            tools: Arc::clone(&session.tools),
            state: Arc::clone(&session.state),
            history: Arc::clone(&session.history),
            private_records: Arc::clone(&session.private_records),
            transport: session.transport.clone(),
            services: session.services.clone(),
            scope: session.scope.clone(),
            cancelled,
            cancel_receiver,
        }
    }
}

pub(super) async fn run_turn(
    mut work: TurnWork,
    mut context: TurnContext,
    events: RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut deadline = context
        .services
        .time()
        .expect("validated time")
        .wait_until(work.request.deadline());
    let result = if work.attempt.ordinal().get() == 1 {
        run_tool_turn(
            &mut work,
            &mut context,
            &events,
            &mut sequence,
            &mut deadline,
            &cancellation,
        )
        .await
    } else {
        run_final_attempt(
            &work.attempt,
            work.initial_request.clone(),
            &work,
            FinalAttemptFlow {
                context: &mut context,
                events: &events,
                sequence: &mut sequence,
                deadline: &mut deadline,
                cancellation: &cancellation,
            },
        )
        .await
    };

    if result.is_err()
        && let Some(submitter) = work.submitter.as_ref()
    {
        submitter.abandon();
    }
    match result {
        Ok((final_attempt, cleanup)) => {
            let output = final_attempt.output.clone();
            if let Err(error) = complete_success(
                &work.attempt,
                final_attempt,
                &mut context,
                &events,
                &mut sequence,
            ) {
                cancellation.fail_session();
                invalidate(&context);
                return TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    cleanup,
                );
            }
            TerminalOutcome::new(TerminalStatus::Completed, cleanup).with_output(
                OperationContent::new(output).expect("validated output remains non-empty"),
            )
        }
        Err(TurnFailure::Stopped(stop, cleanup)) => {
            invalidate(&context);
            let status = match stop {
                StopSignal::Cancelled => TerminalStatus::Cancelled,
                StopSignal::TimedOut => TerminalStatus::TimedOut,
            };
            TerminalOutcome::new(status, cleanup)
        }
        Err(TurnFailure::Provider(error, cleanup)) => {
            cancellation.fail_session();
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
        Err(TurnFailure::Runtime(error, cleanup)) => {
            cancellation.fail_session();
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
    }
}

fn complete_success(
    user_attempt: &DirectInferenceAttempt,
    final_attempt: FinalAttempt,
    context: &mut TurnContext,
    events: &RuntimeEventSender,
    sequence: &mut u64,
) -> Result<(), RuntimeFailure> {
    if !matches!(
        final_attempt.finish_reason.as_str(),
        "stop" | "length" | "content_filter" | "insufficient_system_resource"
    ) || final_attempt
        .usage
        .prompt_tokens
        .saturating_add(final_attempt.usage.completion_tokens)
        != final_attempt.usage.total_tokens
    {
        return Err(failure(
            "swallowtail.deepseek.final_evidence_invalid",
            "DeepSeek final attempt evidence was inconsistent",
        ));
    }
    emit_output(events, sequence, &final_attempt.output)?;
    let final_attempt_id = if user_attempt.ordinal().get() == 1 {
        DirectInferenceAttemptId::new("attempt-2").expect("static attempt id is valid")
    } else {
        user_attempt.attempt_id().clone()
    };
    record_private(context, &final_attempt_id, &final_attempt.reasoning)?;
    if user_attempt.ordinal().get() == 1 {
        context
            .history
            .lock()
            .expect("history lock poisoned")
            .record_first_final(final_attempt)?;
    }
    context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .complete_turn()
}

fn record_private(
    context: &TurnContext,
    attempt_id: &swallowtail_runtime::DirectInferenceAttemptId,
    continuation: &PrivateContinuation,
) -> Result<(), RuntimeFailure> {
    let bytes = NonZeroU64::new(continuation.byte_len() as u64).ok_or_else(|| {
        failure(
            "swallowtail.deepseek.private_continuation_missing",
            "DeepSeek private continuation was empty",
        )
    })?;
    let record = ProviderPrivateContinuationRecord::new(
        context.binding.clone(),
        attempt_id.clone(),
        bytes,
        context.config.maximum_private_continuation_bytes(),
    )?;
    if !record.matches_plan(&context.plan, &context.session_id) {
        return Err(failure(
            "swallowtail.deepseek.private_binding_mismatch",
            "DeepSeek private continuation did not match its bound route",
        ));
    }
    context
        .private_records
        .lock()
        .expect("private record lock poisoned")
        .push(record);
    Ok(())
}

fn invalidate(context: &TurnContext) {
    context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .invalidate();
}

enum TurnFailure {
    Stopped(StopSignal, CleanupOutcome),
    Provider(RuntimeFailure, CleanupOutcome),
    Runtime(RuntimeFailure, CleanupOutcome),
}

use swallowtail_runtime::DirectInferenceAttemptId;

include!("pump/final_attempt.rs");
