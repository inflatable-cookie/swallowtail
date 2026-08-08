#[path = "discovery/probe.rs"]
mod probe;
use probe::{probe_acp, probe_headless};
use swallowtail_core::DiscoveryOutcome;
use swallowtail_runtime::{
    BoxFuture, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, RuntimeFailure,
};

use crate::failure::failure;
use crate::{GeminiAcpDriver, GeminiHeadlessDriver};

impl DiscoveryDriver for GeminiAcpDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.gemini.discovery_target_required",
                "Gemini discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        probe_acp(request, services)
    }
}

impl DiscoveryDriver for GeminiHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.gemini.discovery_target_required",
                "Gemini discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        probe_headless(request, services)
    }
}
