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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// Completion evidence for the owned descendant tree behind one process exit.
///
/// One root process exiting is not evidence about its descendants. Only a
/// host that concretely observed its exact owned tree may report more.
pub enum ProcessTreeCompletion {
    /// Only the root process exit was observed. Owned descendants are
    /// unattested: they may have exited, and they may still be running.
    RootOnly,
    /// The host observed no member remaining in its exact owned tree.
    OwnedTreeEmpty,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Joined child-process exit observation.
///
/// The exit carries root truth and, separately, how far the host could prove
/// completion of the owned descendant tree.
pub struct ProcessExit {
    success: bool,
    code: Option<i32>,
    tree_completion: ProcessTreeCompletion,
}

impl ProcessExit {
    #[must_use]
    /// Creates a root-only exit observation from success truth and optional
    /// exit code.
    ///
    /// The observation makes no claim about owned descendants.
    pub const fn new(success: bool, code: Option<i32>) -> Self {
        Self {
            success,
            code,
            tree_completion: ProcessTreeCompletion::RootOnly,
        }
    }

    #[must_use]
    /// Creates an exit observation that also attests an empty owned tree.
    ///
    /// Only a host whose concrete mechanism observed that no member of its
    /// exact owned tree remains may construct this state. Root exit, exit
    /// code, a graceful stop request, a successful force-stop request, and a
    /// successful nearest-child wait are never that observation. A host that
    /// cannot make the observation reports [`ProcessExit::new`] instead.
    pub const fn attesting_empty_owned_tree(success: bool, code: Option<i32>) -> Self {
        Self {
            success,
            code,
            tree_completion: ProcessTreeCompletion::OwnedTreeEmpty,
        }
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

    #[must_use]
    /// Returns how far owned-tree completion was proved for this exit.
    pub const fn tree_completion(self) -> ProcessTreeCompletion {
        self.tree_completion
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProcessExit, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
        ProcessTreeCompletion,
    };

    #[test]
    fn process_chunks_redact_payloads() {
        let input = ProcessInputChunk::new(b"input-secret".to_vec());
        let output =
            ProcessOutputChunk::new(ProcessOutputStream::Stderr, b"stderr-secret".to_vec());

        assert!(!format!("{input:?}").contains("input-secret"));
        assert!(!format!("{output:?}").contains("stderr-secret"));
    }

    #[test]
    fn ordinary_exit_construction_stays_root_only() {
        for exit in [
            ProcessExit::new(true, Some(0)),
            ProcessExit::new(false, Some(1)),
            ProcessExit::new(false, None),
        ] {
            assert_eq!(exit.tree_completion(), ProcessTreeCompletion::RootOnly);
        }
    }

    #[test]
    fn attested_tree_emptiness_is_a_distinct_explicit_state() {
        let root_only = ProcessExit::new(true, Some(0));
        let attested = ProcessExit::attesting_empty_owned_tree(true, Some(0));

        assert_eq!(root_only.success(), attested.success());
        assert_eq!(root_only.code(), attested.code());
        assert_ne!(root_only, attested);
        assert_eq!(
            attested.tree_completion(),
            ProcessTreeCompletion::OwnedTreeEmpty
        );
    }
}
