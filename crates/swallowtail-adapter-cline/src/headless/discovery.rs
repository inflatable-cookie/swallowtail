use crate::{ClineHeadlessDriver, cline_headless_claim};
use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

const SWALLOWTAIL_CLINE_HEADLESS_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.cline.headless");

impl DiscoveryDriver for ClineHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.cline.headless.discovery_target_required",
                "Cline headless discovery requires one explicit host-approved executable target",
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
            cline_headless_claim(),
            crate::selection::parse_cline_version_output,
            SWALLOWTAIL_CLINE_HEADLESS_PROBE_CODES,
            "Cline",
        ))
    }
}
