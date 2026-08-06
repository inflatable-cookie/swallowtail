use super::runtime_failure;
use crate::{
    Deadline, DirectInferenceAttemptId, OperationContent, RequestId, RuntimeFailure, RuntimeTurnId,
    SessionOptions,
};
use std::num::NonZeroU32;
use swallowtail_core::{DirectAttemptTransport, DirectContinuationConfig, PreflightPlan};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Request to open a resource-free direct-continuation session.
pub struct OpenDirectContinuationSessionRequest {
    request_id: RequestId,
    config: DirectContinuationConfig,
    options: SessionOptions,
}

impl OpenDirectContinuationSessionRequest {
    #[must_use]
    /// Creates a request with default session options.
    pub fn new(request_id: RequestId, config: DirectContinuationConfig) -> Self {
        Self {
            request_id,
            config,
            options: SessionOptions::default(),
        }
    }

    #[must_use]
    /// Replaces the session options, including the declared tool set.
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    /// Returns the consumer request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the immutable direct-continuation bounds and transport policy.
    pub const fn config(&self) -> &DirectContinuationConfig {
        &self.config
    }

    #[must_use]
    /// Returns the session options and declared tools.
    pub const fn options(&self) -> &SessionOptions {
        &self.options
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One consumer-authored user turn in a direct-continuation session.
pub struct DirectContinuationTurnRequest {
    turn_id: RuntimeTurnId,
    content: OperationContent,
    deadline: Deadline,
}

impl DirectContinuationTurnRequest {
    #[must_use]
    /// Creates a turn request with an exact deadline.
    pub const fn new(
        turn_id: RuntimeTurnId,
        content: OperationContent,
        deadline: Deadline,
    ) -> Self {
        Self {
            turn_id,
            content,
            deadline,
        }
    }

    #[must_use]
    /// Returns the consumer turn identity used across all related attempts.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    /// Returns the user-authored operation content.
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    /// Returns the absolute turn deadline.
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Consumer authority that permits one direct inference attempt.
pub enum DirectAttemptAuthorizationKind {
    /// A new user turn authorized the attempt.
    UserTurn,
    /// A complete result set for the preceding tool calls authorized the attempt.
    CorrelatedToolResults,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// A single bounded inference attempt within one direct-continuation turn.
pub struct DirectInferenceAttempt {
    pub(super) attempt_id: DirectInferenceAttemptId,
    pub(super) turn_id: RuntimeTurnId,
    pub(super) ordinal: NonZeroU32,
    pub(super) authorization: DirectAttemptAuthorizationKind,
    pub(super) transport: DirectAttemptTransport,
}

impl DirectInferenceAttempt {
    #[must_use]
    /// Returns the attempt identity used to correlate tool calls.
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    /// Returns the containing user-turn identity.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    /// Returns the one-based attempt ordinal within the session.
    pub const fn ordinal(&self) -> NonZeroU32 {
        self.ordinal
    }

    #[must_use]
    /// Returns the consumer authority that permitted this attempt.
    pub const fn authorization(&self) -> DirectAttemptAuthorizationKind {
        self.authorization
    }

    #[must_use]
    /// Returns the preflight-selected transport for this attempt.
    pub const fn transport(&self) -> DirectAttemptTransport {
        self.transport
    }
}

/// Validates a direct-continuation request against its immutable preflight plan.
///
/// This checks exact configuration equality and the declared-tool bound before
/// any provider effect begins.
pub fn validate_direct_continuation_plan(
    plan: &PreflightPlan,
    request: &OpenDirectContinuationSessionRequest,
) -> Result<(), RuntimeFailure> {
    let required = plan.requirements().direct_continuation().ok_or_else(|| {
        runtime_failure(
            "swallowtail.direct_continuation.plan_missing",
            "Preflight plan does not permit direct continuation",
        )
    })?;
    if required.config() != request.config() {
        return Err(runtime_failure(
            "swallowtail.direct_continuation.plan_mismatch",
            "Direct-continuation request does not match its immutable preflight plan",
        ));
    }
    if request.options().tools().len() > request.config().maximum_declared_tools().get() as usize {
        return Err(runtime_failure(
            "swallowtail.direct_continuation.tool_limit",
            "Declared tools exceed the direct-continuation bound",
        ));
    }
    Ok(())
}
