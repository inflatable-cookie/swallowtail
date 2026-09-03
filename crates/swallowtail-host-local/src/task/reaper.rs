use super::{reap_worker, task_failure};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use swallowtail_runtime::RuntimeFailure;

struct ReapJob {
    worker: JoinHandle<()>,
    reaped: Arc<AtomicBool>,
}

#[derive(Default)]
pub(super) struct ReaperSupervisor {
    shutdown: Mutex<()>,
    state: Mutex<ReaperState>,
}

struct ReaperState {
    accepting: bool,
    reapers: Vec<JoinHandle<()>>,
}

impl ReaperSupervisor {
    pub(super) fn accept(
        &self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
    ) -> Result<(), (RuntimeFailure, JoinHandle<()>)> {
        let job = ReapJob { worker, reaped };
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                return Err((
                    task_failure(
                        "swallowtail.local_task.reaper_unavailable",
                        "Local task reaper ownership is unavailable",
                    ),
                    job.worker,
                ));
            }
        };
        if !state.accepting {
            return Err((
                task_failure(
                    "swallowtail.local_task.reaper_shutdown",
                    "Local task reaper lifecycle is shut down",
                ),
                job.worker,
            ));
        }
        let (sender, receiver) = mpsc::sync_channel(1);
        let reaper = match thread::Builder::new()
            .name("swallowtail-local-task-reaper".to_owned())
            .spawn(move || reap_worker_from(receiver))
        {
            Ok(reaper) => reaper,
            Err(_) => {
                return Err((
                    task_failure(
                        "swallowtail.local_task.reaper_spawn_failed",
                        "Local task reaper could not be started",
                    ),
                    job.worker,
                ));
            }
        };
        state.reapers.push(reaper);
        drop(state);
        sender.send(job).map_err(|mpsc::SendError(job)| {
            (
                task_failure(
                    "swallowtail.local_task.reaper_handoff_failed",
                    "Local task reaper did not accept task ownership",
                ),
                job.worker,
            )
        })
    }

    pub(super) fn shutdown(&self) -> Result<(), RuntimeFailure> {
        let (shutdown_guard, shutdown_poisoned) = match self.shutdown.lock() {
            Ok(guard) => (guard, false),
            Err(poisoned) => (poisoned.into_inner(), true),
        };
        let (mut reapers, state_poisoned) = match self.state.lock() {
            Ok(mut state) => {
                state.accepting = false;
                (std::mem::take(&mut state.reapers), false)
            }
            Err(poisoned) => {
                let mut state = poisoned.into_inner();
                state.accepting = false;
                (std::mem::take(&mut state.reapers), true)
            }
        };
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
}

fn reap_worker_from(receiver: mpsc::Receiver<ReapJob>) {
    if let Ok(job) = receiver.recv() {
        let _ = reap_worker(Some(job.worker), &job.reaped);
    }
}

impl Default for ReaperState {
    fn default() -> Self {
        Self {
            accepting: true,
            reapers: Vec::new(),
        }
    }
}
