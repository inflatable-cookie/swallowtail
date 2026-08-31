use super::*;

impl SharedAgent {
    pub(super) fn complete_read(
        &self,
        state: &mut AgentState,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["result"]["content"] != "fixture file" {
            return Err(fixture_failure());
        }
        if let Some(id) = state.prompt_id.take() {
            let total_tokens = if self.scenario == Scenario::MalformedUsage {
                22
            } else {
                21
            };
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "end_turn",
                    "usage": {
                        "inputTokens": 12,
                        "outputTokens": 4,
                        "cachedReadTokens": 3,
                        "cachedWriteTokens": 2,
                        "totalTokens": total_tokens
                    }
                }}),
            );
        }
        Ok(())
    }

    pub(super) fn permission_response(
        &self,
        state: &mut AgentState,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["result"]["outcome"]["outcome"] == "selected"
            && message["result"]["outcome"]["optionId"] == "reject-once"
        {
            Ok(())
        } else if message["result"]["outcome"]["outcome"] == "selected"
            && message["result"]["outcome"]["optionId"] == "allow-once"
        {
            let id = state.prompt_id.take().ok_or_else(fixture_failure)?;
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "end_turn",
                    "usage": {
                        "inputTokens": 12,
                        "outputTokens": 4,
                        "cachedReadTokens": 3,
                        "cachedWriteTokens": 2,
                        "totalTokens": 21
                    }
                }}),
            );
            Ok(())
        } else {
            Err(fixture_failure())
        }
    }

    pub(super) fn elicitation_response(
        &self,
        state: &mut AgentState,
        message: &Value,
    ) -> Result<(), RuntimeFailure> {
        if message["result"] != json!({"action": "accept", "content": {"question_0": "Panel"}}) {
            return Err(fixture_failure());
        }
        let id = state.prompt_id.take().ok_or_else(fixture_failure)?;
        Self::enqueue(
            state,
            json!({"jsonrpc": "2.0", "id": id, "result": {
                "stopReason": "end_turn",
                "usage": {
                    "inputTokens": 12,
                    "outputTokens": 4,
                    "cachedReadTokens": 3,
                    "cachedWriteTokens": 2,
                    "totalTokens": 21
                }
            }}),
        );
        Ok(())
    }

    pub(super) fn cancel(&self, state: &mut AgentState) -> Result<(), RuntimeFailure> {
        if let Some(id) = state.prompt_id.take() {
            Self::enqueue(
                state,
                json!({"jsonrpc": "2.0", "id": id, "result": {
                    "stopReason": "cancelled",
                    "usage": {
                        "inputTokens": 0,
                        "outputTokens": 0,
                        "cachedReadTokens": 0,
                        "cachedWriteTokens": 0,
                        "totalTokens": 0
                    }
                }}),
            );
        }
        Ok(())
    }
}
