use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::selection::target_is_exact;
use crate::{
    DeepSeekHarnessJsonRpcDriver, deepseek_harness_jsonrpc_claim, deepseek_harness_release_binding,
};

const SWALLOWTAIL_DEEPSEEK_HARNESS_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.deepseek_harness");

impl DiscoveryDriver for DeepSeekHarnessJsonRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.deepseek_harness.discovery_target_required",
                "DeepSeek Harness discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        if !target_is_exact(request.target().executable().as_host_value()) {
            return Box::pin(async {
                Err(crate::failure::failure(
                    "swallowtail.deepseek_harness.target_not_pinned",
                    "DeepSeek Harness discovery requires the exact packaged runtime target",
                ))
            });
        }
        Box::pin(probe_installed_executable_version(
            request,
            services,
            deepseek_harness_jsonrpc_claim(),
            parse_version,
            SWALLOWTAIL_DEEPSEEK_HARNESS_PROBE_CODES,
            "DeepSeek Harness",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    deepseek_harness_release_binding(exact)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_requires_the_exact_runtime_bin_release_line() {
        assert_eq!(
            parse_version(b"0.1.0rc6\n")
                .expect("exact runtime version parses")
                .version()
                .as_str(),
            "0.1.0rc6"
        );
        assert_eq!(
            parse_version(b"0.1.0rc6")
                .expect("exact runtime version without newline parses")
                .version()
                .as_str(),
            "0.1.0rc6"
        );
        for rejected in [
            b"0.1.0rc5\n".as_slice(),
            b"0.1.0rc7\n".as_slice(),
            b"dsh-jsonrpc-agent 0.1.0rc6\n".as_slice(),
            b"0.1.0rc6 \n".as_slice(),
            b"0.1.0rc6\n\n".as_slice(),
            b"".as_slice(),
        ] {
            assert!(parse_version(rejected).is_none());
        }
    }
}
