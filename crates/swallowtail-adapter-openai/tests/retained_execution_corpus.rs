use serde_json::Value;

#[test]
fn retained_execution_corpus_freezes_checkpoint_reconciliation_and_detachment_truth() {
    let value: Value = serde_json::from_str(include_str!(
        "fixtures/openai-responses-2026-07-21/retained-execution.json"
    ))
    .expect("retained execution corpus is JSON");

    assert_eq!(value["checkpoint"]["provider_identity"], "response.id");
    assert_eq!(value["checkpoint"]["cursor"], "sequence_number");
    assert_eq!(value["reconciliation"]["method"], "GET");
    assert_eq!(value["reconciliation"]["maximum_requests"], 1);
    assert_eq!(value["detachment"]["scope"], "structured-run");
    assert_eq!(value["detachment"]["cancel_response"], false);
    assert_eq!(value["detachment"]["delete_response"], false);
    assert_eq!(value["ordinary_terminal_cleanup"]["delete_response"], true);
}
