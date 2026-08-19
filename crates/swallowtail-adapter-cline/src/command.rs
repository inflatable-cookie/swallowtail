/// ACP stdio arguments. Headless `--json`, `--id`, and `--auto-approve` stay out.
pub(crate) fn arguments() -> Vec<String> {
    vec!["--acp".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_is_only_acp_and_never_json_auto_approve_or_id() {
        let args = arguments();
        assert_eq!(args, ["--acp"]);
        for forbidden in [
            "--json",
            "--auto-approve",
            "true",
            "--id",
            "--tui",
            "hub",
            "--yolo",
            "--zen",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for cline.acp"
            );
        }
    }
}
