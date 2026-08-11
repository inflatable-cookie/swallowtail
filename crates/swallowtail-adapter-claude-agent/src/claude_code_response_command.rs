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
            "--tools",
            "",
            "--safe-mode",
            "--disable-slash-commands",
            "--no-chrome",
            "--prompt-suggestions",
            "false",
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
    fn exact_tool_free_arguments_have_no_schema_or_continuation() {
        let arguments = arguments(
            &ModelId::new("claude-sonnet-5").unwrap(),
            Some(&ReasoningMode::new("low").unwrap()),
        );
        assert!(arguments.windows(2).any(|pair| pair == ["--tools", ""]));
        assert!(arguments.iter().any(|value| value == "--safe-mode"));
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--mcp-config", r#"{"mcpServers":{}}"#])
        );
        for forbidden in [
            "--json-schema",
            "--permission-mode",
            "--resume",
            "--continue",
            "--fork-session",
            "--fallback-model",
        ] {
            assert!(!arguments.iter().any(|value| value == forbidden));
        }
    }
}
