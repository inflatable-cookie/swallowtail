/// ACP stdio arguments. `npx`, `--workspace`, and `--model` stay out.
pub(crate) fn arguments() -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_is_empty_and_never_npx_workspace_or_model() {
        let args = arguments();
        assert!(args.is_empty());
        for forbidden in [
            "npx",
            "acp",
            "--workspace",
            "--model",
            "--name",
            "--skills",
            "--memory",
            "--debug",
            "--log-file",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for deepagents.acp"
            );
        }
    }
}
