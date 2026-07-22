use super::{GeminiLiveSession, configure};
use crate::failure::failure;
use crate::live::lifecycle::{cleanup, merge};
use crate::live_protocol::ClientFrame;
use swallowtail_runtime::{CleanupOutcome, RuntimeFailure};

impl GeminiLiveSession {
    pub(crate) async fn rollover_if_pending(&mut self) -> Result<(), RuntimeFailure> {
        let pending_setup = {
            let state = self.rollover.lock().expect("rollover state poisoned");
            if !state.pending() {
                return Ok(());
            }
            if state.exhausted() {
                None
            } else {
                state.handle().map(|handle| {
                    ClientFrame::Setup {
                        handle: Some(handle),
                    }
                    .to_json()
                })
            }
        };
        let setup = match pending_setup {
            Some(setup) => setup,
            None => {
                self.invalidate_for_rollover_failure();
                return Err(rollover_failed());
            }
        };
        let connection = self
            .access
            .as_mut()
            .expect("open session owns access")
            .connect(self.scope.clone(), &self.services)
            .await;
        let (new_worker, new_work) = match connection {
            Ok(connection) => connection,
            Err(_) => {
                self.invalidate_for_rollover_failure();
                return Err(rollover_failed());
            }
        };
        self.connections.add(new_worker.clone());
        let mut new_updates = new_worker.take_updates().expect("new worker owns updates");
        if configure(&new_worker, &mut new_updates, setup)
            .await
            .is_err()
        {
            new_worker.abort();
            let _ = new_work.await;
            self.invalidate_for_rollover_failure();
            return Err(rollover_failed());
        }

        let old_worker = std::mem::replace(&mut self.worker, new_worker);
        let old_work = self.worker_work.replace(new_work);
        *self.updates.lock().expect("updates lock poisoned") = Some(new_updates);
        let close = cleanup(old_worker.close().await);
        let joined = match old_work {
            Some(work) => cleanup(work.await),
            None => CleanupOutcome::NotApplicable,
        };
        self.connections.remove(&old_worker);
        self.historical_cleanup = merge(self.historical_cleanup.clone(), merge(close, joined));
        if matches!(self.historical_cleanup, CleanupOutcome::Failed(_)) {
            self.invalidate_for_rollover_failure();
            return Err(rollover_failed());
        }
        self.rollover
            .lock()
            .expect("rollover state poisoned")
            .complete();
        Ok(())
    }

    fn invalidate_for_rollover_failure(&self) {
        self.reusable
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.state
            .lock()
            .expect("media state lock poisoned")
            .close();
        self.connections.abort_all();
    }
}

fn rollover_failed() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.live_rollover_failed",
        "Gemini Live planned connection rollover could not preserve continuity",
    )
}
