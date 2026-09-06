use futures_executor::block_on;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant,
    ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
    ProcessRequest, ProcessService, RuntimeFailure, ScopeId, ScopedTaskService, TimeService,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scenario {
    Version,
}

#[derive(Clone, Debug)]
pub struct ObservedProcess {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environment_count: usize,
    pub working_resource: Option<swallowtail_runtime::WorkingResourceRef>,
}

#[derive(Clone)]
pub struct FixtureHost {
    output: Arc<Mutex<Option<VecDeque<ProcessOutputChunk>>>>,
    process: Arc<Mutex<Option<ObservedProcess>>>,
}

impl FixtureHost {
    pub fn new(_scenario: Scenario, version: &str) -> Self {
        Self {
            output: Arc::new(Mutex::new(Some(
                [ProcessOutputChunk::new(
                    ProcessOutputStream::Stdout,
                    format!("{version}\n").into_bytes(),
                )]
                .into_iter()
                .collect(),
            ))),
            process: Arc::new(Mutex::new(None)),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(PendingTime))
            .with_process(Arc::new(self.clone()))
    }

    pub fn observed_process(&self) -> ObservedProcess {
        self.process
            .lock()
            .expect("process lock")
            .clone()
            .expect("process observed")
    }

    pub const fn credential_acquires(&self) -> usize {
        0
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.process.lock().expect("process lock") = Some(ObservedProcess {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environment_count: request.environment().len(),
            working_resource: request.working_resource().cloned(),
        });
        let output = self.output.lock().expect("output lock").take().unwrap();
        Box::pin(async move {
            Ok(Box::new(FixtureProcess(Mutex::new(output))) as Box<dyn ProcessHandle>)
        })
    }
}

struct FixtureProcess(Mutex<VecDeque<ProcessOutputChunk>>);

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let output = self.0.lock().expect("output lock").pop_front();
        Box::pin(async move { Ok(output) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

struct ThreadTaskService;
struct ThreadTask(Option<std::thread::JoinHandle<()>>);

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Some(std::thread::spawn(move || {
            block_on(task);
        })))))
    }
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.0.take().expect("task joins once");
        Box::pin(async move {
            handle.join().expect("fixture task does not panic");
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
