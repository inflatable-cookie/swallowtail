use super::runtime_failure;
use crate::{
    Deadline, DirectInferenceAttemptId, OperationContent, RequestId, RuntimeFailure, RuntimeTurnId,
    SessionOptions,
};
use std::num::NonZeroU32;
use swallowtail_core::{DirectAttemptTransport, DirectContinuationConfig, PreflightPlan};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenDirectContinuationSessionRequest {
    request_id: RequestId,
    config: DirectContinuationConfig,
    options: SessionOptions,
}

impl OpenDirectContinuationSessionRequest {
    #[must_use]
    pub fn new(request_id: RequestId, config: DirectContinuationConfig) -> Self {
        Self {
            request_id,
            config,
            options: SessionOptions::default(),
        }
    }

    #[must_use]
    pub fn with_options(mut self, options: SessionOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn config(&self) -> &DirectContinuationConfig {
        &self.config
    }

    #[must_use]
    pub const fn options(&self) -> &SessionOptions {
        &self.options
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectContinuationTurnRequest {
    turn_id: RuntimeTurnId,
    content: OperationContent,
    deadline: Deadline,
}

impl DirectContinuationTurnRequest {
    #[must_use]
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
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }

    #[must_use]
    pub const fn deadline(&self) -> Deadline {
        self.deadline
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectAttemptAuthorizationKind {
    UserTurn,
    CorrelatedToolResults,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectInferenceAttempt {
    pub(super) attempt_id: DirectInferenceAttemptId,
    pub(super) turn_id: RuntimeTurnId,
    pub(super) ordinal: NonZeroU32,
    pub(super) authorization: DirectAttemptAuthorizationKind,
    pub(super) transport: DirectAttemptTransport,
}

impl DirectInferenceAttempt {
    #[must_use]
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn ordinal(&self) -> NonZeroU32 {
        self.ordinal
    }

    #[must_use]
    pub const fn authorization(&self) -> DirectAttemptAuthorizationKind {
        self.authorization
    }

    #[must_use]
    pub const fn transport(&self) -> DirectAttemptTransport {
        self.transport
    }
}

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
