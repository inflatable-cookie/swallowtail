use super::{PendingCallback, ProviderCallbackKind, State, closed, malformed};
use crate::failure::failure;
use crate::local_server::interactive::access::SecretMaterial;
use crate::local_server::protocol::{decode_callback_resolution, decode_question_dismissal};
use crate::local_server::transport::{CurlTransport, Request, session_path};
use std::sync::{Arc, Mutex, Weak};
use swallowtail_runtime::{
    BoxFuture, CallbackPayload, CallbackResponder, CallbackResponse, CallbackResult, HostServices,
    RuntimeFailure, ScopeId,
};

pub(super) struct ResponseContext {
    pub(super) state: Arc<Mutex<State>>,
    pub(super) scope: ScopeId,
    pub(super) provider_session_id: String,
    pub(super) endpoint: String,
    pub(super) secret: Weak<SecretMaterial>,
    pub(super) services: HostServices,
    pub(super) transport: CurlTransport,
}

impl CallbackResponder for ResponseContext {
    fn respond(&self, response: CallbackResponse) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            let pending = claim(&self.state, &response)?;
            let secret = self.secret.upgrade().ok_or_else(closed)?;
            let base = session_path(&self.provider_session_id)?;
            let segment = provider_segment(&pending.provider_id)?;
            let (request, dismiss) =
                request_for_response(base, segment, pending.kind, response.result())?;
            let reply = self
                .transport
                .request(
                    self.scope.clone(),
                    self.endpoint.clone(),
                    request,
                    Some(secret.copy()),
                    &self.services,
                    Arc::new(std::sync::atomic::AtomicBool::new(false)),
                )
                .await?;
            if reply.status != 200 {
                return Err(rejected());
            }
            if dismiss {
                decode_question_dismissal(&reply.body)
            } else {
                decode_callback_resolution(&reply.body)
            }
        })
    }
}

fn claim(
    state: &Arc<Mutex<State>>,
    response: &CallbackResponse,
) -> Result<PendingCallback, RuntimeFailure> {
    let mut state = state.lock().expect("callback state lock poisoned");
    if state.closed {
        return Err(closed());
    }
    let pending = state.pending.get(response.callback_id()).ok_or_else(|| {
        failure(
            "swallowtail.kimi.local_server.callback_unknown_or_duplicate",
            "Kimi callback response is unknown or was already used",
        )
    })?;
    if &pending.operation_id != response.operation_id() {
        return Err(failure(
            "swallowtail.kimi.local_server.callback_turn_mismatch",
            "Kimi callback response belongs to a different turn",
        ));
    }
    Ok(state
        .pending
        .remove(response.callback_id())
        .expect("validated callback remains pending"))
}

fn request_for_response(
    base: String,
    provider_id: String,
    kind: ProviderCallbackKind,
    result: &CallbackResult,
) -> Result<(Request, bool), RuntimeFailure> {
    match (kind, result) {
        (ProviderCallbackKind::Approval, CallbackResult::Success(payload)) => {
            validate_approval_response(payload)?;
            Ok((
                Request::post_json(
                    format!("{base}/approvals/{provider_id}"),
                    payload.as_bytes().to_vec(),
                ),
                false,
            ))
        }
        (ProviderCallbackKind::Question, CallbackResult::Success(payload)) => {
            validate_question_response(payload)?;
            Ok((
                Request::post_json(
                    format!("{base}/questions/{provider_id}"),
                    payload.as_bytes().to_vec(),
                ),
                false,
            ))
        }
        (ProviderCallbackKind::Approval, CallbackResult::Failure { .. }) => Ok((
            Request::post_json(
                format!("{base}/approvals/{provider_id}"),
                br#"{"decision":"cancelled"}"#.to_vec(),
            ),
            false,
        )),
        (ProviderCallbackKind::Question, CallbackResult::Failure { .. }) => Ok((
            Request::post(format!("{base}/questions/{provider_id}:dismiss")),
            true,
        )),
    }
}

fn validate_approval_response(payload: &CallbackPayload) -> Result<(), RuntimeFailure> {
    let value: serde_json::Value =
        serde_json::from_slice(payload.as_bytes()).map_err(|_| malformed())?;
    let object = value.as_object().ok_or_else(malformed)?;
    if !matches!(
        object.get("decision").and_then(serde_json::Value::as_str),
        Some("approved" | "rejected" | "cancelled")
    ) || object
        .get("scope")
        .is_some_and(|scope| scope.as_str() != Some("session"))
    {
        return Err(malformed());
    }
    Ok(())
}

fn validate_question_response(payload: &CallbackPayload) -> Result<(), RuntimeFailure> {
    let value: serde_json::Value =
        serde_json::from_slice(payload.as_bytes()).map_err(|_| malformed())?;
    let answers = value
        .as_object()
        .and_then(|object| object.get("answers"))
        .and_then(serde_json::Value::as_object)
        .ok_or_else(malformed)?;
    if answers.is_empty() || answers.len() > 4 {
        return Err(malformed());
    }
    for answer in answers.values() {
        let answer = answer.as_object().ok_or_else(malformed)?;
        if !matches!(
            answer.get("kind").and_then(serde_json::Value::as_str),
            Some("single" | "multi" | "other" | "multi_with_other" | "skipped")
        ) {
            return Err(malformed());
        }
    }
    Ok(())
}

fn provider_segment(value: &str) -> Result<String, RuntimeFailure> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~'))
    {
        Err(malformed())
    } else {
        Ok(value.to_owned())
    }
}

fn rejected() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.callback_rejected",
        "Kimi local server rejected a callback response",
    )
}
