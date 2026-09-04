use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct DeleteResponseGate {
    state: Arc<(Mutex<DeleteResponseState>, Condvar)>,
}

#[derive(Default)]
struct DeleteResponseState {
    dispatched: bool,
    released: bool,
}

impl DeleteResponseGate {
    pub(super) fn mark_dispatched_and_wait_for_release(&self) {
        let (state, changed) = &*self.state;
        let mut state = state.lock().expect("delete response gate lock poisoned");
        state.dispatched = true;
        changed.notify_all();
        while !state.released {
            state = changed
                .wait(state)
                .expect("delete response gate lock poisoned");
        }
    }

    pub(crate) fn wait_for_dispatch(&self) {
        let (state, changed) = &*self.state;
        let state = state.lock().expect("delete response gate lock poisoned");
        let (state, timeout) = changed
            .wait_timeout_while(state, Duration::from_secs(2), |state| !state.dispatched)
            .expect("delete response gate lock poisoned");
        assert!(state.dispatched, "DELETE was not dispatched before timeout");
        assert!(!timeout.timed_out(), "DELETE dispatch wait timed out");
    }

    pub(crate) fn release(&self) {
        let (state, changed) = &*self.state;
        state
            .lock()
            .expect("delete response gate lock poisoned")
            .released = true;
        changed.notify_all();
    }
}
