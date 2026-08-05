pub(super) fn arguments(provider: &str, model: &str) -> Vec<String> {
    [
        "--mode",
        "rpc",
        "--no-session",
        "--provider",
        provider,
        "--model",
        model,
        "--tools",
        "read,grep,glob,todo,ask",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
        "--approval-mode",
        "always-ask",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

pub(super) fn catalogue_arguments() -> Vec<String> {
    [
        "--mode",
        "rpc",
        "--no-session",
        "--no-tools",
        "--no-extensions",
        "--no-skills",
        "--no-rules",
        "--no-prewalk",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
