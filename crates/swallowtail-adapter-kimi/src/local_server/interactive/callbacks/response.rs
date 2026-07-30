use super::{ProviderCallbackKind, State, closed, malformed};
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
            let secret = self.secret.upgrade().ok_or_else(closed)?;
            let base = session_path(&self.provider_session_id)?;
            let (request, dismiss) = claim(&self.state, &response, base)?;
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
    base: String,
) -> Result<(Request, bool), RuntimeFailure> {
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
    let segment = provider_segment(&pending.provider_id)?;
    let request = request_for_response(
        base,
        segment,
        pending.kind,
        pending.user_input.as_ref(),
        response.result(),
    )?;
    state
        .pending
        .remove(response.callback_id())
        .expect("validated callback remains pending");
    Ok(request)
}

fn request_for_response(
    base: String,
    provider_id: String,
    kind: ProviderCallbackKind,
    user_input: Option<&swallowtail_runtime::HarnessUserInputRequest>,
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
        (ProviderCallbackKind::Question, CallbackResult::Success(_)) => Err(malformed()),
        (ProviderCallbackKind::Question, CallbackResult::UserInput(response)) => {
            let user_input = user_input.ok_or_else(malformed)?;
            let body = typed_question_response(user_input, response)?;
            Ok((
                Request::post_json(format!("{base}/questions/{provider_id}"), body),
                false,
            ))
        }
        (ProviderCallbackKind::Approval, CallbackResult::UserInput(_)) => Err(malformed()),
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

fn typed_question_response(
    request: &swallowtail_runtime::HarnessUserInputRequest,
    response: &swallowtail_runtime::HarnessUserInputResponse,
) -> Result<Vec<u8>, RuntimeFailure> {
    if !request.accepts(response) {
        return Err(malformed());
    }
    let answers = response
        .answers()
        .map(|answer| {
            let value = if answer.is_skipped() {
                serde_json::json!({"kind": "skipped"})
            } else if answer.text().is_some() {
                return Err(malformed());
            } else {
                let mut selected = answer.selected_options();
                let option = selected.next().ok_or_else(malformed)?;
                if selected.next().is_some() {
                    return Err(malformed());
                }
                serde_json::json!({"kind": "single", "option_id": option.as_str()})
            };
            Ok((answer.question_id().as_str().to_owned(), value))
        })
        .collect::<Result<serde_json::Map<String, serde_json::Value>, RuntimeFailure>>()?;
    serde_json::to_vec(&serde_json::json!({"answers": answers})).map_err(|_| malformed())
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
