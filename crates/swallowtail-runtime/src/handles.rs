use crate::{
    BoxEventStream, BoxFuture, BoxRealtimeMediaEventStream, CallbackExchange, CancellationControl,
    CleanupOutcome, HostServices, MediaChunk, MediaInputCommit, RequestId, RuntimeFailure,
    RuntimeRunId, RuntimeSessionId, RuntimeTurnId, ServingEndpointBinding, ServingInstanceId,
    SessionResumeBinding, TerminalOutcome, TurnRequest,
};
use swallowtail_core::{ExecutionHostId, InstanceOwnership, RunRef, SessionRef, TurnRef};

pub trait RunHandle: Send {
    fn request_id(&self) -> &RequestId;
    fn run_id(&self) -> &RuntimeRunId;
    fn provider_run_ref(&self) -> Option<&RunRef>;
    fn take_events(&mut self) -> Option<BoxEventStream>;
    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }
    fn cancellation(&self) -> &dyn CancellationControl;
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait TurnHandle: Send {
    fn turn_id(&self) -> &RuntimeTurnId;
    fn provider_turn_ref(&self) -> Option<&TurnRef>;
    fn take_events(&mut self) -> Option<BoxEventStream>;
    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }
    fn cancellation(&self) -> &dyn CancellationControl;
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait InteractiveSessionHandle: Send {
    fn request_id(&self) -> &RequestId;
    fn session_id(&self) -> &RuntimeSessionId;
    fn provider_session_ref(&self) -> Option<&SessionRef>;
    fn resume_binding(&self) -> Option<&SessionResumeBinding>;
    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>>;
    fn cancellation(&self) -> &dyn CancellationControl;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait RealtimeMediaResponseHandle: Send {
    fn turn_id(&self) -> &RuntimeTurnId;
    fn take_events(&mut self) -> Option<BoxRealtimeMediaEventStream>;
    fn cancellation(&self) -> &dyn CancellationControl;
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait RealtimeMediaSessionHandle: Send {
    fn request_id(&self) -> &RequestId;
    fn session_id(&self) -> &RuntimeSessionId;
    fn append_input<'a>(
        &'a mut self,
        chunk: MediaChunk,
        services: HostServices,
    ) -> BoxFuture<'a, Result<(), RuntimeFailure>>;
    fn commit_input<'a>(
        &'a mut self,
        commit: MediaInputCommit,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn RealtimeMediaResponseHandle>, RuntimeFailure>>;
    fn cancellation(&self) -> &dyn CancellationControl;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// An attached service can be released but exposes no generic stop operation.
pub trait AttachedServingHandle: Send {
    fn serving_instance_id(&self) -> &ServingInstanceId;
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

pub trait OwnedServingHandle: Send {
    fn serving_instance_id(&self) -> &ServingInstanceId;
    fn ownership(&self) -> InstanceOwnership;
    fn execution_host_id(&self) -> &ExecutionHostId;
    fn endpoint_binding(&self) -> &ServingEndpointBinding;
    fn stop(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}
