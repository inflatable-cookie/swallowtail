use crate::validation::failure;
use serde_json::{Value, json};
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, OperationContent, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, RuntimeFailure,
};

mod reader;
use reader::ControlReader;

const MAXIMUM_OUTPUT_BYTES: usize = 2 * 1024 * 1024;
const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
const MAXIMUM_CONTROL_RECORDS: usize = 4096;
const MAXIMUM_SESSION_ID_BYTES: usize = 256;

pub(crate) struct ReasoningSetup {
    pub(crate) session_id: String,
    pub(crate) buffered_values: Vec<Value>,
}

pub(crate) async fn establish_reasoning(
    process: &dyn ProcessHandle,
    reasoning: &ReasoningMode,
    deadline: BoxFuture<'static, DeadlineObservation>,
) -> Result<ReasoningSetup, RuntimeFailure> {
    let mut reader = ControlReader::default();
    let mut deadline = deadline;
    write_control(
        process,
        json!({
            "type": "control_request",
            "request_id": "swallowtail-initialize",
            "request": {"subtype": "initialize"}
        }),
    )
    .await?;
    let initialize = reader
        .response(
            process,
            "swallowtail-initialize",
            "initialize",
            &mut deadline,
        )
        .await?;
    if initialize
        .pointer("/capabilities/can_set_effort")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return Err(failure(
            "swallowtail.qwen.headless.reasoning_control_unavailable",
            "Qwen Code did not advertise exact reasoning control support",
        ));
    }
    let session_id = bounded_session_id(
        initialize
            .get("session_id")
            .and_then(Value::as_str)
            .ok_or_else(control_protocol_failure)?,
    )?;
    write_control(
        process,
        json!({
            "type": "control_request",
            "request_id": "swallowtail-reasoning",
            "request": {"subtype": "set_effort", "effort": reasoning.as_str()}
        }),
    )
    .await?;
    let applied = reader
        .response(
            process,
            "swallowtail-reasoning",
            "set_effort",
            &mut deadline,
        )
        .await?;
    if applied.get("effort").and_then(Value::as_str) != Some(reasoning.as_str())
        || applied.get("applied").and_then(Value::as_bool) != Some(true)
        || !applied.get("override").is_some_and(Value::is_null)
    {
        return Err(failure(
            "swallowtail.qwen.headless.reasoning_not_applied",
            "Qwen Code did not confirm the exact requested reasoning mode",
        ));
    }
    let buffered_values = reader.take_buffered_values()?;
    Ok(ReasoningSetup {
        session_id,
        buffered_values,
    })
}

pub(crate) async fn establish_reasoning_and_write_user(
    process: &dyn ProcessHandle,
    reasoning: &ReasoningMode,
    deadline: BoxFuture<'static, DeadlineObservation>,
    content: &OperationContent,
) -> Result<Vec<Value>, RuntimeFailure> {
    let setup = establish_reasoning(process, reasoning, deadline).await?;
    write_user_message(process, &setup.session_id, content).await?;
    Ok(setup.buffered_values)
}

pub(crate) async fn write_user_message(
    process: &dyn ProcessHandle,
    session_id: &str,
    content: &OperationContent,
) -> Result<(), RuntimeFailure> {
    let mut bytes = serde_json::to_vec(&json!({
        "type": "user",
        "session_id": session_id,
        "message": {"role": "user", "content": content.as_str()},
        "parent_tool_use_id": null
    }))
    .map_err(|_| control_protocol_failure())?;
    bytes.push(b'\n');
    process.write_stdin(ProcessInputChunk::new(bytes)).await?;
    process.close_stdin().await.map_err(|_| {
        failure(
            "swallowtail.qwen.headless.stdin_close_failed",
            "Qwen headless process stdin could not be closed",
        )
    })
}

async fn write_control(process: &dyn ProcessHandle, value: Value) -> Result<(), RuntimeFailure> {
    let mut bytes = serde_json::to_vec(&value).map_err(|_| control_protocol_failure())?;
    bytes.push(b'\n');
    process.write_stdin(ProcessInputChunk::new(bytes)).await
}

async fn next_output(
    process: &dyn ProcessHandle,
    deadline: &mut BoxFuture<'static, DeadlineObservation>,
) -> Result<Option<ProcessOutputChunk>, RuntimeFailure> {
    let mut read = process.read_output();
    poll_fn(|context| {
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Err(failure(
                "swallowtail.qwen.headless.reasoning_control_timed_out",
                "Qwen Code reasoning control timed out",
            )));
        }
        read.as_mut().poll(context)
    })
    .await
}

fn bounded_session_id(value: &str) -> Result<String, RuntimeFailure> {
    if value.is_empty()
        || value.len() > MAXIMUM_SESSION_ID_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        Err(control_protocol_failure())
    } else {
        Ok(value.to_owned())
    }
}

fn control_protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.reasoning_control_invalid",
        "Qwen Code returned an invalid reasoning control response",
    )
}

fn unexpected_control_response_failure() -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.reasoning_control_unexpected_response",
        "Qwen Code returned a control response for an unexpected request",
    )
}
