use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::claude_code_discovery::CLAUDE_CODE_INSTALL_GUIDANCE;
use crate::claude_code_discovery::attach_install_guidance;
use crate::claude_code_response::ClaudeCodeResponseOnlyDriver;
use crate::failure::failure;

const PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.claude_code.response_only");

impl ClaudeCodeResponseOnlyDriver {
    #[must_use]
    /// Returns the descriptive vendor guidance used for an absent executable.
    pub const fn install_guidance() -> swallowtail_core::InstallGuidance {
        CLAUDE_CODE_INSTALL_GUIDANCE
    }
}

impl DiscoveryDriver for ClaudeCodeResponseOnlyDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.claude_code.response_only.discovery_target_required",
                "Claude Code response-only discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(async move {
            probe_installed_executable_version(
                request,
                services,
                crate::claude_code_response_only_claim(),
                parse_version,
                PROBE_CODES,
                "Claude Code response-only",
            )
            .await
            .map(attach_install_guidance)
        })
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let output = output.strip_suffix('\n').unwrap_or(output);
    let version = output.strip_suffix(" (Claude Code)")?;
    crate::claude_code_response_only_binding(version)
}

#[cfg(test)]
mod tests {
    use super::{CLAUDE_CODE_INSTALL_GUIDANCE, parse_version};
    use crate::claude_code_discovery::attach_install_guidance;
    use crate::claude_code_response::ClaudeCodeResponseOnlyDriver;
    use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus};

    #[test]
    fn absent_guidance_is_frozen_and_present_outcomes_have_none() {
        let absent = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Absent, None));
        assert_eq!(
            absent.install_guidance(),
            Some(&CLAUDE_CODE_INSTALL_GUIDANCE)
        );
        assert_eq!(
            ClaudeCodeResponseOnlyDriver::install_guidance(),
            CLAUDE_CODE_INSTALL_GUIDANCE
        );
        let present =
            attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Discovered, None));
        assert!(present.install_guidance().is_none());
    }

    #[test]
    fn parser_accepts_only_strict_stable_version_output() {
        assert_eq!(
            parse_version(b"2.1.229 (Claude Code)\n")
                .expect("version parses")
                .version()
                .as_str(),
            "2.1.229"
        );
        for output in [
            b"2.1.228".as_slice(),
            b"Claude Code 2.1.228\n".as_slice(),
            b"2.1.229-rc.1 (Claude Code)\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
