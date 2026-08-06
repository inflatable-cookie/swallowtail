use serde_json::Value;
use swallowtail_runtime::RuntimeFailure;

const MAXIMUM_IDENTITY_BYTES: usize = 256;

pub(super) fn is_known_payload_type(payload_type: &str) -> bool {
    matches!(
        payload_type,
        "runtime.command.accepted"
            | "session.run.linked"
            | "run.model.configured"
            | "turn.input.user"
            | "run.lifecycle.started"
            | "run.output.delta"
            | "run.terminal.completed"
            | "run.terminal.failed"
            | "run.terminal.cancelled"
            | "task.stream.linked"
    ) || payload_type.starts_with("task.lifecycle.")
}

pub(super) fn required_text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RuntimeFailure> {
    value.get(key).and_then(Value::as_str).ok_or_else(malformed)
}

pub(super) fn bounded_identity(value: &str) -> Result<&str, RuntimeFailure> {
    if value.trim().is_empty()
        || value.len() > MAXIMUM_IDENTITY_BYTES
        || value.chars().any(char::is_control)
    {
        Err(malformed())
    } else {
        Ok(value)
    }
}

pub(super) fn trim_newline(line: &[u8]) -> &[u8] {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    line.strip_suffix(b"\r").unwrap_or(line)
}

pub(super) fn malformed() -> RuntimeFailure {
    fail(
        "malformed_stream",
        "Muse Code emitted malformed headless JSONL",
    )
}

pub(super) fn record_limit() -> RuntimeFailure {
    fail(
        "record_limit",
        "Muse Code exceeded the headless record bound",
    )
}

pub(super) fn stream_limit() -> RuntimeFailure {
    fail(
        "stream_limit",
        "Muse Code exceeded the headless stream bound",
    )
}

pub(super) fn unknown_limit() -> RuntimeFailure {
    fail(
        "unknown_payload_limit",
        "Muse Code emitted an oversized unknown event payload",
    )
}

pub(super) fn output_limit() -> RuntimeFailure {
    fail(
        "output_limit",
        "Muse Code exceeded the headless output bound",
    )
}

pub(super) fn sequence_mismatch() -> RuntimeFailure {
    fail(
        "sequence_mismatch",
        "Muse Code event sequence is not contiguous",
    )
}

pub(super) fn session_mismatch() -> RuntimeFailure {
    fail(
        "session_mismatch",
        "Muse Code event belongs to another session",
    )
}

pub(super) fn causation_mismatch() -> RuntimeFailure {
    fail(
        "causation_mismatch",
        "Muse Code event belongs to another command",
    )
}

pub(super) fn run_mismatch() -> RuntimeFailure {
    fail("run_mismatch", "Muse Code event belongs to another run")
}

pub(super) fn task_mismatch() -> RuntimeFailure {
    fail(
        "task_mismatch",
        "Muse Code event belongs to an unknown task",
    )
}

pub(super) fn model_mismatch() -> RuntimeFailure {
    fail(
        "model_mismatch",
        "Muse Code configured a different provider or model",
    )
}

pub(super) fn output_mismatch() -> RuntimeFailure {
    fail(
        "output_mismatch",
        "Muse Code terminal output disagrees with its deltas",
    )
}

pub(super) fn post_terminal() -> RuntimeFailure {
    fail(
        "post_terminal",
        "Muse Code emitted activity after the terminal event",
    )
}

fn fail(suffix: &'static str, message: &'static str) -> RuntimeFailure {
    crate::failure::failure(
        match suffix {
            "malformed_stream" => "swallowtail.muse_code.headless.malformed_stream",
            "record_limit" => "swallowtail.muse_code.headless.record_limit",
            "stream_limit" => "swallowtail.muse_code.headless.stream_limit",
            "unknown_payload_limit" => "swallowtail.muse_code.headless.unknown_payload_limit",
            "output_limit" => "swallowtail.muse_code.headless.output_limit",
            "sequence_mismatch" => "swallowtail.muse_code.headless.sequence_mismatch",
            "session_mismatch" => "swallowtail.muse_code.headless.session_mismatch",
            "causation_mismatch" => "swallowtail.muse_code.headless.causation_mismatch",
            "run_mismatch" => "swallowtail.muse_code.headless.run_mismatch",
            "task_mismatch" => "swallowtail.muse_code.headless.task_mismatch",
            "model_mismatch" => "swallowtail.muse_code.headless.model_mismatch",
            "output_mismatch" => "swallowtail.muse_code.headless.output_mismatch",
            "post_terminal" => "swallowtail.muse_code.headless.post_terminal",
            _ => "swallowtail.muse_code.headless.invalid_stream",
        },
        message,
    )
}
