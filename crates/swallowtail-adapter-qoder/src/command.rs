/// Required positive CLI turn bound. The host process deadline is the Swallowtail timeout.
pub(crate) const MAXIMUM_TURNS: &str = "8";

/// Headless argv. ACP, SDK stdio, TUI, yolo, and session restore stay out.
pub(crate) fn arguments(cwd: &str, prompt: &str) -> Vec<String> {
    vec![
        "--print".to_owned(),
        "--output-format".to_owned(),
        "stream-json".to_owned(),
        "--permission-mode".to_owned(),
        "dont_ask".to_owned(),
        "--max-turns".to_owned(),
        MAXIMUM_TURNS.to_owned(),
        "--no-session-persistence".to_owned(),
        "--cwd".to_owned(),
        cwd.to_owned(),
        prompt.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn selected_argv_is_print_stream_json_dont_ask_max_turns_and_cwd() {
        let args = arguments("/private/fixture", "private fixture prompt");
        assert_eq!(
            args,
            [
                "--print",
                "--output-format",
                "stream-json",
                "--permission-mode",
                "dont_ask",
                "--max-turns",
                "8",
                "--no-session-persistence",
                "--cwd",
                "/private/fixture",
                "private fixture prompt",
            ]
        );
        for forbidden in [
            "--acp",
            "--yolo",
            "--dangerously-skip-permissions",
            "--continue",
            "--resume",
            "--session-id",
            "--teleport",
            "--worktree",
            "--input-format",
            "login",
            "ide",
            "json",
            "text",
            "bypass_permissions",
            "accept_edits",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for qoder.headless"
            );
        }
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--output-format", "stream-json"])
        );
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--permission-mode", "dont_ask"])
        );
        assert!(args.contains(&"--no-session-persistence".to_owned()));
        assert!(args.windows(2).any(|pair| pair == ["--max-turns", "8"]));
    }
}
