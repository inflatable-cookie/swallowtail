use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{CopilotCliAcpDriver, copilot_cli_acp_claim};

const SWALLOWTAIL_COPILOT_CLI_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.copilot-cli");

impl DiscoveryDriver for CopilotCliAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.copilot-cli.acp.discovery_target_required",
                "Copilot CLI ACP discovery requires one explicit host-approved executable target",
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
            copilot_cli_acp_claim(),
            crate::selection::parse_copilot_cli_version_output,
            SWALLOWTAIL_COPILOT_CLI_PROBE_CODES,
            "Copilot CLI",
        ))
    }
}
