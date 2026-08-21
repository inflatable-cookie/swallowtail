use super::failure::{failure, protocol_failure};
use super::turn::SidecarActiveTurn;
use super::wire::{PiSdkSidecarCommand, encode_command};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, DebugObservationKind, HostServices, ProcessHandle, ProcessInputChunk,
    RuntimeFailure,
};

mod pump;

const MAXIMUM_PENDING_COMMANDS: usize = 16;

pub(crate) struct CommandResult {
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) struct SidecarConnection {
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
    pending: Mutex<BTreeMap<String, PendingCommand>>,
    used_ids: Mutex<BTreeSet<String>>,
    active_turn: Mutex<Option<Arc<SidecarActiveTurn>>>,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl SidecarConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        Arc::new(Self {
            process,
            services,
            pending: Mutex::new(BTreeMap::new()),
            used_ids: Mutex::new(BTreeSet::new()),
            active_turn: Mutex::new(None),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
        })
    }

    pub(crate) fn emit_protocol_debug(&self, error: &RuntimeFailure, stage: &'static str) {
        let diagnostic = error.diagnostic();
        self.services.emit_failure_debug(
            DebugObservationKind::ProtocolParse,
            "pi.sdk-sidecar",
            stage,
            diagnostic.code(),
            diagnostic.message(),
        );
    }

    pub(crate) async fn command(
        &self,
        id: String,
        command: PiSdkSidecarCommand,
        params: Value,
    ) -> Result<CommandResult, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(connection_closed());
        }
        if !self
            .used_ids
            .lock()
            .expect("sidecar command id lock poisoned")
            .insert(id.clone())
        {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.command_id_reused",
                "Pi SDK sidecar command id was reused",
            ));
        }
        let (sender, response) = response_channel();
        {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            if pending.len() >= MAXIMUM_PENDING_COMMANDS {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.command_capacity_exceeded",
                    "Pi SDK sidecar exceeded the pending command limit",
                ));
            }
            pending.insert(id.clone(), PendingCommand { command, sender });
        }
        let bytes = encode_command(&id, command, params).map_err(|_| protocol_failure())?;
        if let Err(error) = self
            .process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
        {
            self.pending
                .lock()
                .expect("sidecar pending lock poisoned")
                .remove(&id);
            return Err(error);
        }
        response.await
    }

    pub(crate) fn set_active_turn(
        &self,
        turn: Arc<SidecarActiveTurn>,
    ) -> Result<(), RuntimeFailure> {
        let mut active = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.turn_active",
                "Pi SDK sidecar session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<SidecarActiveTurn>) {
        let mut active = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned");
        if active
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *active = None;
        }
    }

    pub(crate) async fn begin_close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        let _ = self.process.close_stdin().await;
        let _ = self.process.request_stop().await;
    }

    pub(crate) async fn force_stop(&self) -> Result<(), RuntimeFailure> {
        self.closed.store(true, Ordering::SeqCst);
        self.process.force_stop().await
    }

    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.cleanup
            .lock()
            .expect("sidecar cleanup lock poisoned")
            .clone()
            .unwrap_or_else(|| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.pi.sdk-sidecar.cleanup_missing",
                    "Pi SDK sidecar process cleanup did not complete",
                ))
            })
    }
}

struct PendingCommand {
    command: PiSdkSidecarCommand,
    sender: ResponseSender,
}

#[derive(Default)]
struct ResponseState {
    result: Option<Result<CommandResult, RuntimeFailure>>,
    waiter: Option<Waker>,
}

struct ResponseSender(Arc<Mutex<ResponseState>>);
struct ResponseFuture(Arc<Mutex<ResponseState>>);

fn response_channel() -> (ResponseSender, ResponseFuture) {
    let state = Arc::new(Mutex::new(ResponseState::default()));
    (ResponseSender(Arc::clone(&state)), ResponseFuture(state))
}

impl ResponseSender {
    fn complete(self, result: Result<CommandResult, RuntimeFailure>) {
        let mut state = self.0.lock().expect("sidecar response lock poisoned");
        state.result = Some(result);
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

impl Future for ResponseFuture {
    type Output = Result<CommandResult, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("sidecar response lock poisoned");
        if let Some(result) = state.result.take() {
            Poll::Ready(result)
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

fn connection_closed() -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.connection_closed",
        "Pi SDK sidecar connection is closed",
    )
}
