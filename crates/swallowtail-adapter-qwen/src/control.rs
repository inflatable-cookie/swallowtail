use crate::validation::failure;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, OperationContent, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessOutputStream, RuntimeFailure,
};

const MAXIMUM_OUTPUT_BYTES: usize = 2 * 1024 * 1024;
const MAXIMUM_LINE_BYTES: usize = 1024 * 1024;
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

#[derive(Default)]
struct ControlReader {
    bytes: Vec<u8>,
    lines: VecDeque<Value>,
    buffered_values: Vec<Value>,
}

impl ControlReader {
    async fn response(
        &mut self,
        process: &dyn ProcessHandle,
        request_id: &str,
        payload_subtype: &str,
        deadline: &mut BoxFuture<'static, DeadlineObservation>,
    ) -> Result<Value, RuntimeFailure> {
        loop {
            while let Some(value) = self.lines.pop_front() {
                if value.get("type").and_then(Value::as_str) != Some("control_response") {
                    self.buffered_values.push(value);
                    continue;
                }
                let response = value.get("response").ok_or_else(control_protocol_failure)?;
                if response.get("request_id").and_then(Value::as_str) != Some(request_id) {
                    continue;
                }
                if response.get("subtype").and_then(Value::as_str) != Some("success") {
                    return Err(failure(
                        "swallowtail.qwen.headless.reasoning_control_rejected",
                        "Qwen Code rejected an exact reasoning control request",
                    ));
                }
                let payload = response
                    .get("response")
                    .ok_or_else(control_protocol_failure)?;
                if payload.get("subtype").and_then(Value::as_str) != Some(payload_subtype) {
                    return Err(control_protocol_failure());
                }
                return Ok(payload.clone());
            }
            let output = next_output(process, deadline).await?;
            let Some(output) = output else {
                self.finish_pending()?;
                if let Some(value) = self.lines.pop_front() {
                    self.buffered_values.push(value);
                    continue;
                }
                return Err(failure(
                    "swallowtail.qwen.headless.reasoning_control_response_missing",
                    "Qwen Code ended before confirming reasoning control",
                ));
            };
            if output.stream() == ProcessOutputStream::Stdout {
                self.push(output.bytes())?;
            }
        }
    }

    fn push(&mut self, bytes: &[u8]) -> Result<(), RuntimeFailure> {
        if self.bytes.len().saturating_add(bytes.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(control_protocol_failure());
        }
        self.bytes.extend_from_slice(bytes);
        while let Some(index) = self.bytes.iter().position(|byte| *byte == b'\n') {
            if index > MAXIMUM_LINE_BYTES {
                return Err(control_protocol_failure());
            }
            let mut line = self.bytes.drain(..=index).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                continue;
            }
            self.lines
                .push_back(serde_json::from_slice(&line).map_err(|_| control_protocol_failure())?);
        }
        if self.bytes.len() > MAXIMUM_LINE_BYTES {
            return Err(control_protocol_failure());
        }
        Ok(())
    }

    fn finish_pending(&mut self) -> Result<(), RuntimeFailure> {
        if self.bytes.is_empty() {
            return Ok(());
        }
        let line = std::mem::take(&mut self.bytes);
        self.lines
            .push_back(serde_json::from_slice(&line).map_err(|_| control_protocol_failure())?);
        Ok(())
    }

    fn take_buffered_values(mut self) -> Result<Vec<Value>, RuntimeFailure> {
        while let Some(value) = self.lines.pop_front() {
            if value.get("type").and_then(Value::as_str) == Some("control_response") {
                return Err(control_protocol_failure());
            }
            self.buffered_values.push(value);
        }
        Ok(self.buffered_values)
    }
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
