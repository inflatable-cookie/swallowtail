use crate::diagnostics::AcpDiagnostics;
use crate::transport::{AcpTransport, AcpTransportReader, AcpTransportWriter, ReadStep};
use crate::turn::AcpTurnSink;
use futures_channel::oneshot;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, Message, RpcError, is_session_scoped_metadata_update,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DebugObservationKind, HostServices, RuntimeFailure,
};

const MAXIMUM_PENDING_REQUESTS: usize = 32;

/// Disposition returned by [`AcpConnectionHooks::on_session_update`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionUpdateDisposition {
    /// The hook consumed the update (e.g. during session-load replay); do not route to the active turn.
    Handled,
    /// Route the update normally to the active turn (or ignore if session-scoped metadata).
    RouteToActiveTurn,
}

/// Disposition returned by [`AcpConnectionHooks::handle_request`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestDisposition {
    /// The hook handled the request and wrote any JSON-RPC response itself.
    Handled,
    /// Use the standard ACP reject-permission flow (observe permission, send `session/cancel`, reply `cancelled`).
    RejectPermission,
    /// Reply with `-32601 Method not found` and fail the connection with `callback_unsupported`.
    Unsupported,
}

/// Provider-specific hooks for [`AcpConnection`] dispatch and lifecycle events.
pub trait AcpConnectionHooks: Send + Sync {
    /// Optional hook invoked for `session/update` notifications before active-turn routing.
    fn on_session_update(
        &self,
        _params: &Value,
    ) -> Result<SessionUpdateDisposition, RuntimeFailure> {
        Ok(SessionUpdateDisposition::RouteToActiveTurn)
    }

    /// Optional hook invoked for other provider notifications before rejecting as unsupported.
    ///
    /// Return `Ok(true)` if the notification was handled.
    fn on_notification(&self, _method: &str, _params: &Value) -> Result<bool, RuntimeFailure> {
        Ok(false)
    }

    /// Optional hook invoked when a correlated JSON-RPC response arrives, before waking the caller.
    fn on_response(&self, _id: u64, _result: &Result<Value, RpcError>) {}

    /// Optional hook invoked after a batch of messages from one transport read completes.
    fn on_batch_complete(&self) {}

    /// Handles an incoming JSON-RPC request from the provider.
    fn handle_request<'a>(
        &'a self,
        _connection: &'a AcpConnection,
        _id: Value,
        method: &'a str,
        _params: &'a Value,
    ) -> BoxFuture<'a, Result<RequestDisposition, RuntimeFailure>> {
        let disposition = match method {
            "session/request_permission" => RequestDisposition::RejectPermission,
            _ => RequestDisposition::Unsupported,
        };
        Box::pin(async move { Ok(disposition) })
    }

    /// Optional hook invoked at the start of [`AcpConnection::begin_close`] before closing the transport.
    fn on_begin_close<'a>(&'a self, _connection: &'a AcpConnection) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }

    /// Optional hook invoked when the pump finishes with an error, before failing the active turn.
    fn on_transport_ended<'a>(&'a self, _error: &'a RuntimeFailure) -> BoxFuture<'a, ()> {
        Box::pin(async {})
    }
}

/// Default hooks for standard pure-ACP adapters (rejects permissions, no `fs` callbacks or replay).
#[derive(Clone, Copy, Debug, Default)]
pub struct StandardAcpHooks;

impl AcpConnectionHooks for StandardAcpHooks {}

struct PendingState {
    closed: bool,
    requests: BTreeMap<u64, oneshot::Sender<Result<Value, RuntimeFailure>>>,
}

/// Future resolving to the correlated JSON-RPC response of an in-flight ACP request.
pub struct PendingResponse {
    receiver: oneshot::Receiver<Result<Value, RuntimeFailure>>,
    closed_error: fn() -> RuntimeFailure,
}

impl Future for PendingResponse {
    type Output = Result<Value, RuntimeFailure>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.receiver).poll(context) {
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(_)) => Poll::Ready(Err((self.closed_error)())),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Full-duplex ACP v1 JSON-RPC connection abstracted over stdio and remote transports.
///
/// Uses `futures-channel::oneshot` for correlated responses and holds `closed` inside the
/// same mutex as `pending` so concurrent shutdown can never strand a newly registered request.
pub struct AcpConnection {
    writer: AcpTransportWriter,
    reader: Mutex<Option<AcpTransportReader>>,
    services: HostServices,
    diagnostics: AcpDiagnostics,
    hooks: Arc<dyn AcpConnectionHooks>,
    next_id: AtomicU64,
    pending: Mutex<PendingState>,
    session_id: Mutex<Option<String>>,
    active_turn: Mutex<Option<Arc<dyn AcpTurnSink>>>,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl AcpConnection {
    /// Creates a new ACP connection backed by `transport` with the given `diagnostics` and `hooks`.
    #[must_use]
    pub fn new(
        transport: AcpTransport,
        services: HostServices,
        diagnostics: AcpDiagnostics,
        hooks: Arc<dyn AcpConnectionHooks>,
    ) -> Arc<Self> {
        let (writer, reader) = transport.split();
        Arc::new(Self {
            writer,
            reader: Mutex::new(Some(reader)),
            services,
            diagnostics,
            hooks,
            next_id: AtomicU64::new(1),
            pending: Mutex::new(PendingState {
                closed: false,
                requests: BTreeMap::new(),
            }),
            session_id: Mutex::new(None),
            active_turn: Mutex::new(None),
            cleanup: Mutex::new(None),
        })
    }

    /// Returns the provider-scoped diagnostics table bound to this connection.
    #[must_use]
    pub const fn diagnostics(&self) -> &AcpDiagnostics {
        &self.diagnostics
    }

    /// Emits a structured protocol-parse debug observation through host services.
    pub fn emit_protocol_debug(&self, error: &RuntimeFailure, stage: &'static str) {
        let diagnostic = error.diagnostic();
        self.services.emit_failure_debug(
            DebugObservationKind::ProtocolParse,
            self.diagnostics.route_name,
            stage,
            diagnostic.code(),
            diagnostic.message(),
        );
    }

    /// Sends the standard ACP `initialize` request (id `0`) with explicit `clientCapabilities` and `clientInfo`.
    pub async fn initialize_with(
        &self,
        client_capabilities: Value,
        client_info: Value,
    ) -> Result<Value, RuntimeFailure> {
        let response = self
            .request_with_id(
                0,
                "initialize",
                json!({
                    "protocolVersion": ACP_PROTOCOL_VERSION,
                    "clientCapabilities": client_capabilities,
                    "clientInfo": client_info,
                }),
            )
            .await?;
        if response.get("protocolVersion").and_then(Value::as_u64) != Some(ACP_PROTOCOL_VERSION) {
            return Err((self.diagnostics.version_mismatch)());
        }
        Ok(response)
    }

    /// Sends the default pure-ACP `initialize` request (`fs: false`, `terminal: false`).
    pub async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.initialize_with(
            json!({
                "fs": {
                    "readTextFile": false,
                    "writeTextFile": false
                },
                "terminal": false
            }),
            json!({
                "name": "swallowtail",
                "version": env!("CARGO_PKG_VERSION")
            }),
        )
        .await
    }

    /// Allocates the next correlation id without sending a request yet.
    #[must_use]
    pub fn allocate_request_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Sends a JSON-RPC request and awaits its correlated response.
    pub async fn request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.begin_request(method, params).await?.await
    }

    /// Sends a JSON-RPC request and returns a [`PendingResponse`] future.
    pub async fn begin_request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<PendingResponse, RuntimeFailure> {
        let id = self.allocate_request_id();
        self.begin_request_with_id(id, method, params).await
    }

    /// Sends a JSON-RPC request with a specific correlation id and awaits its response.
    pub async fn request_with_id(
        &self,
        id: u64,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.begin_request_with_id(id, method, params).await?.await
    }

    /// Sends a JSON-RPC request with a specific correlation id and returns its [`PendingResponse`].
    pub async fn begin_request_with_id(
        &self,
        id: u64,
        method: &'static str,
        params: Value,
    ) -> Result<PendingResponse, RuntimeFailure> {
        let (sender, receiver) = oneshot::channel();
        {
            let mut pending = self.pending.lock().expect("ACP pending lock poisoned");
            if pending.closed {
                return Err((self.diagnostics.connection_closed)());
            }
            pending.requests.retain(|_, sender| !sender.is_canceled());
            if pending.requests.len() >= MAXIMUM_PENDING_REQUESTS
                || pending.requests.contains_key(&id)
            {
                return Err((self.diagnostics.correlation_limit)());
            }
            pending.requests.insert(id, sender);
        }
        if let Err(error) = self
            .writer
            .send_request(id, method, params, &self.diagnostics)
            .await
        {
            self.pending
                .lock()
                .expect("ACP pending lock poisoned")
                .requests
                .remove(&id);
            return Err(error);
        }
        Ok(PendingResponse {
            receiver,
            closed_error: self.diagnostics.connection_ended,
        })
    }

    /// Sends a JSON-RPC notification without expecting a response.
    pub async fn notify(&self, method: &'static str, params: Value) -> Result<(), RuntimeFailure> {
        self.writer
            .send_notification(method, params, &self.diagnostics)
            .await
    }

    /// Sends a JSON-RPC result response for a provider-initiated callback request.
    pub async fn send_result(&self, id: Value, result: Value) -> Result<(), RuntimeFailure> {
        self.writer.send_result(id, result, &self.diagnostics).await
    }

    /// Sends a JSON-RPC error response for a provider-initiated callback request.
    pub async fn send_error(
        &self,
        id: Value,
        code: i64,
        message: &'static str,
    ) -> Result<(), RuntimeFailure> {
        self.writer
            .send_error(id, code, message, &self.diagnostics)
            .await
    }

    /// Records the single active provider session identifier for this connection.
    pub fn set_session_id(&self, session_id: String) -> Result<(), RuntimeFailure> {
        let mut current = self.session_id.lock().expect("ACP session lock poisoned");
        if current.is_some() {
            return Err((self.diagnostics.session_duplicate)());
        }
        *current = Some(session_id);
        Ok(())
    }

    /// Returns the currently bound provider session identifier, if set.
    #[must_use]
    pub fn session_id(&self) -> Option<String> {
        self.session_id
            .lock()
            .expect("ACP session lock poisoned")
            .clone()
    }

    /// Verifies that `params["sessionId"]` matches the bound session identifier.
    pub fn verify_session(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let current = self.session_id.lock().expect("ACP session lock poisoned");
        if params.get("sessionId").and_then(Value::as_str) == current.as_deref() {
            Ok(())
        } else {
            Err((self.diagnostics.session_mismatch_callback)())
        }
    }

    /// Registers an active turn sink on this connection, rejecting concurrent active turns.
    pub fn set_active_turn(&self, turn: Arc<dyn AcpTurnSink>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err((self.diagnostics.turn_active)());
        }
        *active = Some(turn);
        Ok(())
    }

    /// Returns the currently registered active turn sink, if any.
    #[must_use]
    pub fn active_turn(&self) -> Option<Arc<dyn AcpTurnSink>> {
        self.active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
    }

    /// Clears the active turn slot if it still points to `turn`.
    pub fn clear_active_turn(&self, turn: &Arc<dyn AcpTurnSink>) {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *active = None;
        }
    }

    /// Initiates graceful transport shutdown (invoking `on_begin_close` first).
    pub async fn begin_close(&self) {
        self.hooks.on_begin_close(self).await;
        self.writer.begin_close().await;
    }

    /// Returns whether the connection pump has closed and failed pending requests.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.pending
            .lock()
            .expect("ACP pending lock poisoned")
            .closed
    }

    /// Forcefully stops the underlying transport.
    pub async fn cancel_session(&self) -> Result<(), RuntimeFailure> {
        self.writer.force_stop().await
    }

    /// Returns the cleanup outcome recorded when the pump finished.
    #[must_use]
    pub fn cleanup_outcome(&self) -> CleanupOutcome {
        self.cleanup
            .lock()
            .expect("ACP cleanup lock poisoned")
            .clone()
            .unwrap_or_else(|| CleanupOutcome::Failed((self.diagnostics.cleanup_missing)()))
    }

    /// Runs the connection receive pump until EOF, protocol error, or transport error.
    pub async fn pump(self: Arc<Self>) {
        let Some(mut reader) = self.reader.lock().expect("ACP reader lock poisoned").take() else {
            return;
        };
        let mut transport_failure = None;
        loop {
            match reader.next_batch(&self.diagnostics).await {
                Ok(ReadStep::Messages(messages)) => {
                    for message in messages {
                        if let Err(error) = self.dispatch(message).await {
                            self.emit_protocol_debug(&error, "acp.pump.dispatch");
                            transport_failure = Some(error);
                            break;
                        }
                    }
                    if transport_failure.is_some() {
                        break;
                    }
                    self.hooks.on_batch_complete();
                }
                Ok(ReadStep::EndOfStream) => break,
                Err((stage, error)) => {
                    self.emit_protocol_debug(&error, stage);
                    transport_failure = Some(error);
                    break;
                }
            }
        }
        let (finish_error, cleanup) = reader
            .finish_and_cleanup(transport_failure.is_some(), &self.diagnostics)
            .await;
        if let Some((stage, error)) = finish_error {
            self.emit_protocol_debug(&error, stage);
            transport_failure = Some(error);
        }
        *self.cleanup.lock().expect("ACP cleanup lock poisoned") = Some(cleanup);
        let error = transport_failure.unwrap_or_else(|| (self.diagnostics.connection_ended)());
        self.hooks.on_transport_ended(&error).await;
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail(&error);
        }
        self.close_and_fail_pending(error);
    }

    /// Atomically marks the connection closed and fails all pending request waiters.
    fn close_and_fail_pending(&self, error: RuntimeFailure) {
        let pending = {
            let mut state = self.pending.lock().expect("ACP pending lock poisoned");
            state.closed = true;
            std::mem::take(&mut state.requests)
        };
        for (_, sender) in pending {
            let _ = sender.send(Err(error.clone()));
        }
    }

    async fn dispatch(&self, message: Message) -> Result<(), RuntimeFailure> {
        match message {
            Message::Response { id, result } => self.dispatch_response(id, result),
            Message::Notification { method, params } => {
                self.dispatch_notification(&method, &params)
            }
            Message::Request { id, method, params } => {
                self.dispatch_request(id, &method, &params).await
            }
        }
    }

    fn dispatch_response(
        &self,
        id: Value,
        result: Result<Value, RpcError>,
    ) -> Result<(), RuntimeFailure> {
        let id = id.as_u64().ok_or_else(self.diagnostics.malformed)?;
        let sender = self
            .pending
            .lock()
            .expect("ACP pending lock poisoned")
            .requests
            .remove(&id)
            .ok_or_else(self.diagnostics.unknown_response)?;
        self.hooks.on_response(id, &result);
        let _ = sender.send(result.map_err(|error| (self.diagnostics.classify_rpc_error)(&error)));
        Ok(())
    }

    fn dispatch_notification(&self, method: &str, params: &Value) -> Result<(), RuntimeFailure> {
        match method {
            "session/update" => {
                if self.hooks.on_session_update(params)? == SessionUpdateDisposition::Handled {
                    return Ok(());
                }
                let active = self.active_turn();
                match active {
                    Some(active) => active.handle_update(params),
                    None if is_session_scoped_metadata_update(params) => Ok(()),
                    None => Err((self.diagnostics.update_without_turn)()),
                }
            }
            "session/cancel" => Err((self.diagnostics.agent_cancel_unsupported)()),
            method if method.starts_with('_') => Ok(()),
            method => {
                if self.hooks.on_notification(method, params)? {
                    Ok(())
                } else {
                    Err((self.diagnostics.notification_unsupported)())
                }
            }
        }
    }

    async fn dispatch_request(
        &self,
        id: Value,
        method: &str,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        if method.starts_with('_') {
            return self.send_error(id, -32601, "Method not found").await;
        }
        match self
            .hooks
            .handle_request(self, id.clone(), method, params)
            .await?
        {
            RequestDisposition::Handled => Ok(()),
            RequestDisposition::RejectPermission => self.reject_permission(id, params).await,
            RequestDisposition::Unsupported => {
                self.send_error(id, -32601, "Method not found").await?;
                Err((self.diagnostics.callback_unsupported)())
            }
        }
    }

    /// Standard ACP reject-permission handler: validates payload, records observation,
    /// sends `session/cancel`, and replies with `{"outcome": {"outcome": "cancelled"}}`.
    pub async fn reject_permission(&self, id: Value, params: &Value) -> Result<(), RuntimeFailure> {
        self.verify_session(params)?;
        let options = params
            .get("options")
            .and_then(Value::as_array)
            .ok_or_else(self.diagnostics.malformed)?;
        if options.len() > 32
            || params
                .get("toolCall")
                .and_then(|tool| tool.get("toolCallId"))
                .and_then(Value::as_str)
                .is_none()
        {
            return Err((self.diagnostics.malformed)());
        }
        let turn = self
            .active_turn()
            .ok_or_else(self.diagnostics.permission_without_turn)?;
        turn.observe_permission(&id)?;
        self.notify("session/cancel", json!({"sessionId": turn.session_id()}))
            .await?;
        self.send_result(id, json!({"outcome": {"outcome": "cancelled"}}))
            .await
    }
}
