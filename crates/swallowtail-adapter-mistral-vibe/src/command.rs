use crate::MistralVibeMaxTurns;

/// Required positive CLI turn bound. The host process deadline is the Swallowtail timeout.
pub(crate) const MAXIMUM_TURNS: &str = "8";

/// Adapter-private harness pin. Upstream `2.25.1` resolves the session
/// harness from flags, the ambient GrowthBook rollout cache, and native
/// module availability; `--legacy-harness` has first precedence and forces
/// the legacy Python harness the frozen corpus covers on every install
/// channel. The Unified Harness stays unmapped.
pub(crate) const LEGACY_HARNESS_PIN: &str = "--legacy-harness";

/// Headless argv. ACP, TUI, continue/resume, teleport, yolo, and the
/// experimental Unified Harness stay out.
pub(crate) fn arguments(
    cwd: &str,
    prompt: &str,
    max_turns: Option<MistralVibeMaxTurns>,
) -> Vec<String> {
    vec![
        "--prompt".to_owned(),
        prompt.to_owned(),
        "--output".to_owned(),
        "streaming".to_owned(),
        "--max-turns".to_owned(),
        max_turns.map_or_else(|| MAXIMUM_TURNS.to_owned(), |turns| turns.get().to_string()),
        "--trust".to_owned(),
        "--agent".to_owned(),
        "plan".to_owned(),
        "--workdir".to_owned(),
        cwd.to_owned(),
        LEGACY_HARNESS_PIN.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use crate::MistralVibeMaxTurns;

    fn assert_fixed_flags(args: &[String]) {
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
            "--experimental-harness",
            "--smart-approve",
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
        assert!(args.contains(&"--legacy-harness".to_owned()));
    }

    #[test]
    fn omitted_argv_keeps_current_max_turns_eight() {
        let args = arguments("/private/fixture", "private fixture prompt", None);
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
                "--legacy-harness",
            ]
        );
        assert_fixed_flags(&args);
        assert!(args.windows(2).any(|pair| pair == ["--max-turns", "8"]));
    }

    #[test]
    fn selected_argv_emits_admitted_one_and_eight() {
        let one = MistralVibeMaxTurns::try_new(1).expect("admitted");
        let eight = MistralVibeMaxTurns::try_new(8).expect("admitted");
        let args_one = arguments("/private/fixture", "private fixture prompt", Some(one));
        let args_eight = arguments("/private/fixture", "private fixture prompt", Some(eight));
        assert!(args_one.windows(2).any(|pair| pair == ["--max-turns", "1"]));
        assert!(
            args_eight
                .windows(2)
                .any(|pair| pair == ["--max-turns", "8"])
        );
        assert_fixed_flags(&args_one);
        assert_fixed_flags(&args_eight);
    }
}
