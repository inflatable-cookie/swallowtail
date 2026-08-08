use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices, InstalledProbeCodes,
    InstalledExecutableDiscoveryRequest, RuntimeFailure, installed_probe_codes,
    probe_installed_executable_version,
};

use crate::failure::failure;
use crate::{ClaudeAgentAcpDriver, claude_agent_acp_binding, claude_agent_acp_claim};


const SWALLOWTAIL_CLAUDE_AGENT_PROBE_CODES: InstalledProbeCodes = installed_probe_codes!("swallowtail.claude_agent");
impl DiscoveryDriver for ClaudeAgentAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.claude_agent.discovery_target_required",
                "Claude Agent discovery requires one explicit host-approved executable target",
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
            claude_agent_acp_claim(),
            parse_version,
            SWALLOWTAIL_CLAUDE_AGENT_PROBE_CODES,
            "Claude Agent",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    claude_agent_acp_binding(value)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_the_bare_wrapper_semver() {
        assert_eq!(
            parse_version(b"0.61.0\n")
                .expect("version parses")
                .version()
                .as_str(),
            "0.61.0"
        );
        for output in [
            b"claude-agent-acp 0.61.0".as_slice(),
            b"0.61.0 extra".as_slice(),
            b" 0.61.0\n".as_slice(),
            b"0.61.0\n\n".as_slice(),
            b"latest".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
