use crate::error::{RemoteAcpError, capacity_error, protocol_error};

pub(crate) struct SseDecoder {
    maximum_event_bytes: usize,
    pending: Vec<u8>,
    data: String,
}

impl SseDecoder {
    pub(crate) fn new(maximum_event_bytes: usize) -> Self {
        Self {
            maximum_event_bytes,
            pending: Vec::new(),
            data: String::new(),
        }
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) -> Result<Vec<String>, RemoteAcpError> {
        if self.pending.len().saturating_add(bytes.len())
            > self.maximum_event_bytes.saturating_mul(2)
        {
            return Err(capacity_error());
        }
        self.pending.extend_from_slice(bytes);
        let mut events = Vec::new();
        while let Some(position) = self.pending.iter().position(|byte| *byte == b'\n') {
            let mut line = self.pending.drain(..=position).collect::<Vec<_>>();
            line.pop();
            if line.last() == Some(&b'\r') {
                line.pop();
            }
            if line.is_empty() {
                if !self.data.is_empty() {
                    if self.data.ends_with('\n') {
                        self.data.pop();
                    }
                    events.push(std::mem::take(&mut self.data));
                }
                continue;
            }
            if line.first() == Some(&b':') {
                continue;
            }
            if let Some(value) = line.strip_prefix(b"data:") {
                let value = value.strip_prefix(b" ").unwrap_or(value);
                let value = std::str::from_utf8(value).map_err(|_| protocol_error())?;
                if self
                    .data
                    .len()
                    .saturating_add(value.len())
                    .saturating_add(1)
                    > self.maximum_event_bytes
                {
                    return Err(capacity_error());
                }
                self.data.push_str(value);
                self.data.push('\n');
            }
        }
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::SseDecoder;

    #[test]
    fn fragmented_events_are_bounded_and_comments_are_ignored() {
        let mut decoder = SseDecoder::new(64);
        assert!(
            decoder
                .push(b": keepalive\ndata: {\"json")
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            decoder.push(b"rpc\":\"2.0\"}\n\n").unwrap(),
            ["{\"jsonrpc\":\"2.0\"}"]
        );
        assert!(SseDecoder::new(4).push(b"data: too-large\n").is_err());
    }
}
