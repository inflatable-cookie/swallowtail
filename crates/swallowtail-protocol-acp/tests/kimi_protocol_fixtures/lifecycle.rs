use super::{
    CANCEL, LOAD_EARLY_RESPONSE, LOAD_SUCCESS, LOAD_WRONG_SESSION, PROMPT, RESUME_SUCCESS,
    RESUME_WITH_REPLAY, SESSION_ID, historical_updates, response_index,
};
use crate::support::{methods, parse_transcript};
use serde_json::json;

#[test]
fn load_replays_history_in_order_before_its_response() {
    let frames = parse_transcript(LOAD_SUCCESS).expect("load transcript parses");
    assert_eq!(frames[0].method(), Some("session/load"));
    let response_index = response_index(&frames, json!(2)).expect("load response exists");
    let replay = historical_updates(&frames);
    assert_eq!(replay.len(), 2);
    assert!(replay.iter().all(|(index, _)| *index < response_index));
    assert_eq!(
        replay[0].1["params"]["update"]["sessionUpdate"],
        "user_message_chunk"
    );
    assert_eq!(
        replay[1].1["params"]["update"]["sessionUpdate"],
        "agent_message_chunk"
    );
    assert!(
        replay
            .iter()
            .all(|(_, message)| message["params"]["sessionId"] == SESSION_ID)
    );
    assert_eq!(
        frames.last().expect("command update exists").message()["params"]["update"]["sessionUpdate"],
        "available_commands_update"
    );
}

#[test]
fn load_rejects_early_response_or_wrong_session_replay() {
    let early = parse_transcript(LOAD_EARLY_RESPONSE).expect("early response parses");
    let response = response_index(&early, json!(2)).expect("early response exists");
    assert!(
        historical_updates(&early)
            .iter()
            .any(|(index, _)| *index > response)
    );

    let wrong = parse_transcript(LOAD_WRONG_SESSION).expect("wrong-session replay parses");
    assert_eq!(response_index(&wrong, json!(2)), None);
    assert!(
        historical_updates(&wrong)
            .iter()
            .any(|(_, message)| { message["params"]["sessionId"] != SESSION_ID })
    );
}

#[test]
fn resume_returns_without_history_replay_and_rejects_replayed_history() {
    let frames = parse_transcript(RESUME_SUCCESS).expect("resume transcript parses");
    assert_eq!(frames[0].method(), Some("session/resume"));
    assert!(historical_updates(&frames).is_empty());
    assert!(response_index(&frames, json!(3)).is_some());

    let invalid = parse_transcript(RESUME_WITH_REPLAY).expect("invalid resume parses");
    assert_eq!(historical_updates(&invalid).len(), 1);
    assert!(response_index(&invalid, json!(3)).is_some());
}

#[test]
fn prompt_and_native_turn_cancel_remain_separate_terminal_flows() {
    let prompt = parse_transcript(PROMPT).expect("prompt transcript parses");
    assert_eq!(
        methods(&prompt),
        ["session/prompt", "session/update", "session/update"]
    );
    let output = prompt
        .iter()
        .filter(|frame| frame.method() == Some("session/update"))
        .filter_map(|frame| frame.message()["params"]["update"]["content"]["text"].as_str())
        .collect::<String>();
    assert_eq!(output, "Kimi fixture response.");
    assert_eq!(
        prompt.last().expect("prompt response").message()["result"]["stopReason"],
        "end_turn"
    );

    let cancel = parse_transcript(CANCEL).expect("cancel transcript parses");
    assert_eq!(
        methods(&cancel),
        ["session/prompt", "session/update", "session/cancel"]
    );
    assert_eq!(
        cancel.last().expect("cancel response").message()["result"]["stopReason"],
        "cancelled"
    );
}
