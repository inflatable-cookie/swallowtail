mod support;

#[path = "kimi_protocol_fixtures/authority.rs"]
mod authority;
#[path = "kimi_protocol_fixtures/lifecycle.rs"]
mod lifecycle;

use serde_json::Value;
use support::Frame;

const ROOT: &str = "fixtures/acp-v1-kimi-code-0.28.1";
const PROTOCOL: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/protocol.json");
const AUTHORITY: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/authority.json");
const INITIALIZE: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/initialize.ndjson");
const VERSION_DRIFT: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/version-drift.ndjson");
const CAPABILITY_DRIFT: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/capability-drift.ndjson");
const NEW_SESSION: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/new-session.ndjson");
const LOAD_SUCCESS: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/load-success.ndjson");
const LOAD_EARLY_RESPONSE: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/load-response-before-replay.ndjson");
const LOAD_WRONG_SESSION: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/load-wrong-session.ndjson");
const RESUME_SUCCESS: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/resume-success.ndjson");
const RESUME_WITH_REPLAY: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/resume-with-replay.ndjson");
const PROMPT: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/prompt-success.ndjson");
const CANCEL: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/cancel.ndjson");
const FILESYSTEM_WRITE: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/filesystem-write.ndjson");
const FILESYSTEM_WRITE_REJECTED: &str =
    include_str!("fixtures/acp-v1-kimi-code-0.28.1/filesystem-write-rejected.ndjson");
const AUTH_REQUIRED: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/auth-required.ndjson");
const DISCONNECT: &str = include_str!("fixtures/acp-v1-kimi-code-0.28.1/disconnect.ndjson");

const SESSION_ID: &str = "kimi-session-bound";

fn response_index(frames: &[Frame], id: Value) -> Option<usize> {
    frames
        .iter()
        .position(|frame| frame.method().is_none() && frame.id() == Some(&id))
}

fn historical_updates(frames: &[Frame]) -> Vec<(usize, &Value)> {
    frames
        .iter()
        .enumerate()
        .filter_map(|(index, frame)| {
            let message = frame.message();
            let kind = message["params"]["update"]["sessionUpdate"].as_str()?;
            matches!(
                kind,
                "user_message_chunk"
                    | "agent_message_chunk"
                    | "agent_thought_chunk"
                    | "tool_call"
                    | "tool_call_update"
                    | "plan"
            )
            .then_some((index, message))
        })
        .collect()
}
