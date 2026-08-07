use super::super::access::{SecretMaterial};
use super::super::checkpoint::KimiCursorCheckpoint;
use super::super::websocket::{Subscription, SubscriptionInput};
use super::failure::{
    binding_failure, cancelled, checkpoint_required, protocol_failure, stale_checkpoint, timed_out,
};
use crate::local_server::protocol::{
    InteractiveSessionRecord, RestReply, TurnEndReason, WsEvent, WsFrame,
    decode_interactive_session, decode_rest, decode_ws_frame,
};
use crate::local_server::transport::{Request, session_path};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{
    HostServices, ProviderSessionReconciliationPlan, ProviderSessionReconciliationRequest,
    RuntimeFailure,
};

#[allow(clippy::too_many_arguments)]
pub(super) async fn observe(
    driver: &crate::KimiLocalServerDriver,
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
    scope: &swallowtail_runtime::ScopeId,
    endpoint: String,
    directory: String,
    secret: Arc<SecretMaterial>,
    mut cursor: KimiCursorCheckpoint,
) -> Result<swallowtail_runtime::ProviderSessionReconciliationObservation, RuntimeFailure> {
    let agreement = plan.agreement();
    let provider_session = agreement
        .binding()
        .provider_session_ref()
        .as_provider_value();
    let expected_turn = agreement
        .provider_turn_ref()
        .ok_or_else(checkpoint_required)?
        .as_provider_value()
        .parse::<u64>()
        .map_err(|_| checkpoint_required())?;
    let before = fetch_session(
        driver,
        scope,
        &endpoint,
        &secret,
        provider_session,
        services,
    )
    .await?;
    validate_session(&before, provider_session, &directory)?;
    if before.last_seq < cursor.seq {
        return Err(stale_checkpoint());
    }
    control(request, services)?;
    let mut subscription = Subscription::open(
        SubscriptionInput {
            scope: scope.clone(),
            endpoint: endpoint.clone(),
            secret: secret.copy(),
            session_id: provider_session.to_owned(),
            cursor_seq: cursor.seq,
            cursor_epoch: Some(cursor.epoch.clone()),
            deadline: agreement.deadline(),
        },
        services,
    )
    .await?;
    let target = subscription.replay_target().0;
    if target < cursor.seq {
        let _ = subscription.close().await;
        return Err(stale_checkpoint());
    }
    let mut observed_state = None;
    while cursor.seq < target {
        let frame = next_frame(&mut subscription, request, agreement.deadline(), services).await?;
        if let Some(state) =
            apply_reconciliation_frame(&frame, provider_session, expected_turn, &mut cursor)?
        {
            observed_state = Some(state);
            if state.is_terminal() {
                break;
            }
        }
    }
    // The finite replay target is already reached. Closing still joins the
    // worker; a peer close racing our local close does not invalidate the
    // accepted observation snapshot.
    let _ = subscription.close().await;
    control(request, services)?;
    let after = fetch_session(
        driver,
        scope,
        &endpoint,
        &secret,
        provider_session,
        services,
    )
    .await?;
    validate_session(&after, provider_session, &directory)?;
    let state = observed_state.unwrap_or({
        if after.last_seq != target {
            swallowtail_runtime::InterruptedTurnState::Unknown
        } else if after.busy {
            swallowtail_runtime::InterruptedTurnState::Active
        } else {
            swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
        }
    });
    Ok(
        swallowtail_runtime::ProviderSessionReconciliationObservation::exact_turn(
            state,
            agreement
                .provider_turn_ref()
                .expect("exact checkpoint has turn reference")
                .clone(),
            Vec::new(),
            true,
        ),
    )
}

async fn fetch_session(
    driver: &crate::KimiLocalServerDriver,
    scope: &swallowtail_runtime::ScopeId,
    endpoint: &str,
    secret: &Arc<SecretMaterial>,
    provider_session: &str,
    services: &HostServices,
) -> Result<InteractiveSessionRecord, RuntimeFailure> {
    let response = driver
        .transport
        .request(
            scope.clone(),
            endpoint.to_owned(),
            Request::get(session_path(provider_session)?),
            Some(secret.copy()),
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
    if response.status != 200 {
        return match decode_rest(response.status, &response.body) {
            Ok(RestReply::Failure(_)) => Err(binding_failure()),
            _ => Err(protocol_failure()),
        };
    }
    decode_interactive_session(&response.body)
}

async fn next_frame(
    subscription: &mut Subscription,
    request: &ProviderSessionReconciliationRequest,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut timer =
        deadline.and_then(|deadline| services.time().map(|time| time.wait_until(deadline)));
    std::future::poll_fn(|context| {
        if request.cancellation().is_requested() {
            return std::task::Poll::Ready(Err(cancelled()));
        }
        if let std::task::Poll::Ready(frame) = subscription.poll_next(context) {
            return std::task::Poll::Ready(match frame {
                Some(frame) => frame,
                None => Err(protocol_failure()),
            });
        }
        if timer
            .as_mut()
            .is_some_and(|timer| timer.as_mut().poll(context).is_ready())
        {
            return std::task::Poll::Ready(Err(timed_out()));
        }
        std::task::Poll::Pending
    })
    .await
}

pub(super) fn apply_reconciliation_frame(
    frame: &[u8],
    provider_session: &str,
    expected_turn: u64,
    cursor: &mut KimiCursorCheckpoint,
) -> Result<Option<swallowtail_runtime::InterruptedTurnState>, RuntimeFailure> {
    let WsFrame::Event(envelope) = decode_ws_frame(frame)? else {
        return Err(protocol_failure());
    };
    if envelope.session_id != provider_session {
        return Err(binding_failure());
    }
    if envelope.volatile {
        return Ok(None);
    }
    if envelope.epoch.as_deref() != Some(cursor.epoch.as_str())
        || envelope.durable_seq != cursor.seq.saturating_add(1)
    {
        return Err(stale_checkpoint());
    }
    cursor.seq = envelope.durable_seq;
    let observed_turn = event_turn_id(&envelope.event);
    if observed_turn.is_some_and(|turn| turn != expected_turn) {
        return Err(binding_failure());
    }
    Ok(match envelope.event {
        WsEvent::TurnEnded { reason, .. } => Some(match reason {
            TurnEndReason::Completed => swallowtail_runtime::InterruptedTurnState::Completed,
            TurnEndReason::Cancelled => swallowtail_runtime::InterruptedTurnState::Cancelled,
            TurnEndReason::Failed | TurnEndReason::Blocked => {
                swallowtail_runtime::InterruptedTurnState::Failed
            }
        }),
        WsEvent::AwaitingApproval | WsEvent::AwaitingQuestion => {
            Some(swallowtail_runtime::InterruptedTurnState::WaitingForProviderInput)
        }
        WsEvent::SessionAborted => Some(swallowtail_runtime::InterruptedTurnState::Cancelled),
        WsEvent::TurnStarted { .. }
        | WsEvent::AssistantDelta { .. }
        | WsEvent::ThinkingDelta { .. }
        | WsEvent::StepStarted { .. }
        | WsEvent::StepEnded { .. }
        | WsEvent::ToolStarted { .. }
        | WsEvent::ToolUpdated { .. }
        | WsEvent::ToolEnded { .. }
        | WsEvent::Retrying { .. } => Some(swallowtail_runtime::InterruptedTurnState::Active),
        _ => None,
    })
}

fn event_turn_id(event: &WsEvent) -> Option<u64> {
    match event {
        WsEvent::TurnStarted { turn_id }
        | WsEvent::AssistantDelta { turn_id, .. }
        | WsEvent::ThinkingDelta { turn_id, .. }
        | WsEvent::StepStarted { turn_id, .. }
        | WsEvent::StepEnded { turn_id, .. }
        | WsEvent::ToolStarted { turn_id, .. }
        | WsEvent::ToolUpdated { turn_id, .. }
        | WsEvent::ToolEnded { turn_id, .. }
        | WsEvent::TurnEnded { turn_id, .. }
        | WsEvent::Retrying { turn_id, .. } => Some(*turn_id),
        _ => None,
    }
}

fn validate_session(
    record: &InteractiveSessionRecord,
    provider_session: &str,
    directory: &str,
) -> Result<(), RuntimeFailure> {
    if record.id != provider_session || record.archived || record.working_directory != directory {
        Err(binding_failure())
    } else {
        Ok(())
    }
}

pub(super) fn control(
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.cancellation().is_requested() {
        return Err(cancelled());
    }
    if request.agreement().deadline().is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Err(timed_out());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::apply_reconciliation_frame;
    use super::super::super::checkpoint::KimiCursorCheckpoint;
    use swallowtail_runtime::InterruptedTurnState;

    #[test]
    fn retained_terminal_event_preserves_exact_turn_truth() {
        let mut cursor = checkpoint();
        let state = apply_reconciliation_frame(
            &event(12, "fixture-session", "fixture-epoch", 7),
            "fixture-session",
            7,
            &mut cursor,
        )
        .expect("exact event projects");

        assert_eq!(state, Some(InterruptedTurnState::Completed));
        assert_eq!(cursor.seq, 12);
    }

    #[test]
    fn foreign_turn_session_epoch_and_cursor_gap_fail_closed() {
        for (frame, code) in [
            (
                event(12, "fixture-session", "fixture-epoch", 8),
                "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
            ),
            (
                event(12, "foreign-session", "fixture-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_binding_mismatch",
            ),
            (
                event(12, "fixture-session", "foreign-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
            ),
            (
                event(13, "fixture-session", "fixture-epoch", 7),
                "swallowtail.kimi.local_server.reconciliation_checkpoint_stale",
            ),
        ] {
            let error = apply_reconciliation_frame(&frame, "fixture-session", 7, &mut checkpoint())
                .expect_err("mismatched retained event rejects");
            assert_eq!(error.diagnostic().code(), code);
        }
    }

    fn checkpoint() -> KimiCursorCheckpoint {
        KimiCursorCheckpoint {
            seq: 11,
            epoch: "fixture-epoch".to_owned(),
        }
    }

    fn event(seq: u64, session: &str, epoch: &str, turn: u64) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "type": "turn.ended",
            "seq": seq,
            "timestamp": "fixture",
            "session_id": session,
            "epoch": epoch,
            "payload": {"turnId": turn, "reason": "completed"}
        }))
        .expect("fixture event encodes")
    }
}
