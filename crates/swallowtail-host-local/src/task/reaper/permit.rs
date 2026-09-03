use super::ReaperSupervisor;
use super::join::{ReapJoin, ReapJoinCompletion};
use crate::task::joined::task_failure;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;
use swallowtail_runtime::RuntimeFailure;

pub(super) enum ReapCommand {
    Release,
    Reap {
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
        join_completion: Option<Arc<ReapJoinCompletion>>,
    },
}

pub(in crate::task) struct ReapPermit {
    supervisor: Arc<ReaperSupervisor>,
    state: Arc<Mutex<ReapPermitState>>,
}

pub(in crate::task) struct ReapCompletion {
    state: Arc<Mutex<ReapPermitState>>,
}

enum ReapPermitState {
    Live(mpsc::Sender<ReapCommand>),
    Completed,
    HandedOff,
}

impl ReapPermit {
    pub(super) fn new(
        supervisor: Arc<ReaperSupervisor>,
        sender: mpsc::Sender<ReapCommand>,
    ) -> Self {
        Self {
            supervisor,
            state: Arc::new(Mutex::new(ReapPermitState::Live(sender))),
        }
    }

    pub(in crate::task) fn completion(&self) -> ReapCompletion {
        ReapCompletion {
            state: Arc::clone(&self.state),
        }
    }

    pub(in crate::task) fn belongs_to(&self, supervisor: &Arc<ReaperSupervisor>) -> bool {
        Arc::ptr_eq(&self.supervisor, supervisor)
    }

    pub(in crate::task) fn accept(
        self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
    ) -> Result<(), (RuntimeFailure, JoinHandle<()>, Self)> {
        self.accept_with(worker, reaped, None)
    }

    pub(in crate::task) fn accept_for_join(
        self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
    ) -> Result<ReapJoin, (RuntimeFailure, JoinHandle<()>, Self)> {
        let completion = Arc::new(ReapJoinCompletion::default());
        self.accept_with(worker, reaped, Some(Arc::clone(&completion)))?;
        Ok(ReapJoin::new(completion))
    }

    fn accept_with(
        self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
        join_completion: Option<Arc<ReapJoinCompletion>>,
    ) -> Result<(), (RuntimeFailure, JoinHandle<()>, Self)> {
        let sender = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            match std::mem::replace(&mut *state, ReapPermitState::HandedOff) {
                ReapPermitState::Live(sender) => sender,
                ReapPermitState::Completed => {
                    *state = ReapPermitState::Completed;
                    drop(state);
                    return Err((
                        task_failure(
                            "swallowtail.local_task.already_finished",
                            "Finished local tasks must use ordinary join",
                        ),
                        worker,
                        self,
                    ));
                }
                ReapPermitState::HandedOff => {
                    unreachable!("reap permit cannot be accepted twice")
                }
            }
        };
        match sender.send(ReapCommand::Reap {
            worker,
            reaped,
            join_completion,
        }) {
            Ok(()) => Ok(()),
            Err(mpsc::SendError(ReapCommand::Reap { worker, .. })) => {
                *self
                    .state
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner) =
                    ReapPermitState::Completed;
                Err((
                    task_failure(
                        "swallowtail.local_task.reaper_handoff_failed",
                        "Reserved local task reaper did not accept task ownership",
                    ),
                    worker,
                    self,
                ))
            }
            Err(mpsc::SendError(ReapCommand::Release)) => {
                unreachable!("reap handoff sends only a reap command")
            }
        }
    }
}

impl Drop for ReapCompletion {
    fn drop(&mut self) {
        release_if_live(&self.state);
    }
}

impl Drop for ReapPermit {
    fn drop(&mut self) {
        release_if_live(&self.state);
    }
}

fn release_if_live(state: &Mutex<ReapPermitState>) {
    let mut state = state
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let ReapPermitState::Live(sender) =
        std::mem::replace(&mut *state, ReapPermitState::Completed)
    {
        let _ = sender.send(ReapCommand::Release);
    }
}
