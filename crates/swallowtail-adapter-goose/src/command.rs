/// ACP stdio arguments. `serve`, `--with-builtin`, and `--enable-scheduler` stay out.
pub(crate) fn arguments() -> Vec<String> {
    vec!["acp".to_owned()]
}

#[cfg(test)]
mod tests {
    use super::arguments;

    #[test]
    fn acp_argv_is_only_acp_and_never_serve_or_with_builtin() {
        let args = arguments();
        assert_eq!(args, ["acp"]);
        for forbidden in [
            "serve",
            "--with-builtin",
            "developer",
            "--enable-scheduler",
            "--dangerously-unauthenticated",
            "desktop",
            "tui",
            "recipe",
        ] {
            assert!(
                !args.iter().any(|argument| argument == forbidden),
                "{forbidden} must not be selected for goose.acp"
            );
        }
    }
}
