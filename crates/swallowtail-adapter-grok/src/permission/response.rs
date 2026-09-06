use super::{PendingKind, State, closed};
use crate::connection::AcpConnection;
use crate::failure::{failure, malformed};
use serde_json::Value;
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, Weak};
use swallowtail_runtime::{
    BoxFuture, CallbackResponder, CallbackResponse, CallbackResult, RuntimeFailure,
};

pub(super) struct PermissionCallbackResponder {
    pub(super) state: Arc<Mutex<State>>,
    pub(super) connection: Weak<AcpConnection>,
}

impl CallbackResponder for PermissionCallbackResponder {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let state = Arc::clone(&self.state);
        let connection = self.connection.clone();
        Box::pin(async move {
            let (provider_id, option_id) = claim_response(&state, &response)?;
            let connection = connection.upgrade().ok_or_else(closed)?;
            connection.respond_permission(provider_id, &option_id).await
        })
    }
}

fn claim_response(
    state: &Arc<Mutex<State>>,
    response: &CallbackResponse,
) -> Result<(Value, String), RuntimeFailure> {
    let mut state = state.lock().expect("permission callback lock poisoned");
    if state.closed {
        return Err(closed());
    }
    let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
        failure(
            "swallowtail.grok.acp.permission_callback_unknown_or_duplicate",
            "Grok Build permission response is unknown or was already used",
        )
    })?;
    if &pending.operation_id != response.operation_id() {
        return Err(failure(
            "swallowtail.grok.acp.permission_callback_turn_mismatch",
            "Grok Build permission response belongs to a different turn",
        ));
    }
    let PendingKind::Permission {
        options,
        reject_option_id,
    } = &pending.kind;
    let option_id = match response.result() {
        CallbackResult::Failure { .. } => reject_option_id.clone(),
        CallbackResult::Success(payload) => selected_option(payload, options)?,
        CallbackResult::UserInput(_) => {
            return Err(failure(
                "swallowtail.grok.acp.permission_callback_result_invalid",
                "Grok Build permission callback received a user-input response",
            ));
        }
    };
    let provider_id = pending.provider_id.clone();
    state
        .pending
        .remove(response.callback_id())
        .expect("validated callback remains pending");
    Ok((provider_id, option_id))
}

fn selected_option(
    payload: &swallowtail_runtime::CallbackPayload,
    options: &BTreeSet<String>,
) -> Result<String, RuntimeFailure> {
    let value: Value = serde_json::from_slice(payload.as_bytes()).map_err(|_| malformed())?;
    let object = value
        .as_object()
        .filter(|object| object.len() == 1)
        .ok_or_else(malformed)?;
    let option_id = object
        .get("optionId")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    if options.contains(option_id) {
        Ok(option_id.to_owned())
    } else {
        Err(failure(
            "swallowtail.grok.acp.permission_option_unoffered",
            "Grok Build permission response selected an unavailable option",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permission::{PendingCallback, PendingKind};
    use serde_json::json;
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    use swallowtail_runtime::{
        CallbackFailureKind, CallbackId, CallbackOperationId, RuntimeTurnId,
    };

    #[test]
    fn consumer_failure_claims_the_offered_one_shot_rejection_once() {
        let callback_id = CallbackId::new("permission-callback").expect("valid callback");
        let turn_id = RuntimeTurnId::new("permission-turn").expect("valid turn");
        let state = Arc::new(Mutex::new(State {
            requests: VecDeque::new(),
            pending: BTreeMap::from([(
                callback_id.clone(),
                PendingCallback {
                    provider_id: json!(900),
                    operation_id: CallbackOperationId::Turn(turn_id.clone()),
                    kind: PendingKind::Permission {
                        options: BTreeSet::from([
                            "allow-once".to_owned(),
                            "reject-once".to_owned(),
                        ]),
                        reject_option_id: "reject-once".to_owned(),
                    },
                },
            )]),
            provider_ids: BTreeSet::from(["number:900".to_owned()]),
            closed: false,
            waiter: None,
        }));
        let response = CallbackResponse::new(
            callback_id,
            turn_id,
            CallbackResult::Failure {
                kind: CallbackFailureKind::ConsumerFailed,
                detail: None,
            },
        );
        let (provider_id, option_id) =
            claim_response(&state, &response).expect("failure maps to rejection");
        assert_eq!(provider_id, json!(900));
        assert_eq!(option_id, "reject-once");
        assert!(claim_response(&state, &response).is_err());
    }
}
