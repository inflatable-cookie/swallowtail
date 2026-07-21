use super::*;
use serde_json::Value;
use swallowtail_runtime::{
    OperationContent, OwnedRemoteResourceKind, RemoteResourceDeletionOutcome, SchemaDocument,
    ToolDeclaration,
};

const ROOT: &str = "../../tests/fixtures/managed-agents-2026-04-01";
const AGENT: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/agent.json"
));
const ENVIRONMENT_CREATE: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/environment-create.json"
));
const ENVIRONMENT: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/environment.json"
));
const SESSION_CREATE: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/session-create.json"
));
const SESSION: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/session.json"
));
const USER_MESSAGE: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/user-message.json"
));
const CUSTOM_TOOL_RESULT: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/custom-tool-result.json"
));
const INTERRUPT: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/interrupt.json"
));
const SUCCESS: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/success.sse"
));
const REQUIRES_ACTION: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/requires-action.sse"
));
const RESCHEDULING: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/rescheduling.sse"
));
const DISCONNECT: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/disconnect.sse"
));
const HISTORY: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/history.json"
));
const PROVIDER_FAILURE: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/provider-failure.sse"
));
const PREVIEW: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/preview.sse"
));
const SCHEMA_DRIFT: &str = include_str!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/schema-drift.sse"
));
const DELETE_SESSION: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/delete-session.json"
));
const DELETE_ENVIRONMENT: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/delete-environment.json"
));
const DELETION_FAILURE: &[u8] = include_bytes!(concat!(
    "../../tests/fixtures/managed-agents-2026-04-01/deletion-failure.json"
));

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_fixture",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"key":{"type":"string"}},"required":["key"]}"#
                .to_vec(),
            1024,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool is valid")
    .with_description(
        OperationContent::new("Return one deterministic fixture value.").expect("valid content"),
    )
}

fn body(request: &Request) -> Value {
    serde_json::from_slice(request.body.as_ref().expect("request has body"))
        .expect("request body is JSON")
}

fn fixture_value(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).expect("fixture is JSON")
}

#[test]
fn corpus_is_present_and_freezes_exact_headers_and_subset() {
    assert!(std::path::Path::new(ROOT).is_relative());
    assert_eq!(API_VERSION, "2023-06-01");
    assert_eq!(BETA_VERSION, "managed-agents-2026-04-01");
    assert_eq!(ENDPOINT_AUDIENCE, "api.anthropic.com");

    let manifest: Value = serde_json::from_str(include_str!(concat!(
        "../../tests/fixtures/managed-agents-2026-04-01/protocol.json"
    )))
    .expect("manifest is JSON");
    assert_eq!(manifest["preview_events_accepted"], false);
    assert_eq!(manifest["history_limit"], 1000);
    assert_eq!(manifest["maximum_reconciliations"], 1);
    assert_eq!(
        manifest["environment"]["networking"]["allowed_hosts"],
        serde_json::json!([])
    );
}

#[test]
fn requests_match_the_frozen_corpus_and_keep_identities_distinct() {
    let agent = Request::agent("agent_fixture", 7);
    assert_eq!(agent.method, Method::Get);
    assert_eq!(agent.path, "/v1/agents/agent_fixture");
    assert_eq!(agent.query, [("version".to_owned(), "7".to_owned())]);

    let environment = Request::create_environment("swallowtail-managed-run");
    assert_eq!(environment.method, Method::Post);
    assert_eq!(body(&environment), fixture_value(ENVIRONMENT_CREATE));

    let tool = fixture_tool();
    let session = Request::create_session(
        "agent_fixture",
        7,
        "claude-fixture-model",
        "env_fixture",
        &[&tool],
    )
    .expect("session request is valid");
    assert_eq!(session.path, "/v1/sessions");
    assert_eq!(body(&session), fixture_value(SESSION_CREATE));

    let message = OperationContent::new("Return the fixture summary.").expect("valid content");
    assert_eq!(
        body(&Request::message("session_fixture", &message)),
        fixture_value(USER_MESSAGE)
    );
    let result = OperationContent::new("fixture-value").expect("valid content");
    assert_eq!(
        body(&Request::custom_tool_result(
            "session_fixture",
            "event_tool",
            &result,
        )),
        fixture_value(CUSTOM_TOOL_RESULT)
    );
    assert_eq!(
        body(&Request::interrupt("session_fixture")),
        fixture_value(INTERRUPT)
    );

    assert_eq!(
        Request::session("session_fixture").path,
        "/v1/sessions/session_fixture"
    );
    assert_eq!(
        Request::stream("session_fixture").path,
        "/v1/sessions/session_fixture/events/stream"
    );
    assert_eq!(Request::history("session_fixture").query.len(), 2);
    assert_eq!(
        Request::delete_session("session_fixture").method,
        Method::Delete
    );
    assert_eq!(
        Request::delete_environment("env_fixture").method,
        Method::Delete
    );
}

#[test]
fn response_bindings_and_deletion_truth_are_exact_and_separate() {
    validate_agent(AGENT, "agent_fixture", 7, "claude-fixture-model").expect("agent is exact");
    validate_environment(ENVIRONMENT, "env_fixture").expect("environment is exact");
    validate_session(
        SESSION,
        "session_fixture",
        "env_fixture",
        "agent_fixture",
        7,
        "claude-fixture-model",
    )
    .expect("session is exact");

    assert_eq!(
        parse_deletion(
            DELETE_SESSION,
            "session_fixture",
            OwnedRemoteResourceKind::Session,
        )
        .expect("session deletion is confirmed"),
        RemoteResourceDeletionOutcome::Confirmed
    );
    assert_eq!(
        parse_deletion(
            DELETE_ENVIRONMENT,
            "env_fixture",
            OwnedRemoteResourceKind::Environment,
        )
        .expect("environment deletion is confirmed"),
        RemoteResourceDeletionOutcome::Confirmed
    );
    assert!(
        parse_deletion(
            DELETE_SESSION,
            "session_fixture",
            OwnedRemoteResourceKind::Environment,
        )
        .is_err()
    );

    let failure = parse_deletion(
        DELETION_FAILURE,
        "env_fixture",
        OwnedRemoteResourceKind::Environment,
    )
    .expect_err("provider failure is not confirmation");
    assert!(!format!("{failure:?}").contains("fixture-secret-never-log"));
}

#[test]
fn custom_tool_translation_rejects_implicit_or_provider_specific_shapes() {
    let schema =
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 128).expect("schema is bounded");
    let undescribed = ToolDeclaration::new(
        "tool",
        schema,
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("declaration is structurally valid");
    assert!(Request::create_session("agent_fixture", 7, "model", "env", &[&undescribed]).is_err());
}

#[path = "tests/event_tests.rs"]
mod event_tests;
