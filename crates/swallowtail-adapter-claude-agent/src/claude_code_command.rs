use swallowtail_core::{ModelId, ReasoningMode};

pub(crate) fn arguments(model: &ModelId, reasoning: Option<&ReasoningMode>) -> Vec<String> {
    let mut arguments = [
        "-p",
        "--input-format",
        "text",
        "--output-format",
        "stream-json",
        "--verbose",
        "--no-session-persistence",
        "--model",
        model.as_str(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if let Some(reasoning) = reasoning.filter(|mode| mode.as_str() != "default") {
        arguments.extend(["--effort".to_owned(), reasoning.as_str().to_owned()]);
    }
    arguments.extend(
        [
            "--permission-mode",
            "plan",
            "--tools",
            "Read,Glob,Grep",
            "--setting-sources",
            "user,project,local",
            "--mcp-config",
            r#"{"mcpServers":{}}"#,
            "--strict-mcp-config",
        ]
        .into_iter()
        .map(str::to_owned),
    );
    arguments
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use swallowtail_core::{ModelId, ReasoningMode};

    #[test]
    fn default_effort_is_omitted_and_explicit_effort_is_forwarded() {
        let model = ModelId::new("claude-opus-5").expect("model is valid");
        let default = arguments(
            &model,
            Some(&ReasoningMode::new("default").expect("mode is valid")),
        );
        assert!(!default.iter().any(|argument| argument == "--effort"));
        let high = arguments(
            &model,
            Some(&ReasoningMode::new("high").expect("mode is valid")),
        );
        assert!(high.windows(2).any(|pair| pair == ["--effort", "high"]));
    }
}
