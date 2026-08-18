use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{
    MUSE_CODE_RELEASE_REVISION, MuseHeadlessDriver, muse_code_release_binding, muse_headless_claim,
};

const SWALLOWTAIL_MUSE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.muse");
impl DiscoveryDriver for MuseHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.muse_code.discovery_target_required",
                "Muse Code discovery requires one explicit signed payload target",
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
            muse_headless_claim(),
            parse_version,
            SWALLOWTAIL_MUSE_PROBE_CODES,
            "Muse",
        ))
    }
}

fn parse_version(output: &[u8]) -> Option<swallowtail_core::InterfaceVersionBinding> {
    let output = std::str::from_utf8(output).ok()?;
    let exact = output.strip_suffix('\n').unwrap_or(output);
    let rest = exact.strip_prefix("Muse Code ")?;
    let (_display, revision_part) = rest.split_once(" (")?;
    let revision = revision_part.strip_suffix(')')?;
    if revision != MUSE_CODE_RELEASE_REVISION {
        return None;
    }
    muse_code_release_binding(revision)
}

#[cfg(test)]
mod tests {
    use super::parse_version;

    #[test]
    fn parser_requires_the_exact_direct_payload_version_line() {
        assert_eq!(
            parse_version(b"Muse Code 0.2.1 (0.2.1-R1215.1)\n")
                .expect("exact version parses")
                .version()
                .as_str(),
            "0.2.1-R1215.1"
        );
        for rejected in [
            b"Muse Code 0.1.0 (0.1.0-R708.1)\n".as_slice(),
            b"Muse Code 0.2.1 (0.2.1-R1215.2)\n".as_slice(),
            b"muse 0.2.1-R1215.1\n".as_slice(),
            b"Muse Code 0.2.1 (0.2.1-R1215.1) extra\n".as_slice(),
        ] {
            assert!(parse_version(rejected).is_none());
        }
    }
}
