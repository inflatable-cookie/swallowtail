use futures_executor::block_on;
use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread::{self, JoinHandle};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant,
    ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
    ProcessRequest, ProcessService, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedProcessRequest {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environments: Vec<String>,
    pub working_resource: Option<String>,
}

#[derive(Default)]
pub struct ProcessState {
    request: Mutex<Option<ObservedProcessRequest>>,
    stdin: Mutex<Vec<u8>>,
    stdin_closed: AtomicBool,
    force_stopped: AtomicBool,
    waited: AtomicBool,
}

impl ProcessState {
    pub fn started(&self) -> bool {
        self.request
            .lock()
            .expect("request lock is available")
            .is_some()
    }

    pub fn request(&self) -> ObservedProcessRequest {
        self.request
            .lock()
            .expect("request lock is available")
            .clone()
            .expect("process request was captured")
    }

    pub fn stdin(&self) -> Vec<u8> {
        self.stdin.lock().expect("stdin lock is available").clone()
    }

    pub fn stdin_closed(&self) -> bool {
        self.stdin_closed.load(Ordering::SeqCst)
    }

    pub fn force_stopped(&self) -> bool {
        self.force_stopped.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.waited.load(Ordering::SeqCst)
    }
}

pub struct FakeProcessService {
    state: Arc<ProcessState>,
    output: Mutex<Option<VecDeque<ProcessOutputChunk>>>,
    exit: ProcessExit,
    hold_open: bool,
}

impl FakeProcessService {
    pub fn completed(jsonl: &str) -> (Arc<Self>, Arc<ProcessState>) {
        Self::with_exit(jsonl, ProcessExit::new(true, Some(0)))
    }

    pub fn with_exit(jsonl: &str, exit: ProcessExit) -> (Arc<Self>, Arc<ProcessState>) {
        let output = if jsonl.is_empty() {
            Vec::new()
        } else {
            vec![ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                jsonl.as_bytes().to_vec(),
            )]
        };
        Self::new(output, exit, false)
    }

    pub fn held_open() -> (Arc<Self>, Arc<ProcessState>) {
        Self::new([], ProcessExit::new(false, Some(130)), true)
    }

    fn new(
        output: impl IntoIterator<Item = ProcessOutputChunk>,
        exit: ProcessExit,
        hold_open: bool,
    ) -> (Arc<Self>, Arc<ProcessState>) {
        let state = Arc::new(ProcessState::default());
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                output: Mutex::new(Some(output.into_iter().collect())),
                exit,
                hold_open,
            }),
            state,
        )
    }
}

impl ProcessService for FakeProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .state
            .request
            .lock()
            .expect("request lock is available") = Some(ObservedProcessRequest {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environments: request
                .environment()
                .map(|value| value.as_host_value().to_owned())
                .collect(),
            working_resource: request
                .working_resource()
                .map(|value| value.as_host_value().to_owned()),
        });
        let output = self
            .output
            .lock()
            .expect("output lock is available")
            .take()
            .expect("fake process starts once");
        let handle = FakeProcessHandle {
            state: Arc::clone(&self.state),
            output: Mutex::new(output),
            exit: self.exit,
            hold_open: self.hold_open,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FakeProcessHandle {
    state: Arc<ProcessState>,
    output: Mutex<VecDeque<ProcessOutputChunk>>,
    exit: ProcessExit,
    hold_open: bool,
}

impl ProcessHandle for FakeProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .stdin
            .lock()
            .expect("stdin lock is available")
            .extend_from_slice(chunk.bytes());
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            loop {
                if let Some(chunk) = self
                    .output
                    .lock()
                    .expect("output lock is available")
                    .pop_front()
                {
                    return Ok(Some(chunk));
                }
                if !self.hold_open || self.state.force_stopped.load(Ordering::SeqCst) {
                    return Ok(None);
                }
                thread::yield_now();
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.force_stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        let exit = self.exit;
        Box::pin(async move { Ok(exit) })
    }
}

struct ThreadTaskService;
struct ThreadTask(Mutex<Option<JoinHandle<()>>>);

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Mutex::new(Some(thread::spawn(
            move || block_on(task),
        ))))))
    }
}

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.0
                .lock()
                .expect("task lock is available")
                .take()
                .expect("task joins once")
                .join()
                .expect("fixture task does not panic");
            Ok(())
        })
    }
}

pub struct PendingTimeService;

impl TimeService for PendingTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(PendingDeadline)
    }
}

struct PendingDeadline;

impl Future for PendingDeadline {
    type Output = DeadlineObservation;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

pub struct ImmediateTimeService;

impl TimeService for ImmediateTimeService {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(1_000)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

pub fn host_services(process: Arc<dyn ProcessService>, time: Arc<dyn TimeService>) -> HostServices {
    HostServices::new(ExecutionHostId::new("host.local").expect("host id is valid"))
        .with_task(Arc::new(ThreadTaskService))
        .with_process(process)
        .with_time(time)
}
