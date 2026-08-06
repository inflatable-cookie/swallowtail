use crate::{InstalledExecutableObservation, SafeDiagnostic};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Terminal state of one bounded discovery attempt.
pub enum DiscoveryStatus {
    /// No candidate was found.
    Absent,
    /// A compatible candidate was observed.
    Discovered,
    /// A candidate was found but is outside supported compatibility.
    Incompatible,
    /// Candidate output could not be safely interpreted.
    Malformed,
    /// Discovery exceeded its deadline.
    TimedOut,
    /// Discovery was cancelled before completion.
    Cancelled,
    /// Discovery failed for another safe, diagnosed reason.
    Failed,
    /// Discovery completed but owned cleanup failed.
    CleanupFailed,
}

/// Safe discovery result. It never promotes a candidate into configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveryOutcome {
    status: DiscoveryStatus,
    installed_executable: Option<InstalledExecutableObservation>,
    diagnostic: Option<SafeDiagnostic>,
}

impl DiscoveryOutcome {
    /// Creates a discovery outcome without executable-version evidence.
    #[must_use]
    pub const fn new(status: DiscoveryStatus, diagnostic: Option<SafeDiagnostic>) -> Self {
        Self {
            status,
            installed_executable: None,
            diagnostic,
        }
    }

    #[must_use]
    /// Creates a discovered or incompatible result from executable evidence.
    pub fn installed_executable(observation: InstalledExecutableObservation) -> Self {
        let status = if observation.is_permitted() {
            DiscoveryStatus::Discovered
        } else {
            DiscoveryStatus::Incompatible
        };
        Self {
            status,
            installed_executable: Some(observation),
            diagnostic: None,
        }
    }

    #[must_use]
    /// Returns the terminal discovery status.
    pub const fn status(&self) -> DiscoveryStatus {
        self.status
    }

    #[must_use]
    /// Returns exact installed-executable evidence, when observed.
    pub const fn installed_executable_observation(
        &self,
    ) -> Option<&InstalledExecutableObservation> {
        self.installed_executable.as_ref()
    }

    #[must_use]
    /// Returns the redacted discovery diagnostic, when supplied.
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}
