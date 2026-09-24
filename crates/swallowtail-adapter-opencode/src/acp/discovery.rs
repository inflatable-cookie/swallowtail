use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use super::{OpenCodeAcpDriver, opencode_acp_claim};

const SWALLOWTAIL_OPENCODE_ACP_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.opencode.acp");

impl DiscoveryDriver for OpenCodeAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(super::failure::failure(
                "swallowtail.opencode.acp.discovery_target_required",
                "OpenCode ACP discovery requires one explicit host-approved executable target",
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
            opencode_acp_claim(),
            super::selection::parse_opencode_acp_version_output,
            SWALLOWTAIL_OPENCODE_ACP_PROBE_CODES,
            "OpenCode",
        ))
    }
}
