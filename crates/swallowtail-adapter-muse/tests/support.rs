#![allow(dead_code)]

use futures_executor::block_on;
use std::collections::VecDeque;
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
    pub working_resource: Option<String>,
}

pub struct FixtureHost {
    process: Arc<FixtureProcessService>,
    observed: Arc<Mutex<Vec<ObservedProcess>>>,
}

impl FixtureHost {
    pub fn scripted(outputs: impl IntoIterator<Item = &'static str>) -> Self {
        let observed = Arc::new(Mutex::new(Vec::new()));
        Self {
            process: Arc::new(FixtureProcessService {
                outputs: Mutex::new(
                    outputs
                        .into_iter()
                        .map(|output| {
                            VecDeque::from([ProcessOutputChunk::new(
                                swallowtail_runtime::ProcessOutputStream::Stdout,
                                output.as_bytes().to_vec(),
                            )])
                        })
                        .collect(),
                ),
                observed: Arc::clone(&observed),
            }),
            observed,
        }
    }

    pub fn services(&self, host_id: ExecutionHostId) -> HostServices {
        HostServices::new(host_id)
            .with_task(Arc::new(ThreadTaskService))
            .with_process(self.process.clone())
            .with_time(Arc::new(PendingTime))
    }

    pub fn observations(&self) -> Vec<ObservedProcess> {
        self.observed.lock().expect("observation lock").clone()
    }

    pub fn started(&self) -> bool {
        !self.observed.lock().expect("observation lock").is_empty()
    }
}

struct FixtureProcessService {
    outputs: Mutex<VecDeque<VecDeque<ProcessOutputChunk>>>,
    observed: Arc<Mutex<Vec<ObservedProcess>>>,
}

impl ProcessService for FixtureProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.observed
            .lock()
            .expect("observation lock")
            .push(ObservedProcess {
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
        let outputs = self
            .outputs
            .lock()
            .expect("output lock")
            .pop_front()
            .expect("scripted process output");
        Box::pin(async move {
            Ok(Box::new(FixtureProcess {
                outputs: Mutex::new(outputs),
            }) as Box<dyn ProcessHandle>)
        })
    }
}

struct FixtureProcess {
    outputs: Mutex<VecDeque<ProcessOutputChunk>>,
}

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move { Ok(self.outputs.lock().expect("output lock").pop_front()) })
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

struct ThreadTask(Option<std::thread::JoinHandle<()>>);

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

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move {
            std::future::pending::<()>().await;
            DeadlineObservation::new(deadline, deadline.instant())
        })
    }
}
