use crate::support;

use support::{Direction, methods, parse_json, parse_transcript};

const ROOT: &str = "fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0";
const CORPUS: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/session-continuity-corpus.json");
const RELEASES: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/release-corpus.json");
const LOAD: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/load-success.ndjson");
const RESUME: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/resume-success.ndjson");
const WRONG_SESSION: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/load-wrong-session.ndjson");
const EARLY_RESPONSE: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/load-early-response.ndjson");
const RESUME_REPLAY: &str =
    include_str!("fixtures/acp-v1-claude-agent-acp-0.53.0-0.61.0/resume-with-replay.ndjson");

#[test]
fn every_qualified_release_is_frozen_for_load_and_resume() {
    let corpus = parse_json(CORPUS);
    let releases = parse_json(RELEASES);
    let qualified = corpus["qualified_range"]["qualified_versions"]
        .as_array()
        .expect("qualified versions");
    let observed: Vec<_> = releases["releases"]
        .as_array()
        .expect("release corpus")
        .iter()
        .filter_map(|release| {
            let version = release["version"].as_str().expect("version");
            (version != "0.52.0").then_some(version)
        })
        .collect();
    assert_eq!(
        qualified
            .iter()
            .map(|version| version.as_str().expect("qualified version"))
            .collect::<Vec<_>>(),
        observed
    );
    assert_eq!(
        corpus["qualified_range"]["excluded_unpublished_package"],
        "0.58.0"
    );
    assert_eq!(
        corpus["negotiation"]["load"],
        "agentCapabilities.loadSession"
    );
    assert_eq!(
        corpus["negotiation"]["resume"],
        "agentCapabilities.sessionCapabilities.resume"
    );
}

#[test]
fn load_replay_finishes_in_order_before_ready() {
    let frames = parse_transcript(LOAD).expect("load transcript");
    assert_eq!(frames[0].direction(), Direction::ClientToAgent);
    assert_eq!(frames[1].direction(), Direction::AgentToClient);
    assert_eq!(
        methods(&frames),
        [
            "session/load",
            "session/update",
            "session/update",
            "session/update"
        ]
    );
    let response = frames
        .iter()
        .position(|frame| frame.id() == Some(&serde_json::json!(20)) && frame.method().is_none())
        .expect("load response");
    let replay: Vec<_> = frames
        .iter()
        .enumerate()
        .filter(|(_, frame)| {
            matches!(
                frame.message()["params"]["update"]["sessionUpdate"].as_str(),
                Some("user_message_chunk" | "agent_message_chunk")
            )
        })
        .map(|(index, frame)| {
            assert!(index < response);
            frame.message()["params"]["update"]["sessionUpdate"]
                .as_str()
                .expect("replay kind")
        })
        .collect();
    assert_eq!(replay, ["user_message_chunk", "agent_message_chunk"]);
}

#[test]
fn resume_is_replay_free_and_negative_transcripts_are_rejected_by_shape() {
    let resume = parse_transcript(RESUME).expect("resume transcript");
    assert_eq!(methods(&resume), ["session/resume", "session/update"]);
    assert!(resume.iter().all(|frame| {
        !matches!(
            frame.message()["params"]["update"]["sessionUpdate"].as_str(),
            Some("user_message_chunk" | "agent_message_chunk")
        )
    }));

    let wrong = parse_transcript(WRONG_SESSION).expect("wrong-session transcript");
    assert_ne!(
        wrong[0].message()["params"]["sessionId"],
        wrong[1].message()["params"]["sessionId"]
    );
    let early = parse_transcript(EARLY_RESPONSE).expect("early-response transcript");
    assert!(early[1].id().is_some());
    assert_eq!(early[2].method(), Some("session/update"));
    let replay = parse_transcript(RESUME_REPLAY).expect("resume-replay transcript");
    assert_eq!(replay[0].method(), Some("session/resume"));
    assert_eq!(
        replay[1].message()["params"]["update"]["sessionUpdate"],
        "user_message_chunk"
    );
}

#[test]
fn continuity_cleanup_retains_history_and_has_no_detached_work() {
    let corpus = parse_json(CORPUS);
    let cleanup = &corpus["failure_and_cleanup"];
    assert_eq!(cleanup["close_persistent_history"], "preserved");
    assert_eq!(cleanup["disconnect"], "fail_without_usable_handle");
    assert_eq!(cleanup["credential_release"], "awaited_last");
    assert_eq!(cleanup["detached_tasks_allowed"], false);
    assert_eq!(corpus["resume"]["replay_phase"], false);
}

#[test]
fn claude_continuity_fixtures_are_bounded_and_safe() {
    for fixture in [
        CORPUS,
        RELEASES,
        LOAD,
        RESUME,
        WRONG_SESSION,
        EARLY_RESPONSE,
        RESUME_REPLAY,
    ] {
        assert!(fixture.len() < 256 * 1024);
        for forbidden in [
            "/Users/",
            "ANTHROPIC_API_KEY",
            "sk-ant-",
            "Bearer ",
            "Toms-MacBook-Pro",
        ] {
            assert!(!fixture.contains(forbidden), "{ROOT} leaked {forbidden}");
        }
    }
}
