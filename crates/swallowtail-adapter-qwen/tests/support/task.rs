use futures_executor::block_on;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use swallowtail_runtime::{BoxFuture, JoinedTask, RuntimeFailure, ScopeId, ScopedTaskService};

#[derive(Default)]
pub struct TaskState {
    joined: AtomicBool,
}

impl TaskState {
    pub fn joined(&self) -> bool {
        self.joined.load(Ordering::SeqCst)
    }
}

pub struct ThreadTaskService {
    state: Arc<TaskState>,
}

impl ThreadTaskService {
    pub const fn new(state: Arc<TaskState>) -> Self {
        Self { state }
    }
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask {
            handle: Mutex::new(Some(thread::spawn(move || block_on(task)))),
            state: Arc::clone(&self.state),
        }))
    }
}

struct ThreadTask {
    handle: Mutex<Option<JoinHandle<()>>>,
    state: Arc<TaskState>,
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.handle
                .lock()
                .expect("task lock is available")
                .take()
                .expect("task joins once")
                .join()
                .expect("fixture task does not panic");
            self.state.joined.store(true, Ordering::SeqCst);
            Ok(())
        })
    }
}
