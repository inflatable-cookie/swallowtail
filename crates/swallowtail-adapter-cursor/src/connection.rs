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
use swallowtail_core::{SafeDiagnostic, SessionRef};
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, FramingLimits, Message, NdjsonDecoder, encode_error, encode_notification,
    encode_request, encode_result, is_session_scoped_metadata_update,
};
use swallowtail_runtime::{
    CleanupOutcome, ProcessHandle, ProcessInputChunk, ProcessOutputStream, ResourceLease,
    RuntimeFailure, WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
};

const MAXIMUM_PENDING_REQUESTS: usize = 32;
const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_RECEIVE_FRAME_BYTES: usize = 16 * 1024 * 1024;
const MAXIMUM_RECEIVE_BUFFER_BYTES: usize = 32 * 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(MAXIMUM_RECEIVE_FRAME_BYTES, MAXIMUM_RECEIVE_BUFFER_BYTES);

struct AttachmentRecoveryPhase {
    response_id: u64,
    session: SessionRef,
    response_seen: bool,
    updates: usize,
    bytes: usize,
    batch_completion: Option<ResponseSender>,
}

pub(crate) struct AcpConnection {
    process: Arc<dyn ProcessHandle>,
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    next_id: AtomicU64,
    pending: Mutex<BTreeMap<u64, ResponseSender>>,
    session_id: Mutex<Option<String>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    attachment_recovery: Mutex<Option<AttachmentRecoveryPhase>>,
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
            attachment_recovery: Mutex::new(None),
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
                "swallowtail.cursor.acp.version_mismatch",
                "Cursor Agent negotiated an incompatible ACP version",
            ));
        }
        Ok(response)
    }

    pub(crate) async fn request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.begin_request(method, params).await?.await
    }

    pub(crate) async fn recover_session_attachment(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<Value, RuntimeFailure> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (batch_sender, batch_completion) = response_channel();
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned") = Some(AttachmentRecoveryPhase {
            response_id: id,
            session: session.clone(),
            response_seen: false,
            updates: 0,
            bytes: 0,
            batch_completion: Some(batch_sender),
        });
        let pending = self
            .begin_request_with_id(
                id,
                "session/load",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await;
        let response = match pending {
            Ok(response) => {
                let response = response.await;
                let batch = batch_completion.await;
                match (response, batch) {
                    (Ok(response), Ok(_)) => Ok(response),
                    (Err(error), _) | (_, Err(error)) => Err(error),
                }
            }
            Err(error) => Err(error),
        };
        let phase = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .take();
        match (response, phase) {
            (
                Ok(response),
                Some(AttachmentRecoveryPhase {
                    response_seen: true,
                    ..
                }),
            ) => Ok(response),
            (Err(error), _) => Err(error),
            _ => Err(protocol_failure()),
        }
    }

    fn complete_attachment_recovery_batch(&self) {
        let sender = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .as_mut()
            .filter(|phase| phase.response_seen)
            .and_then(|phase| phase.batch_completion.take());
        if let Some(sender) = sender {
            sender.complete(Ok(Value::Null));
        }
    }

    fn fail_attachment_recovery(&self, error: RuntimeFailure) {
        let sender = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .as_mut()
            .and_then(|phase| phase.batch_completion.take());
        if let Some(sender) = sender {
            sender.complete(Err(error));
        }
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
                "swallowtail.cursor.acp.connection_closed",
                "Cursor Agent ACP connection is closed",
            ));
        }
        let (sender, response) = response_channel();
        {
            let mut pending = self.pending.lock().expect("ACP pending lock poisoned");
            if pending.len() >= MAXIMUM_PENDING_REQUESTS || pending.insert(id, sender).is_some() {
                return Err(failure(
                    "swallowtail.cursor.acp.correlation_limit",
                    "Cursor Agent ACP request correlation limit was exceeded",
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
                "swallowtail.cursor.acp.session_duplicate",
                "Cursor Agent returned more than one ACP session",
            ));
        }
        *current = Some(session_id);
        Ok(())
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.cursor.acp.turn_active",
                "Cursor Agent session already has an active turn",
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
                    "swallowtail.cursor.acp.cleanup_missing",
                    "Cursor Agent process cleanup did not complete",
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

include!("connection/dispatch.rs");
include!("connection/pump.rs");
include!("connection/response.rs");
