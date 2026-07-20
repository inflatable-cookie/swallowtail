use crate::turn_state::ActiveTurn;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CallbackId, CleanupOutcome, ProcessHandle, ProcessInputChunk, ProcessOutputStream,
    RuntimeFailure, RuntimeTurnId, TerminalStatus,
};

pub(crate) struct RpcConnection {
    process: Arc<dyn ProcessHandle>,
    next_id: AtomicU64,
    next_callback_id: AtomicU64,
    pending: Mutex<BTreeMap<u64, ResponseSender>>,
    ignored_responses: Mutex<BTreeSet<u64>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    closing: AtomicBool,
    session_cancelled: AtomicBool,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl RpcConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>) -> Arc<Self> {
        Arc::new(Self {
            process,
            next_id: AtomicU64::new(1),
            next_callback_id: AtomicU64::new(1),
            pending: Mutex::new(BTreeMap::new()),
            ignored_responses: Mutex::new(BTreeSet::new()),
            active_turn: Mutex::new(None),
            closing: AtomicBool::new(false),
            session_cancelled: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
        })
    }

    pub(crate) async fn initialize(&self, experimental_api: bool) -> Result<(), RuntimeFailure> {
        let mut params = serde_json::json!({
            "clientInfo": {
                "name": "swallowtail",
                "title": "Swallowtail",
                "version": env!("CARGO_PKG_VERSION")
            }
        });
        if experimental_api {
            params
                .as_object_mut()
                .expect("static initialize parameters are an object")
                .insert(
                    "capabilities".to_owned(),
                    serde_json::json!({ "experimentalApi": true }),
                );
        }
        self.request_with_id(0, "initialize", params).await?;
        self.notify("initialized", serde_json::json!({})).await
    }

    pub(crate) async fn request(
        &self,
        method: &str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.request_with_id(id, method, params).await
    }

    async fn request_with_id(
        &self,
        id: u64,
        method: &str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.codex.app_server.connection_closed",
                "Codex app-server connection is closed",
            ));
        }
        let (sender, response) = response_channel();
        self.pending
            .lock()
            .expect("RPC pending-response lock poisoned")
            .insert(id, sender);
        let message = serde_json::json!({"id": id, "method": method, "params": params});
        if let Err(write_failure) = self.write_message(&message).await {
            self.pending
                .lock()
                .expect("RPC pending-response lock poisoned")
                .remove(&id);
            return Err(write_failure);
        }
        response.await
    }

    pub(crate) async fn notify(&self, method: &str, params: Value) -> Result<(), RuntimeFailure> {
        self.write_message(&serde_json::json!({"method": method, "params": params}))
            .await
    }

    pub(crate) async fn request_without_waiting(
        &self,
        method: &str,
        params: Value,
    ) -> Result<(), RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.ignored_responses
            .lock()
            .expect("ignored-response lock poisoned")
            .insert(id);
        let message = serde_json::json!({"id": id, "method": method, "params": params});
        if let Err(error) = self.write_message(&message).await {
            self.ignored_responses
                .lock()
                .expect("ignored-response lock poisoned")
                .remove(&id);
            return Err(error);
        }
        Ok(())
    }

    pub(crate) async fn respond_server_request(
        &self,
        id: Value,
        result: Value,
    ) -> Result<(), RuntimeFailure> {
        self.write_message(&serde_json::json!({"id": id, "result": result}))
            .await
    }

    pub(crate) async fn reject_abandoned_callbacks(
        &self,
        requests: Vec<Value>,
    ) -> Result<(), RuntimeFailure> {
        for id in requests {
            self.reject_server_request(&id, -32000, "Dynamic tool callback abandoned")
                .await?;
        }
        Ok(())
    }

    pub(crate) fn allocate_callback_id(&self, turn_id: &RuntimeTurnId) -> CallbackId {
        let sequence = self.next_callback_id.fetch_add(1, Ordering::SeqCst);
        CallbackId::new(format!("{}:callback:{sequence}", turn_id.as_str()))
            .expect("runtime turn id produces a valid callback id")
    }

    async fn write_message(&self, value: &Value) -> Result<(), RuntimeFailure> {
        let mut bytes = serde_json::to_vec(value).map_err(|_| malformed_outbound())?;
        bytes.push(b'\n');
        self.process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("active-turn lock poisoned");
        if active.as_ref().is_some_and(|turn| !turn.is_finished()) {
            return Err(failure(
                "swallowtail.codex.app_server.turn_already_active",
                "Codex app-server session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        let mut active = self.active_turn.lock().expect("active-turn lock poisoned");
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *active = None;
        }
    }

    pub(crate) async fn close_input(&self) -> Result<(), RuntimeFailure> {
        self.closing.store(true, Ordering::SeqCst);
        self.process.close_stdin().await
    }

    pub(crate) async fn cancel_session(&self) -> Result<(), RuntimeFailure> {
        self.session_cancelled.store(true, Ordering::SeqCst);
        self.process.force_stop().await
    }

    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.cleanup
            .lock()
            .expect("RPC cleanup lock poisoned")
            .clone()
            .unwrap_or_else(|| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.codex.app_server.cleanup_missing",
                    "Codex app-server cleanup did not complete",
                ))
            })
    }

    pub(crate) async fn pump(self: Arc<Self>) {
        let mut pending_bytes = Vec::new();
        let mut protocol_failure = None;
        loop {
            match self.process.read_output().await {
                Ok(Some(chunk)) if chunk.stream() == ProcessOutputStream::Stdout => {
                    pending_bytes.extend_from_slice(chunk.bytes());
                    while let Some(newline) = pending_bytes.iter().position(|byte| *byte == b'\n') {
                        let line: Vec<_> = pending_bytes.drain(..=newline).collect();
                        if let Err(error) = self.dispatch(trim_newline(&line)).await {
                            protocol_failure = Some(error);
                            break;
                        }
                    }
                    if protocol_failure.is_some() {
                        break;
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(error) => {
                    protocol_failure = Some(error);
                    break;
                }
            }
        }
        if protocol_failure.is_none()
            && !pending_bytes.is_empty()
            && let Err(error) = self.dispatch(&pending_bytes).await
        {
            protocol_failure = Some(error);
        }
        if protocol_failure.is_some() {
            let _ = self.process.force_stop().await;
        }
        let exit = self.process.wait().await;
        self.closed.store(true, Ordering::SeqCst);
        let cleanup = if exit.is_ok() {
            CleanupOutcome::Clean
        } else {
            CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.process_cleanup_failed",
                "Codex app-server process cleanup failed",
            ))
        };
        *self.cleanup.lock().expect("RPC cleanup lock poisoned") = Some(cleanup.clone());

        let terminal = if self.session_cancelled.load(Ordering::SeqCst) {
            TerminalStatus::Cancelled
        } else if let Some(error) = protocol_failure {
            TerminalStatus::RuntimeFailed(error.diagnostic().clone())
        } else if !self.closing.load(Ordering::SeqCst) {
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.connection_ended",
                "Codex app-server connection ended unexpectedly",
            ))
        } else {
            TerminalStatus::Cancelled
        };
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("active-turn lock poisoned")
            .take()
        {
            turn.finish(terminal, cleanup);
        }
        self.fail_pending(failure(
            "swallowtail.codex.app_server.connection_ended",
            "Codex app-server connection ended",
        ));
    }

    async fn dispatch(&self, line: &[u8]) -> Result<(), RuntimeFailure> {
        if line.iter().all(u8::is_ascii_whitespace) {
            return Ok(());
        }
        let message: Value = serde_json::from_slice(line).map_err(|_| malformed_inbound())?;
        if let Some(id_value) = message.get("id") {
            if message.get("method").is_some() {
                return self.dispatch_server_request(&message).await;
            }
            let id = id_value.as_u64().ok_or_else(malformed_inbound)?;
            if self
                .ignored_responses
                .lock()
                .expect("ignored-response lock poisoned")
                .remove(&id)
            {
                return Ok(());
            }
            let sender = self
                .pending
                .lock()
                .expect("RPC pending-response lock poisoned")
                .remove(&id)
                .ok_or_else(|| {
                    failure(
                        "swallowtail.codex.app_server.unknown_response",
                        "Codex app-server returned an unknown response id",
                    )
                })?;
            let response = if message.get("error").is_some() {
                Err(failure(
                    "swallowtail.codex.app_server.request_failed",
                    "Codex app-server rejected a request",
                ))
            } else {
                message.get("result").cloned().ok_or_else(malformed_inbound)
            };
            sender.complete(response);
            return Ok(());
        }
        let method = message
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_inbound)?;
        let params = message.get("params").cloned().unwrap_or(Value::Null);
        let active_turn = {
            self.active_turn
                .lock()
                .expect("active-turn lock poisoned")
                .clone()
        };
        if let Some(turn) = active_turn {
            turn.handle_notification(method, &params)?;
            if turn.is_finished() {
                self.reject_abandoned_callbacks(turn.take_abandoned_provider_requests())
                    .await?;
                self.clear_active_turn(&turn);
            }
        }
        Ok(())
    }

    async fn dispatch_server_request(&self, message: &Value) -> Result<(), RuntimeFailure> {
        let id = message.get("id").ok_or_else(malformed_inbound)?;
        let method = message
            .get("method")
            .and_then(Value::as_str)
            .ok_or_else(malformed_inbound)?;
        let turn = self
            .active_turn
            .lock()
            .expect("active-turn lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.callback_without_turn",
                    "Codex app-server requested a callback without an active turn",
                )
            });
        let turn = match turn {
            Ok(turn) => turn,
            Err(error) => {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                return Err(error);
            }
        };
        let params = match message.get("params") {
            Some(params) => params,
            None => {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                return Err(malformed_inbound());
            }
        };
        let callback_id = self.allocate_callback_id(turn.runtime_id());
        if method == "item/tool/call" {
            if let Err(error) = turn.handle_tool_call(id.clone(), params, callback_id) {
                self.reject_server_request(id, -32602, "Dynamic tool callback rejected")
                    .await?;
                return Err(error);
            }
            return Ok(());
        }

        let observation = match turn.handle_provider_request(id, method, params, callback_id) {
            Ok(observation) => observation,
            Err(error) => {
                self.reject_server_request(id, -32601, "Client callback unsupported")
                    .await?;
                return Err(error);
            }
        };
        self.reject_server_request(id, -32001, "Provider request observed; turn stopped")
            .await?;
        self.request_without_waiting(
            "turn/interrupt",
            serde_json::json!({
                "threadId": params["threadId"].clone(),
                "turnId": params["turnId"].clone()
            }),
        )
        .await?;
        turn.finish(
            TerminalStatus::ProviderRequestObserved(observation),
            CleanupOutcome::NotApplicable,
        );
        self.clear_active_turn(&turn);
        Ok(())
    }

    async fn reject_server_request(
        &self,
        id: &Value,
        code: i64,
        message: &str,
    ) -> Result<(), RuntimeFailure> {
        self.write_message(&serde_json::json!({
            "id": id,
            "error": {
                "code": code,
                "message": message
            }
        }))
        .await
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending = std::mem::take(
            &mut *self
                .pending
                .lock()
                .expect("RPC pending-response lock poisoned"),
        );
        for (_, sender) in pending {
            sender.complete(Err(error.clone()));
        }
    }
}

struct ResponseState {
    result: Option<Result<Value, RuntimeFailure>>,
    waiter: Option<Waker>,
}

struct ResponseSender(Arc<Mutex<ResponseState>>);

impl ResponseSender {
    fn complete(self, result: Result<Value, RuntimeFailure>) {
        let mut state = self.0.lock().expect("RPC response lock poisoned");
        state.result = Some(result);
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

struct ResponseFuture(Arc<Mutex<ResponseState>>);

impl Future for ResponseFuture {
    type Output = Result<Value, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("RPC response lock poisoned");
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

fn response_channel() -> (ResponseSender, ResponseFuture) {
    let state = Arc::new(Mutex::new(ResponseState {
        result: None,
        waiter: None,
    }));
    (ResponseSender(Arc::clone(&state)), ResponseFuture(state))
}

fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

fn malformed_inbound() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.malformed_message",
        "Codex app-server returned a malformed protocol message",
    )
}

fn malformed_outbound() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.encode_failed",
        "Codex app-server request could not be encoded",
    )
}

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
