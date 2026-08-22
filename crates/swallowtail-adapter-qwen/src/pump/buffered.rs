use crate::events::QwenEventParser;
use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{RuntimeEventSender, RuntimeFailure};

pub(crate) fn push_buffered_values(
    parser: &mut QwenEventParser,
    events: &RuntimeEventSender,
    values: Vec<Value>,
) -> Result<(), RuntimeFailure> {
    for value in values {
        let mut bytes = serde_json::to_vec(&value).map_err(|_| {
            RuntimeFailure::new(SafeDiagnostic::new(
                "swallowtail.qwen.headless.malformed_stream",
                "Qwen Code emitted malformed stream output",
            ))
        })?;
        bytes.push(b'\n');
        for event in parser.push(&bytes)? {
            events.send(event)?;
        }
    }
    Ok(())
}
