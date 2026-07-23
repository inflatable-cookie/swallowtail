use super::{
    DirectAttemptAuthorizationKind, DirectContinuationTurnRequest, DirectInferenceAttempt,
    DirectToolCall, DirectToolResult, runtime_failure,
};
use crate::{DirectInferenceAttemptId, DirectToolCallId, RuntimeFailure, RuntimeTurnId};
use std::collections::BTreeSet;
use std::num::NonZeroU32;
use swallowtail_core::DirectContinuationConfig;

/// Pure authorization state. It never performs provider or tool work.
pub struct DirectContinuationState {
    config: DirectContinuationConfig,
    user_turns: u32,
    attempts: u32,
    returned_tool_calls: u32,
    active_turn: Option<RuntimeTurnId>,
    pending_calls: BTreeSet<DirectToolCallId>,
    invalidated: bool,
}

impl DirectContinuationState {
    #[must_use]
    pub const fn new(config: DirectContinuationConfig) -> Self {
        Self {
            config,
            user_turns: 0,
            attempts: 0,
            returned_tool_calls: 0,
            active_turn: None,
            pending_calls: BTreeSet::new(),
            invalidated: false,
        }
    }

    pub fn authorize_user_turn(
        &mut self,
        request: &DirectContinuationTurnRequest,
    ) -> Result<DirectInferenceAttempt, RuntimeFailure> {
        self.require_valid()?;
        if self.active_turn.is_some() {
            return Err(state_failure(
                "A direct-continuation turn is already active",
            ));
        }
        if self.user_turns >= self.config.maximum_user_turns().get() {
            self.invalidate();
            return Err(limit_failure("Direct-continuation user-turn bound reached"));
        }
        self.user_turns += 1;
        self.active_turn = Some(request.turn_id().clone());
        self.authorize_attempt(
            request.turn_id().clone(),
            DirectAttemptAuthorizationKind::UserTurn,
        )
    }

    pub fn pause_for_tool_calls(
        &mut self,
        attempt: &DirectInferenceAttempt,
        calls: &[DirectToolCall],
    ) -> Result<(), RuntimeFailure> {
        self.require_valid()?;
        if calls.is_empty() || !self.pending_calls.is_empty() {
            return Err(state_failure(
                "Tool-call pause is not valid in the current state",
            ));
        }
        if self.active_turn.as_ref() != Some(attempt.turn_id()) {
            return Err(state_failure("Tool calls do not belong to the active turn"));
        }
        let count = match u32::try_from(calls.len()) {
            Ok(count) => count,
            Err(_) => {
                self.invalidate();
                return Err(limit_failure("Tool-call count exceeds its bound"));
            }
        };
        if self.returned_tool_calls.saturating_add(count)
            > self.config.maximum_returned_tool_calls().get()
        {
            self.invalidate();
            return Err(limit_failure("Returned tool-call bound reached"));
        }
        for call in calls {
            if call.attempt_id() != attempt.attempt_id()
                || !self.pending_calls.insert(call.call_id().clone())
            {
                self.pending_calls.clear();
                return Err(state_failure("Tool-call correlation is invalid"));
            }
        }
        self.returned_tool_calls += count;
        Ok(())
    }

    pub fn authorize_tool_results(
        &mut self,
        results: &[DirectToolResult],
    ) -> Result<DirectInferenceAttempt, RuntimeFailure> {
        self.require_valid()?;
        if self.pending_calls.is_empty() || results.len() != self.pending_calls.len() {
            return Err(state_failure("Exact pending tool results are required"));
        }
        let supplied: BTreeSet<_> = results
            .iter()
            .map(|result| result.call_id().clone())
            .collect();
        if supplied != self.pending_calls || supplied.len() != results.len() {
            return Err(state_failure("Tool-result correlation is invalid"));
        }
        self.pending_calls.clear();
        let turn_id = self
            .active_turn
            .clone()
            .ok_or_else(|| state_failure("No direct-continuation turn is active"))?;
        self.authorize_attempt(
            turn_id,
            DirectAttemptAuthorizationKind::CorrelatedToolResults,
        )
    }

    pub fn complete_turn(&mut self) -> Result<(), RuntimeFailure> {
        self.require_valid()?;
        if self.active_turn.is_none() || !self.pending_calls.is_empty() {
            return Err(state_failure(
                "Direct-continuation turn cannot complete now",
            ));
        }
        self.active_turn = None;
        Ok(())
    }

    pub fn invalidate(&mut self) {
        self.invalidated = true;
        self.active_turn = None;
        self.pending_calls.clear();
    }

    #[must_use]
    pub fn pending_tool_calls(&self) -> usize {
        self.pending_calls.len()
    }

    fn authorize_attempt(
        &mut self,
        turn_id: RuntimeTurnId,
        authorization: DirectAttemptAuthorizationKind,
    ) -> Result<DirectInferenceAttempt, RuntimeFailure> {
        if self.attempts >= self.config.maximum_inference_attempts().get() {
            self.invalidate();
            return Err(limit_failure(
                "Direct-continuation inference-attempt bound reached",
            ));
        }
        self.attempts += 1;
        let ordinal = NonZeroU32::new(self.attempts).expect("incremented attempt is positive");
        let transport = if ordinal.get() == 1 {
            self.config.initial_attempt_transport()
        } else {
            self.config.continued_attempt_transport()
        };
        let attempt_id = DirectInferenceAttemptId::new(format!("attempt-{}", ordinal.get()))
            .expect("generated attempt id is nonempty");
        Ok(DirectInferenceAttempt {
            attempt_id,
            turn_id,
            ordinal,
            authorization,
            transport,
        })
    }

    fn require_valid(&self) -> Result<(), RuntimeFailure> {
        if self.invalidated {
            Err(state_failure("Direct-continuation session is invalidated"))
        } else {
            Ok(())
        }
    }
}

fn state_failure(message: &'static str) -> RuntimeFailure {
    runtime_failure("swallowtail.direct_continuation.invalid_state", message)
}

fn limit_failure(message: &'static str) -> RuntimeFailure {
    runtime_failure("swallowtail.direct_continuation.limit_reached", message)
}
