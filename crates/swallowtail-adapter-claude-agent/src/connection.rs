use crate::failure::{failure, malformed, protocol_failure};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::future::Future;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, FramingLimits, Message, NdjsonDecoder, encode_error, encode_notification,
    encode_request, encode_result,
};
use swallowtail_runtime::{
    CleanupOutcome, ProcessHandle, ProcessInputChunk, ProcessOutputStream, ResourceLease,
    RuntimeFailure, WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
};

use self::response::{PendingResponse, ResponseSender, response_channel};

const MAXIMUM_PENDING_REQUESTS: usize = 32;
const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_RECEIVED_FRAME_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_RECEIVE_BUFFER_BYTES: usize = 8 * 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(MAXIMUM_RECEIVED_FRAME_BYTES, MAXIMUM_RECEIVE_BUFFER_BYTES);

pub(crate) struct AcpConnection {
    process: Arc<dyn ProcessHandle>,
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    next_id: AtomicU64,
    pending: Mutex<BTreeMap<u64, ResponseSender>>,
    session_id: Mutex<Option<String>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Arc<dyn WorkingResourceIoService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            process,
            resource,
            resource_io,
            next_id: AtomicU64::new(1),
            pending: Mutex::new(BTreeMap::new()),
            session_id: Mutex::new(None),
            active_turn: Mutex::new(None),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
        })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        let response = self
            .request_with_id(
                0,
                "initialize",
                json!({
                    "protocolVersion": ACP_PROTOCOL_VERSION,
                    "clientCapabilities": {
                        "fs": {"readTextFile": true, "writeTextFile": false}
                    },
                    "clientInfo": {
                        "name": "swallowtail",
                        "title": "Swallowtail",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            )
            .await?;
        if response.get("protocolVersion").and_then(Value::as_u64) != Some(ACP_PROTOCOL_VERSION) {
            return Err(failure(
                "swallowtail.claude_agent.acp.version_mismatch",
                "Claude Agent negotiated an incompatible ACP version",
            ));
        }
        Ok(response)
    }

    pub(crate) async fn new_session(
        &self,
        cwd: String,
        model: &str,
    ) -> Result<Value, RuntimeFailure> {
        self.request(
            "session/new",
            json!({
                "cwd": cwd,
                "mcpServers": [],
                "_meta": {
                    "claudeCode": {
                        "options": {
                            "tools": ["Read", "Glob", "Grep"],
                            "settings": {
                                "model": model,
                                "availableModels": [model]
                            }
                        }
                    }
                }
            }),
        )
        .await
    }

    pub(crate) async fn close_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        let response = self
            .request("session/close", json!({"sessionId": session_id}))
            .await?;
        if response.as_object().is_some_and(serde_json::Map::is_empty) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.claude_agent.acp.close_malformed",
                "Claude Agent returned a malformed ACP session-close response",
            ))
        }
    }

    pub(crate) async fn set_config_option(
        &self,
        session_id: &str,
        config_id: &'static str,
        value: &str,
    ) -> Result<Value, RuntimeFailure> {
        self.request(
            "session/set_config_option",
            json!({"sessionId": session_id, "configId": config_id, "value": value}),
        )
        .await
    }

    pub(crate) async fn request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.begin_request(method, params).await?.await
    }

    pub(crate) async fn begin_request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<PendingResponse, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.begin_request_with_id(id, method, params).await
    }

    async fn request_with_id(
        &self,
        id: u64,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.begin_request_with_id(id, method, params).await?.await
    }

    async fn begin_request_with_id(
        &self,
        id: u64,
        method: &'static str,
        params: Value,
    ) -> Result<PendingResponse, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.claude_agent.acp.connection_closed",
                "Claude Agent ACP connection is closed",
            ));
        }
        let (sender, response) = response_channel();
        {
            let mut pending = self.pending.lock().expect("ACP pending lock poisoned");
            if pending.len() >= MAXIMUM_PENDING_REQUESTS || pending.insert(id, sender).is_some() {
                return Err(failure(
                    "swallowtail.claude_agent.acp.correlation_limit",
                    "Claude Agent ACP request correlation limit was exceeded",
                ));
            }
        }
        let bytes = encode_request(id, method, params).map_err(|_| protocol_failure())?;
        if let Err(error) = self.write(bytes).await {
            self.pending
                .lock()
                .expect("ACP pending lock poisoned")
                .remove(&id);
            return Err(error);
        }
        Ok(PendingResponse(response))
    }

    pub(crate) async fn notify(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<(), RuntimeFailure> {
        let bytes = encode_notification(method, params).map_err(|_| protocol_failure())?;
        self.write(bytes).await
    }

    pub(crate) fn set_session_id(&self, session_id: String) -> Result<(), RuntimeFailure> {
        let mut current = self.session_id.lock().expect("ACP session lock poisoned");
        if current.is_some() {
            return Err(failure(
                "swallowtail.claude_agent.acp.session_duplicate",
                "Claude Agent returned more than one ACP session",
            ));
        }
        *current = Some(session_id);
        Ok(())
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.claude_agent.acp.turn_active",
                "Claude Agent session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *active = None;
        }
    }

    pub(crate) async fn begin_close(&self) {
        let _ = self.process.close_stdin().await;
        let _ = self.process.request_stop().await;
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub(crate) async fn cancel_session(&self) -> Result<(), RuntimeFailure> {
        self.process.force_stop().await
    }

    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.cleanup
            .lock()
            .expect("ACP cleanup lock poisoned")
            .clone()
            .unwrap_or_else(|| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.claude_agent.acp.cleanup_missing",
                    "Claude Agent process cleanup did not complete",
                ))
            })
    }

    async fn write(&self, bytes: Vec<u8>) -> Result<(), RuntimeFailure> {
        self.process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
    }

    fn fail_pending(&self, error: RuntimeFailure) {
        let pending = std::mem::take(&mut *self.pending.lock().expect("ACP pending lock poisoned"));
        for (_, sender) in pending {
            sender.complete(Err(error.clone()));
        }
    }
}

mod dispatch;
mod pump;
mod response;
