use swallowtail_core::{DiscoveryOutcome, InstallGuidance};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::claude_code::ClaudeCodeHeadlessDriver;
use crate::failure::failure;

const SWALLOWTAIL_CLAUDE_CODE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.claude_code");

// Vendor source: https://code.claude.com/docs/en/getting-started (frozen 2026-09-06).
// This text is descriptive only; discovery never executes the install command.
pub(crate) const CLAUDE_CODE_INSTALL_GUIDANCE: InstallGuidance = InstallGuidance::new(
    "Claude Code",
    "curl -fsSL https://claude.ai/install.sh | bash",
    "https://code.claude.com/docs/en/getting-started",
    "2026-09-06",
);

pub(crate) fn attach_install_guidance(outcome: DiscoveryOutcome) -> DiscoveryOutcome {
    outcome.with_install_guidance(CLAUDE_CODE_INSTALL_GUIDANCE)
}

/// Extracts the version from `claude --version` output for one route binding.
///
/// Both routes read the same `"<version> (Claude Code)\n"` shape and differ
/// only in the claim that judges the version, so the strict shape check lives
/// here once and each route passes its own binding parser.
pub(crate) fn parse_version_output(
    output: &[u8],
    binding: impl FnOnce(&str) -> Option<swallowtail_core::InterfaceVersionBinding>,
) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let output = output.strip_suffix('\n').unwrap_or(output);
    binding(output.strip_suffix(" (Claude Code)")?)
}

impl ClaudeCodeHeadlessDriver {
    #[must_use]
    /// Returns the descriptive vendor guidance used for an absent executable.
    pub const fn install_guidance() -> InstallGuidance {
        CLAUDE_CODE_INSTALL_GUIDANCE
    }
}

impl DiscoveryDriver for ClaudeCodeHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.claude_code.discovery_target_required",
                "Claude Code discovery requires one explicit host-approved executable target",
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
                crate::claude_code_headless_claim(),
                parse_version,
                SWALLOWTAIL_CLAUDE_CODE_PROBE_CODES,
                "Claude Code",
            )
            .await
            .map(attach_install_guidance)
        })
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    parse_version_output(output, crate::claude_code_headless_binding)
}

#[cfg(test)]
mod tests {
    use super::{CLAUDE_CODE_INSTALL_GUIDANCE, attach_install_guidance, parse_version};
    use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus};

    #[test]
    fn absent_guidance_is_frozen_and_present_outcomes_have_none() {
        let absent = attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Absent, None));
        let guidance = absent.install_guidance().expect("absent guidance");
        assert_eq!(guidance, &CLAUDE_CODE_INSTALL_GUIDANCE);
        assert_eq!(
            super::ClaudeCodeHeadlessDriver::install_guidance(),
            CLAUDE_CODE_INSTALL_GUIDANCE
        );
        assert_eq!(guidance.display_name(), "Claude Code");
        assert_eq!(
            guidance.command(),
            "curl -fsSL https://claude.ai/install.sh | bash"
        );
        assert_eq!(
            guidance.documentation_url(),
            "https://code.claude.com/docs/en/getting-started"
        );
        assert_eq!(guidance.frozen_on(), "2026-09-06");

        let present =
            attach_install_guidance(DiscoveryOutcome::new(DiscoveryStatus::Discovered, None));
        assert!(present.install_guidance().is_none());
    }

    #[test]
    fn parser_accepts_only_claude_code_version_output() {
        assert_eq!(
            parse_version(b"2.1.220 (Claude Code)\n")
                .expect("version parses")
                .version()
                .as_str(),
            "2.1.220"
        );
        for output in [
            b"2.1.220".as_slice(),
            b"Claude Code 2.1.220".as_slice(),
            b" 2.1.220 (Claude Code)\n".as_slice(),
            b"2.1.220 (Claude Code)\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
