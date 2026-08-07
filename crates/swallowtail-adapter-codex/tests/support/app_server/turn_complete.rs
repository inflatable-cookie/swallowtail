impl ScriptedAppServerHandle {
    fn complete_turn(&self, status: &str) {
        let thread_id = self
            .state
            .active_thread
            .lock()
            .expect("active thread lock is available")
            .clone()
            .expect("a turn is active");
        if status == "completed" {
            self.state.push(serde_json::json!({
                "method": "item/completed",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "completedAtMs": 1,
                    "item": {"id": "item-empty", "type": "agentMessage", "text": ""}
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/agentMessage/delta",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "itemId": "item-1",
                    "delta": "final "
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/agentMessage/delta",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "itemId": "item-1",
                    "delta": " "
                }
            }));
            self.state.push(serde_json::json!({
                "method": "item/completed",
                "params": {
                    "threadId": thread_id,
                    "turnId": "turn-provider-1",
                    "completedAtMs": 1,
                    "item": {"id": "item-1", "type": "agentMessage", "text": "final answer"}
                }
            }));
        }
        self.state.push(serde_json::json!({
            "method": "turn/completed",
            "params": {
                "threadId": thread_id,
                "turn": {"id": "turn-provider-1", "items": [], "status": status}
            }
        }));
    }
}

fn message_requires_experimental_api(message: &serde_json::Value) -> bool {
    const EXPERIMENTAL_FIELDS: &[&str] = &[
        "allowProviderModelFallback",
        "collaborationMode",
        "dynamicTools",
        "runtimeWorkspaceRoots",
    ];
    message
        .get("params")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|params| {
            EXPERIMENTAL_FIELDS
                .iter()
                .any(|field| params.contains_key(*field))
        })
}
