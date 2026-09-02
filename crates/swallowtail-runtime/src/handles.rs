#![deny(missing_docs)]

use crate::{
    BoxEventStream, BoxFuture, BoxRealtimeMediaEventStream, CallbackExchange, CancellationControl,
    CleanupOutcome, DirectContinuationTurnRequest, DirectToolExchange, HarnessCommandResponse,
    HarnessScheduledMessage, HostServices, MediaChunk, MediaInputCommit,
    NegotiatedSessionModelOptions, OperationDetachmentControl, ProviderSessionManagementBinding,
    RequestId, RuntimeFailure, RuntimeRunId, RuntimeSessionId, RuntimeTurnId,
    ServingEndpointBinding, ServingInstanceId, SessionCleanupRequest, SessionResumeBinding,
    TerminalOutcome, TurnRequest,
};
use swallowtail_core::{ExecutionHostId, InstanceOwnership, RunRef, SessionRef, TurnRef};

/// Owns one in-flight structured run and its operation-scoped resources.
///
/// Event, callback, management, and terminal receivers are take-once views.
/// Call [`RunHandle::close`] after terminal observation to join scoped work and
/// release host leases.
pub trait RunHandle: Send {
    /// Returns the caller-assigned request identity.
    fn request_id(&self) -> &RequestId;
    /// Returns the runtime identity assigned to this run.
    fn run_id(&self) -> &RuntimeRunId;
    /// Returns the provider's opaque run reference when the route exposes one.
    fn provider_run_ref(&self) -> Option<&RunRef>;
    /// Takes the ordered portable event stream, if it has not already been taken.
    fn take_events(&mut self) -> Option<BoxEventStream>;
    /// Takes the provider-callback exchange when this run supports callbacks.
    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }
    /// Takes inactive-session management authority produced by this run.
    fn take_management_binding(&mut self) -> Option<ProviderSessionManagementBinding> {
        None
    }
    /// Returns the run's cancellation control.
    fn cancellation(&self) -> &dyn CancellationControl;
    /// Returns explicit detachment control when the provider can outlive the observer.
    fn detachment(&self) -> Option<&dyn OperationDetachmentControl> {
        None
    }
    /// Takes the future that resolves exactly once with the terminal outcome.
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    /// Joins scoped work and releases operation-owned resources.
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// Owns one in-flight turn within an interactive session.
///
/// A turn handle borrows no session state after creation. Consumers must close
/// it before starting another turn when the driver requires serialized turns.
pub trait TurnHandle: Send {
    /// Returns the caller-assigned runtime turn identity.
    fn turn_id(&self) -> &RuntimeTurnId;
    /// Returns the provider's opaque turn reference when available.
    fn provider_turn_ref(&self) -> Option<&TurnRef>;
    /// Takes the ordered portable event stream, if not already taken.
    fn take_events(&mut self) -> Option<BoxEventStream>;
    /// Takes the provider-callback exchange when this turn supports callbacks.
    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }
    /// Takes the direct tool exchange when the route delegates tool execution.
    fn take_direct_tool_exchange(&mut self) -> Option<DirectToolExchange> {
        None
    }
    /// Schedules a provider-harness message during the live turn.
    ///
    /// The default implementation reports that scheduling is unsupported.
    fn schedule_harness_message(
        &mut self,
        _message: HarnessScheduledMessage,
    ) -> BoxFuture<'_, Result<HarnessCommandResponse, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.harness_message_scheduling_unsupported",
                "Turn does not support harness message scheduling",
            )))
        })
    }
    /// Returns the turn's cancellation control.
    fn cancellation(&self) -> &dyn CancellationControl;
    /// Returns explicit detachment control when the provider can outlive the observer.
    fn detachment(&self) -> Option<&dyn OperationDetachmentControl> {
        None
    }
    /// Takes the future that resolves exactly once with the terminal outcome.
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    /// Joins scoped work and releases turn-owned resources.
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// Owns a reusable interactive provider session.
///
/// The handle exposes only authority established while opening, loading,
/// resuming, or recovering this exact session.
pub trait InteractiveSessionHandle: Send {
    /// Returns the request that created this runtime handle.
    fn request_id(&self) -> &RequestId;
    /// Returns the runtime identity assigned to the session.
    fn session_id(&self) -> &RuntimeSessionId;
    /// Returns the provider's opaque session reference when available.
    fn provider_session_ref(&self) -> Option<&SessionRef>;
    /// Returns persistable ordinary resume authority when the route supplies it.
    fn resume_binding(&self) -> Option<&SessionResumeBinding>;
    /// Returns inactive-session management authority when separately granted.
    fn management_binding(&self) -> Option<&ProviderSessionManagementBinding> {
        None
    }
    /// Model selectors advertised while opening or attaching this session.
    ///
    /// This must not be treated as a side-effect-free pre-session catalogue.
    fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        None
    }
    /// Starts one ordinary turn using the supplied host services.
    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>>;
    /// Starts one turn from portable direct-continuation state.
    ///
    /// The default implementation reports that direct continuation is unsupported.
    fn start_direct_continuation_turn<'a>(
        &'a mut self,
        _request: DirectContinuationTurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.direct_continuation.unsupported",
                "Driver does not support locally continued direct turns",
            )))
        })
    }
    /// Returns session-wide cancellation control.
    fn cancellation(&self) -> &dyn CancellationControl;
    /// Closes the session within the caller's host-monotonic cleanup boundary.
    fn close(
        self: Box<Self>,
        request: SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome>;
}

/// Owns one in-flight response on a realtime media session.
pub trait RealtimeMediaResponseHandle: Send {
    /// Returns the runtime turn identity for this response.
    fn turn_id(&self) -> &RuntimeTurnId;
    /// Takes the ordered realtime event stream, if not already taken.
    fn take_events(&mut self) -> Option<BoxRealtimeMediaEventStream>;
    /// Returns the response's cancellation control.
    fn cancellation(&self) -> &dyn CancellationControl;
    /// Takes the future that resolves exactly once with the terminal outcome.
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>>;
    /// Joins response work and releases response-owned resources.
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// Owns a duplex realtime media session.
pub trait RealtimeMediaSessionHandle: Send {
    /// Returns the request that opened this session.
    fn request_id(&self) -> &RequestId;
    /// Returns the runtime identity assigned to the session.
    fn session_id(&self) -> &RuntimeSessionId;
    /// Appends one media chunk to the current uncommitted input.
    fn append_input<'a>(
        &'a mut self,
        chunk: MediaChunk,
        services: HostServices,
    ) -> BoxFuture<'a, Result<(), RuntimeFailure>>;
    /// Commits the pending input and starts one response.
    fn commit_input<'a>(
        &'a mut self,
        commit: MediaInputCommit,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn RealtimeMediaResponseHandle>, RuntimeFailure>>;
    /// Returns session-wide cancellation control.
    fn cancellation(&self) -> &dyn CancellationControl;
    /// Closes the session and releases its scoped resources.
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// An attached service can be released but exposes no generic stop operation.
pub trait AttachedServingHandle: Send {
    /// Returns the stable identity of the attached serving instance.
    fn serving_instance_id(&self) -> &ServingInstanceId;
    /// Releases the attachment without claiming authority to stop the service.
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}

/// Owns a serving process that the host authorized Swallowtail to stop.
pub trait OwnedServingHandle: Send {
    /// Returns the stable identity of the serving instance.
    fn serving_instance_id(&self) -> &ServingInstanceId;
    /// Returns the instance ownership classification.
    fn ownership(&self) -> InstanceOwnership;
    /// Returns the execution host on which the instance is running.
    fn execution_host_id(&self) -> &ExecutionHostId;
    /// Returns the endpoint admitted for consumers of the instance.
    fn endpoint_binding(&self) -> &ServingEndpointBinding;
    /// Stops the owned instance and joins cleanup work.
    fn stop(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome>;
}
