use swallowtail_core::ModelId;
use swallowtail_runtime::OperationContent;

pub(crate) fn arguments(model: &ModelId, content: &OperationContent) -> Vec<String> {
    [
        "--model",
        model.as_str(),
        "--prompt",
        content.as_str(),
        "--output-format",
        "stream-json",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use swallowtail_core::ModelId;
    use swallowtail_runtime::OperationContent;

    /// The reviewable record of this argv lives in the currentness corpus.
    /// Binding it here keeps the fixture honest without widening public API.
    const CORPUS: &str = include_str!("../tests/fixtures/kimi-code-0.39.1/protocol.json");

    #[test]
    fn the_selected_argv_is_exactly_model_prompt_and_stream_json() {
        let model = ModelId::new("fixture-model").expect("static model id is valid");
        let content = OperationContent::new("fixture prompt").expect("static content is valid");
        assert_eq!(
            arguments(&model, &content),
            [
                "--model",
                "fixture-model",
                "--prompt",
                "fixture prompt",
                "--output-format",
                "stream-json",
            ]
        );
    }

    #[test]
    fn the_corpus_records_the_argv_this_driver_builds() {
        let corpus: serde_json::Value =
            serde_json::from_str(CORPUS).expect("currentness corpus is valid JSON");
        let recorded = corpus["executing_path"]["headless_argv"]
            .as_array()
            .expect("recorded argv is an array")
            .iter()
            .map(|value| value.as_str().expect("argv entry is text").to_owned())
            .collect::<Vec<_>>();

        let model = ModelId::new("fixture-model").expect("static model id is valid");
        let content = OperationContent::new("fixture prompt").expect("static content is valid");
        let built = arguments(&model, &content);

        assert_eq!(recorded.len(), built.len());
        // Flag positions and literal values must match exactly; the two value
        // slots are placeholders in the corpus.
        for (index, (recorded, built)) in recorded.iter().zip(built.iter()).enumerate() {
            if index == 1 || index == 3 {
                assert!(
                    recorded.starts_with('<') && recorded.ends_with('>'),
                    "corpus slot {index} must stay a placeholder, found {recorded}"
                );
            } else {
                assert_eq!(recorded, built, "corpus argv diverges at {index}");
            }
        }
    }
}
