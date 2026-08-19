/// Headless argv. ACP, resume, yolo, and auto-approve true stay out.
pub(crate) fn arguments(cwd: &str, prompt: &str) -> Vec<String> {
    vec![
        "--json".to_owned(),
        "--auto-approve".to_owned(),
        "false".to_owned(),
        "-c".to_owned(),
        cwd.to_owned(),
        prompt.to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn selected_argv_is_json_auto_approve_false_cwd_and_prompt() {
        let args = arguments("/private/fixture", "private fixture prompt");
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
            "--acp", "--id", "--yolo", "--zen", "--tui", "-i", "hub", "-k",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for cline.headless"
            );
        }
        assert!(
            !args
                .windows(2)
                .any(|pair| pair == ["--auto-approve", "true"])
        );
    }
}
