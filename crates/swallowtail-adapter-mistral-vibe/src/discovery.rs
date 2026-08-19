use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, InstalledProbeCodes, RuntimeFailure,
    installed_probe_codes, probe_installed_executable_version,
};

use crate::{MistralVibeHeadlessDriver, mistral_vibe_headless_claim};

const SWALLOWTAIL_MISTRAL_VIBE_PROBE_CODES: InstalledProbeCodes =
    installed_probe_codes!("swallowtail.mistral-vibe.headless");

impl DiscoveryDriver for MistralVibeHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.mistral-vibe.headless.discovery_target_required",
                "Mistral Vibe headless discovery requires one explicit host-approved executable target",
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
            mistral_vibe_headless_claim(),
            crate::selection::parse_vibe_version_output,
            SWALLOWTAIL_MISTRAL_VIBE_PROBE_CODES,
            "Mistral Vibe",
        ))
    }
}
