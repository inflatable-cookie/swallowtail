#![allow(dead_code)]

use futures_executor::block_on;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant,
    ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessRequest,
    ProcessService, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedProcess {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environments: Vec<String>,
}

pub struct FixtureHost {
    process: Arc<FixtureProcessService>,
    process_state: Arc<ProcessState>,
    joined: Arc<AtomicBool>,
}

impl FixtureHost {
    pub fn completed(outputs: impl IntoIterator<Item = ProcessOutputChunk>) -> Self {
        Self::with_exit(outputs, ProcessExit::new(true, Some(0)))
    }

    pub fn with_exit(
        outputs: impl IntoIterator<Item = ProcessOutputChunk>,
        exit: ProcessExit,
    ) -> Self {
        let process_state = Arc::new(ProcessState::default());
        Self {
            process: Arc::new(FixtureProcessService {
                outputs: Mutex::new(Some(outputs.into_iter().collect())),
                exit,
                state: Arc::clone(&process_state),
            }),
            process_state,
            joined: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService {
                joined: Arc::clone(&self.joined),
            }))
            .with_time(Arc::new(PendingTime))
            .with_process(self.process.clone())
    }

    pub fn observed(&self) -> ObservedProcess {
        self.process_state
            .observed
            .lock()
            .expect("fixture process lock is available")
            .clone()
            .expect("process was observed")
    }

    pub fn started(&self) -> bool {
        self.process_state
            .observed
            .lock()
            .expect("fixture process lock is available")
            .is_some()
    }

    pub fn stdin_closed(&self) -> bool {
        self.process_state.stdin_closed.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.process_state.waited.load(Ordering::SeqCst)
    }

    pub fn joined(&self) -> bool {
        self.joined.load(Ordering::SeqCst)
    }
}

struct FixtureProcessService {
    outputs: Mutex<Option<VecDeque<ProcessOutputChunk>>>,
    exit: ProcessExit,
    state: Arc<ProcessState>,
}

#[derive(Default)]
struct ProcessState {
    observed: Mutex<Option<ObservedProcess>>,
    stdin_closed: AtomicBool,
    waited: AtomicBool,
}

impl ProcessService for FixtureProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .state
            .observed
            .lock()
            .expect("fixture process lock is available") = Some(ObservedProcess {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environments: request
                .environment()
                .map(|value| value.as_host_value().to_owned())
                .collect(),
        });
        let outputs = self
            .outputs
            .lock()
            .expect("fixture output lock is available")
            .take()
            .expect("fixture process starts once");
        let handle = FixtureProcessHandle {
            outputs: Mutex::new(outputs),
            exit: self.exit,
            state: Arc::clone(&self.state),
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FixtureProcessHandle {
    outputs: Mutex<VecDeque<ProcessOutputChunk>>,
    exit: ProcessExit,
    state: Arc<ProcessState>,
}

impl ProcessHandle for FixtureProcessHandle {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let output = self
            .outputs
            .lock()
            .expect("fixture output lock is available")
            .pop_front();
        Box::pin(async move { Ok(output) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        let exit = self.exit;
        Box::pin(async move { Ok(exit) })
    }
}

struct ThreadTaskService {
    joined: Arc<AtomicBool>,
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask {
            handle: Some(std::thread::spawn(move || block_on(task))),
            joined: Arc::clone(&self.joined),
        }))
    }
}

struct ThreadTask {
    handle: Option<std::thread::JoinHandle<()>>,
    joined: Arc<AtomicBool>,
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.handle.take().expect("fixture task joins once");
        Box::pin(async move {
            handle.join().expect("fixture task does not panic");
            self.joined.store(true, Ordering::SeqCst);
            Ok(())
        })
    }
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}
