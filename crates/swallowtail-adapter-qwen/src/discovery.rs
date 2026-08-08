use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::validation::failure;
use crate::{QwenHeadlessDriver, qwen_code_binding, qwen_headless_claim};

const QWEN_PROBE_CODES: InstalledProbeCodes = installed_probe_codes!("swallowtail.qwen");

impl DiscoveryDriver for QwenHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.qwen.discovery_target_required",
                "Qwen discovery requires one explicit host-approved executable target",
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
            qwen_headless_claim(),
            parse_version,
            QWEN_PROBE_CODES,
            "Qwen",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    if value.bytes().any(|byte| byte.is_ascii_whitespace()) {
        return None;
    }
    qwen_code_binding(value)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_bare_qwen_semver() {
        assert_eq!(
            parse_version(b"0.19.11\n")
                .expect("version parses")
                .version()
                .as_str(),
            "0.19.11"
        );
        for output in [
            b"qwen 0.19.11".as_slice(),
            b"0.19.11 extra".as_slice(),
            b" 0.19.11\n".as_slice(),
            b"0.19.11\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
