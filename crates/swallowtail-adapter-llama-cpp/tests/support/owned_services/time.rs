use super::ScriptedOwnedServices;
use futures_util::future;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, TimeService,
};

impl TimeService for ScriptedOwnedServices {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        if self.state.lock().expect("state lock").expire_deadlines
            || deadline.instant().ticks() <= 1_000
        {
            Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
        } else {
            Box::pin(future::pending())
        }
    }
}
