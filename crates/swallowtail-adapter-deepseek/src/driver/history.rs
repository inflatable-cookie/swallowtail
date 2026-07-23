use crate::failure::failure;
use crate::protocol::{FinalAttempt, PrivateContinuation, ToolAttempt};
use swallowtail_core::DirectContinuationConfig;
use swallowtail_runtime::{DirectToolResult, RuntimeFailure};

pub(super) struct SessionHistory {
    maximum_bytes: u64,
    first: Option<FirstTurnHistory>,
}

impl SessionHistory {
    pub(super) fn new(config: &DirectContinuationConfig) -> Self {
        Self {
            maximum_bytes: config.maximum_private_history_bytes().get(),
            first: None,
        }
    }

    pub(super) fn record_tool_attempt(
        &mut self,
        user: &str,
        attempt: ToolAttempt,
    ) -> Result<(), RuntimeFailure> {
        if self.first.is_some() {
            return Err(state_failure());
        }
        let first = FirstTurnHistory {
            user: SecretText::new(user),
            reasoning: attempt.reasoning,
            call_id: attempt.call.call_id().as_str().to_owned(),
            tool_name: attempt.call.tool_name().to_owned(),
            arguments: SecretText::new(
                std::str::from_utf8(attempt.call.arguments().as_bytes())
                    .map_err(|_| state_failure())?,
            ),
            result: None,
            final_reasoning: None,
            answer: None,
        };
        self.require_bound(first.byte_len())?;
        self.first = Some(first);
        Ok(())
    }

    pub(super) fn record_tool_result(
        &mut self,
        result: &DirectToolResult,
    ) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(state_failure)?;
        if first.result.is_some() || result.call_id().as_str() != first.call_id {
            return Err(state_failure());
        }
        let value =
            std::str::from_utf8(result.content().as_bytes()).map_err(|_| state_failure())?;
        first.result = Some(SecretText::new(value));
        self.require_current_bound()
    }

    pub(super) fn record_first_final(
        &mut self,
        attempt: FinalAttempt,
    ) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(state_failure)?;
        if first.result.is_none() || first.answer.is_some() {
            return Err(state_failure());
        }
        first.final_reasoning = Some(attempt.reasoning);
        first.answer = Some(SecretText::new(&attempt.output));
        self.require_current_bound()
    }

    pub(super) fn first(&self) -> Result<&FirstTurnHistory, RuntimeFailure> {
        self.first.as_ref().ok_or_else(state_failure)
    }

    pub(super) fn is_complete(&self) -> bool {
        self.first
            .as_ref()
            .is_some_and(FirstTurnHistory::is_complete)
    }

    fn require_current_bound(&self) -> Result<(), RuntimeFailure> {
        self.require_bound(self.first.as_ref().map_or(0, FirstTurnHistory::byte_len))
    }

    fn require_bound(&self, bytes: usize) -> Result<(), RuntimeFailure> {
        if u64::try_from(bytes).is_err() || bytes as u64 > self.maximum_bytes {
            Err(failure(
                "swallowtail.deepseek.history_bound_exceeded",
                "DeepSeek private session history exceeded its selected bound",
            ))
        } else {
            Ok(())
        }
    }
}

pub(super) struct FirstTurnHistory {
    user: SecretText,
    reasoning: PrivateContinuation,
    call_id: String,
    tool_name: String,
    arguments: SecretText,
    result: Option<SecretText>,
    final_reasoning: Option<PrivateContinuation>,
    answer: Option<SecretText>,
}

impl FirstTurnHistory {
    pub(super) fn user(&self) -> &str {
        self.user.as_str()
    }
    pub(super) fn reasoning(&self) -> &PrivateContinuation {
        &self.reasoning
    }
    pub(super) fn call_id(&self) -> &str {
        &self.call_id
    }
    pub(super) fn tool_name(&self) -> &str {
        &self.tool_name
    }
    pub(super) fn arguments(&self) -> &str {
        self.arguments.as_str()
    }
    pub(super) fn result(&self) -> Result<&str, RuntimeFailure> {
        self.result
            .as_ref()
            .map(SecretText::as_str)
            .ok_or_else(state_failure)
    }
    pub(super) fn final_reasoning(&self) -> Result<&PrivateContinuation, RuntimeFailure> {
        self.final_reasoning.as_ref().ok_or_else(state_failure)
    }
    pub(super) fn answer(&self) -> Result<&str, RuntimeFailure> {
        self.answer
            .as_ref()
            .map(SecretText::as_str)
            .ok_or_else(state_failure)
    }

    fn is_complete(&self) -> bool {
        self.result.is_some() && self.final_reasoning.is_some() && self.answer.is_some()
    }

    fn byte_len(&self) -> usize {
        self.user.byte_len()
            + self.reasoning.byte_len()
            + self.call_id.len()
            + self.tool_name.len()
            + self.arguments.byte_len()
            + self.result.as_ref().map_or(0, SecretText::byte_len)
            + self
                .final_reasoning
                .as_ref()
                .map_or(0, PrivateContinuation::byte_len)
            + self.answer.as_ref().map_or(0, SecretText::byte_len)
    }
}

struct SecretText(Vec<u8>);

impl SecretText {
    fn new(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("source text remains UTF-8")
    }
    fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

fn state_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.history_state_invalid",
        "DeepSeek private continuation history was not in the required state",
    )
}
