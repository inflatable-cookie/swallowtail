use super::*;
use swallowtail_runtime::{ActivityKind, ProcessExit, RuntimeRunId};

const META: &str = include_str!("../../tests/fixtures/muse-code-0.1.0-R708.1/meta-success.jsonl");
const ECHO: &str = include_str!("../../tests/fixtures/muse-code-0.1.0-R708.1/echo-success.jsonl");
const UNKNOWN: &str =
    include_str!("../../tests/fixtures/muse-code-0.1.0-R708.1/unknown-event.jsonl");

fn fixture_parser() -> MuseEventParser {
    MuseEventParser::new(
        ActivityOperationId::Run(RuntimeRunId::new("muse-fixture-run").unwrap()),
        ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
    )
}

#[test]
fn exact_meta_projection_completes_with_correlated_output() {
    let mut parser = fixture_parser();
    let mut events = Vec::new();
    for chunk in META.as_bytes().chunks(37) {
        events.extend(parser.push(chunk).expect("chunk parses"));
    }
    let (trailing, terminal) = parser.finish().expect("stream finishes");
    events.extend(trailing);
    let outcome = terminal.outcome(ProcessExit::new(true, Some(0)));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "MUSE_FIXTURE_OK"
    );
    assert!(
        events
            .iter()
            .any(|event| { matches!(event.kind(), RuntimeEventKind::OutputDelta) })
    );
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(observation)
                if matches!(observation.kind(), ActivityKind::Unknown(namespace)
                    if namespace.as_str()
                        == "muse-code.headless.event.session.workspace_branch.observed")
        )
    }));
}

#[test]
fn bounded_unknown_is_namespaced_without_terminal_authority() {
    let mut parser = fixture_parser();
    let events = parser.push(UNKNOWN.as_bytes()).expect("unknown parses");
    let (_, terminal) = parser.finish().expect("stream finishes");
    assert_eq!(
        terminal.outcome(ProcessExit::new(true, Some(0))).status(),
        &TerminalStatus::Completed
    );
    let unknown = events.iter().find_map(|event| match event.kind() {
        RuntimeEventKind::Activity(observation)
            if matches!(observation.kind(), ActivityKind::Unknown(_)) =>
        {
            Some(observation)
        }
        _ => None,
    });
    assert!(unknown.is_some());
}

#[test]
fn exact_task_lifecycle_projects_without_claiming_a_task_list() {
    let mut parser = fixture_parser();
    let events = parser.push(ECHO.as_bytes()).expect("echo lifecycle parses");
    let tasks = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(observation)
                if observation.kind() == &ActivityKind::Task =>
            {
                Some(observation)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(tasks.len() >= 9);
    assert!(tasks.iter().all(|task| task.task_list().is_none()));
    assert!(
        tasks
            .iter()
            .all(|task| task.provider_activity_ref().is_some())
    );
}

#[test]
fn reordered_cross_session_post_terminal_and_model_drift_fail_closed() {
    let lines = META.lines().collect::<Vec<_>>();
    let cases = [
        lines
            .iter()
            .enumerate()
            .map(|(index, line)| {
                if index == 1 {
                    line.replace("\"sequence\":2", "\"sequence\":9")
                } else {
                    (*line).to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n",
        META.replacen("fixture-session-meta", "fixture-session-foreign", 1),
        format!("{META}{}\n", lines[5]),
        META.replace("muse-spark-1.2", "muse-spark-foreign"),
    ];
    for case in cases {
        let mut parser = fixture_parser();
        assert!(parser.push(case.as_bytes()).is_err());
    }
}

#[test]
fn record_stream_and_unknown_payload_bounds_fail_closed() {
    let mut parser = fixture_parser();
    assert!(parser.push(&vec![b'x'; MAXIMUM_RECORD_BYTES + 1]).is_err());

    let mut parser = fixture_parser();
    assert!(parser.push(&vec![b'x'; MAXIMUM_STREAM_BYTES + 1]).is_err());

    let oversized = UNKNOWN.replace(
        "bounded fixture notice",
        &"x".repeat(MAXIMUM_UNKNOWN_PAYLOAD_BYTES),
    );
    let mut parser = fixture_parser();
    assert!(parser.push(oversized.as_bytes()).is_err());
}
