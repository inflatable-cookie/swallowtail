/// Required positive conversation iteration bound. The host process deadline is
/// the Swallowtail timeout.
#[allow(dead_code)]
pub(crate) const MAXIMUM_ITERATIONS: u64 = 8;

/// Owned-child argv. Wildcard bind, ACP, SDK, and login stay out.
pub(crate) fn arguments() -> Vec<String> {
    vec![
        "-m".to_owned(),
        "openhands.agent_server".to_owned(),
        "--host".to_owned(),
        "127.0.0.1".to_owned(),
        "--port".to_owned(),
        "0".to_owned(),
    ]
}

/// Discovery argv. `python --version` is the interpreter, not the package.
pub(crate) fn discovery_arguments() -> Vec<String> {
    vec![
        "-c".to_owned(),
        "from importlib.metadata import version; print(version('openhands-agent-server'))"
            .to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn selected_argv_is_module_loopback_and_explicit_port() {
        let args = arguments();
        assert_eq!(
            args,
            [
                "-m",
                "openhands.agent_server",
                "--host",
                "127.0.0.1",
                "--port",
                "0",
            ]
        );
        for forbidden in [
            "0.0.0.0",
            "::",
            "[::]",
            "--cors-origins",
            "subscription_login",
            "acp",
            "NeverConfirm",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for openhands.agent-server"
            );
        }
        assert!(args.windows(2).any(|pair| pair == ["--host", "127.0.0.1"]));
        assert!(
            !args
                .windows(2)
                .any(|pair| pair[0] == "--host" && pair[1] != "127.0.0.1")
        );
    }
}
