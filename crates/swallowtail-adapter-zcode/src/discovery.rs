use swallowtail_core::{DiscoveryOutcome, DiscoveryStatus, InstalledExecutableObservation};
use swallowtail_runtime::{
    BoxFuture, DebugObservationKind, DiscoveryDriver, DiscoveryRequest, HostServices,
    InstalledExecutableDiscoveryRequest, RuntimeFailure,
    validate_installed_executable_discovery_services,
};

use crate::selection::{ZCODE_RELEASE_VERSION, validate_target_payload};
use crate::{ZcodeAppServerDriver, zcode_app_server_claim, zcode_release_binding};

impl DiscoveryDriver for ZcodeAppServerDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(crate::failure::failure(
                "swallowtail.zcode.app_server.discovery_target_required",
                "ZCode discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(async move { classify_pinned_payload(request, services) })
    }
}

fn classify_pinned_payload(
    request: InstalledExecutableDiscoveryRequest,
    services: HostServices,
) -> Result<DiscoveryOutcome, RuntimeFailure> {
    validate_target_payload(request.target().executable().as_host_value())?;
    validate_installed_executable_discovery_services(&request, &services)?;
    let claim = zcode_app_server_claim();
    if request.target().version_axis() != claim.axis() {
        return Err(crate::failure::failure(
            "swallowtail.zcode.app_server.discovery_axis_mismatch",
            "ZCode discovery target uses a different version axis",
        ));
    }
    if request.cancellation().is_requested() {
        return Ok(DiscoveryOutcome::new(
            DiscoveryStatus::Cancelled,
            Some(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.zcode.app_server.discovery_cancelled",
                "ZCode installed discovery was cancelled",
            )),
        ));
    }
    let binding = zcode_release_binding(ZCODE_RELEASE_VERSION).expect("static ZCode release binds");
    let observation = InstalledExecutableObservation::classify(
        request.execution_host_id().clone(),
        binding,
        &claim,
    )
    .map_err(|_| {
        let message = "ZCode version observation could not be classified";
        services.emit_failure_debug(
            DebugObservationKind::InterfaceVersion,
            "ZCode",
            "installed_discovery.classify",
            "swallowtail.zcode.app_server.discovery_classification_failed",
            message,
        );
        crate::failure::failure(
            "swallowtail.zcode.app_server.discovery_classification_failed",
            message,
        )
    })?;
    Ok(DiscoveryOutcome::installed_executable(observation))
}

#[cfg(test)]
mod tests {
    use super::zcode_release_binding;
    use crate::ZCODE_RELEASE_VERSION;

    #[test]
    fn pin_text_is_the_discovery_version_not_launcher_or_desktop() {
        assert_eq!(
            zcode_release_binding(ZCODE_RELEASE_VERSION)
                .expect("exact runtime version binds")
                .version()
                .as_str(),
            "0.16.3"
        );
        for rejected in [
            "",
            "0.16.2",
            "3.7.7",
            "3.7.7-13",
            "zcode-app-cli 3.7.7-13",
            "0.16.3 ",
            "0.16.3\n",
        ] {
            assert!(zcode_release_binding(rejected).is_none(), "{rejected:?}");
        }
    }
}
