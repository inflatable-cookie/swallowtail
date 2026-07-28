use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_anthropic::{
    AnthropicInferenceAttemptInput, AnthropicModelSelection, AnthropicSessionProfileInput,
    AnthropicWebSearchInput,
};
use swallowtail_core::{Capability, ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRole, CleanupOutcome, Deadline, DirectContinuationTurnRequest,
    DirectToolResult, DirectToolResultContent, MonotonicInstant, OperationContent, RequestId,
    RuntimeTurnId, SchemaDocument, TerminalStatus, ToolDeclaration,
};

#[test]
fn prepared_image_and_search_runs_bind_exact_wire_and_cleanup() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.inputs").unwrap());
    let prepared = fixture.prepared();
    let image =
        AttachmentDescriptor::new(fixture.attachment_ref(), "image/png", AttachmentRole::Input)
            .unwrap()
            .with_known_length(8);
    let image_run = prepared
        .prepare_inference_attempt(
            fixture
                .attempt_input("anthropic-image")
                .with_attachments([image]),
        )
        .expect("image input prepares");
    assert!(has_capability(image_run.plan(), Capability::Attachments));
    let _ = complete(image_run.start_run(fixture.services()));
    assert_eq!(fixture.attachment_releases(), 1);
    let image_body: serde_json::Value =
        serde_json::from_slice(&fixture.server.requests()[0].body).unwrap();
    assert_eq!(
        image_body["messages"][0]["content"][0]["source"]["data"],
        "iVBORw0KGgo="
    );

    let search_fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.search").unwrap(),
        StreamFixture::WebSearch,
    );
    let search_prepared = search_fixture.prepared();
    let search_run = search_prepared
        .prepare_inference_attempt(
            AnthropicInferenceAttemptInput::new(
                RequestId::new("anthropic-search").unwrap(),
                model("claude-fixture-search-capable"),
                OperationContent::new("Search for the bounded fixture fact.").unwrap(),
                NonZeroU64::new(64).unwrap(),
            )
            .with_web_search(AnthropicWebSearchInput::new(["example.com"])),
        )
        .expect("search input prepares");
    assert!(has_capability(
        search_run.plan(),
        Capability::ExternalSearch
    ));
    assert!(has_capability(
        search_run.plan(),
        Capability::ProviderExternalNetwork
    ));
    let search_events = complete(search_run.start_run(search_fixture.services()));
    assert!(search_events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ExternalSearchProgress
    )));
    let search_body: serde_json::Value =
        serde_json::from_slice(&search_fixture.server.requests()[0].body).unwrap();
    assert_eq!(search_body["tools"][0]["type"], "web_search_20250305");
    assert_eq!(search_body["tools"][0]["max_uses"], 2);
    assert_eq!(search_body["tools"][0]["allowed_domains"][0], "example.com");
}

#[test]
fn prepared_direct_session_waits_for_exact_consumer_result_and_allows_later_turn() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.continuation").unwrap(),
        StreamFixture::ToolContinuation,
    );
    let prepared = fixture.prepared();
    let schema = SchemaDocument::inline(
        br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
        4096,
    )
    .unwrap();
    let tool = ToolDeclaration::new(
        "lookup_customer",
        schema,
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap()
    .with_description(
        OperationContent::new("Return the bounded fixture customer record.").unwrap(),
    );
    let session = prepared
        .prepare_session(AnthropicSessionProfileInput::new(
            RequestId::new("anthropic-session").unwrap(),
            model("claude-fixture-primary"),
            [tool],
        ))
        .expect("direct session prepares");
    assert!(has_capability(
        session.plan(),
        Capability::DirectToolContinuation
    ));
    let mut session = block_on(session.open_session(fixture.services())).expect("session opens");
    let mut turn = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("anthropic-turn-1").unwrap(),
            OperationContent::new("Look up the approved fixture customer.").unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(100_000)),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    let mut exchange = turn
        .take_direct_tool_exchange()
        .expect("consumer tool exchange exists");
    let mut calls = exchange.take_calls().expect("tool call stream exists");
    let mut events = turn.take_events().expect("event stream exists");
    let terminal = turn.take_terminal_outcome().expect("terminal exists");
    let submitter = exchange.submitter();
    let outcome = block_on(async {
        let call = match calls.next().await {
            Some(Ok(call)) => call,
            other => {
                let mut observed = Vec::new();
                while let Some(event) = events.next().await {
                    observed.push(event);
                }
                let outcome = terminal.await;
                panic!("tool call missing: {other:?}; events={observed:?}; outcome={outcome:?}");
            }
        };
        assert_eq!(call.call_id().as_str(), "toolu_fixture_1");
        assert_eq!(call.tool_name(), "lookup_customer");
        submitter
            .submit(vec![DirectToolResult::new(
                call.call_id().clone(),
                DirectToolResultContent::new(b"approved fixture result".to_vec(), 65_536).unwrap(),
            )])
            .await
            .expect("exact tool result continues");
        assert!(submitter.submit(Vec::new()).await.is_err());
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(fixture.server.inference_attempts(), 2);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);

    let mut later = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("anthropic-turn-2").unwrap(),
            OperationContent::new("Summarize the approved fixture result.").unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(100_000)),
        ),
        fixture.services(),
    ))
    .expect("later turn starts");
    assert!(later.take_direct_tool_exchange().is_none());
    let mut events = later.take_events().unwrap();
    let terminal = later.take_terminal_outcome().unwrap();
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.unwrap();
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(fixture.server.inference_attempts(), 3);
    assert_eq!(block_on(later.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);

    let requests = fixture.server.requests();
    let continuation: serde_json::Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(
        continuation["messages"][2]["content"][0]["tool_use_id"],
        "toolu_fixture_1"
    );
    let later: serde_json::Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(later["messages"].as_array().unwrap().len(), 5);
}

fn model(id: &str) -> AnthropicModelSelection {
    AnthropicModelSelection::new(
        ModelRouteId::new(format!("anthropic.{id}")).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new(id).unwrap(),
    )
}

fn has_capability(plan: &swallowtail_core::PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
}

fn complete(
    future: swallowtail_runtime::BoxFuture<
        'static,
        Result<Box<dyn swallowtail_runtime::RunHandle>, swallowtail_runtime::RuntimeFailure>,
    >,
) -> Vec<swallowtail_runtime::RuntimeEvent> {
    let mut run = block_on(future).expect("prepared run starts");
    let mut events = run.take_events().unwrap();
    let terminal = run.take_terminal_outcome().unwrap();
    let (events, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.unwrap());
        }
        (observed, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    events
}
