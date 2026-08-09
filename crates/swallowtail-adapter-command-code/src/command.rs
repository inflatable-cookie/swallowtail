use swallowtail_core::ModelId;

/// Maximum agentic turns permitted on the bounded read-only route.
pub(crate) const MAXIMUM_TURNS: &str = "8";

fn base_arguments(model: &ModelId) -> Vec<String> {
    [
        "-p",
        "--output-format",
        "json",
        "--permission-mode",
        "plan",
        "--skip-onboarding",
        "--no-auto-update",
        "--trust",
        "--no-skills",
        "--max-turns",
        MAXIMUM_TURNS,
        "-m",
        model.as_str(),
    ]
    .map(str::to_owned)
    .into_iter()
    .collect()
}

/// Structured-run arguments: retained session storage is prohibited.
pub(crate) fn arguments(model: &ModelId) -> Vec<String> {
    let mut arguments = base_arguments(model);
    let insert_at = arguments
        .iter()
        .position(|argument| argument == "--skip-onboarding")
        .expect("skip-onboarding is present")
        + 1;
    arguments.insert(insert_at, "--no-session".to_owned());
    arguments
}

/// Interactive first-turn arguments: same as structured except session retention is allowed.
pub(crate) fn interactive_arguments(model: &ModelId) -> Vec<String> {
    base_arguments(model)
}

/// Later interactive turns append the exact private session id from a prior clean turn.
pub(crate) fn resumed_arguments(model: &ModelId, session_id: &str) -> Vec<String> {
    let mut arguments = interactive_arguments(model);
    arguments.extend(["--resume".to_owned(), session_id.to_owned()]);
    arguments
}

#[cfg(test)]
mod tests {
    use super::{arguments, interactive_arguments, resumed_arguments};
    use swallowtail_core::ModelId;

    fn model() -> ModelId {
        ModelId::new("fixture-model").expect("model")
    }

    #[test]
    fn structured_command_is_exact_read_only_prepared_and_never_carries_the_prompt() {
        let args = arguments(&model());
        assert_eq!(
            args,
            vec![
                "-p",
                "--output-format",
                "json",
                "--permission-mode",
                "plan",
                "--skip-onboarding",
                "--no-session",
                "--no-auto-update",
                "--trust",
                "--no-skills",
                "--max-turns",
                "8",
                "-m",
                "fixture-model",
            ]
        );
        for forbidden in [
            "--yolo",
            "--dangerously-skip-permissions",
            "--dont-ask",
            "--continue",
            "--fork-session",
            "--resume",
        ] {
            assert!(!args.iter().any(|arg| arg == forbidden));
        }
        assert!(!args.iter().any(|arg| arg.contains("fixture prompt")));
    }

    #[test]
    fn interactive_first_turn_omits_no_session_and_resume_selectors() {
        let args = interactive_arguments(&model());
        assert_eq!(
            args,
            vec![
                "-p",
                "--output-format",
                "json",
                "--permission-mode",
                "plan",
                "--skip-onboarding",
                "--no-auto-update",
                "--trust",
                "--no-skills",
                "--max-turns",
                "8",
                "-m",
                "fixture-model",
            ]
        );
        for forbidden in ["--no-session", "--continue", "--fork-session", "--resume"] {
            assert!(!args.iter().any(|arg| arg == forbidden));
        }
    }

    #[test]
    fn resumed_turn_appends_exact_resume_id_without_ambient_selectors() {
        let session_id = "00000000-0000-4000-8000-000000000101";
        let args = resumed_arguments(&model(), session_id);
        assert_eq!(
            args.windows(2)
                .find(|pair| pair[0] == "--resume")
                .map(|pair| pair.to_vec()),
            Some(vec!["--resume".to_owned(), session_id.to_owned()])
        );
        assert!(!args.iter().any(|arg| arg == "--no-session"));
        assert!(!args.iter().any(|arg| arg == "--continue"));
        assert!(!args.iter().any(|arg| arg == "--fork-session"));
        assert_eq!(args.iter().filter(|arg| *arg == "--resume").count(), 1);
    }
}
