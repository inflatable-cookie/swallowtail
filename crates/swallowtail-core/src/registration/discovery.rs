use crate::{InstalledExecutableObservation, SafeDiagnostic};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryStatus {
    Absent,
    Discovered,
    Incompatible,
    Malformed,
    TimedOut,
    Cancelled,
    Failed,
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
    #[must_use]
    pub const fn new(status: DiscoveryStatus, diagnostic: Option<SafeDiagnostic>) -> Self {
        Self {
            status,
            installed_executable: None,
            diagnostic,
        }
    }

    #[must_use]
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
    pub const fn status(&self) -> DiscoveryStatus {
        self.status
    }

    #[must_use]
    pub const fn installed_executable_observation(
        &self,
    ) -> Option<&InstalledExecutableObservation> {
        self.installed_executable.as_ref()
    }

    #[must_use]
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}
