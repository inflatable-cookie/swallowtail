#[path = "frame/cursor.rs"]
mod cursor;

use self::cursor::{
    align_delta, align_offset, apply_cursor, bind_turn, utf16_len, validate_callback_turn,
};
use super::PumpInput;
use crate::failure::failure;
use crate::local_server::activity::KimiLocalActivityProjection;
use crate::local_server::interactive::callbacks::ProviderCallbackKind;
use crate::local_server::interactive::websocket::resync_failure;
use crate::local_server::protocol::{
    TurnEndReason, WsEvent, WsFrame, decode_pending_approvals, decode_pending_questions,
    decode_ws_frame,
};
use crate::local_server::transport::{Request, session_path};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{
    OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeTurnId,
};

#[allow(clippy::too_many_arguments)]
pub(super) async fn apply_frame(
    input: &mut PumpInput,
    frame: &[u8],
    sequence: &mut u64,
    output: &mut String,
    reasoning_len: &mut usize,
    provider_turn: &mut Option<u64>,
    recovery: &mut Option<(u64, u64, u64)>,
    activity: &mut KimiLocalActivityProjection,
) -> Result<Option<TurnEndReason>, RuntimeFailure> {
    match decode_ws_frame(frame)? {
        WsFrame::Event(envelope) => {
            if envelope.session_id != input.session_id {
                return Ok(None);
            }
            if !apply_cursor(&input.cursor, &envelope)? {
                return Ok(None);
            }
            for observation in activity.project(&envelope.event)? {
                emit(
                    input,
                    sequence,
                    RuntimeEventKind::Activity(observation),
                    None,
                    *provider_turn,
                )?;
            }
            match envelope.event {
                WsEvent::TurnStarted { turn_id } => {
                    bind_turn(provider_turn, turn_id)?;
                    emit(
                        input,
                        sequence,
                        RuntimeEventKind::Progress,
                        None,
                        *provider_turn,
                    )?;
                }
                WsEvent::AssistantDelta { turn_id, delta } => {
                    bind_turn(provider_turn, turn_id)?;
                    if align_delta(envelope.offset, output)? {
                        output.push_str(&delta);
                        emit_content(
                            input,
                            sequence,
                            RuntimeEventKind::OutputDelta,
                            delta,
                            *provider_turn,
                        )?;
                    }
                }
                WsEvent::ThinkingDelta { turn_id, delta } => {
                    bind_turn(provider_turn, turn_id)?;
                    if align_offset(envelope.offset, *reasoning_len)? {
                        *reasoning_len += utf16_len(&delta);
                        emit_content(
                            input,
                            sequence,
                            RuntimeEventKind::ReasoningProgress,
                            delta,
                            *provider_turn,
                        )?;
                    }
                }
                WsEvent::TurnEnded { turn_id, reason } => {
                    bind_turn(provider_turn, turn_id)?;
                    if reason == TurnEndReason::Completed && !output.trim().is_empty() {
                        emit_content(
                            input,
                            sequence,
                            RuntimeEventKind::OutputAvailable,
                            output.clone(),
                            *provider_turn,
                        )?;
                    }
                    return Ok(Some(reason));
                }
                WsEvent::Retrying {
                    turn_id,
                    step,
                    failed_attempt,
                    next_attempt,
                    max_attempts,
                } => {
                    bind_turn(provider_turn, turn_id)?;
                    if failed_attempt == 0
                        || next_attempt != failed_attempt.saturating_add(1)
                        || next_attempt > max_attempts
                        || recovery.is_some_and(
                            |(previous_step, previous_next, previous_maximum)| {
                                step != previous_step
                                    || failed_attempt != previous_next
                                    || max_attempts != previous_maximum
                            },
                        )
                    {
                        return Err(protocol_failure());
                    }
                    *recovery = Some((step, next_attempt, max_attempts));
                    emit(
                        input,
                        sequence,
                        RuntimeEventKind::Progress,
                        None,
                        *provider_turn,
                    )?;
                }
                WsEvent::AwaitingApproval => {
                    enqueue_pending(
                        input,
                        sequence,
                        provider_turn,
                        ProviderCallbackKind::Approval,
                    )
                    .await?;
                }
                WsEvent::AwaitingQuestion => {
                    enqueue_pending(
                        input,
                        sequence,
                        provider_turn,
                        ProviderCallbackKind::Question,
                    )
                    .await?;
                }
                WsEvent::SessionAborted => return Ok(Some(TurnEndReason::Cancelled)),
                WsEvent::StepStarted { turn_id, .. }
                | WsEvent::StepEnded { turn_id, .. }
                | WsEvent::ToolStarted { turn_id, .. }
                | WsEvent::ToolUpdated { turn_id, .. }
                | WsEvent::ToolEnded { turn_id, .. } => {
                    bind_turn(provider_turn, turn_id)?;
                }
                WsEvent::ShellStarted { .. }
                | WsEvent::ShellUpdated { .. }
                | WsEvent::ShellEnded { .. }
                | WsEvent::SubagentSpawned { .. }
                | WsEvent::SubagentUpdated { .. }
                | WsEvent::SubagentEnded { .. }
                | WsEvent::CompactionStarted
                | WsEvent::CompactionEnded { .. }
                | WsEvent::TaskStarted { .. }
                | WsEvent::TaskEnded { .. }
                | WsEvent::Unknown(_) => {}
                WsEvent::Warning | WsEvent::Progress => {
                    emit(
                        input,
                        sequence,
                        RuntimeEventKind::Progress,
                        None,
                        *provider_turn,
                    )?;
                }
                WsEvent::ProviderError => {
                    return Err(failure(
                        "swallowtail.kimi.local_server.provider_error",
                        "Kimi local server reported a provider error",
                    ));
                }
            }
            Ok(None)
        }
        WsFrame::ResyncRequired { .. } => Err(resync_failure()),
        WsFrame::Error { fatal: true } => Err(protocol_failure()),
        WsFrame::Error { fatal: false } => {
            emit(
                input,
                sequence,
                RuntimeEventKind::Progress,
                None,
                *provider_turn,
            )?;
            Ok(None)
        }
        _ => Err(protocol_failure()),
    }
}

async fn enqueue_pending(
    input: &mut PumpInput,
    sequence: &mut u64,
    provider_turn: &Option<u64>,
    kind: ProviderCallbackKind,
) -> Result<(), RuntimeFailure> {
    if input.callbacks.is_none() {
        return Err(failure(
            "swallowtail.kimi.local_server.provider_request_rejected",
            "Kimi local server requested an undeclared provider interaction",
        ));
    }
    let secret = input.secret.upgrade().ok_or_else(protocol_failure)?;
    let base = session_path(&input.session_id)?;
    let path = match kind {
        ProviderCallbackKind::Approval => format!("{base}/approvals"),
        ProviderCallbackKind::Question => format!("{base}/questions"),
    };
    let response = input
        .transport
        .request(
            turn_scope(&input.runtime_turn_id)?,
            input.endpoint.clone(),
            Request::get_pending(path),
            Some(secret.copy()),
            &input.services,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
    if response.status != 200 {
        return Err(provider_interaction_failure());
    }
    let pending = match kind {
        ProviderCallbackKind::Approval => {
            decode_pending_approvals(&response.body, &input.session_id)?
        }
        ProviderCallbackKind::Question => {
            decode_pending_questions(&response.body, &input.session_id)?
        }
    };
    if pending.is_empty() {
        return Err(provider_interaction_failure());
    }
    for request in pending {
        validate_callback_turn(&request, provider_turn)?;
        let callback_id = input
            .callbacks
            .as_ref()
            .expect("callback policy checked")
            .enqueue(
                &input.runtime_turn_id,
                *sequence,
                input.deadline,
                kind,
                request,
            )?;
        let event = RuntimeEvent::new(*sequence, RuntimeEventKind::CallbackRequested(callback_id));
        let event = match *provider_turn {
            Some(provider_turn) => {
                event.with_reconciliation_checkpoint(operation_checkpoint(input, provider_turn)?)
            }
            None => event,
        };
        input.events.send(event)?;
        if provider_turn.is_some() {
            input
                .checkpoint_ready
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
        *sequence += 1;
    }
    Ok(())
}

fn emit(
    input: &PumpInput,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: Option<OperationContent>,
    provider_turn: Option<u64>,
) -> Result<(), RuntimeFailure> {
    let event = match content {
        Some(content) => RuntimeEvent::with_content(*sequence, kind, content),
        None => RuntimeEvent::new(*sequence, kind),
    };
    let event = match provider_turn {
        Some(provider_turn) => {
            event.with_reconciliation_checkpoint(operation_checkpoint(input, provider_turn)?)
        }
        None => event,
    };
    input.events.send(event)?;
    if provider_turn.is_some() {
        input
            .checkpoint_ready
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
    *sequence += 1;
    Ok(())
}

fn emit_content(
    input: &PumpInput,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: String,
    provider_turn: Option<u64>,
) -> Result<(), RuntimeFailure> {
    match OperationContent::new(content) {
        Ok(content) => emit(input, sequence, kind, Some(content), provider_turn),
        Err(_) => Ok(()),
    }
}

fn operation_checkpoint(
    input: &PumpInput,
    provider_turn: u64,
) -> Result<swallowtail_runtime::ProviderOperationCheckpoint, RuntimeFailure> {
    let cursor = input.cursor.lock().expect("cursor lock poisoned");
    let epoch = cursor.epoch.as_deref().ok_or_else(protocol_failure)?;
    let encoded = super::super::super::checkpoint::encode(cursor.seq, epoch)?;
    swallowtail_runtime::ProviderOperationCheckpoint::new(
        swallowtail_core::SessionRef::new(input.session_id.clone())
            .map_err(|_| protocol_failure())?,
        input.runtime_turn_id.clone(),
        swallowtail_core::TurnRef::new(provider_turn.to_string())
            .map_err(|_| protocol_failure())?,
        encoded,
    )
    .map_err(|_| protocol_failure())
}

fn turn_scope(turn: &RuntimeTurnId) -> Result<swallowtail_runtime::ScopeId, RuntimeFailure> {
    swallowtail_runtime::ScopeId::new(format!("kimi-local:turn:{}", turn.as_str()))
        .map_err(|_| protocol_failure())
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.turn_protocol_failed",
        "Kimi local-server turn protocol failed",
    )
}

fn provider_interaction_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.provider_interaction_failed",
        "Kimi local-server provider interaction could not be correlated",
    )
}
