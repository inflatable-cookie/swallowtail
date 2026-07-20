#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalProcessLimits {
    arguments: usize,
    argument_bytes: usize,
    stdin_bytes: usize,
    stdout_bytes: usize,
    stderr_bytes: usize,
}

impl LocalProcessLimits {
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

    #[must_use]
    pub const fn arguments(self) -> usize {
        self.arguments
    }

    #[must_use]
    pub const fn argument_bytes(self) -> usize {
        self.argument_bytes
    }

    #[must_use]
    pub const fn stdin_bytes(self) -> usize {
        self.stdin_bytes
    }

    #[must_use]
    pub const fn stdout_bytes(self) -> usize {
        self.stdout_bytes
    }

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalMaterializationLimits {
    attachment_bytes: u64,
    schema_bytes: u64,
}

impl LocalMaterializationLimits {
    #[must_use]
    pub const fn new(attachment_bytes: u64, schema_bytes: u64) -> Self {
        Self {
            attachment_bytes,
            schema_bytes,
        }
    }

    #[must_use]
    pub const fn attachment_bytes(self) -> u64 {
        self.attachment_bytes
    }

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
