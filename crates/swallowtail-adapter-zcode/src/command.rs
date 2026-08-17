/// App-server argv: protocol process only, never TUI or `--print`.
///
/// The packaged payload rejects `--settings`. Host-approved settings are
/// read from `$HOME/.zcode/cli/config.json`.
pub(crate) fn arguments() -> Vec<String> {
    vec!["app-server".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn spawn_is_app_server_only_and_never_yolo() {
        let args = arguments();
        assert_eq!(args, vec!["app-server"]);
        for forbidden in [
            "--settings",
            "tui",
            "--print",
            "-p",
            "--prompt",
            "--mode",
            "yolo",
        ] {
            assert!(!args.iter().any(|argument| argument == forbidden));
        }
    }
}
