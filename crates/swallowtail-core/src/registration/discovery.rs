use crate::{InstalledExecutableObservation, SafeDiagnostic};

/// Vendor-sourced, descriptive installation text for an absent harness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InstallGuidance {
    display_name: &'static str,
    command: &'static str,
    documentation_url: &'static str,
    frozen_on: &'static str,
}

impl InstallGuidance {
    /// Creates text-only install guidance frozen from a vendor documentation page.
    #[must_use]
    pub const fn new(
        display_name: &'static str,
        command: &'static str,
        documentation_url: &'static str,
        frozen_on: &'static str,
    ) -> Self {
        Self {
            display_name,
            command,
            documentation_url,
            frozen_on,
        }
    }

    #[must_use]
    /// Returns the vendor harness display name.
    pub const fn display_name(self) -> &'static str {
        self.display_name
    }

    #[must_use]
    /// Returns the vendor's recommended installation command.
    pub const fn command(self) -> &'static str {
        self.command
    }

    #[must_use]
    /// Returns the vendor documentation URL used as the source.
    pub const fn documentation_url(self) -> &'static str {
        self.documentation_url
    }

    #[must_use]
    /// Returns the ISO date on which this guidance was frozen.
    pub const fn frozen_on(self) -> &'static str {
        self.frozen_on
    }
}

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
    install_guidance: Option<InstallGuidance>,
}

impl DiscoveryOutcome {
    /// Creates a discovery outcome without executable-version evidence.
    #[must_use]
    pub const fn new(status: DiscoveryStatus, diagnostic: Option<SafeDiagnostic>) -> Self {
        Self {
            status,
            installed_executable: None,
            diagnostic,
            install_guidance: None,
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
            install_guidance: None,
        }
    }

    #[must_use]
    /// Adds descriptive install guidance only to an absent outcome.
    pub const fn with_install_guidance(mut self, guidance: InstallGuidance) -> Self {
        if matches!(self.status, DiscoveryStatus::Absent) {
            self.install_guidance = Some(guidance);
        }
        self
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

    #[must_use]
    /// Returns descriptive install guidance for an absent harness, when supplied.
    pub const fn install_guidance(&self) -> Option<&InstallGuidance> {
        self.install_guidance.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{DiscoveryOutcome, DiscoveryStatus, InstallGuidance};

    #[test]
    fn install_guidance_is_limited_to_absent_outcomes() {
        let guidance = InstallGuidance::new(
            "Fixture Harness",
            "fixture install command",
            "https://vendor.example/install",
            "2026-09-06",
        );
        let absent =
            DiscoveryOutcome::new(DiscoveryStatus::Absent, None).with_install_guidance(guidance);
        assert_eq!(absent.install_guidance(), Some(&guidance));
        assert_eq!(
            absent.install_guidance().unwrap().command(),
            "fixture install command"
        );

        let present = DiscoveryOutcome::new(DiscoveryStatus::Discovered, None)
            .with_install_guidance(guidance);
        assert!(present.install_guidance().is_none());
    }
}
