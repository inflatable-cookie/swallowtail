use futures_util::future;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use swallowtail_core::{EndpointAudience, ExecutionHostId, ModelArtifactBinding, SafeDiagnostic};
use swallowtail_runtime::{
    AuthorizedEndpoint, BoxFuture, CleanupOutcome, EndpointRef, MaterializedModelArtifactRef,
    ModelArtifactLease, ModelArtifactService, NetworkGrant, NetworkPolicyService,
    ObservedServingEndpoint, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
    ServingEndpointBinding, ServingEndpointLease, ServingEndpointService,
};

#[path = "owned_services/time.rs"]
mod time;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnedCall {
    ArtifactAcquire,
    ProcessStart,
    EndpointPublish,
    NetworkAuthorize,
    GracefulStop,
    ForceStop,
    ProcessWait,
    EndpointRelease,
    ArtifactRelease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessStop {
    Graceful,
    NeedsForce,
}

struct State {
    output: VecDeque<ProcessOutputChunk>,
    running: bool,
    stop: ProcessStop,
    expire_deadlines: bool,
    waiters: Vec<Waker>,
    calls: Vec<OwnedCall>,
    arguments: Vec<String>,
    observed_endpoint: Option<String>,
}

#[derive(Clone)]
pub struct ScriptedOwnedServices {
    state: Arc<Mutex<State>>,
}

impl ScriptedOwnedServices {
    pub fn new(stderr: impl Into<Vec<u8>>, stop: ProcessStop) -> Self {
        let bytes = stderr.into();
        let chunks = (!bytes.is_empty()).then_some(bytes).into_iter();
        Self::with_chunks(chunks, stop)
    }

    pub fn with_chunks(stderr: impl IntoIterator<Item = Vec<u8>>, stop: ProcessStop) -> Self {
        let output = stderr
            .into_iter()
            .map(|bytes| ProcessOutputChunk::new(ProcessOutputStream::Stderr, bytes))
            .collect();
        Self {
            state: Arc::new(Mutex::new(State {
                output,
                running: true,
                stop,
                expire_deadlines: false,
                waiters: Vec::new(),
                calls: Vec::new(),
                arguments: Vec::new(),
                observed_endpoint: None,
            })),
        }
    }

    pub fn exited() -> Self {
        let services = Self::new(Vec::new(), ProcessStop::Graceful);
        services.state.lock().expect("state lock").running = false;
        services
    }

    pub fn readiness_timeout() -> Self {
        let services = Self::new(Vec::new(), ProcessStop::Graceful);
        services.state.lock().expect("state lock").expire_deadlines = true;
        services
    }

    pub fn calls(&self) -> Vec<OwnedCall> {
        self.state.lock().expect("state lock").calls.clone()
    }

    pub fn arguments(&self) -> Vec<String> {
        self.state.lock().expect("state lock").arguments.clone()
    }

    fn record(&self, call: OwnedCall) {
        self.state.lock().expect("state lock").calls.push(call);
    }
}

struct ScriptedProcessHandle(ScriptedOwnedServices);

impl ProcessHandle for ScriptedProcessHandle {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let result = {
            let mut state = self.0.state.lock().expect("state lock");
            state
                .output
                .pop_front()
                .or_else(|| (!state.running).then_some(None).flatten())
        };
        match result {
            Some(chunk) => Box::pin(async move { Ok(Some(chunk)) }),
            None if !self.0.state.lock().expect("state lock").running => {
                Box::pin(async { Ok(None) })
            }
            None => Box::pin(future::pending()),
        }
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self.0.state.lock().expect("state lock");
        state.calls.push(OwnedCall::GracefulStop);
        if state.stop == ProcessStop::Graceful {
            state.running = false;
            for waiter in state.waiters.drain(..) {
                waiter.wake();
            }
        }
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self.0.state.lock().expect("state lock");
        state.calls.push(OwnedCall::ForceStop);
        state.running = false;
        for waiter in state.waiters.drain(..) {
            waiter.wake();
        }
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.0.record(OwnedCall::ProcessWait);
        let state = Arc::clone(&self.0.state);
        Box::pin(std::future::poll_fn(move |context| {
            let mut state = state.lock().expect("state lock");
            if state.running {
                if !state
                    .waiters
                    .iter()
                    .any(|waiter| waiter.will_wake(context.waker()))
                {
                    state.waiters.push(context.waker().clone());
                }
                std::task::Poll::Pending
            } else {
                std::task::Poll::Ready(Ok(ProcessExit::new(true, Some(0))))
            }
        }))
    }
}

impl ProcessService for ScriptedOwnedServices {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let arguments = request.arguments().map(str::to_owned).collect();
        let mut state = self.state.lock().expect("state lock");
        state.calls.push(OwnedCall::ProcessStart);
        state.arguments = arguments;
        let handle = ScriptedProcessHandle(self.clone());
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

impl ModelArtifactService for ScriptedOwnedServices {
    fn acquire(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        binding: ModelArtifactBinding,
    ) -> BoxFuture<'static, Result<ModelArtifactLease, RuntimeFailure>> {
        self.record(OwnedCall::ArtifactAcquire);
        let lease = ModelArtifactLease::read_only(
            scope,
            execution_host_id,
            binding,
            MaterializedModelArtifactRef::new("/private/models/fixture.gguf")
                .expect("materialization is valid"),
        );
        Box::pin(async move { Ok(lease) })
    }

    fn release(&self, _lease: ModelArtifactLease) -> BoxFuture<'static, CleanupOutcome> {
        self.record(OwnedCall::ArtifactRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

impl ServingEndpointService for ScriptedOwnedServices {
    fn publish(
        &self,
        scope: ScopeId,
        execution_host_id: ExecutionHostId,
        audience: EndpointAudience,
        observed: ObservedServingEndpoint,
    ) -> BoxFuture<'static, Result<ServingEndpointLease, RuntimeFailure>> {
        let mut state = self.state.lock().expect("state lock");
        state.calls.push(OwnedCall::EndpointPublish);
        state.observed_endpoint = Some(observed.as_driver_value().to_owned());
        let lease = ServingEndpointLease::new(ServingEndpointBinding::new(
            scope,
            execution_host_id,
            EndpointRef::new("llama-cpp-owned-endpoint").expect("endpoint ref is valid"),
            audience,
        ));
        Box::pin(async move { Ok(lease) })
    }

    fn release(&self, _lease: ServingEndpointLease) -> BoxFuture<'static, CleanupOutcome> {
        self.record(OwnedCall::EndpointRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl NetworkPolicyService for ScriptedOwnedServices {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        let authorized = {
            let mut state = self.state.lock().expect("state lock");
            state.calls.push(OwnedCall::NetworkAuthorize);
            state.observed_endpoint.clone()
        };
        let result = authorized
            .ok_or_else(|| fixture_failure("Published endpoint is unavailable"))
            .and_then(|value| {
                AuthorizedEndpoint::new(value)
                    .map(|authorized| NetworkGrant::new(scope, endpoint, audience, authorized))
                    .map_err(|_| fixture_failure("Published endpoint is invalid"))
            });
        Box::pin(async move { result })
    }
}

fn fixture_failure(message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new("fixture.owned_service_failed", message))
}
