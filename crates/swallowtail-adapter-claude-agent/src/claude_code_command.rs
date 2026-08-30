use crate::ClaudeCodeMaximumTurns;
use crate::claude_code_watcher::WatcherCommandFiles;
use swallowtail_core::{ModelId, ReasoningMode};

pub(crate) fn arguments(
    model: &ModelId,
    reasoning: Option<&ReasoningMode>,
    maximum_turns: Option<ClaudeCodeMaximumTurns>,
    watchers: Option<&WatcherCommandFiles>,
) -> Vec<String> {
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
        ["--permission-mode", "plan", "--tools", "Read,Glob,Grep"]
            .into_iter()
            .map(str::to_owned),
    );
    match watchers {
        Some(files) => {
            arguments.extend([
                "--bare".to_owned(),
                "--mcp-config".to_owned(),
                files.mcp_config.clone(),
                "--strict-mcp-config".to_owned(),
                "--settings".to_owned(),
                files.settings.clone(),
                "--add-dir".to_owned(),
                files.add_dir.clone(),
                "--include-hook-events".to_owned(),
            ]);
        }
        None => arguments.extend(
            [
                "--setting-sources",
                "user,project,local",
                "--mcp-config",
                r#"{"mcpServers":{}}"#,
                "--strict-mcp-config",
            ]
            .into_iter()
            .map(str::to_owned),
        ),
    }
    if let Some(maximum_turns) = maximum_turns {
        arguments.extend(["--max-turns".to_owned(), maximum_turns.as_u32().to_string()]);
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use crate::ClaudeCodeMaximumTurns;
    use swallowtail_core::{ModelId, ReasoningMode};

    #[test]
    fn default_effort_is_omitted_and_explicit_effort_is_forwarded() {
        let model = ModelId::new("claude-opus-5").expect("model is valid");
        let default = arguments(
            &model,
            Some(&ReasoningMode::new("default").expect("mode is valid")),
            None,
            None,
        );
        assert!(!default.iter().any(|argument| argument == "--effort"));
        let high = arguments(
            &model,
            Some(&ReasoningMode::new("high").expect("mode is valid")),
            None,
            None,
        );
        assert!(high.windows(2).any(|pair| pair == ["--effort", "high"]));
    }

    #[test]
    fn omitted_maximum_turns_preserves_the_exact_prior_command() {
        let model = ModelId::new("claude-opus-5").expect("model is valid");
        assert_eq!(
            arguments(&model, None, None, None),
            [
                "-p",
                "--input-format",
                "text",
                "--output-format",
                "stream-json",
                "--verbose",
                "--no-session-persistence",
                "--model",
                "claude-opus-5",
                "--permission-mode",
                "plan",
                "--tools",
                "Read,Glob,Grep",
                "--setting-sources",
                "user,project,local",
                r#"--mcp-config"#,
                r#"{"mcpServers":{}}"#,
                "--strict-mcp-config",
            ]
        );
    }

    #[test]
    fn selected_maximum_turns_appends_one_canonical_argument_pair() {
        let model = ModelId::new("claude-opus-5").expect("model is valid");
        let omitted = arguments(&model, None, None, None);
        let selected = arguments(
            &model,
            None,
            Some(ClaudeCodeMaximumTurns::from_u64(3).expect("value is admitted")),
            None,
        );
        assert_eq!(selected[..omitted.len()], omitted[..]);
        assert_eq!(selected[omitted.len()..], ["--max-turns", "3"]);
        assert_eq!(
            selected
                .iter()
                .filter(|argument| *argument == "--max-turns")
                .count(),
            1
        );
    }

    #[test]
    fn maximum_turns_composes_with_reasoning_without_reordering_it() {
        let model = ModelId::new("claude-opus-5").expect("model is valid");
        let selected = arguments(
            &model,
            Some(&ReasoningMode::new("high").expect("mode is valid")),
            Some(ClaudeCodeMaximumTurns::from_u64(30).expect("value is admitted")),
            None,
        );
        assert!(selected.windows(2).any(|pair| pair == ["--effort", "high"]));
        assert_eq!(selected[selected.len() - 2..], ["--max-turns", "30"]);
    }
}
