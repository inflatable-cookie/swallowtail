use serde_json::{Value, json};
use swallowtail_adapter_alibaba_model_studio::{
    ConversationRef, MAXIMUM_REPLAY_BYTES, MAXIMUM_REPLAY_ITEMS, MAXIMUM_REPLAY_PAGE_BYTES,
    MAXIMUM_REPLAY_PAGE_ITEMS, MAXIMUM_REPLAY_PAGES, Method, WireRequest,
    parse_conversation_retrieval, parse_replay_page,
};
use swallowtail_runtime::SessionReplayKind;

mod support;

use support::{bytes, conversation, json_fixture};

#[test]
fn retained_manifest_retrieval_and_ordered_pagination_are_exact_and_bounded() {
    let protocol: Value = json_fixture("protocol.json");
    assert_eq!(protocol["retained_recovery_evidence_date"], "2026-08-05");
    assert_eq!(
        protocol["retained_replay_maximum_page_items"],
        MAXIMUM_REPLAY_PAGE_ITEMS
    );
    assert_eq!(
        protocol["retained_replay_maximum_pages"],
        MAXIMUM_REPLAY_PAGES
    );
    assert_eq!(
        protocol["retained_replay_maximum_items"],
        MAXIMUM_REPLAY_ITEMS
    );
    assert_eq!(
        protocol["retained_replay_maximum_page_bytes"],
        MAXIMUM_REPLAY_PAGE_BYTES
    );
    assert_eq!(
        protocol["retained_replay_maximum_bytes"],
        MAXIMUM_REPLAY_BYTES
    );

    let conversation = conversation();
    let retrieve = WireRequest::retrieve_conversation(&conversation);
    assert_eq!(retrieve.method(), Method::Get);
    assert_eq!(
        retrieve.path(),
        "/compatible-mode/v1/conversations/conv_fixture_01"
    );
    let metadata =
        parse_conversation_retrieval(bytes("conversation-retrieved.json"), &conversation)
            .expect("exact retained conversation is retrieved");
    assert_eq!(metadata.conversation(), &conversation);
    assert_eq!(metadata.created_at(), 1_784_700_000_123);
    let foreign = ConversationRef::new("conv_foreign").expect("foreign fixture id is valid");
    assert!(parse_conversation_retrieval(bytes("conversation-retrieved.json"), &foreign).is_err());

    let first = parse_replay_page(bytes("items-page-1.json"), &conversation, 0)
        .expect("first replay page is valid");
    assert_eq!(first.replay().len(), 2);
    assert!(first.next_after().is_some());
    assert_eq!(
        first
            .replay()
            .map(|item| item.sequence())
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(
        first.replay().map(|item| item.kind()).collect::<Vec<_>>(),
        vec![
            SessionReplayKind::UserMessage,
            SessionReplayKind::AgentMessage
        ]
    );
    assert_eq!(
        first
            .replay()
            .map(|item| item.content().expect("message content").as_str())
            .collect::<Vec<_>>(),
        vec!["First fixture input", "Hello world."]
    );

    let next = WireRequest::list_items_after(&conversation, first.next_after());
    assert_eq!(next.method(), Method::Get);
    assert_eq!(
        next.path(),
        "/compatible-mode/v1/conversations/conv_fixture_01/items?limit=100&order=asc&after=msg_output_01"
    );
    let second = parse_replay_page(bytes("items-page-2.json"), &conversation, 2)
        .expect("final replay page is valid");
    assert!(second.next_after().is_none());
    assert_eq!(
        second
            .replay()
            .map(|item| item.sequence())
            .collect::<Vec<_>>(),
        vec![2, 3]
    );
    assert!(first.content_bytes() + second.content_bytes() < MAXIMUM_REPLAY_BYTES);

    let malformed = json!({
        "object": "list",
        "data": [{
            "id": "msg_bad",
            "type": "message",
            "status": "completed",
            "role": "tool",
            "content": [{"type": "output_text", "text": "not replayable"}]
        }],
        "first_id": "msg_bad",
        "last_id": "msg_bad",
        "has_more": false
    });
    assert!(
        parse_replay_page(
            &serde_json::to_vec(&malformed).expect("JSON"),
            &conversation,
            0
        )
        .is_err()
    );
    assert!(
        parse_replay_page(&vec![b' '; MAXIMUM_REPLAY_PAGE_BYTES + 1], &conversation, 0).is_err()
    );
}

#[test]
fn retained_recovery_failure_corpus_keeps_every_non_ready_disposition() {
    let cases: Value = json_fixture("retained-recovery-cases.json");
    assert_eq!(cases["evidence_date"], "2026-08-05");
    let cases = cases["cases"].as_array().expect("cases are an array");
    assert_eq!(cases.len(), 8);
    for name in [
        "exact",
        "foreign",
        "missing",
        "deleted",
        "malformed",
        "oversized",
        "stale",
        "uncertain",
    ] {
        assert!(cases.iter().any(|case| case["case"] == name));
    }
    assert!(
        cases
            .iter()
            .filter(|case| case["case"] != "exact")
            .all(|case| {
                case["disposition"]
                    .as_str()
                    .is_some_and(|disposition| disposition.contains("without"))
            })
    );
}
