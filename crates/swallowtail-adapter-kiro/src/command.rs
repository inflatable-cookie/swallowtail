/// ACP stdio arguments. `--cloud`, `--agent`, and headless chat stay out.
pub(crate) fn arguments() -> Vec<String> {
    vec!["acp".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_is_only_acp_and_never_cloud_agent_or_headless() {
        let args = arguments();
        assert_eq!(args, ["acp"]);
        for forbidden in [
            "--cloud",
            "--agent",
            "chat",
            "--no-interactive",
            "--trust-all-tools",
            "--resume-id",
            "login",
            "kiro-cli-chat",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for kiro.acp"
            );
        }
    }
}
