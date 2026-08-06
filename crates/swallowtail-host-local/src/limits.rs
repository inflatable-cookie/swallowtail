/// Host-enforced limits for child-process arguments and standard streams.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalProcessLimits {
    arguments: usize,
    argument_bytes: usize,
    stdin_bytes: usize,
    stdout_bytes: usize,
    stderr_bytes: usize,
}

impl LocalProcessLimits {
    /// Creates explicit argument, input, output, and error-output limits.
    #[must_use]
    pub const fn new(
        arguments: usize,
        argument_bytes: usize,
        stdin_bytes: usize,
        stdout_bytes: usize,
        stderr_bytes: usize,
    ) -> Self {
        Self {
            arguments,
            argument_bytes,
            stdin_bytes,
            stdout_bytes,
            stderr_bytes,
        }
    }

    /// Returns the maximum argument count.
    #[must_use]
    pub const fn arguments(self) -> usize {
        self.arguments
    }

    /// Returns the maximum combined argument bytes.
    #[must_use]
    pub const fn argument_bytes(self) -> usize {
        self.argument_bytes
    }

    /// Returns the maximum standard-input bytes.
    #[must_use]
    pub const fn stdin_bytes(self) -> usize {
        self.stdin_bytes
    }

    /// Returns the maximum standard-output bytes.
    #[must_use]
    pub const fn stdout_bytes(self) -> usize {
        self.stdout_bytes
    }

    /// Returns the maximum standard-error bytes.
    #[must_use]
    pub const fn stderr_bytes(self) -> usize {
        self.stderr_bytes
    }
}

impl Default for LocalProcessLimits {
    fn default() -> Self {
        Self::new(
            128,
            256 * 1024,
            8 * 1024 * 1024,
            8 * 1024 * 1024,
            1024 * 1024,
        )
    }
}

/// Host-enforced byte limits for file materialization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalMaterializationLimits {
    attachment_bytes: u64,
    schema_bytes: u64,
}

impl LocalMaterializationLimits {
    /// Creates explicit attachment and schema byte limits.
    #[must_use]
    pub const fn new(attachment_bytes: u64, schema_bytes: u64) -> Self {
        Self {
            attachment_bytes,
            schema_bytes,
        }
    }

    /// Returns the maximum attachment bytes.
    #[must_use]
    pub const fn attachment_bytes(self) -> u64 {
        self.attachment_bytes
    }

    /// Returns the maximum schema bytes.
    #[must_use]
    pub const fn schema_bytes(self) -> u64 {
        self.schema_bytes
    }
}

impl Default for LocalMaterializationLimits {
    fn default() -> Self {
        Self::new(16 * 1024 * 1024, 1024 * 1024)
    }
}
