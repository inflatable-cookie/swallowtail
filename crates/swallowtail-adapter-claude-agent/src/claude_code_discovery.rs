use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::claude_code::ClaudeCodeHeadlessDriver;
use crate::failure::failure;

const SWALLOWTAIL_CLAUDE_CODE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.claude_code");
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
        Box::pin(probe_installed_executable_version(
            request,
            services,
            crate::claude_code_headless_claim(),
            parse_version,
            SWALLOWTAIL_CLAUDE_CODE_PROBE_CODES,
            "Claude Code",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let output = output.strip_suffix('\n').unwrap_or(output);
    let version = output.strip_suffix(" (Claude Code)")?;
    crate::claude_code_headless_binding(version)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

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
