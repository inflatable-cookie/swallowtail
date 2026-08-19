/// Required positive CLI turn bound. The host process deadline is the Swallowtail timeout.
pub(crate) const MAXIMUM_TURNS: &str = "8";

/// Headless argv. ACP, TUI, continue/resume, teleport, and yolo stay out.
pub(crate) fn arguments(cwd: &str, prompt: &str) -> Vec<String> {
    vec![
        "--prompt".to_owned(),
        prompt.to_owned(),
        "--output".to_owned(),
        "streaming".to_owned(),
        "--max-turns".to_owned(),
        MAXIMUM_TURNS.to_owned(),
        "--trust".to_owned(),
        "--agent".to_owned(),
        "plan".to_owned(),
        "--workdir".to_owned(),
        cwd.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn selected_argv_is_streaming_plan_trust_max_turns_and_workdir() {
        let args = arguments("/private/fixture", "private fixture prompt");
        assert_eq!(
            args,
            [
                "--prompt",
                "private fixture prompt",
                "--output",
                "streaming",
                "--max-turns",
                "8",
                "--trust",
                "--agent",
                "plan",
                "--workdir",
                "/private/fixture",
            ]
        );
        for forbidden in [
            "vibe-acp",
            "--continue",
            "--resume",
            "--teleport",
            "--auto-approve",
            "--yolo",
            "--setup",
            "--worktree",
            "--max-price",
            "json",
            "text",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for mistral-vibe.headless"
            );
        }
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--output", "streaming"])
        );
        assert!(args.windows(2).any(|pair| pair == ["--agent", "plan"]));
        assert!(args.contains(&"--trust".to_owned()));
        assert!(args.windows(2).any(|pair| pair == ["--max-turns", "8"]));
    }
}
