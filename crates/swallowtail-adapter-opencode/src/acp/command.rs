/// ACP stdio arguments. Plugin loading is not part of the selected surface.
pub(crate) fn arguments() -> Vec<String> {
    vec!["acp".to_owned(), "--pure".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_pins_pure_and_never_host_plugins_or_network_flags() {
        let args = arguments();
        assert_eq!(args, ["acp", "--pure"]);
        for forbidden in [
            "--port",
            "--hostname",
            "--mdns",
            "--cors",
            "--print-logs",
            "login",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for opencode.acp"
            );
        }
    }
}
