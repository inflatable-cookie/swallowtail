use swallowtail_core::ModelId;

pub(crate) fn arguments(model: &ModelId, session_id: &str) -> Vec<String> {
    [
        "--output-format",
        "stream-json",
        "--model",
        model.as_str(),
        "--approval-mode",
        "plan",
        "--extensions",
        "none",
        "--allowed-mcp-server-names",
        "",
        "--skip-trust",
        "--session-id",
        session_id,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
