use swallowtail_core::{ModelId, ReasoningMode};

pub(crate) struct MuseCommand<'a> {
    pub(crate) prompt: &'a str,
    pub(crate) model: &'a ModelId,
    pub(crate) effort: &'a ReasoningMode,
}

pub(crate) fn arguments(command: MuseCommand<'_>) -> Vec<String> {
    [
        "exec",
        "--json",
        "--provider",
        "meta",
        "--model",
        command.model.as_str(),
        "--reasoning-effort",
        command.effort.as_str(),
        "--no-parallel-tool-calls",
        "--max-model-steps",
        "64",
        "--max-tool-output-bytes",
        "262144",
        "--user-input-auto-resolve",
        "--disable-web-tools",
        "--no-foreign-personal-context",
        "--no-session-log",
        "--disable-write",
        "--disable-shell",
        command.prompt,
    ]
    .map(str::to_owned)
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_is_exact_read_only_ephemeral_and_explicit() {
        let args = arguments(MuseCommand {
            prompt: "private fixture prompt",
            model: &ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
            effort: &ReasoningMode::new("low").unwrap(),
        });
        for required in [
            "exec",
            "--json",
            "meta",
            crate::MUSE_SPARK_MODEL_ID,
            "low",
            "--no-session-log",
            "--disable-write",
            "--disable-shell",
            "--disable-web-tools",
            "--user-input-auto-resolve",
        ] {
            assert!(args.iter().any(|arg| arg == required));
        }
        for forbidden in ["muse", "--yolo", "--disable-sandbox", "--trust-workspace"] {
            assert!(!args.iter().any(|arg| arg == forbidden));
        }
        assert_eq!(
            args.last().map(String::as_str),
            Some("private fixture prompt")
        );
    }
}
