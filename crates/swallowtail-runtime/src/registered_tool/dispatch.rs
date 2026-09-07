//! Object-safe host dispatch port, call-bound cancellation, and progress sink.

use super::call::{RegisteredToolCall, RegisteredToolOutcome, RegisteredToolProgress};
use super::identity::RegisteredToolCallId;
use super::payload::RegisteredToolPayload;
use crate::{BoxFuture, RuntimeFailure};
use std::fmt;
use std::num::NonZeroU64;
use std::sync::Arc;

/// Kernel-owned source of one call's cancellation state.
pub trait RegisteredToolCancellationSource: Send + Sync {
    /// Reports whether this exact call is cancelled.
    fn is_cancelled(&self) -> bool;
}

/// Call-bound cancellation observation handed to one dispatcher.
///
/// The handle observes only. It cannot cancel another call, cancel the lease,
/// or extend any deadline.
#[derive(Clone)]
pub struct RegisteredToolCancellation {
    call_id: RegisteredToolCallId,
    source: Arc<dyn RegisteredToolCancellationSource>,
}

impl RegisteredToolCancellation {
    /// Binds one cancellation observation to exactly one call.
    #[must_use]
    pub fn new(
        call_id: RegisteredToolCallId,
        source: Arc<dyn RegisteredToolCancellationSource>,
    ) -> Self {
        Self { call_id, source }
    }

    /// Returns the call this observation is bound to.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        &self.call_id
    }

    /// Reports whether the bound call is cancelled.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.source.is_cancelled()
    }
}

impl fmt::Debug for RegisteredToolCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolCancellation")
            .field("call_id", &self.call_id)
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

/// Kernel-owned bounded progress queue for one call.
pub trait RegisteredToolProgressChannel: Send + Sync {
    /// Admits one correlated progress notification at the serialized point.
    ///
    /// Duplicate, regressive, foreign, stale-generation, post-cancel,
    /// post-terminal, post-close, expired, and queue-overflow notifications
    /// fail here, as does a binding whose live consumer admission is revoked.
    /// The live check is asynchronous by contract, so admission is too.
    fn admit(&self, progress: RegisteredToolProgress) -> BoxFuture<'_, Result<(), RuntimeFailure>>;
}

/// Bounded lease-associated progress sink handed to one dispatcher.
#[derive(Clone)]
pub struct RegisteredToolProgressSink {
    call: RegisteredToolCall,
    channel: Arc<dyn RegisteredToolProgressChannel>,
}

impl RegisteredToolProgressSink {
    /// Binds one bounded progress sink to exactly one committed call.
    ///
    /// Only the kernel constructs this: a dispatcher cannot fabricate a sink
    /// for another call.
    pub(super) fn new(
        call: RegisteredToolCall,
        channel: Arc<dyn RegisteredToolProgressChannel>,
    ) -> Self {
        Self { call, channel }
    }

    /// Publishes one bounded ordered progress item for the bound call.
    ///
    /// The kernel re-checks identity, generation, order, queue bounds, the
    /// effective call deadline, and the live consumer admission verdict before
    /// accepting the item. Because that live check is asynchronous by contract,
    /// publishing resolves through a future rather than returning immediately.
    pub fn publish(
        &self,
        sequence: NonZeroU64,
        payload: RegisteredToolPayload,
    ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let progress = match RegisteredToolProgress::mint(&self.call, sequence, payload) {
            Ok(progress) => progress,
            Err(error) => {
                let error = error.into_runtime_failure();
                return Box::pin(async move { Err(error) });
            }
        };
        self.channel.admit(progress)
    }

    /// Returns the call this sink is bound to.
    #[must_use]
    pub const fn call_id(&self) -> &RegisteredToolCallId {
        self.call.call_id()
    }
}

impl fmt::Debug for RegisteredToolProgressSink {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RegisteredToolProgressSink")
            .field("call_id", self.call.call_id())
            .finish()
    }
}

/// Everything one dispatcher may use besides the validated call itself.
#[derive(Clone, Debug)]
pub struct RegisteredToolDispatchContext {
    cancellation: RegisteredToolCancellation,
    progress: RegisteredToolProgressSink,
}

impl RegisteredToolDispatchContext {
    /// Binds one call-bound cancellation observation to one progress sink.
    #[must_use]
    pub const fn new(
        cancellation: RegisteredToolCancellation,
        progress: RegisteredToolProgressSink,
    ) -> Self {
        Self {
            cancellation,
            progress,
        }
    }

    /// Returns the call-bound cancellation observation.
    #[must_use]
    pub const fn cancellation(&self) -> &RegisteredToolCancellation {
        &self.cancellation
    }

    /// Returns the bounded progress sink.
    #[must_use]
    pub const fn progress(&self) -> &RegisteredToolProgressSink {
        &self.progress
    }
}

/// Object-safe linked host dispatch port.
///
/// Implementations validate schema and invoke consumer policy using the already
/// validated call token. They never mint a validated binding, check an
/// independent bearer, construct a second admission model, or publish progress
/// for another call.
pub trait RegisteredToolDispatcher: Send + Sync {
    /// Dispatches one validated call and settles exactly one outcome.
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>>;
}
