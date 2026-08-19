/// ACP stdio arguments. TCP `--port`, `--yolo`, and server-start flags stay out.
pub(crate) fn arguments() -> Vec<String> {
    vec!["--acp".to_owned(), "--stdio".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_is_stdio_acp_and_never_port_or_yolo() {
        let args = arguments();
        assert_eq!(args, ["--acp", "--stdio"]);
        for forbidden in [
            "--port",
            "--yolo",
            "--allow-all",
            "--available-tools",
            "--excluded-tools",
            "--effort",
            "--reasoning-effort",
            "login",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for copilot-cli.acp"
            );
        }
    }
}
