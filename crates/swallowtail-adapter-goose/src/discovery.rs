use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{GooseAcpDriver, goose_acp_claim};

const SWALLOWTAIL_GOOSE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.goose");

impl DiscoveryDriver for GooseAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.goose.acp.discovery_target_required",
                "Goose ACP discovery requires one explicit host-approved executable target",
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
            goose_acp_claim(),
            crate::selection::parse_goose_version_output,
            SWALLOWTAIL_GOOSE_PROBE_CODES,
            "Goose",
        ))
    }
}
