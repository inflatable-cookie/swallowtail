use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version as probe,
};

use crate::failure::failure;
use crate::{KimiAcpDriver, kimi_acp_claim, kimi_code_binding};

pub(crate) const KIMI_PROBE_CODES: InstalledProbeCodes = installed_probe_codes!("swallowtail.kimi");

impl DiscoveryDriver for KimiAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.kimi.discovery_target_required",
                "Kimi discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(probe(
            request,
            services,
            kimi_acp_claim(),
            parse_version,
            KIMI_PROBE_CODES,
            "Kimi",
        ))
    }
}

pub(crate) fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    kimi_code_binding(value)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_one_exact_kimi_semver() {
        assert_eq!(
            parse_version(b"0.29.0\n")
                .expect("version parses")
                .version()
                .as_str(),
            "0.29.0"
        );
        for output in [
            b"kimi 0.29.0".as_slice(),
            b"0.29.0 extra".as_slice(),
            b" 0.29.0\n".as_slice(),
            b"0.29.0\n\n".as_slice(),
            b"latest".as_slice(),
            b"private payload".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
