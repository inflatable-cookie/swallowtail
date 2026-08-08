use crate::safe_excerpt::{normalized_ascii, sanitize_stderr};
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
    CallbackId, CleanupOutcome, DebugObservation, DebugObservationKind, HostServices,
    ProcessHandle, ProcessInputChunk, ProcessOutputStream, RuntimeFailure, RuntimeTurnId,
    TerminalStatus,
};

pub(crate) struct RpcConnection {
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
    next_id: AtomicU64,
    next_callback_id: AtomicU64,
    pending: Mutex<BTreeMap<u64, ResponseSender>>,
    ignored_responses: Mutex<BTreeSet<u64>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    lifecycle_notifications: Mutex<Vec<LifecycleNotification>>,
    closing: AtomicBool,
    session_cancelled: AtomicBool,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
    stderr_tail: Mutex<Vec<u8>>,
}

impl RpcConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        Arc::new(Self {
            process,
            services,
            next_id: AtomicU64::new(1),
            next_callback_id: AtomicU64::new(1),
            pending: Mutex::new(BTreeMap::new()),
            ignored_responses: Mutex::new(BTreeSet::new()),
            active_turn: Mutex::new(None),
            lifecycle_notifications: Mutex::new(Vec::new()),
            closing: AtomicBool::new(false),
            session_cancelled: AtomicBool::new(false),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
            stderr_tail: Mutex::new(Vec::new()),
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
        self.dispatch_request(method, params).await?.await
    }

    pub(crate) async fn dispatch_request(
        &self,
        method: &str,
        params: Value,
    ) -> Result<ResponseFuture, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.dispatch_request_with_id(id, method, params).await
    }

    async fn request_with_id(
        &self,
        id: u64,
        method: &str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.dispatch_request_with_id(id, method, params)
            .await?
            .await
    }

    async fn dispatch_request_with_id(
        &self,
        id: u64,
        method: &str,
        params: Value,
    ) -> Result<ResponseFuture, RuntimeFailure> {
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
        Ok(response)
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

    pub(crate) fn lifecycle_notifications(&self) -> Vec<LifecycleNotification> {
        self.lifecycle_notifications
            .lock()
            .expect("lifecycle-notification lock poisoned")
            .clone()
    }
}

include!("rpc/pump.rs");

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

pub(crate) struct ResponseFuture(Arc<Mutex<ResponseState>>);

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LifecycleNotification {
    method: String,
    thread_id: String,
}

impl LifecycleNotification {
    fn from_message(method: &str, params: &Value) -> Option<Self> {
        if !matches!(
            method,
            "thread/archived" | "thread/unarchived" | "thread/deleted"
        ) {
            return None;
        }
        Some(Self {
            method: method.to_owned(),
            thread_id: params.get("threadId")?.as_str()?.to_owned(),
        })
    }

    pub(crate) fn matches(&self, method: &str, thread_id: &str) -> bool {
        self.method == method && self.thread_id == thread_id
    }
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
