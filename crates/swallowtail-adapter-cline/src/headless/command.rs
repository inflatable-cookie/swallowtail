use swallowtail_core::HarnessMode;

/// Headless argv. ACP, resume, yolo, and auto-approve true stay out.
pub(crate) fn arguments(cwd: &str, prompt: &str, harness_mode: Option<HarnessMode>) -> Vec<String> {
    let mut args = vec![
        "--json".to_owned(),
        "--auto-approve".to_owned(),
        "false".to_owned(),
    ];
    if harness_mode == Some(HarnessMode::Plan) {
        args.push("--plan".to_owned());
    }
    args.push("-c".to_owned());
    args.push(cwd.to_owned());
    args.push(prompt.to_owned());
    args
}

#[cfg(test)]
mod tests {
    use super::arguments;
    use swallowtail_core::HarnessMode;

    #[test]
    fn selected_argv_is_json_auto_approve_false_cwd_and_prompt() {
        let args = arguments("/private/fixture", "private fixture prompt", None);
        assert_eq!(
            args,
            [
                "--json",
                "--auto-approve",
                "false",
                "-c",
                "/private/fixture",
                "private fixture prompt"
            ]
        );
        for forbidden in [
            "--acp", "--id", "--yolo", "--zen", "--tui", "-i", "hub", "-k", "--plan",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for omitted cline.headless"
            );
        }
        assert!(
            !args
                .windows(2)
                .any(|pair| pair == ["--auto-approve", "true"])
        );
    }

    #[test]
    fn plan_places_canonical_flag_before_cwd_and_prompt() {
        let args = arguments(
            "/private/fixture",
            "private fixture prompt",
            Some(HarnessMode::Plan),
        );
        assert_eq!(
            args,
            [
                "--json",
                "--auto-approve",
                "false",
                "--plan",
                "-c",
                "/private/fixture",
                "private fixture prompt"
            ]
        );
        for forbidden in ["--acp", "--id", "--yolo", "--zen", "-p"] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for cline.headless Plan"
            );
        }
    }
}
