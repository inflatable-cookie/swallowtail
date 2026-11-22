use super::failure::{failure, protocol_failure};
use super::replay::ReplayCollector;
use super::turn::SidecarActiveTurn;
use super::wire::{MAXIMUM_COMMAND_ID_BYTES, PiSdkSidecarCommand, encode_command};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
#[cfg(test)]
use std::sync::Barrier;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{SafeDiagnostic, SessionRef};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, DebugObservationKind, HostServices,
    MonotonicInstant, ProcessHandle, ProcessInputChunk, RuntimeFailure,
};

#[cfg(test)]
mod connection_tests;
mod pump;

const MAXIMUM_PENDING_COMMANDS: usize = 16;
/// How long one `close` acknowledgement may hold the shutdown stages.
const CLOSE_ACKNOWLEDGEMENT_GRACE_TICKS: u64 = 500;

pub(crate) struct CommandResult {
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) struct SidecarConnection {
    process: Arc<dyn ProcessHandle>,
    services: HostServices,
    pending: Mutex<BTreeMap<String, PendingCommand>>,
    superseded_aborts: Mutex<BTreeSet<String>>,
    abandoned_close: Mutex<Option<String>>,
    used_ids: Mutex<BTreeSet<String>>,
    used_caller_turn_ids: Mutex<BTreeSet<String>>,
    used_caller_command_ids: Mutex<BTreeSet<String>>,
    next_wire_id: AtomicU64,
    active_turn: Mutex<Option<Arc<SidecarActiveTurn>>>,
    replay: Mutex<Option<ReplayCollector>>,
    closed: AtomicBool,
    terminal_error: Mutex<Option<SafeDiagnostic>>,
    cleanup: Mutex<Option<CleanupOutcome>>,
    #[cfg(test)]
    send_registration_gate: Mutex<Option<Arc<(Barrier, Barrier)>>>,
}

impl SidecarConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        Arc::new(Self {
            process,
            services,
            pending: Mutex::new(BTreeMap::new()),
            superseded_aborts: Mutex::new(BTreeSet::new()),
            abandoned_close: Mutex::new(None),
            used_ids: Mutex::new(BTreeSet::new()),
            used_caller_turn_ids: Mutex::new(BTreeSet::new()),
            used_caller_command_ids: Mutex::new(BTreeSet::new()),
            next_wire_id: AtomicU64::new(1),
            active_turn: Mutex::new(None),
            replay: Mutex::new(None),
            closed: AtomicBool::new(false),
            terminal_error: Mutex::new(None),
            cleanup: Mutex::new(None),
            #[cfg(test)]
            send_registration_gate: Mutex::new(None),
        })
    }

    /// Allocates a unique, bounded, connection-owned wire command ID.
    ///
    /// All caller-derived commands (`prompt`, `abort`, `deadline-abort`,
    /// `close`, `steer`, `follow-up`) use this namespace so no caller-supplied
    /// identifier can collide with another command's wire ID or exceed
    /// `MAXIMUM_COMMAND_ID_BYTES`.
    pub(crate) fn next_command_id(&self, prefix: &str) -> String {
        let seq = self.next_wire_id.fetch_add(1, Ordering::SeqCst);
        let id = format!("{prefix}:w-{seq}");
        debug_assert!(id.len() <= MAXIMUM_COMMAND_ID_BYTES);
        id
    }

    /// Reserves a caller-supplied turn ID for `prompt` independently of wire
    /// IDs and scheduled command IDs so repeating a settled turn ID is rejected
    /// before dispatch without colliding with connection-owned wire IDs.
    pub(crate) fn reserve_caller_turn_id(&self, turn_id: &str) -> Result<(), RuntimeFailure> {
        if !self
            .used_caller_turn_ids
            .lock()
            .expect("sidecar caller turn id lock poisoned")
            .insert(turn_id.to_owned())
        {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.command_id_reused",
                "Pi SDK sidecar command id was reused",
            ));
        }
        Ok(())
    }

    /// Reserves a caller-supplied scheduled command ID independently of wire
    /// IDs so duplicate caller command IDs are rejected without colliding with
    /// connection-owned wire IDs.
    pub(crate) fn reserve_caller_command_id(&self, caller_id: &str) -> Result<(), RuntimeFailure> {
        if !self
            .used_caller_command_ids
            .lock()
            .expect("sidecar caller command id lock poisoned")
            .insert(caller_id.to_owned())
        {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.command_id_reused",
                "Pi SDK sidecar command id was reused",
            ));
        }
        Ok(())
    }

    pub(crate) fn services(&self) -> &HostServices {
        &self.services
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
        self.send_command(id, command, params, None, None).await
    }

    pub(crate) async fn command_tracked(
        &self,
        id: String,
        command: PiSdkSidecarCommand,
        params: Value,
        dispatched: &AtomicBool,
    ) -> Result<CommandResult, RuntimeFailure> {
        self.send_command(id, command, params, Some(dispatched), None)
            .await
    }

    pub(crate) async fn abort_command(
        &self,
        id: String,
        params: Value,
        turn: Arc<SidecarActiveTurn>,
    ) -> Result<CommandResult, RuntimeFailure> {
        self.send_command(id, PiSdkSidecarCommand::Abort, params, None, Some(turn))
            .await
    }

    pub(crate) fn complete_sibling_aborts(&self, turn: &Arc<SidecarActiveTurn>, success: bool) {
        let siblings: Vec<(String, ResponseSender)> = {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            let ids: Vec<String> = pending
                .iter()
                .filter(|(_, entry)| {
                    entry
                        .abort_turn
                        .as_ref()
                        .is_some_and(|other| Arc::ptr_eq(other, turn))
                })
                .map(|(id, _)| id.clone())
                .collect();
            let mut drained = Vec::with_capacity(ids.len());
            for id in ids {
                if let Some(entry) = pending.remove(&id) {
                    drained.push((id, entry.sender));
                }
            }
            drained
        };
        if siblings.is_empty() {
            return;
        }
        let mut superseded = self
            .superseded_aborts
            .lock()
            .expect("sidecar superseded-aborts lock poisoned");
        for (id, sender) in siblings {
            superseded.insert(id);
            sender.complete(Ok(CommandResult {
                success,
                data: None,
            }));
        }
    }

    async fn send_command(
        &self,
        id: String,
        command: PiSdkSidecarCommand,
        params: Value,
        dispatched: Option<&AtomicBool>,
        abort_turn: Option<Arc<SidecarActiveTurn>>,
    ) -> Result<CommandResult, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(self.closed_failure());
        }
        #[cfg(test)]
        if let Some(gate) = self
            .send_registration_gate
            .lock()
            .expect("sidecar test send gate lock poisoned")
            .clone()
        {
            gate.0.wait();
            gate.1.wait();
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
        // Encode before registering the correlation id: a record that cannot be
        // encoded must not consume one of the bounded pending slots forever.
        let bytes = encode_command(&id, command, params).map_err(|_| protocol_failure())?;
        let (sender, response) = response_channel();
        let is_abort = abort_turn.is_some();
        {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            // The pump can close the connection after the fast check above.
            // Recheck under the lock used to drain pending work, so a sender
            // cannot register after that drain and wait forever.
            if self.closed.load(Ordering::SeqCst) {
                return Err(self.closed_failure());
            }
            if pending.len() >= MAXIMUM_PENDING_COMMANDS {
                return Err(failure(
                    "swallowtail.pi.sdk-sidecar.command_capacity_exceeded",
                    "Pi SDK sidecar exceeded the pending command limit",
                ));
            }
            pending.insert(
                id.clone(),
                PendingCommand {
                    command,
                    sender,
                    abort_turn,
                },
            );
        }
        struct PendingGuard<'a> {
            pending: &'a Mutex<BTreeMap<String, PendingCommand>>,
            superseded_aborts: Option<&'a Mutex<BTreeSet<String>>>,
            id: Option<String>,
        }
        impl Drop for PendingGuard<'_> {
            fn drop(&mut self) {
                if let Some(id) = self.id.take() {
                    let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
                    if pending.remove(&id).is_some()
                        && let Some(superseded) = self.superseded_aborts
                    {
                        superseded
                            .lock()
                            .expect("sidecar superseded-aborts lock poisoned")
                            .insert(id);
                    }
                }
            }
        }
        let mut guard = PendingGuard {
            pending: &self.pending,
            superseded_aborts: is_abort.then_some(&self.superseded_aborts),
            id: Some(id),
        };
        if let Some(dispatched) = dispatched {
            dispatched.store(true, Ordering::SeqCst);
        }
        self.process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await?;
        let result = response.await;
        guard.id = None;
        result
    }

    pub(crate) fn set_active_turn(
        &self,
        turn: Arc<SidecarActiveTurn>,
    ) -> Result<(), RuntimeFailure> {
        let mut active = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned");
        if self.closed.load(Ordering::SeqCst) {
            return Err(self.closed_failure());
        }
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.turn_active",
                "Pi SDK sidecar session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    /// Closes the connection to any further commands or turns when `turn`'s
    /// abort was rejected while `turn` is still the active unfinished turn.
    ///
    /// If `turn` already settled (or a replacement turn now owns the slot),
    /// the late refusal is ignored so it cannot terminate a replacement turn.
    pub(crate) fn close_on_rejected_abort(
        &self,
        turn: &Arc<SidecarActiveTurn>,
        diagnostic: SafeDiagnostic,
    ) -> bool {
        let mut active = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned");
        let is_current_unfinished = !turn.is_finished()
            && active
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, turn));
        if !is_current_unfinished {
            return false;
        }
        let pending = {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            *self
                .terminal_error
                .lock()
                .expect("sidecar terminal-error lock poisoned") = Some(diagnostic.clone());
            self.closed.store(true, Ordering::SeqCst);
            std::mem::take(&mut *pending)
        };
        let active_turn = active.take();
        drop(active);
        for (_, pending) in pending {
            pending
                .sender
                .complete(Err(RuntimeFailure::new(diagnostic.clone())));
        }
        if let Some(active_turn) = active_turn
            && !active_turn.is_finished()
        {
            active_turn.fail_rejected_abort(diagnostic);
        }
        true
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

    /// Arms the bounded replay collector for one in-flight `session_replay`
    /// command. Only one replay phase may be armed at a time.
    pub(crate) fn arm_replay(&self, session: SessionRef) -> Result<(), RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(connection_closed());
        }
        let mut replay = self.replay.lock().expect("sidecar replay lock poisoned");
        if replay.is_some() {
            return Err(failure(
                "swallowtail.pi.sdk-sidecar.replay_active",
                "Pi SDK sidecar already has an active replay phase",
            ));
        }
        *replay = Some(ReplayCollector::new(session));
        Ok(())
    }

    pub(crate) fn take_replay(&self) -> Option<ReplayCollector> {
        self.replay
            .lock()
            .expect("sidecar replay lock poisoned")
            .take()
    }

    pub(crate) async fn begin_close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        let _ = self.process.close_stdin().await;
        let _ = self.process.request_stop().await;
    }

    /// Closes one session through the documented `close` command, then always
    /// runs the process shutdown stages.
    ///
    /// The acknowledgement is best effort: the shutdown stages run as soon as
    /// one grace expires, so a harness that never answers `close` cannot keep
    /// stdin open or the stop request unsent. Any outstanding acknowledgement
    /// is dropped before process shutdown begins.
    pub(crate) async fn close_session(&self, id: String) {
        let mut close = Box::pin(self.command(id.clone(), PiSdkSidecarCommand::Close, json!({})));
        let mut grace = self.close_acknowledgement_wait();
        let acknowledged = std::future::poll_fn(|context| {
            if close.as_mut().poll(context).is_ready() {
                return Poll::Ready(true);
            }
            if grace
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
            {
                return Poll::Ready(false);
            }
            Poll::Pending
        })
        .await;
        if !acknowledged {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            if pending.remove(&id).is_some() {
                // The pump can still receive this exact acknowledgement during shutdown.
                *self
                    .abandoned_close
                    .lock()
                    .expect("sidecar close lock poisoned") = Some(id);
            }
        }
        drop(close);
        self.begin_close().await;
    }

    fn close_acknowledgement_wait(&self) -> Option<BoxFuture<'static, DeadlineObservation>> {
        let time = self.services.time().cloned()?;
        let deadline = Deadline::at(MonotonicInstant::from_ticks(
            time.now()
                .ticks()
                .saturating_add(CLOSE_ACKNOWLEDGEMENT_GRACE_TICKS),
        ));
        Some(time.wait_until(deadline))
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
    abort_turn: Option<Arc<SidecarActiveTurn>>,
}

#[derive(Default)]
struct ResponseState {
    result: Option<Result<CommandResult, RuntimeFailure>>,
    waiter: Option<Waker>,
}

pub(super) struct ResponseSender(Arc<Mutex<ResponseState>>);
struct ResponseFuture(Arc<Mutex<ResponseState>>);

fn response_channel() -> (ResponseSender, ResponseFuture) {
    let state = Arc::new(Mutex::new(ResponseState::default()));
    (ResponseSender(Arc::clone(&state)), ResponseFuture(state))
}

impl ResponseSender {
    pub(super) fn complete(self, result: Result<CommandResult, RuntimeFailure>) {
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

impl SidecarConnection {
    /// Closes command admission and resolves every waiter when the pump can no
    /// longer receive responses. The pending lock makes closure and draining
    /// one linearized transition with command registration.
    pub(crate) fn fail_connection(&self, error: &RuntimeFailure) {
        let pending = {
            let mut pending = self.pending.lock().expect("sidecar pending lock poisoned");
            *self
                .terminal_error
                .lock()
                .expect("sidecar terminal-error lock poisoned") = Some(error.diagnostic().clone());
            self.closed.store(true, Ordering::SeqCst);
            std::mem::take(&mut *pending)
        };
        for (_, pending) in pending {
            pending
                .sender
                .complete(Err(RuntimeFailure::new(error.diagnostic().clone())));
        }
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("sidecar active lock poisoned")
            .take()
            && !turn.is_finished()
        {
            turn.fail_connection(error.diagnostic().clone());
        }
    }

    fn closed_failure(&self) -> RuntimeFailure {
        match self
            .terminal_error
            .lock()
            .expect("sidecar terminal-error lock poisoned")
            .clone()
        {
            Some(diagnostic) => RuntimeFailure::new(diagnostic),
            None => connection_closed(),
        }
    }
}

fn connection_closed() -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.connection_closed",
        "Pi SDK sidecar connection is closed",
    )
}
