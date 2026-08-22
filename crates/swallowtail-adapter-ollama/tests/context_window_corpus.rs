use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_ollama::{OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION};

#[test]
fn corpus_covers_the_qualified_runtime_window() {
    let corpus: Value = serde_json::from_str(include_str!(
        "fixtures/ollama-num-ctx-v0.14.0-v0.32.15/corpus.json"
    ))
    .expect("corpus parses");
    let points: BTreeSet<_> = corpus["qualification_points"]
        .as_array()
        .expect("qualification points are an array")
        .iter()
        .map(|point| point["version"].as_str().expect("version is text"))
        .collect();

    assert_eq!(
        points,
        BTreeSet::from([
            "0.14.0", "0.18.0", "0.30.0", "0.32.1", "0.32.14", "0.32.15"
        ])
    );
    assert!(points.contains(OLLAMA_BASELINE_VERSION));
    assert_eq!(OLLAMA_LATEST_QUALIFIED_VERSION, "0.32.15");
    assert_eq!(
        corpus["excluded_versions"],
        serde_json::json!(["0.32.2", "0.32.10"])
    );
    assert_eq!(corpus["profile_dispositions"]["structured_run"], "deliver-now");
    assert_eq!(
        corpus["profile_dispositions"]["interactive_session"],
        "deliver-now"
    );
}
