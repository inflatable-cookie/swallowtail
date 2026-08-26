use swallowtail_core::{DiscoveryOutcome, InterfaceVersionBinding};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::failure::failure;
use crate::{PiRpcDriver, pi_package_binding, pi_rpc_claim};

const PI_PROBE_CODES: InstalledProbeCodes = installed_probe_codes!("swallowtail.pi");

impl DiscoveryDriver for PiRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.pi.discovery_target_required",
                "Pi discovery requires one explicit host-approved executable target",
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
            pi_rpc_claim(),
            parse_version,
            PI_PROBE_CODES,
            "Pi",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    pi_package_binding(value)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_bare_pi_semver() {
        for candidate in ["0.80.10", "0.83.0", "0.84.2", "0.84.3"] {
            assert_eq!(
                parse_version(format!("{candidate}\n").as_bytes())
                    .expect("version parses")
                    .version()
                    .as_str(),
                candidate
            );
        }
        for output in [
            b"pi 0.80.10".as_slice(),
            b"0.80.10 extra".as_slice(),
            b" 0.80.10\n".as_slice(),
            b"0.80.10\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
