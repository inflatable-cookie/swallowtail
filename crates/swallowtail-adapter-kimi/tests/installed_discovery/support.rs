use futures_executor::block_on;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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
    stdin_closed: AtomicBool,
    force_stopped: AtomicBool,
    waited: AtomicBool,
}

impl ProcessState {
    pub fn started(&self) -> bool {
        self.request.lock().expect("request lock").is_some()
    }

    pub fn request(&self) -> ObservedProcessRequest {
        self.request
            .lock()
            .expect("request lock")
            .clone()
            .expect("process request captured")
    }

    pub fn stdin_closed(&self) -> bool {
        self.stdin_closed.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.waited.load(Ordering::SeqCst)
    }
}

pub struct FakeProcessService {
    state: Arc<ProcessState>,
    output: Mutex<Option<VecDeque<ProcessOutputChunk>>>,
}

impl FakeProcessService {
    pub fn completed(output: &str) -> (Arc<Self>, Arc<ProcessState>) {
        let state = Arc::new(ProcessState::default());
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                output: Mutex::new(Some(
                    [ProcessOutputChunk::new(
                        ProcessOutputStream::Stdout,
                        output.as_bytes().to_vec(),
                    )]
                    .into_iter()
                    .collect(),
                )),
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
        *self.state.request.lock().expect("request lock") = Some(ObservedProcessRequest {
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
        let output = self.output.lock().expect("output lock").take().unwrap();
        let handle = FakeProcessHandle {
            state: Arc::clone(&self.state),
            output: Mutex::new(output),
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FakeProcessHandle {
    state: Arc<ProcessState>,
    output: Mutex<VecDeque<ProcessOutputChunk>>,
}

impl ProcessHandle for FakeProcessHandle {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let output = self.output.lock().expect("output lock").pop_front();
        Box::pin(async move { Ok(output) })
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
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
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
                .expect("task lock")
                .take()
                .expect("task joins once")
                .join()
                .expect("fixture task does not panic");
            Ok(())
        })
    }
}

pub struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

pub fn services(host: ExecutionHostId, process: Arc<dyn ProcessService>) -> HostServices {
    HostServices::new(host)
        .with_task(Arc::new(ThreadTaskService))
        .with_time(Arc::new(PendingTime))
        .with_process(process)
}
