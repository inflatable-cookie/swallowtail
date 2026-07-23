use crate::failure::{failure, protocol_failure};
use crate::turn::ActiveTurn;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, JoinedTask, ProcessHandle, ProcessInputChunk, RuntimeFailure,
    ScopedTaskService, TimeService,
};

mod pump;

const MAXIMUM_PENDING_COMMANDS: usize = 16;

pub(crate) struct CommandResult {
    pub(crate) success: bool,
    pub(crate) data: Option<Value>,
}

pub(crate) struct PiConnection {
    process: Arc<dyn ProcessHandle>,
    task: Arc<dyn ScopedTaskService>,
    time: Arc<dyn TimeService>,
    callback_tasks: Mutex<Vec<Box<dyn JoinedTask>>>,
    pending: Mutex<BTreeMap<String, PendingCommand>>,
    used_ids: Mutex<BTreeSet<String>>,
    active_turn: Mutex<Option<Arc<ActiveTurn>>>,
    closed: AtomicBool,
    cleanup: Mutex<Option<CleanupOutcome>>,
}

impl PiConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        task: Arc<dyn ScopedTaskService>,
        time: Arc<dyn TimeService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            process,
            task,
            time,
            callback_tasks: Mutex::new(Vec::new()),
            pending: Mutex::new(BTreeMap::new()),
            used_ids: Mutex::new(BTreeSet::new()),
            active_turn: Mutex::new(None),
            closed: AtomicBool::new(false),
            cleanup: Mutex::new(None),
        })
    }

    pub(crate) async fn command(
        &self,
        id: String,
        command: &'static str,
        value: Value,
    ) -> Result<CommandResult, RuntimeFailure> {
        if self.closed.load(Ordering::SeqCst) {
            return Err(connection_closed());
        }
        if !self
            .used_ids
            .lock()
            .expect("Pi command id lock poisoned")
            .insert(id.clone())
        {
            return Err(failure(
                "swallowtail.pi.rpc.command_id_reused",
                "Pi RPC command id was reused",
            ));
        }
        let (sender, response) = response_channel();
        {
            let mut pending = self.pending.lock().expect("Pi pending lock poisoned");
            if pending.len() >= MAXIMUM_PENDING_COMMANDS {
                return Err(failure(
                    "swallowtail.pi.rpc.command_capacity_exceeded",
                    "Pi RPC exceeded the pending command limit",
                ));
            }
            pending.insert(id.clone(), PendingCommand { command, sender });
        }
        if let Err(error) = self.write_value(value).await {
            self.pending
                .lock()
                .expect("Pi pending lock poisoned")
                .remove(&id);
            return Err(error);
        }
        response.await
    }

    pub(crate) async fn write_value(&self, value: Value) -> Result<(), RuntimeFailure> {
        let mut bytes = serde_json::to_vec(&value).map_err(|_| protocol_failure())?;
        bytes.push(b'\n');
        self.process
            .write_stdin(ProcessInputChunk::new(bytes))
            .await
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        let mut active = self.active_turn.lock().expect("Pi active lock poisoned");
        if active.as_ref().is_some_and(|active| !active.is_finished()) {
            return Err(failure(
                "swallowtail.pi.rpc.turn_active",
                "Pi RPC session already has an active turn",
            ));
        }
        *active = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        let mut active = self.active_turn.lock().expect("Pi active lock poisoned");
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
            .expect("Pi cleanup lock poisoned")
            .clone()
            .unwrap_or_else(|| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.pi.rpc.cleanup_missing",
                    "Pi RPC process cleanup did not complete",
                ))
            })
    }

    pub(crate) async fn join_callback_tasks(&self) -> CleanupOutcome {
        let tasks = std::mem::take(
            &mut *self
                .callback_tasks
                .lock()
                .expect("Pi callback-task lock poisoned"),
        );
        if tasks.is_empty() {
            return CleanupOutcome::NotApplicable;
        }
        for task in tasks {
            if task.join().await.is_err() {
                return CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.pi.rpc.callback_join_failed",
                    "Pi RPC callback deadline task did not join cleanly",
                ));
            }
        }
        CleanupOutcome::Clean
    }
}

struct PendingCommand {
    command: &'static str,
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
        let mut state = self.0.lock().expect("Pi response lock poisoned");
        state.result = Some(result);
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }
}

impl Future for ResponseFuture {
    type Output = Result<CommandResult, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.lock().expect("Pi response lock poisoned");
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
        "swallowtail.pi.rpc.connection_closed",
        "Pi RPC connection is closed",
    )
}
