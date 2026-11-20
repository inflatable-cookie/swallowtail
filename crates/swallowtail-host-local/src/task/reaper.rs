use super::joined::{reap_worker, task_failure};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use swallowtail_runtime::RuntimeFailure;

mod join;
mod permit;

use permit::ReapCommand;
pub(super) use permit::ReapPermit;

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
    reaper_panicked: bool,
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
        prune_finished_reapers(&mut state);
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
        let settlement = ReservationSettlement {
            supervisor: Arc::clone(self),
            settled: AtomicBool::new(false),
        };
        let reaper = thread::Builder::new()
            .name("swallowtail-local-task-reaper".to_owned())
            .spawn(move || reap_worker_from(receiver, settlement))
            .map_err(|_| {
                task_failure(
                    "swallowtail.local_task.reap_reservation_failed",
                    "Local task reaper capacity could not be reserved",
                )
            })?;
        state.issued_reservations += 1;
        state.reapers.push(reaper);
        Ok(ReapPermit::new(Arc::clone(self), sender))
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
        let mut reaper_panicked = state.reaper_panicked;
        let mut reapers = std::mem::take(&mut state.reapers);
        drop(state);

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

    #[cfg(test)]
    fn tracked_reaper_count(&self) -> usize {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .reapers
            .len()
    }
}

fn prune_finished_reapers(state: &mut ReaperState) {
    let mut index = 0;
    while index < state.reapers.len() {
        if state.reapers[index].is_finished() {
            let reaper = state.reapers.swap_remove(index);
            state.reaper_panicked |= reaper.join().is_err();
        } else {
            index += 1;
        }
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

struct SettlementGuard<'a>(&'a ReservationSettlement);

impl Drop for SettlementGuard<'_> {
    fn drop(&mut self) {
        self.0.settle();
    }
}

fn reap_worker_from(receiver: mpsc::Receiver<ReapCommand>, settlement: ReservationSettlement) {
    let _guard = SettlementGuard(&settlement);
    if let Ok(ReapCommand::Reap {
        worker,
        reaped,
        join_completion,
    }) = receiver.recv()
    {
        let outcome = reap_worker(Some(worker), &reaped);
        if let Some(completion) = join_completion {
            completion.complete(outcome);
        }
    }
}

impl Default for ReaperState {
    fn default() -> Self {
        Self {
            accepting_reservations: true,
            issued_reservations: 0,
            reapers: Vec::new(),
            reaper_panicked: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Wake, Waker};
    use std::time::{Duration, Instant};

    struct PanickingWaker;

    impl Wake for PanickingWaker {
        fn wake(self: Arc<Self>) {
            panic!("synthetic waker panic");
        }
    }

    #[test]
    fn finished_reaper_handles_are_pruned_before_shutdown() {
        let supervisor = Arc::new(ReaperSupervisor::new(2));
        for _ in 0..25 {
            let deadline = Instant::now() + Duration::from_secs(1);
            let permit = loop {
                match supervisor.reserve() {
                    Ok(permit) => break permit,
                    Err(_) if Instant::now() < deadline => thread::yield_now(),
                    Err(error) => panic!("reservation failed: {error:?}"),
                }
            };
            drop(permit);
            // Wait until the reservation settles so capacity=2 allows the next round.
            let settle_deadline = Instant::now() + Duration::from_secs(1);
            while supervisor
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .issued_reservations
                != 0
            {
                assert!(Instant::now() < settle_deadline, "permit did not settle");
                thread::yield_now();
            }
        }
        assert!(
            supervisor.tracked_reaper_count() <= 2,
            "finished reaper handles accumulated: {}",
            supervisor.tracked_reaper_count()
        );
        supervisor
            .shutdown()
            .expect("clean shutdown after pruned reapers");
    }

    #[test]
    fn panicking_reaper_settles_reservation_and_does_not_deadlock_shutdown() {
        let supervisor = Arc::new(ReaperSupervisor::new(1));
        let permit = supervisor.reserve().expect("reservation succeeds");
        let (tx, rx) = mpsc::channel();
        let worker = thread::spawn(move || {
            let _ = rx.recv();
        });
        let reaped = Arc::new(AtomicBool::new(false));
        let mut join_future = permit
            .accept_for_join(worker, reaped)
            .ok()
            .expect("accept_for_join succeeds")
            .into_future();
        let waker = Waker::from(Arc::new(PanickingWaker));
        let mut context = Context::from_waker(&waker);
        assert!(Pin::new(&mut join_future).poll(&mut context).is_pending());

        // Letting the worker exit triggers completion.complete() -> PanickingWaker::wake() panic
        // on the reaper thread. SettlementGuard must still settle so shutdown() returns Err
        // instead of deadlocking.
        tx.send(()).expect("worker receives release");
        let (done_tx, done_rx) = mpsc::channel();
        let shutdown_supervisor = Arc::clone(&supervisor);
        thread::spawn(move || {
            let result = shutdown_supervisor.shutdown();
            let _ = done_tx.send(result);
        });
        let result = done_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("shutdown must not deadlock when reaper panics");
        let err = result.expect_err("panicking reaper reports shutdown failure");
        assert_eq!(
            err.diagnostic().code(),
            "swallowtail.local_task.reaper_shutdown_failed"
        );
    }
}
