#[derive(Clone, Eq, PartialEq)]
/// One bounded named SSE frame with redacted payload bytes.
pub struct SseFrame {
    inner: swallowtail_transport_http::SseFrame,
}

impl SseFrame {
    #[must_use]
    /// Returns the provider event name.
    pub fn name(&self) -> &str {
        self.inner
            .name()
            .expect("Model Studio SSE frame carries a validated non-empty name")
    }

    #[must_use]
    pub(crate) fn data(&self) -> &[u8] {
        self.inner.data()
    }
}

impl std::fmt::Debug for SseFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SseFrame")
            .field("name", &self.name())
            .field("data", &"<redacted>")
            .finish()
    }
}

#[derive(Default)]
/// Incremental bounded decoder for Model Studio SSE frames.
pub struct SseDecoder {
    inner: swallowtail_transport_http::SseDecoder,
}

impl SseDecoder {
    /// Accepts one transport chunk and returns every complete decoded frame.
    pub fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, AlibabaProtocolFailure> {
        let mut frames = Vec::new();
        for frame in self.inner.push(chunk).map_err(map_sse_error)? {
            let valid_name = frame.name().is_some_and(|name| !name.is_empty());
            if !valid_name || frame.data().is_empty() {
                return Err(AlibabaProtocolFailure::invalid("SSE frame"));
            }
            frames.push(SseFrame { inner: frame });
        }
        Ok(frames)
    }

    /// Verifies that disconnect left no partial semantic frame buffered.
    pub fn finish(self) -> Result<(), AlibabaProtocolFailure> {
        self.inner.finish().map_err(map_sse_error)
    }
}

fn map_sse_error(error: swallowtail_transport_http::SseDecodeError) -> AlibabaProtocolFailure {
    match error.kind() {
        swallowtail_transport_http::SseDecodeErrorKind::InvalidUtf8 => {
            AlibabaProtocolFailure::invalid("SSE UTF-8")
        }
        swallowtail_transport_http::SseDecodeErrorKind::LimitExceeded => {
            AlibabaProtocolFailure::invalid("SSE event bound")
        }
        swallowtail_transport_http::SseDecodeErrorKind::IncompleteFrame => {
            AlibabaProtocolFailure::invalid("disconnected SSE event")
        }
    }
}
