use super::{
    failure::{failure, malformed, protocol_failure},
    turn::ActiveTurn,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    task::{Context, Poll, Waker},
};
use swallowtail_core::SafeDiagnostic;
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits,
    Message, NdjsonDecoder, encode_error, encode_notification, encode_request, encode_result,
    is_session_scoped_metadata_update,
};
use swallowtail_runtime::{
    CleanupOutcome, DebugObservationKind, HostServices, ProcessHandle, ProcessInputChunk,
    ProcessOutputStream, RuntimeFailure,
};

const MAXIMUM_PENDING_REQUESTS: usize = 32;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn connection_closed() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.connection_closed",
        "OpenCode ACP connection is closed",
    )
}

pub(crate) struct AcpConnection {
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
    next_id: AtomicU64,
    pending: Mutex<BTreeMap<u64, ResponseSender>>,
    session_id: Mutex<Option<String>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl AcpConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        Arc::new(Self {
            process,
            services,
            next_id: AtomicU64::new(1),
            pending: Mutex::new(BTreeMap::new()),
            session_id: Mutex::new(None),
            active_turn: Mutex::new(None),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
        })
    }

    pub(crate) fn emit_protocol_debug(&self, error: &RuntimeFailure, stage: &'static str) {
        let diagnostic = error.diagnostic();
        self.services.emit_failure_debug(
            DebugObservationKind::ProtocolParse,
            "opencode.acp",
            stage,
            diagnostic.code(),
            diagnostic.message(),
        );
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        let response = self
            .request_with_id(
                0,
                "initialize",
                json!({
                    "protocolVersion": ACP_PROTOCOL_VERSION,
                    "clientCapabilities": {
                        "fs": {
                            "readTextFile": false,
                            "writeTextFile": false
                        },
                        "terminal": false
                    },
                    "clientInfo": {
                        "name": "swallowtail",
                        "version": env!("CARGO_PKG_VERSION")
                    }
                }),
            )
            .await?;
        if response.get("protocolVersion").and_then(Value::as_u64) != Some(ACP_PROTOCOL_VERSION) {
            return Err(failure(
                "swallowtail.opencode.acp.version_mismatch",
                "OpenCode answered a provider-fixed ACP protocolVersion other than 1",
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
            return Err(connection_closed());
        }
        let bytes = encode_request(id, method, params).map_err(|_| protocol_failure())?;
        let (sender, response) = response_channel();
        {
            let mut pending = self.pending.lock().expect("ACP pending lock poisoned");
            // The pump can close the connection after the fast check above.
            // Recheck under the lock `fail_pending` drains, so a sender cannot
            // register after that drain and wait forever.
            if self.closed.load(Ordering::SeqCst) {
                return Err(connection_closed());
            }
            if pending.len() >= MAXIMUM_PENDING_REQUESTS || pending.insert(id, sender).is_some() {
                return Err(failure(
                    "swallowtail.opencode.acp.correlation_limit",
                    "OpenCode ACP request correlation limit was exceeded",
                ));
            }
        }
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
                "swallowtail.opencode.acp.session_duplicate",
                "OpenCode returned more than one ACP session",
            ));
        }
        *current = Some(session_id);
        Ok(())
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("ACP active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.opencode.acp.turn_active",
                "OpenCode session already has an active turn",
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
        let session_id = self
            .session_id
            .lock()
            .expect("ACP session lock poisoned")
            .clone();
        if let Some(session_id) = session_id {
            // Write the documented `session/close` request before the stdin
            // close, but never wait on its response: a harness that stops
            // answering must not keep stdin open and the stop request unsent.
            // The pump owns settlement of any pending correlation id.
            if let Ok(close) = self
                .begin_request("session/close", json!({"sessionId": session_id}))
                .await
            {
                drop(close);
            }
        }
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
                    "swallowtail.opencode.acp.cleanup_missing",
                    "OpenCode process cleanup did not complete",
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

include!("connection_dispatch.rs");
include!("connection_pump.rs");

include!("connection_response.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use futures_executor::block_on;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{BoxFuture, ProcessExit, ProcessOutputChunk};

    /// Accepts writes but never answers, as a harness that hangs on
    /// `session/close` does.
    struct SilentCloseProcess {
        writes: Arc<AtomicUsize>,
        stdin_closed: Arc<AtomicBool>,
        stop_requested: Arc<AtomicBool>,
    }

    impl ProcessHandle for SilentCloseProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.stdin_closed.store(true, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.stop_requested.store(true, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(std::future::pending())
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(std::future::pending())
        }
    }

    #[test]
    fn receive_framing_profile_is_explicit() {
        assert_eq!(
            RECEIVE_FRAMING_LIMITS.maximum_frame_bytes(),
            DEFAULT_MAX_FRAME_BYTES
        );
        assert_eq!(
            RECEIVE_FRAMING_LIMITS.maximum_buffer_bytes(),
            DEFAULT_MAX_BUFFER_BYTES
        );
    }

    #[test]
    fn a_silent_session_close_still_closes_stdin_and_requests_stop() {
        let writes = Arc::new(AtomicUsize::new(0));
        let stdin_closed = Arc::new(AtomicBool::new(false));
        let stop_requested = Arc::new(AtomicBool::new(false));
        let process = SilentCloseProcess {
            writes: Arc::clone(&writes),
            stdin_closed: Arc::clone(&stdin_closed),
            stop_requested: Arc::clone(&stop_requested),
        };
        let connection = AcpConnection::new(
            Arc::new(process),
            HostServices::new(
                ExecutionHostId::new("opencode.acp.close-test").expect("host id is valid"),
            ),
        );
        connection
            .set_session_id("ses_fixture".to_owned())
            .expect("session id sets once");

        // A harness that never answers `session/close` must not hold the
        // shutdown stages: the call has to return on its own.
        let (sender, receiver) = std::sync::mpsc::channel();
        let closing = {
            let connection = Arc::clone(&connection);
            std::thread::spawn(move || {
                block_on(connection.begin_close());
                let _ = sender.send(());
            })
        };
        receiver
            .recv_timeout(Duration::from_secs(10))
            .expect("begin_close must not wait on a silent session/close response");
        closing.join().expect("close thread joins");

        assert_eq!(
            writes.load(Ordering::SeqCst),
            1,
            "the documented session/close request must still be written"
        );
        assert!(
            stdin_closed.load(Ordering::SeqCst),
            "stdin must close even when session/close is never answered"
        );
        assert!(
            stop_requested.load(Ordering::SeqCst),
            "the stop request must run even when session/close is never answered"
        );
    }
}
