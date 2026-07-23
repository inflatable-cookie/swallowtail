use super::{limit_failure, protocol_failure};
use swallowtail_runtime::RuntimeFailure;

const MAX_LINE_BYTES: usize = 262_144;
const MAX_STREAM_BYTES: usize = 1_048_576;
const MAX_RECORDS: usize = 4_096;

#[derive(Default)]
pub(crate) struct NdjsonDecoder {
    buffer: Vec<u8>,
    bytes: usize,
    records: usize,
}

impl NdjsonDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<Vec<u8>>, RuntimeFailure> {
        self.bytes = self
            .bytes
            .checked_add(chunk.len())
            .ok_or_else(limit_failure)?;
        if self.bytes > MAX_STREAM_BYTES {
            return Err(limit_failure());
        }
        self.buffer.extend_from_slice(chunk);
        if self.buffer.len() > MAX_LINE_BYTES && !self.buffer.contains(&b'\n') {
            return Err(limit_failure());
        }
        let mut lines = Vec::new();
        while let Some(position) = self.buffer.iter().position(|byte| *byte == b'\n') {
            let mut line = self.buffer.drain(..=position).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() || line.len() > MAX_LINE_BYTES {
                return Err(protocol_failure("stream record"));
            }
            self.records += 1;
            if self.records > MAX_RECORDS {
                return Err(limit_failure());
            }
            lines.push(line);
        }
        Ok(lines)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        if self.buffer.is_empty() {
            Ok(())
        } else {
            Err(crate::failure::failure(
                "swallowtail.ollama.stream_disconnected",
                "Ollama native stream disconnected during a record",
            ))
        }
    }
}
