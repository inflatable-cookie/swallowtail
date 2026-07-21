fn replay_item(
    session: SessionRef,
    sequence: u64,
    kind: &str,
    update: &Value,
) -> Result<SessionReplayItem, RuntimeFailure> {
    let replay_kind = match kind {
        "user_message_chunk" => SessionReplayKind::UserMessage,
        "agent_message_chunk" => SessionReplayKind::AgentMessage,
        "agent_thought_chunk" => SessionReplayKind::AgentReasoning,
        "tool_call" => SessionReplayKind::ToolCall,
        "tool_call_update" => SessionReplayKind::ToolCallUpdate,
        "plan" => SessionReplayKind::Plan,
        "available_commands_update" | "config_option_update" | "current_mode_update" => {
            SessionReplayKind::Configuration
        }
        _ => {
            return Err(failure(
                "swallowtail.kimi.acp.replay_unsupported",
                "Kimi Code returned unsupported replay content",
            ));
        }
    };
    match kind {
        "user_message_chunk" | "agent_message_chunk" | "agent_thought_chunk" => {
            let text = update
                .get("content")
                .filter(|content| content.get("type").and_then(Value::as_str) == Some("text"))
                .and_then(|content| content.get("text"))
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            let content = OperationContent::new(text).map_err(|_| malformed())?;
            Ok(SessionReplayItem::with_content(
                session,
                sequence,
                replay_kind,
                content,
            ))
        }
        _ => Ok(SessionReplayItem::new(session, sequence, replay_kind)),
    }
}

fn passive_update(kind: &str) -> Result<(), RuntimeFailure> {
    match kind {
        "available_commands_update" | "config_option_update" | "current_mode_update" => Ok(()),
        _ => Err(failure(
            "swallowtail.kimi.acp.update_without_turn",
            "Kimi Code updated a session outside an allowed lifecycle phase",
        )),
    }
}

fn session_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.acp.session_mismatch",
        "Kimi Code message does not match the bound session",
    )
}
