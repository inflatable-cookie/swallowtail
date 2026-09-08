//! Card 146 corpus guards for the exact `0.3.259` MCP-status row shapes and
//! the frozen Card 145 Desktop diagnostic tuple.
//!
//! Nothing here executes the official SDK, opens a session, or contacts a
//! provider. The two fixtures are the bounded regression corpus: the row
//! corpus pins every declared status variant and every declared optional
//! metadata field against the safe projection contract, and the tuple corpus
//! freezes Research 297's Desktop identities so later work cannot drift from
//! the accepted capsule.

use serde_json::Value;

const MCP_STATUS_ROWS: &str =
    include_str!("fixtures/claude-agent-sdk-0.3.259/mcp-status-rows.json");
const CARD_145_TUPLE: &str =
    include_str!("fixtures/claude-agent-sdk-0.3.259/card-145-desktop-diagnostic-tuple.json");

fn row_corpus() -> Value {
    serde_json::from_str(MCP_STATUS_ROWS).expect("mcp-status-rows corpus is valid JSON")
}

fn tuple_corpus() -> Value {
    serde_json::from_str(CARD_145_TUPLE).expect("Card 145 tuple corpus is valid JSON")
}

/// The exact accepted identities from Research 297, the Card 145 packet, and
/// the operator-approved handoff. The fixture must keep these byte-for-byte.
#[test]
fn the_card_145_tuple_corpus_freezes_the_accepted_desktop_identities() {
    let corpus = tuple_corpus();
    for (field, expected) in [
        ("desktop_task_id", "9dc9b50f-5bb2-4912-be7d-a6159e8c758d"),
        ("merge", "117e09e02dafd505d9f1d6e2b1380b56bbb22a19"),
        ("closeout", "5d7f1681222222d6f1586176c05733c6f1daf7c3"),
        (
            "capsule_sha256",
            "81cb0fd6c5a717e9ca66b73c7a746e66870219ebf2c1330081dfd76549608569",
        ),
        (
            "swallowtail_source",
            "6a93f1d916945aa2b402df7329dc994570e005c8",
        ),
    ] {
        assert_eq!(
            corpus[field], expected,
            "{field} drifted from the accepted Card 145 identity"
        );
    }
    assert_eq!(corpus["desktop_pr"]["number"], 172);
    assert_eq!(
        corpus["desktop_pr"]["accepted_head"],
        "8ac46df322bec428e744adb5d22fba3a6ad9a7cc"
    );
    assert_eq!(corpus["desktop_pr"]["review_comment"], "5589754115");
}

/// The tuple corpus keeps the bounded outcome that permits no quota,
/// qualification, or release inference.
#[test]
fn the_card_145_tuple_corpus_keeps_the_bounded_outcome_truth() {
    let corpus = tuple_corpus();
    let outcome = &corpus["bounded_outcome"];
    assert_eq!(outcome["opens"], 1);
    assert_eq!(outcome["stage"], "sidecar_rejected");
    assert_eq!(outcome["subcode"], "mcp_status_invalid");
    assert_eq!(
        outcome["route_code"],
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    assert_eq!(outcome["provider_readiness_reached"], false);
    for counter in [
        "prompt_turns",
        "tool_dispatches",
        "permission_callbacks",
        "controls",
        "retries",
        "reconnects",
        "respawns",
    ] {
        assert_eq!(outcome[counter], 0, "{counter} must stay zero");
    }
    assert_eq!(outcome["cleanup"], "confirmed");
    let tuple = &corpus["tuple"];
    assert_eq!(tuple["sdk"], "@anthropic-ai/claude-agent-sdk@0.3.259");
    assert_eq!(
        tuple["sidecar_source_tag"],
        "swallowtail-claude-agent-sdk-sidecar@0.4.4"
    );
    assert_eq!(tuple["strict_mcp_config"], true);
}

/// The row corpus enumerates exactly the declared `0.3.259` status axis and
/// the five declared optional metadata fields, anchored to the frozen
/// declaration range.
#[test]
fn the_row_corpus_enumerates_the_declared_status_axis_and_optional_fields() {
    let corpus = row_corpus();
    assert_eq!(corpus["sdk"]["package"], "@anthropic-ai/claude-agent-sdk");
    assert_eq!(corpus["sdk"]["version"], "0.3.259");
    assert_eq!(
        corpus["sdk"]["mcp_server_status_declaration"],
        "package/sdk.d.ts:1114-1158"
    );
    let statuses: Vec<&str> = corpus["row_statuses"]
        .as_array()
        .expect("row statuses are an array")
        .iter()
        .map(|value| value.as_str().expect("status is text"))
        .collect();
    assert_eq!(
        statuses,
        vec!["connected", "pending", "failed", "needs-auth", "disabled"]
    );
    let declared: Vec<&str> = corpus["declared_row_fields"]
        .as_array()
        .expect("declared row fields are an array")
        .iter()
        .map(|value| value.as_str().expect("field is text"))
        .collect();
    for field in [
        "name",
        "status",
        "serverInfo",
        "error",
        "config",
        "scope",
        "tools",
    ] {
        assert!(
            declared.contains(&field),
            "{field} must stay in the declared row-field set"
        );
    }
    assert_eq!(
        declared.len(),
        7,
        "no field beyond the declared seven may be admitted"
    );
}

/// Every status variant has its exact bounded evidence mapping, including the
/// required-server admission rule that Card 145's contradiction violated.
#[test]
fn the_row_corpus_maps_each_status_to_its_bounded_evidence() {
    let corpus = row_corpus();
    let evidence = &corpus["evidence_by_status"];
    assert_eq!(
        evidence["connected"]["evidence"],
        serde_json::json!({"name": "fixture", "status": "connected"})
    );
    assert_eq!(evidence["connected"]["required_server"], "open admitted");
    assert_eq!(
        evidence["pending"]["evidence"],
        serde_json::json!({"name": "fixture", "status": "pending"})
    );
    assert_eq!(
        evidence["pending"]["required_server"],
        "bounded failure mcp_server_failed"
    );
    assert_eq!(
        evidence["failed"]["evidence"],
        serde_json::json!({
            "name": "fixture",
            "status": "failed",
            "failureCode": "mcp_server_failed"
        })
    );
    assert_eq!(
        evidence["needs-auth"]["evidence"],
        serde_json::json!({
            "name": "fixture",
            "status": "failed",
            "failureCode": "mcp_server_needs_auth"
        })
    );
    assert_eq!(
        evidence["needs-auth"]["required_server"],
        "bounded failure mcp_server_needs_auth"
    );
    assert_eq!(
        evidence["disabled"]["evidence"],
        serde_json::json!({
            "name": "fixture",
            "status": "failed",
            "failureCode": "mcp_server_failed"
        })
    );
    assert_eq!(
        corpus["projection"]["retained"],
        serde_json::json!([
            "name",
            "canonical status kind",
            "failureCode on failed rows only"
        ])
    );
    assert_eq!(
        corpus["projection"]["discarded"],
        serde_json::json!(["serverInfo", "error", "config", "scope", "tools"])
    );
}

/// The faithful fixture row carries every declared optional field; the
/// rejected shapes each violate exactly one declared constraint, so the
/// corpus cannot silently narrow the fail-closed set.
#[test]
fn the_row_corpus_keeps_accepted_and_rejected_shapes_honest() {
    let corpus = row_corpus();
    let accepted = &corpus["accepted_row"];
    let declared: Vec<String> = corpus["declared_row_fields"]
        .as_array()
        .expect("declared row fields are an array")
        .iter()
        .map(|value| value.as_str().expect("field is text").to_owned())
        .collect();
    for field in &declared {
        assert!(
            accepted.get(field).is_some(),
            "the faithful fixture row must carry every declared field: {field}"
        );
    }
    let text = accepted.to_string();
    assert!(
        text.contains("fixture-only"),
        "fixture marker values must be present so the non-leak oracle can detect projection"
    );
    let configured_name = accepted["name"]
        .as_str()
        .expect("accepted row names a server");
    for entry in corpus["rejected_row_shapes"]
        .as_array()
        .expect("rejected shapes are an array")
    {
        let row = &entry["row"];
        let shape = entry["shape"].as_str().expect("shape is text");
        let is_object = row.is_object();
        let keys: Vec<String> = row
            .as_object()
            .map(|object| object.keys().cloned().collect())
            .unwrap_or_default();
        let has_undeclared_key = keys.iter().any(|key| !declared.contains(key));
        let status = row
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let bad_status =
            !["connected", "pending", "failed", "needs-auth", "disabled"].contains(&status);
        let declared_status_but_empty = is_object && row.get("status").is_none();
        let foreign_name = row
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|name| name != configured_name);
        let violates = !is_object
            || has_undeclared_key
            || bad_status
            || declared_status_but_empty
            || foreign_name;
        assert!(
            violates,
            "rejected shape {shape} must violate a declared constraint: {row}"
        );
    }
}
