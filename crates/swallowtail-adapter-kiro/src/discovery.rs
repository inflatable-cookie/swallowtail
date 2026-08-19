use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{KiroAcpDriver, kiro_acp_claim};

const SWALLOWTAIL_KIRO_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.kiro");

impl DiscoveryDriver for KiroAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.kiro.acp.discovery_target_required",
                "Kiro ACP discovery requires one explicit host-approved executable target",
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
            kiro_acp_claim(),
            crate::selection::parse_kiro_cli_version_output,
            SWALLOWTAIL_KIRO_PROBE_CODES,
            "Kiro",
        ))
    }
}
