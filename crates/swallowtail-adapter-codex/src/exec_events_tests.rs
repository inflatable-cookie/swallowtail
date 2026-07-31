use super::ExecEventParser;
use semver::Version;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityKind, ProviderObservation, RuntimeEventKind, RuntimeRunId, TerminalStatus, TokenUsage,
};

const ACTIVITY_CORPUS: &str = include_str!("../tests/fixtures/activity/exec.jsonl");

fn parser() -> ExecEventParser {
    ExecEventParser::new(
        RuntimeRunId::new("exec-parser-test").unwrap(),
        Version::new(0, 145, 0),
    )
}

#[test]
fn parser_handles_split_jsonl_and_preserves_final_output() {
    let mut parser = parser();
    assert!(
        parser
            .push(br#"{"type":"turn.started"}"#)
            .expect("partial line is buffered")
            .is_empty()
    );
    let events = parser
        .push(
            b"\n{\"type\":\"item.completed\",\"item\":{\"id\":\"message-1\",\"type\":\"agent_message\",\"text\":\"done\"}}\n{\"type\":\"turn.completed\"}\n",
        )
        .expect("valid JSONL is parsed");

    assert_eq!(events.len(), 3);
    assert!(matches!(events[1].kind(), RuntimeEventKind::Activity(_)));
    assert_eq!(events[2].kind(), &RuntimeEventKind::Progress);
    assert_eq!(
        events[2].content().map(|value| value.as_str()),
        Some("done")
    );
    let (trailing, terminal) = parser.finish().expect("stream finishes");
    assert!(trailing.is_empty());
    let outcome = terminal.outcome(true);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.output().map(|value| value.as_str()), Some("done"));
}

#[test]
fn parser_preserves_safe_search_reasoning_and_usage_progress() {
    let mut parser = parser();
    let events = parser
        .push(concat!(
            "{\"type\":\"item.completed\",\"item\":{\"id\":\"search-1\",\"type\":\"web_search\",\"query\":\"official manual\"}}\n",
            "{\"type\":\"item.completed\",\"item\":{\"id\":\"reasoning-1\",\"type\":\"reasoning\",\"summary\":\"Checking evidence\"}}\n",
            "{\"type\":\"turn.completed\",\"usage\":{\"input_tokens\":12,\"output_tokens\":4}}\n"
        ).as_bytes())
        .expect("progress JSONL is parsed");

    assert!(matches!(events[0].kind(), RuntimeEventKind::Activity(_)));
    assert_eq!(events[1].kind(), &RuntimeEventKind::ExternalSearchProgress);
    assert_eq!(
        events[1].content().map(|value| value.as_str()),
        Some("official manual")
    );
    assert!(matches!(events[2].kind(), RuntimeEventKind::Activity(_)));
    assert_eq!(events[3].kind(), &RuntimeEventKind::ReasoningProgress);
    assert_eq!(
        events[3].content().map(|value| value.as_str()),
        Some("Checking evidence")
    );
    let RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)) = events[4].kind()
    else {
        panic!("usage remains a typed provider observation");
    };
    assert_eq!(usage, &TokenUsage::new(Some(12), Some(4)));
    assert!(events[2].content().is_none());
}

#[test]
fn queryless_other_search_does_not_abort_later_structured_output() {
    let case: Value = ACTIVITY_CORPUS
        .lines()
        .map(|line| serde_json::from_str(line).expect("activity corpus line is valid"))
        .find(|case: &Value| case["case"] == "search-other-navigation-lifecycle")
        .expect("queryless navigation case exists");
    let mut jsonl = case["events"]
        .as_array()
        .expect("events are an array")
        .iter()
        .map(|event| serde_json::to_string(event).expect("fixture event serializes"))
        .collect::<Vec<_>>()
        .join("\n");
    jsonl.push('\n');

    let mut parser = parser();
    let events = parser
        .push(jsonl.as_bytes())
        .expect("queryless navigation and later proposal parse");
    let activities = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(activities.len(), 3);
    assert_eq!(activities[0].activity_id(), activities[1].activity_id());
    assert_eq!(
        activities[0].provider_activity_ref(),
        activities[1].provider_activity_ref()
    );
    assert!(activities[..2].iter().all(|item| item.content().is_none()));
    let output = events
        .iter()
        .find(|event| event.kind() == &RuntimeEventKind::OutputAvailable)
        .and_then(|event| event.content())
        .expect("valid structured proposal remains available");
    assert_eq!(output.as_str(), r#"{"proposal":"valid"}"#);

    let (trailing, terminal) = parser.finish().expect("stream finishes");
    assert!(trailing.is_empty());
    let outcome = terminal.outcome(true);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(|value| value.as_str()),
        Some(r#"{"proposal":"valid"}"#)
    );
}

#[test]
fn malformed_jsonl_is_safe_and_redacted() {
    let secret = "not-json-private-output";
    let failure = parser()
        .push(format!("{secret}\n").as_bytes())
        .expect_err("malformed output fails");

    assert!(!format!("{failure:?}").contains(secret));
}

#[test]
fn multi_release_core_corpus_preserves_selected_and_additive_events() {
    let mut parser = parser();
    let events = parser
        .push(include_bytes!(
            "../tests/fixtures/compatibility/exec-core.jsonl"
        ))
        .expect("frozen multi-release corpus parses");

    assert_eq!(events.len(), 10);
    assert_eq!(events[0].kind(), &RuntimeEventKind::Progress);
    assert_eq!(events[3].kind(), &RuntimeEventKind::ExternalSearchProgress);
    assert_eq!(events[5].kind(), &RuntimeEventKind::ReasoningProgress);
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(activity.kind(), ActivityKind::Unknown(_))
        )
    }));
    let (trailing, terminal) = parser.finish().expect("corpus stream finishes");
    assert!(trailing.is_empty());
    let outcome = terminal.outcome(true);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(|value| value.as_str()),
        Some("final answer")
    );
}
