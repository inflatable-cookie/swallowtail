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
            let provider_response = claim_response(&state, &response)?;
            let connection = connection.upgrade().ok_or_else(closed)?;
            match provider_response {
                ProviderResponse::Permission {
                    provider_id,
                    option_id,
                } => connection.respond_permission(provider_id, &option_id).await,
                ProviderResponse::Elicitation {
                    provider_id,
                    response,
                } => connection.respond_elicitation(provider_id, response).await,
            }
        })
    }
}

enum ProviderResponse {
    Permission {
        provider_id: Value,
        option_id: String,
    },
    Elicitation {
        provider_id: Value,
        response: Value,
    },
}

fn claim_response(
    state: &Arc<Mutex<State>>,
    response: &CallbackResponse,
) -> Result<ProviderResponse, RuntimeFailure> {
    let mut state = state.lock().expect("permission callback lock poisoned");
    if state.closed {
        return Err(closed());
    }
    let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
        failure(
            "swallowtail.claude_agent.acp.permission_callback_unknown_or_duplicate",
            "Claude Agent permission response is unknown or was already used",
        )
    })?;
    if &pending.operation_id != response.operation_id() {
        return Err(failure(
            "swallowtail.claude_agent.acp.permission_callback_turn_mismatch",
            "Claude Agent permission response belongs to a different turn",
        ));
    }
    let provider_response = match &pending.kind {
        PendingKind::Permission {
            options,
            reject_option_id,
        } => {
            let option_id = match response.result() {
                CallbackResult::Failure { .. } => reject_option_id.clone(),
                CallbackResult::Success(payload) => selected_option(payload, options)?,
                CallbackResult::UserInput(_) => {
                    return Err(failure(
                        "swallowtail.claude_agent.acp.permission_callback_result_invalid",
                        "Claude Agent permission callback received a user-input response",
                    ));
                }
            };
            ProviderResponse::Permission {
                provider_id: pending.provider_id.clone(),
                option_id,
            }
        }
        PendingKind::UserInput(request) => {
            let translated = match response.result() {
                CallbackResult::Failure { .. } => crate::elicitation::declined_response(),
                CallbackResult::UserInput(response) => crate::elicitation::accepted_response(
                    crate::elicitation::response_content(request, response)?,
                ),
                CallbackResult::Success(_) => {
                    return Err(failure(
                        "swallowtail.claude_agent.acp.elicitation_callback_result_invalid",
                        "Claude Agent elicitation callback received an opaque success response",
                    ));
                }
            };
            ProviderResponse::Elicitation {
                provider_id: pending.provider_id.clone(),
                response: translated,
            }
        }
    };
    state
        .pending
        .remove(response.callback_id())
        .expect("validated callback remains pending");
    Ok(provider_response)
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
            "swallowtail.claude_agent.acp.permission_option_unoffered",
            "Claude Agent permission response selected an unavailable option",
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

        let ProviderResponse::Permission {
            provider_id,
            option_id,
        } = claim_response(&state, &response).expect("failure maps to rejection")
        else {
            panic!("permission response expected");
        };
        assert_eq!(provider_id, json!(900));
        assert_eq!(option_id, "reject-once");
        assert!(claim_response(&state, &response).is_err());
    }
}
