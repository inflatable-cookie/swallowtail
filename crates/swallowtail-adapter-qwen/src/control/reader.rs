use super::{
    MAXIMUM_CONTROL_RECORDS, MAXIMUM_LINE_BYTES, MAXIMUM_OUTPUT_BYTES, control_protocol_failure,
    next_output, unexpected_control_response_failure,
};
use serde_json::Value;
use std::collections::VecDeque;
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, ProcessHandle, ProcessOutputStream, RuntimeFailure,
};

#[derive(Default)]
pub(super) struct ControlReader {
    bytes: Vec<u8>,
    lines: VecDeque<Value>,
    buffered_values: Vec<Value>,
    observed_bytes: usize,
    record_count: usize,
}

impl ControlReader {
    pub(super) async fn response(
        &mut self,
        process: &dyn ProcessHandle,
        request_id: &str,
        payload_subtype: &str,
        deadline: &mut BoxFuture<'static, DeadlineObservation>,
    ) -> Result<Value, RuntimeFailure> {
        loop {
            if let Some(payload) = self.take_response(request_id, payload_subtype)? {
                return Ok(payload);
            }
            let output = next_output(process, deadline).await?;
            let Some(output) = output else {
                self.finish_pending()?;
                if let Some(payload) = self.take_response(request_id, payload_subtype)? {
                    return Ok(payload);
                }
                return Err(super::failure(
                    "swallowtail.qwen.headless.reasoning_control_response_missing",
                    "Qwen Code ended before confirming reasoning control",
                ));
            };
            if output.stream() == ProcessOutputStream::Stdout {
                self.push(output.bytes())?;
            }
        }
    }

    pub(super) fn take_response(
        &mut self,
        request_id: &str,
        payload_subtype: &str,
    ) -> Result<Option<Value>, RuntimeFailure> {
        while let Some(value) = self.lines.pop_front() {
            if value.get("type").and_then(Value::as_str) != Some("control_response") {
                self.buffered_values.push(value);
                continue;
            }
            let response = value.get("response").ok_or_else(control_protocol_failure)?;
            if response.get("request_id").and_then(Value::as_str) != Some(request_id) {
                return Err(unexpected_control_response_failure());
            }
            if response.get("subtype").and_then(Value::as_str) != Some("success") {
                return Err(super::failure(
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
            return Ok(Some(payload.clone()));
        }
        Ok(None)
    }

    pub(super) fn push(&mut self, bytes: &[u8]) -> Result<(), RuntimeFailure> {
        if self.observed_bytes.saturating_add(bytes.len()) > MAXIMUM_OUTPUT_BYTES {
            return Err(control_protocol_failure());
        }
        self.observed_bytes += bytes.len();
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
            let value = serde_json::from_slice(&line).map_err(|_| control_protocol_failure())?;
            self.push_record(value)?;
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
        let value = serde_json::from_slice(&line).map_err(|_| control_protocol_failure())?;
        self.push_record(value)?;
        Ok(())
    }

    fn push_record(&mut self, value: Value) -> Result<(), RuntimeFailure> {
        if self.record_count >= MAXIMUM_CONTROL_RECORDS {
            return Err(control_protocol_failure());
        }
        self.record_count += 1;
        self.lines.push_back(value);
        Ok(())
    }

    pub(super) fn take_buffered_values(mut self) -> Result<Vec<Value>, RuntimeFailure> {
        while let Some(value) = self.lines.pop_front() {
            if value.get("type").and_then(Value::as_str) == Some("control_response") {
                return Err(control_protocol_failure());
            }
            self.buffered_values.push(value);
        }
        Ok(self.buffered_values)
    }
}

#[cfg(test)]
mod tests;
