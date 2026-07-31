use std::ffi::OsString;
use std::fmt;
use std::path::{Path, PathBuf};

pub(crate) const MAXIMUM_BOOTSTRAP_ENVIRONMENT_BINDINGS: usize = 32;
pub(crate) const MAXIMUM_BOOTSTRAP_ENVIRONMENT_BYTES: usize = 16 * 1024;

/// One host-approved native or interpreted process launch.
///
/// The launch stays behind an opaque `ExecutableRef`. Its program, immutable
/// prefix arguments, and bootstrap environment are never portable runtime
/// records.
#[derive(Clone)]
pub struct LocalExecutableLaunch {
    program: PathBuf,
    prefix_arguments: Vec<OsString>,
    bootstrap_environment: Vec<(OsString, OsString)>,
}

impl LocalExecutableLaunch {
    /// Creates the zero-prefix, empty-bootstrap native launch shape.
    #[must_use]
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            prefix_arguments: Vec::new(),
            bootstrap_environment: Vec::new(),
        }
    }

    /// Fixes arguments that the host places before driver-owned arguments.
    #[must_use]
    pub fn with_prefix_arguments(mut self, arguments: impl IntoIterator<Item = OsString>) -> Self {
        self.prefix_arguments = arguments.into_iter().collect();
        self
    }

    /// Fixes launcher-only environment applied after ambient environment is
    /// cleared and before explicit request environment.
    #[must_use]
    pub fn with_bootstrap_environment(
        mut self,
        values: impl IntoIterator<Item = (OsString, OsString)>,
    ) -> Self {
        self.bootstrap_environment = values.into_iter().collect();
        self
    }

    pub(crate) fn program(&self) -> &Path {
        &self.program
    }

    pub(crate) fn prefix_arguments(&self) -> &[OsString] {
        &self.prefix_arguments
    }

    pub(crate) fn bootstrap_environment(&self) -> &[(OsString, OsString)] {
        &self.bootstrap_environment
    }
}

impl fmt::Debug for LocalExecutableLaunch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalExecutableLaunch")
            .field("program", &"<redacted>")
            .field("prefix_argument_count", &self.prefix_arguments.len())
            .field(
                "bootstrap_environment_count",
                &self.bootstrap_environment.len(),
            )
            .finish()
    }
}
