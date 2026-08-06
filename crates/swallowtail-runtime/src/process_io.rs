use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Child-process output channel that produced a bounded chunk.
pub enum ProcessOutputStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

#[derive(Clone, Eq, PartialEq)]
/// Opaque bytes written to an owned child process.
pub struct ProcessInputChunk {
    bytes: Vec<u8>,
}

impl ProcessInputChunk {
    #[must_use]
    /// Creates an input chunk from opaque bytes.
    pub fn new(bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    #[must_use]
    /// Borrows the opaque input bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    /// Consumes the chunk and returns its opaque bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

impl fmt::Debug for ProcessInputChunk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessInputChunk")
            .field("bytes", &format_args!("<{} bytes>", self.bytes.len()))
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
/// Opaque bytes read from one child-process output channel.
pub struct ProcessOutputChunk {
    stream: ProcessOutputStream,
    bytes: Vec<u8>,
}

impl ProcessOutputChunk {
    #[must_use]
    /// Creates an output chunk attributed to an exact channel.
    pub fn new(stream: ProcessOutputStream, bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            stream,
            bytes: bytes.into(),
        }
    }

    #[must_use]
    /// Returns the output channel that produced the chunk.
    pub const fn stream(&self) -> ProcessOutputStream {
        self.stream
    }

    #[must_use]
    /// Borrows the opaque output bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for ProcessOutputChunk {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProcessOutputChunk")
            .field("stream", &self.stream)
            .field("bytes", &format_args!("<{} bytes>", self.bytes.len()))
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Joined child-process exit observation.
pub struct ProcessExit {
    success: bool,
    code: Option<i32>,
}

impl ProcessExit {
    #[must_use]
    /// Creates an exit observation from success truth and optional exit code.
    pub const fn new(success: bool, code: Option<i32>) -> Self {
        Self { success, code }
    }

    #[must_use]
    /// Returns whether the process reported successful termination.
    pub const fn success(self) -> bool {
        self.success
    }

    #[must_use]
    /// Returns the platform exit code when available.
    pub const fn code(self) -> Option<i32> {
        self.code
    }
}

#[cfg(test)]
mod tests {
    use super::{ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream};

    #[test]
    fn process_chunks_redact_payloads() {
        let input = ProcessInputChunk::new(b"input-secret".to_vec());
        let output =
            ProcessOutputChunk::new(ProcessOutputStream::Stderr, b"stderr-secret".to_vec());

        assert!(!format!("{input:?}").contains("input-secret"));
        assert!(!format!("{output:?}").contains("stderr-secret"));
    }
}
