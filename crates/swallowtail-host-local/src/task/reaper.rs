use super::joined::{reap_worker, task_failure};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use swallowtail_runtime::RuntimeFailure;

enum ReapCommand {
    Release,
    Reap {
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
    },
}

pub(super) struct ReapPermit {
    supervisor: Arc<ReaperSupervisor>,
    state: Arc<Mutex<ReapPermitState>>,
}

pub(super) struct ReapCompletion {
    state: Arc<Mutex<ReapPermitState>>,
}

enum ReapPermitState {
    Live(mpsc::Sender<ReapCommand>),
    Completed,
    HandedOff,
}

impl ReapPermit {
    pub(super) fn completion(&self) -> ReapCompletion {
        ReapCompletion {
            state: Arc::clone(&self.state),
        }
    }

    pub(super) fn belongs_to(&self, supervisor: &Arc<ReaperSupervisor>) -> bool {
        Arc::ptr_eq(&self.supervisor, supervisor)
    }

    pub(super) fn accept(
        self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
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
        match sender.send(ReapCommand::Reap { worker, reaped }) {
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
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let ReapPermitState::Live(sender) =
            std::mem::replace(&mut *state, ReapPermitState::Completed)
        {
            let _ = sender.send(ReapCommand::Release);
        }
    }
}

impl Drop for ReapPermit {
    fn drop(&mut self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let ReapPermitState::Live(sender) =
            std::mem::replace(&mut *state, ReapPermitState::Completed)
        {
            let _ = sender.send(ReapCommand::Release);
        }
    }
}

pub(super) struct ReaperSupervisor {
    shutdown: Mutex<()>,
    state: Mutex<ReaperState>,
    settled: Condvar,
    capacity: usize,
}

struct ReaperState {
    accepting_reservations: bool,
    issued_reservations: usize,
    reapers: Vec<JoinHandle<()>>,
}

impl ReaperSupervisor {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            shutdown: Mutex::new(()),
            state: Mutex::new(ReaperState::default()),
            settled: Condvar::new(),
            capacity,
        }
    }

    pub(super) fn reserve(self: &Arc<Self>) -> Result<ReapPermit, RuntimeFailure> {
        let mut state = self.state.lock().map_err(|_| {
            task_failure(
                "swallowtail.local_task.reap_reservation_unavailable",
                "Local task reap reservation state is unavailable",
            )
        })?;
        if !state.accepting_reservations {
            return Err(task_failure(
                "swallowtail.local_task.reap_reservation_shutdown",
                "Local task reap reservation admission is shut down",
            ));
        }
        if state.issued_reservations >= self.capacity {
            return Err(task_failure(
                "swallowtail.local_task.reap_reservation_capacity",
                "Local task reap reservation capacity is unavailable",
            ));
        }

        let (sender, receiver) = mpsc::channel();
        let settlement = Arc::new(ReservationSettlement {
            supervisor: Arc::clone(self),
            settled: AtomicBool::new(false),
        });
        let worker_settlement = Arc::clone(&settlement);
        let reaper = thread::Builder::new()
            .name("swallowtail-local-task-reaper".to_owned())
            .spawn(move || reap_worker_from(receiver, worker_settlement))
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.reap_reservation_failed",
                    "Local task reaper capacity could not be reserved",
                )
            })?;
        state.issued_reservations += 1;
        state.reapers.push(reaper);
        Ok(ReapPermit {
            supervisor: Arc::clone(self),
            state: Arc::new(Mutex::new(ReapPermitState::Live(sender))),
        })
    }

    pub(super) fn shutdown(&self) -> Result<(), RuntimeFailure> {
        let (shutdown_guard, shutdown_poisoned) = match self.shutdown.lock() {
            Ok(guard) => (guard, false),
            Err(poisoned) => (poisoned.into_inner(), true),
        };
        let (mut state, mut state_poisoned) = match self.state.lock() {
            Ok(state) => (state, false),
            Err(poisoned) => (poisoned.into_inner(), true),
        };
        state.accepting_reservations = false;
        while state.issued_reservations != 0 {
            match self.settled.wait(state) {
                Ok(next) => state = next,
                Err(poisoned) => {
                    state = poisoned.into_inner();
                    state_poisoned = true;
                }
            }
        }
        let mut reapers = std::mem::take(&mut state.reapers);
        drop(state);

        let mut reaper_panicked = false;
        for reaper in reapers.drain(..) {
            reaper_panicked |= reaper.join().is_err();
        }
        drop(shutdown_guard);
        if shutdown_poisoned || state_poisoned || reaper_panicked {
            Err(task_failure(
                "swallowtail.local_task.reaper_shutdown_failed",
                "Local task reaper lifecycle did not shut down cleanly",
            ))
        } else {
            Ok(())
        }
    }

    fn settle_reservation(&self) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(poisoned) => poisoned.into_inner(),
        };
        state.issued_reservations = state.issued_reservations.saturating_sub(1);
        self.settled.notify_all();
    }
}

struct ReservationSettlement {
    supervisor: Arc<ReaperSupervisor>,
    settled: AtomicBool,
}

impl ReservationSettlement {
    fn settle(&self) {
        if !self.settled.swap(true, Ordering::AcqRel) {
            self.supervisor.settle_reservation();
        }
    }
}

fn reap_worker_from(receiver: mpsc::Receiver<ReapCommand>, settlement: Arc<ReservationSettlement>) {
    if let Ok(ReapCommand::Reap { worker, reaped }) = receiver.recv() {
        let _ = reap_worker(Some(worker), &reaped);
    }
    settlement.settle();
}

impl Default for ReaperState {
    fn default() -> Self {
        Self {
            accepting_reservations: true,
            issued_reservations: 0,
            reapers: Vec::new(),
        }
    }
}
