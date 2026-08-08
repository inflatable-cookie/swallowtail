use swallowtail_core::{DiscoveryOutcome, InterfaceVersionBinding};
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::failure::failure;
use crate::{OhMyPiRpcDriver, oh_my_pi_package_binding, oh_my_pi_rpc_claim};

const OH_MY_PI_PROBE_CODES: InstalledProbeCodes = installed_probe_codes!("swallowtail.oh_my_pi");

impl DiscoveryDriver for OhMyPiRpcDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.oh_my_pi.discovery_target_required",
                "OhMyPi discovery requires one explicit host-approved executable target",
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
            oh_my_pi_rpc_claim(),
            parse_version,
            OH_MY_PI_PROBE_CODES,
            "OhMyPi",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let value = output.strip_suffix('\n').unwrap_or(output);
    oh_my_pi_package_binding(value.strip_prefix("omp/")?)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_accepts_only_the_exact_omp_version_banner() {
        for candidate in ["17.2.9", "17.3.0"] {
            assert_eq!(
                parse_version(format!("omp/{candidate}\n").as_bytes())
                    .expect("version parses")
                    .version()
                    .as_str(),
                candidate
            );
        }
        for output in [
            b"17.2.9".as_slice(),
            b"omp 17.2.9".as_slice(),
            b"pi/17.2.9".as_slice(),
            b"17.2.9 extra".as_slice(),
            b" omp/17.2.9\n".as_slice(),
            b"omp/17.2.9\n\n".as_slice(),
        ] {
            assert!(parse_version(output).is_none());
        }
    }
}
