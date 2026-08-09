use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{CommandCodeHeadlessDriver, command_code_headless_claim, command_code_release_binding};

const SWALLOWTAIL_COMMAND_CODE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.command_code");

impl DiscoveryDriver for CommandCodeHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.command_code.discovery_target_required",
                "Command Code discovery requires one explicit host-approved executable target",
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
            command_code_headless_claim(),
            parse_version,
            SWALLOWTAIL_COMMAND_CODE_PROBE_CODES,
            "Command Code",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    command_code_release_binding(exact)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_requires_the_exact_release_line_and_tolerates_one_trailing_newline() {
        assert_eq!(
            parse_version(b"1.15.1\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "1.15.1"
        );
        assert_eq!(
            parse_version(b"1.15.1")
                .expect("exact version without newline parses")
                .version()
                .as_str(),
            "1.15.1"
        );
        for rejected in [
            b"1.15.2\n".as_slice(),
            b"command-code 1.15.1\n".as_slice(),
            b"1.15.1 \n".as_slice(),
            b"1.15.1\n\n".as_slice(),
            b" 1.15.1\n".as_slice(),
            b"".as_slice(),
        ] {
            assert!(parse_version(rejected).is_none());
        }
    }
}
