use std::sync::{Arc, Condvar, Mutex};

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
        let mut state = state.lock().expect("delete response gate lock poisoned");
        while !state.dispatched {
            state = changed
                .wait(state)
                .expect("delete response gate lock poisoned");
        }
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
