use swallowtail_core::ModelId;
use swallowtail_runtime::OperationContent;

pub(crate) fn arguments(model: &ModelId, content: &OperationContent) -> Vec<String> {
    [
        "--model",
        model.as_str(),
        "--prompt",
        content.as_str(),
        "--output-format",
        "stream-json",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}
