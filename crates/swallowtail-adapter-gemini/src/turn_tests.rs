mod tests {
    use super::ActiveTurn;
    use futures_executor::block_on;
    use serde_json::json;
    use swallowtail_runtime::{RuntimeTurnId, TerminalStatus};

    #[test]
    fn text_updates_finish_once_and_mode_widening_fails() {
        let (turn, _events, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-fixture").expect("valid turn"),
            "session-fixture".to_owned(),
        )
        .expect("turn opens");
        turn.handle_update(&json!({
            "sessionId": "session-fixture",
            "update": {
                "sessionUpdate": "agent_message_chunk",
                "content": {"type": "text", "text": "fixture output"}
            }
        }))
        .expect("text update succeeds");
        assert!(
            turn.handle_update(&json!({
                "sessionId": "session-fixture",
                "update": {"sessionUpdate": "current_mode_update", "currentModeId": "yolo"}
            }))
            .is_err()
        );
        turn.finish_prompt("end_turn");
        let outcome = block_on(terminal);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.output().expect("output exists").as_str(),
            "fixture output"
        );
    }

    #[test]
    fn permission_requests_become_observe_and_stop_outcomes() {
        let (turn, _events, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-permission").expect("valid turn"),
            "session-fixture".to_owned(),
        )
        .expect("turn opens");
        turn.observe_permission(&json!(900))
            .expect("permission is observed");
        turn.finish_prompt("cancelled");
        assert!(matches!(
            block_on(terminal).status(),
            TerminalStatus::ProviderRequestObserved(_)
        ));
    }
}
