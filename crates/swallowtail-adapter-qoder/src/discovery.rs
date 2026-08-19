use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{QoderHeadlessDriver, qoder_headless_claim};

const SWALLOWTAIL_QODER_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.qoder.headless");

impl DiscoveryDriver for QoderHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.qoder.headless.discovery_target_required",
                "Qoder headless discovery requires one explicit host-approved executable target",
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
            qoder_headless_claim(),
            crate::selection::parse_qoder_version_output,
            SWALLOWTAIL_QODER_PROBE_CODES,
            "Qoder",
        ))
    }
}
