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
    reapers: Mutex<Vec<JoinHandle<()>>>,
}

impl ReaperSupervisor {
    pub(super) fn accept(
        &self,
        worker: JoinHandle<()>,
        reaped: Arc<AtomicBool>,
    ) -> Result<(), (RuntimeFailure, JoinHandle<()>)> {
        let job = ReapJob { worker, reaped };
        let mut reapers = match self.reapers.lock() {
            Ok(reapers) => reapers,
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
        reapers.push(reaper);
        drop(reapers);
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
}

impl Drop for ReaperSupervisor {
    fn drop(&mut self) {
        let reapers = self
            .reapers
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for reaper in reapers.drain(..) {
            let _ = reaper.join();
        }
    }
}

fn reap_worker_from(receiver: mpsc::Receiver<ReapJob>) {
    if let Ok(job) = receiver.recv() {
        let _ = reap_worker(Some(job.worker), &job.reaped);
    }
}
