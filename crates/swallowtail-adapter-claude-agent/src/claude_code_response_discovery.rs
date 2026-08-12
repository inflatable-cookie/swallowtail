use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::claude_code_response::ClaudeCodeResponseOnlyDriver;
use crate::failure::failure;

const PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.claude_code.response_only");

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
        Box::pin(probe_installed_executable_version(
            request,
            services,
            crate::claude_code_response_only_claim(),
            parse_version,
            PROBE_CODES,
            "Claude Code response-only",
        ))
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
    use super::parse_version;

    #[test]
    fn parser_accepts_only_exact_response_only_version_output() {
        assert_eq!(
            parse_version(b"2.1.228 (Claude Code)\n")
                .expect("version parses")
                .version()
                .as_str(),
            "2.1.228"
        );
        for output in [
            b"2.1.220 (Claude Code)\n".as_slice(),
            b"2.1.227 (Claude Code)\n".as_slice(),
            b"2.1.228".as_slice(),
            b"Claude Code 2.1.228\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
